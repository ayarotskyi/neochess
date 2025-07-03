import { VStack, type StackProps } from '@chakra-ui/react';
import MoveStatistics from './MoveStatistics';
import PositionInfo from './PositionInfo';

const Statistic = (props: StackProps) => {
  return (
    <VStack align="stretch" spaceY="24px" {...props}>
      <MoveStatistics />
      <PositionInfo />
    </VStack>
  );
};

export default Statistic;
