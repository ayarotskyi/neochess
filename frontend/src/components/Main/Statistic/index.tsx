import { VStack, type StackProps } from '@chakra-ui/react';
import PositionMoves from './PositionMoves';
import PositionInfo from './PositionInfo';
import useMoveStats from '@/hooks/useMoveStats';

const Statistic = (props: StackProps) => {
  useMoveStats();
  return (
    <VStack align="stretch" spaceY="24px" {...props}>
      <PositionMoves />
      <PositionInfo />
    </VStack>
  );
};

export default Statistic;
