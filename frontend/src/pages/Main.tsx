import { HStack, type StackProps } from '@chakra-ui/react';
import Statistic from '../components/Statistic';
import PositionAnalyzer from '../components/PositionAnalyzer';

const Root = (props: StackProps) => {
  return (
    <HStack
      align="stretch"
      flex={1}
      spaceX="48px"
      px="10%"
      py={5}
      overflow="hidden"
      maxH="100%"
      {...props}
    >
      <PositionAnalyzer flex={5} />
      <Statistic flex={2} />
    </HStack>
  );
};

export default Root;
