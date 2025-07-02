import { Stack, type StackProps } from '@chakra-ui/react';

const Board = (props: StackProps) => {
  return (
    <Stack
      border="1px solid rgba(6, 182, 212, 0.5)"
      boxShadow="0px 0px 30px rgba(0, 255, 255, 0.3)"
      background="rgba(17, 24, 39, 0.5)"
      borderRadius={8}
      {...props}
    ></Stack>
  );
};

export default Board;
