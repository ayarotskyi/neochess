import { statToColor } from '@/common';
import { useAnalyzerStore } from '@/store/analyzer';
import { MoveResult, useGameStore } from '@/store/game';
import {
  Box,
  Circle,
  HStack,
  Text,
  VStack,
  type StackProps,
} from '@chakra-ui/react';
import { Chess } from 'chessops';
import { parseFen } from 'chessops/fen';
import { makeSan } from 'chessops/san';
import { useCallback, useMemo } from 'react';
import { useShallow } from 'zustand/shallow';

type Props = StackProps & {
  statIndex: number;
};

const MoveStatistics = ({ statIndex, ...props }: Props) => {
  const stat = useAnalyzerStore(
    useShallow((state) => state.moveStatistics![statIndex]),
  );
  const fen = useGameStore((state) => state.fen);
  const hovered = stat.hovered;
  const color = statToColor(stat);
  const san = useMemo(
    () => makeSan(Chess.fromSetup(parseFen(fen).unwrap()).unwrap(), stat.move),
    [fen, stat.move],
  );

  const setHoveredByIndex = useAnalyzerStore((state) => state.setHovered);
  const setHovered = useCallback(
    (hovered: boolean) => setHoveredByIndex(statIndex, hovered),
    [setHoveredByIndex, statIndex],
  );

  const play = useGameStore((state) => state.play);
  const unselectSquare = useGameStore((state) => state.unselectSquare);
  const onClick = useCallback(() => {
    const moveResult = play(stat.move);
    if (moveResult === MoveResult.Success) {
      unselectSquare();
    }
  }, [play, stat.move, unselectSquare]);

  return (
    <VStack
      bg="rgba(31, 41, 55, 0.5)"
      border={`1px solid ${hovered ? 'oklch(62.7% .265 303.9)' : 'rgba(55, 65, 81, 0.5)'}`}
      cursor="pointer"
      padding="13px"
      spaceY="8px"
      align="stretch"
      userSelect="none"
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
      onClick={onClick}
      {...props}
    >
      <HStack>
        <Text
          flex={1}
          textStyle="mono"
          fontWeight={700}
          fontSize="16px"
          lineHeight="24px"
          color="#FFFFFF"
        >
          {san}
        </Text>
        <Circle size="12px" bg={color} boxShadow={`0px 0px 10px ${color}`} />
      </HStack>
      <HStack spaceX="8px">
        <Box
          bg="rgba(8, 145, 178, 0.2)"
          border="1px solid rgba(6, 182, 212, 0.5)"
          borderRadius={9999}
          py="3px"
          px="11px"
        >
          <Text
            textStyle="default"
            fontWeight={600}
            fontSize="12px"
            lineHeight="16px"
            color="#67E8F9"
          >
            {(stat.playRate * 100).toFixed(1)}% played
          </Text>
        </Box>
        <Box
          bg="rgba(22, 163, 74, 0.2)"
          border="1px solid rgba(34, 197, 94, 0.5)"
          borderRadius={9999}
          py="3px"
          px="11px"
        >
          <Text
            textStyle="default"
            fontWeight={600}
            fontSize="12px"
            lineHeight="16px"
            color="#86EFAC"
          >
            {(stat.winRate * 100).toFixed(1)}% win
          </Text>
        </Box>
      </HStack>
    </VStack>
  );
};

export default MoveStatistics;
