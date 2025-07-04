import { VStack, type StackProps } from '@chakra-ui/react';
import PositionMoves from './PositionMoves';
import PositionInfo from './PositionInfo';

const Statistic = (props: StackProps) => {
  return (
    <VStack align="stretch" spaceY="24px" {...props}>
      <PositionMoves />
      <PositionInfo />
    </VStack>
  );
};

export default Statistic;
