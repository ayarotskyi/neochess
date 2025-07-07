import useDraggable, { type DropCallback } from '@/hooks/useDraggable';
import { useGameStore } from '@/store/game';
import { PieceComponents } from '@/utils';
import { Flex, Text, type StackProps } from '@chakra-ui/react';
import { Chess, makeSquare, type Color, type Square } from 'chessops';
import { parseFen } from 'chessops/fen';
import { memo, useCallback } from 'react';
import type React from 'react';
import { useShallow } from 'zustand/shallow';

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

      const isSucessful = play({ from: square, to: resultingSquare });

      if (!isSucessful) {
        resetPosition();
      }
    },
    [play, playAs, square],
  );

  const PieceComponent =
    piece && PieceComponents[`${piece.color}_${piece.role}`];

  const {
    draggableElementRefCallback,
    targetElementRefCallback,
    onMouseDown: dragOnMouseDown,
  } = useDraggable(onDrop);

  const isSelected = useGameStore((state) => state.selectedSquare === square);
  const hasPiece = piece !== undefined;
  const onMouseDown = useCallback<React.MouseEventHandler<HTMLDivElement>>(
    (event) => {
      const state = useGameStore.getState();
      if (state.selectedSquare !== null) {
        const isSucessful = play({ from: state.selectedSquare, to: square });
        if (!isSucessful) {
          if (state.selectedSquare !== square && hasPiece) {
            state.selectSquare(square);
          } else {
            state.unselectSquare();
          }
          dragOnMouseDown(event);
        }
      } else if (hasPiece) {
        state.selectSquare(square);
        dragOnMouseDown(event);
      }
    },
    [dragOnMouseDown, hasPiece, play, square],
  );

  const color: Color =
    ((square % 8) + Math.floor(square / 8)) % 2 === 0 ? 'black' : 'white';
  return (
    <Flex
      width="12.5%"
      maxH="100%"
      aspectRatio={1}
      bg={
        isSelected
          ? 'rgba(168, 85, 247, 0.5)'
          : color === 'black'
            ? '#111827'
            : '#1F2937'
      }
      _hover={{
        bg: isSelected
          ? 'rgba(168, 85, 247, 0.5)'
          : color === 'black'
            ? 'rgba(147, 51, 234, 0.2)'
            : '#374151',
      }}
      cursor={hasPiece ? 'pointer' : 'default'}
      border="1px solid rgba(55, 65, 81, 0.3)"
      position="relative"
      align="center"
      justify="center"
      onMouseDown={onMouseDown}
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
            height="80%"
            width="80%"
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
