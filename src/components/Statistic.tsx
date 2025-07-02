import { VStack, type StackProps } from '@chakra-ui/react';
import Moves from './Moves';
import PositionInfo from './PositionInfo';

const Statistic = (props: StackProps) => {
  return (
    <VStack align="stretch" spaceY="24px" {...props}>
      <Moves flex={1} />
      <PositionInfo />
    </VStack>
  );
};

export default Statistic;
