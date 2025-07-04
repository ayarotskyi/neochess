import { stringToColor } from '@/utils';
import {
  Box,
  Circle,
  HStack,
  Text,
  VStack,
  type StackProps,
} from '@chakra-ui/react';

export type MoveStatisticsType = {
  notation: string;
  playRate: number;
  winRate: number;
};

type Props = StackProps & {
  move: MoveStatisticsType;
};

const MoveStatistics = ({ move, ...props }: Props) => {
  const color = stringToColor(move.notation + move.playRate + move.winRate);
  return (
    <VStack
      bg="rgba(31, 41, 55, 0.5)"
      border="1px solid rgba(55, 65, 81, 0.5)"
      padding="13px"
      spaceY="8px"
      align="stretch"
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
          {move.notation}
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
            {Math.round(move.playRate * 100)}% played
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
            {Math.round(move.winRate * 100)}% win
          </Text>
        </Box>
      </HStack>
    </VStack>
  );
};

export default MoveStatistics;
