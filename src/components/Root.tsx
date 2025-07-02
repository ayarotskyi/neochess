import { HStack, Stack, type StackProps } from '@chakra-ui/react';
import Board from './Board';
import Statistic from './Statistic';

const Root = (props: StackProps) => {
  return (
    <Stack flex={1} px="10%" py={5} {...props}>
      <HStack align="stretch" flex={1} spaceX="48px">
        <Board flex={5} />
        <Statistic flex={2} />
      </HStack>
    </Stack>
  );
};

export default Root;
