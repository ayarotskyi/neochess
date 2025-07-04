import {
  Chess,
  type Color,
  type Move,
  type Piece,
  type Square,
} from 'chessops';
import { create } from 'zustand';

export type GameStoreType = {
  board: Map<Square, Piece>;
  positionMoves: undefined | Move[];
  playAs: Color;
};

export const useGameStore = create<GameStoreType>((set) => ({
  board: (() => {
    const game = Chess.default();

    const map = new Map();

    for (const [square, piece] of game.board) {
      map.set(square, piece);
    }

    return map;
  })(),
  positionMoves: [],
  playAs: 'white',
}));
