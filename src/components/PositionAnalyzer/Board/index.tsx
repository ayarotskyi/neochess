import { Box, type StackProps } from '@chakra-ui/react';

const Board = (props: StackProps) => {
  return (
    <Box
      {...props}
      aspectRatio={1}
      height="100%"
      width="auto"
      bg="rgba(255, 255, 255, 0.002)"
      border="2px solid rgba(6, 182, 212, 0.5)"
      boxShadow="0px 0px 40px rgba(0, 255, 255, 0.2)"
    ></Box>
  );
};

export default Board;
