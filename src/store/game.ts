import { Chess, fen, type Color, type Move } from 'chessops';
import { create } from 'zustand';

export type GameStoreType = {
  fen: string;
  playAs: Color;
  play: (move: Move, resetPosition: () => void) => void;
};

export const useGameStore = create<GameStoreType>((set) => ({
  fen: fen.INITIAL_FEN,
  playAs: 'white',
  play: (move, resetPosition) => {
    set((state) => {
      const game = Chess.fromSetup(fen.parseFen(state.fen).unwrap()).unwrap();

      if (!game.isLegal(move)) {
        resetPosition();
        return state;
      }

      try {
        game.play(move);
      } catch {
        resetPosition();
        return state;
      }

      return {
        fen: fen.makeFen(game.toSetup()),
      };
    });
  },
}));
