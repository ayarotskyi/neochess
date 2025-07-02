import { Stack, type StackProps } from '@chakra-ui/react';

const PositionInfo = (props: StackProps) => {
  return (
    <Stack
      bg="rgba(17, 24, 39, 0.5)"
      border="1px solid rgba(34, 197, 94, 0.5)"
      boxShadow="0px 0px 30px rgba(34, 197, 94, 0.3)"
      borderRadius={8}
      padding={25}
      {...props}
    ></Stack>
  );
};

export default PositionInfo;
