import useDraggable, { type DropCallback } from '@/hooks/useDraggable';
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
import { useGameStore } from '@/store/game';
import { Flex, Text, type StackProps } from '@chakra-ui/react';
import {
  Chess,
  makeSquare,
  type Color,
  type Role,
  type Square,
} from 'chessops';
import { parseFen } from 'chessops/fen';
import { memo, useCallback, type SVGProps } from 'react';
import type React from 'react';
import { useShallow } from 'zustand/shallow';

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
};

const SquareComponent = ({ square, ...props }: Props) => {
  const playAs = useGameStore((state) => state.playAs);
  const play = useGameStore((state) => state.play);
  const piece = useGameStore(
    useShallow((state) => {
      const board = Chess.fromSetup(parseFen(state.fen).unwrap()).unwrap()
        .board;

      return board.get(square);
    }),
  );

  const onDrop = useCallback<DropCallback>(
    (xUnits, yUnits, resetPosition) => {
      const resultingSquare =
        square +
        (playAs === 'white' ? -1 : 1) * Math.floor(yUnits) * 8 +
        (playAs === 'white' ? 1 : -1) * Math.floor(xUnits);

      play({ from: square, to: resultingSquare }, resetPosition);
    },
    [play, playAs, square],
  );

  const PieceComponent =
    piece && PieceComponents[`${piece.color}_${piece.role}`];

  const { draggableElementRefCallback, targetElementRefCallback } =
    useDraggable(onDrop);

  const color: Color =
    ((square % 8) + Math.floor(square / 8)) % 2 === 0 ? 'black' : 'white';
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
      ref={targetElementRefCallback}
      {...props}
    >
      {PieceComponent && (
        <Flex
          width="100%"
          height="100%"
          position="absolute"
          align="center"
          justify="center"
          ref={draggableElementRefCallback}
        >
          <PieceComponent
            height="60%"
            width="60%"
            filter="drop-shadow(0 0 10px rgba(255,255,255,0.8))"
            pointerEvents="none"
            style={{ zIndex: 2 }}
          />
        </Flex>
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
        {makeSquare(square)}
      </Text>
    </Flex>
  );
};

export default memo(SquareComponent);
