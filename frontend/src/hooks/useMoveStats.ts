import { gql } from '@/__generated__';
import { Color } from '@/__generated__/graphql';
import { toaster } from '@/components/ui/toaster';
import ParamsContext from '@/contexts/ParamsContext';
import { useAnalyzerStore } from '@/store/analyzer';
import { useGameStore } from '@/store/game';
import { usePositionStore, type PositionStore } from '@/store/position';
import { useQuery } from '@apollo/client';
import { parseUci } from 'chessops';
import { useContext, useEffect } from 'react';
import { useShallow } from 'zustand/shallow';

const GET_MOVE_STATS = gql(`
  query GetMoveStats(
    $positionFen: String!,
    $username: String!,
    $playAs: Color!,
    $platformName: PlatformName!,
    $fromTimestampSeconds: Int,
    $toTimestampSeconds: Int
  ) {
    getMoveStats(
      positionFen: $positionFen,
      username: $username,
      playAs: $playAs,
      platformName: $platformName,
      fromTimestampSeconds: $fromTimestampSeconds,
      toTimestampSeconds: $toTimestampSeconds,
    ) {
      moveUci
      avgOpponentElo
      wins
      total
      draws
    }
  }
`);

const useMoveStats = () => {
  const { positionFen, playAs } = useGameStore(
    useShallow((state) => ({
      positionFen:
        state.fenStack[state.fenStack.length - 1 - state.backtrackStep],
      playAs: state.playAs,
    })),
  );

  const { fromTimestampSeconds, toTimestampSeconds } = useAnalyzerStore(
    useShallow((state) => ({
      fromTimestampSeconds: state.timeRange.fromUnix,
      toTimestampSeconds: state.timeRange.toUnix,
    })),
  );
  const { platformName, username } = useContext(ParamsContext);

  const { data, loading, error } = useQuery(GET_MOVE_STATS, {
    variables: {
      positionFen,
      playAs: playAs === 'white' ? Color.White : Color.Black,
      platformName,
      username,
      fromTimestampSeconds,
      toTimestampSeconds,
    },
  });

  const updateStats = useAnalyzerStore((state) => state.updateStats);
  const setPositionStatistics = usePositionStore(
    (state) => state.setStatistics,
  );

  useEffect(() => {
    if (loading) {
      updateStats(null);
      setPositionStatistics(null);
      return;
    }

    if (error) {
      console.error(error);
      toaster.create({
        title: 'Error fetching moves for current position',
        type: 'error',
      });
      return;
    }

    const totalGames =
      data?.getMoveStats.reduce((acc, val) => acc + val.total, 0) ?? 0;

    const stats = data?.getMoveStats.map((stat) => ({
      move: parseUci(stat.moveUci)!,
      playRate: stat.total / totalGames,
      winRate: stat.wins / stat.total,
    }));
    stats?.sort((a, b) => b.playRate - a.playRate);

    updateStats(stats || null);

    setPositionStatistics(
      data?.getMoveStats.length
        ? data.getMoveStats.reduce(
            (acc, value) => ({
              totalGames: acc.totalGames + value.total,
              wins: acc.wins + value.wins,
              lastPlayedUnix: acc.lastPlayedUnix,
              avgOpponentElo:
                acc.avgOpponentElo +
                (value.avgOpponentElo * value.total) / totalGames,
            }),
            {
              totalGames: 0,
              wins: 0,
              avgOpponentElo: 0,
              lastPlayedUnix: 0,
            } as NonNullable<PositionStore['statistics']>,
          )
        : null,
    );
  }, [data, error, loading, setPositionStatistics, updateStats]);
};

export default useMoveStats;
