import { fen, type Color } from 'chessops';
import { create } from 'zustand';

export type GameStoreType = {
  fen: string;
  playAs: Color;
};

export const useGameStore = create<GameStoreType>((set) => ({
  fen: fen.INITIAL_BOARD_FEN,
  playAs: 'white',
}));
