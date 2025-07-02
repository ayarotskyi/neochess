import { VStack, type StackProps } from '@chakra-ui/react';

const Moves = (props: StackProps) => {
  return (
    <VStack
      bg="rgba(17, 24, 39, 0.5)"
      border="1px solid rgba(168, 85, 247, 0.5)"
      boxShadow="0px 0px 30px rgba(147, 51, 234, 0.3)"
      borderRadius={8}
      {...props}
    ></VStack>
  );
};

export default Moves;
