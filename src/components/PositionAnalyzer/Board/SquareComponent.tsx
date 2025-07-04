import { Box, Text, type StackProps } from '@chakra-ui/react';
import { makeSquare, type Color, type Square } from 'chessops';

type Props = StackProps & {
  square: Square;
};

const SquareComponent = ({ square, ...props }: Props) => {
  const squareName = makeSquare(square);
  const color: Color =
    ((square % 8) + Math.floor(square / 8)) % 2 === 0 ? 'black' : 'white';
  return (
    <Box
      width="12.5%"
      maxH="100%"
      aspectRatio={1}
      bg={color === 'black' ? '#111827' : '#1F2937'}
      border="1px solid rgba(55, 65, 81, 0.3)"
      alignContent="end"
      justifyItems="end"
      {...props}
    >
      <Text
        textStyle="mono"
        fontWeight={400}
        fontSize="12px"
        lineHeight="16px"
        color="#6B7280"
        bottom={0}
        right={0}
        userSelect="none"
      >
        {squareName}
      </Text>
    </Box>
  );
};

export default SquareComponent;
