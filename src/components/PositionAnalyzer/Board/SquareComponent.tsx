import BlackBishop from '@/icons/BlackBishop';
import BlackKing from '@/icons/BlackKing';
import BlackKnight from '@/icons/BlackKnight';
import BlackPawn from '@/icons/BlackPawn';
import BlackQueen from '@/icons/BlackQueen';
import BlackRook from '@/icons/BlackRook';
import WhiteBishop from '@/icons/WhiteBishop';
import WhiteKing from '@/icons/WhiteKing';
import WhiteKnight from '@/icons/WhiteKnight';
import WhitePawn from '@/icons/WhitePawn';
import WhiteQueen from '@/icons/WhiteQueen';
import WhiteRook from '@/icons/WhiteRook';
import { Flex, Text, type StackProps } from '@chakra-ui/react';
import {
  makeSquare,
  type Color,
  type Piece,
  type Role,
  type Square,
} from 'chessops';
import type { SVGProps } from 'react';
import type React from 'react';

export type ColorRole = `${Color}_${Role}`;

const PieceComponents: Record<ColorRole, React.FC<SVGProps<SVGSVGElement>>> = {
  white_bishop: WhiteBishop,
  white_king: WhiteKing,
  white_knight: WhiteKnight,
  white_pawn: WhitePawn,
  white_queen: WhiteQueen,
  white_rook: WhiteRook,
  black_bishop: BlackBishop,
  black_king: BlackKing,
  black_knight: BlackKnight,
  black_pawn: BlackPawn,
  black_queen: BlackQueen,
  black_rook: BlackRook,
};

type Props = StackProps & {
  square: Square;
  piece?: Piece;
};

const SquareComponent = ({ square, piece, ...props }: Props) => {
  const squareName = makeSquare(square);
  const color: Color =
    ((square % 8) + Math.floor(square / 8)) % 2 === 0 ? 'black' : 'white';
  const PieceComponent =
    piece && PieceComponents[`${piece.color}_${piece.role}`];
  return (
    <Flex
      width="12.5%"
      maxH="100%"
      aspectRatio={1}
      bg={color === 'black' ? '#111827' : '#1F2937'}
      _hover={{
        bg: color === 'black' ? 'rgba(147, 51, 234, 0.2)' : '#374151',
      }}
      cursor="pointer"
      border="1px solid rgba(55, 65, 81, 0.3)"
      position="relative"
      align="center"
      justify="center"
      {...props}
    >
      {PieceComponent && (
        <PieceComponent
          height="60%"
          width="60%"
          color={piece.color === 'black' ? 'black' : 'white'}
          filter="drop-shadow(0 0 10px rgba(255,255,255,0.8))"
        />
      )}
      <Text
        textStyle="mono"
        fontWeight={400}
        fontSize="12px"
        lineHeight="16px"
        color="#6B7280"
        bottom={0}
        right={0}
        userSelect="none"
        position="absolute"
      >
        {squareName}
      </Text>
    </Flex>
  );
};

export default SquareComponent;
