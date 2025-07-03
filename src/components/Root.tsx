import { HStack, type StackProps } from '@chakra-ui/react';
import Board from './Board';
import Statistic from './Statistic';

const Root = (props: StackProps) => {
  return (
    <HStack align="stretch" flex={1} spaceX="48px" px="10%" py={5} {...props}>
      <Board flex={5} />
      <Statistic flex={2} />
    </HStack>
  );
};

export default Root;
