import { Flex, type FlexProps } from '@chakra-ui/react';
import { makeUci, RANK_NAMES } from 'chessops';
import { useGameStore } from '@/store/game';
import RankComponent from './RankComponent';
import PromotionPopover from './PromotionPopover';
import { useAnalyzerStore } from '@/store/analyzer';
import Arrow from './Arrow';
import { stringToColor } from '@/common';

const Arrows = () => {
  const statistics = useAnalyzerStore((state) => state.moveStatistics);

  return statistics.map((stat) => {
    const uci = makeUci(stat.move);
    return (
      <Arrow
        move={stat.move}
        key={uci}
        color={stringToColor(uci + stat.playRate + stat.winRate)}
        opacity={0.3 + 0.6 * stat.winRate}
        scale={0.5 + 0.5 * stat.playRate}
      />
    );
  });
};

const Squares = () => {
  const playAs = useGameStore((state) => state.playAs);

  return (
    <Flex
      aspectRatio={1}
      h="100%"
      maxW="100%"
      flexDir={playAs === 'white' ? 'column-reverse' : 'column'}
      align="center"
      bg="rgba(255, 255, 255, 0.002)"
      border="2px solid rgba(6, 182, 212, 0.5)"
      boxShadow="0px 0px 40px rgba(0, 255, 255, 0.2)"
      position="relative"
    >
      {new Array(RANK_NAMES.length).fill(null).map((_, rank) => (
        <RankComponent rank={rank} key={rank} />
      ))}
      <Arrows />
      <PromotionPopover />
    </Flex>
  );
};

const Board = (props: FlexProps) => {
  return (
    <Flex
      {...props}
      flex={1}
      maxH="100%"
      aspectRatio={1}
      justify="center"
      align="center"
    >
      <Squares />
    </Flex>
  );
};

export default Board;
