import { Stack, type StackProps } from '@chakra-ui/react';

export type MoveType = {
  notation: string;
  playRate: number;
  winRate: number;
};

type Props = StackProps & {
  move: MoveType;
};

const Move = (props: Props) => {
  return (
    <Stack
      bg="rgba(31, 41, 55, 0.5)"
      border="1px solid rgba(55, 65, 81, 0.5)"
      padding="13px"
      {...props}
    ></Stack>
  );
};

export default Move;
