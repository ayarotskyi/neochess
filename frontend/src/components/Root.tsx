import { HStack, type StackProps } from '@chakra-ui/react';
import Statistic from './Statistic';
import PositionAnalyzer from './PositionAnalyzer';

const Root = (props: StackProps) => {
  return (
    <HStack
      align="stretch"
      flex={1}
      spaceX="48px"
      px="10%"
      py={5}
      overflow="hidden"
      {...props}
    >
      <PositionAnalyzer flex={5} />
      <Statistic flex={2} />
    </HStack>
  );
};

export default Root;
