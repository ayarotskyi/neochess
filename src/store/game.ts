import { Chess, fen, type Color, type NormalMove, type Square } from 'chessops';
import { create } from 'zustand';

export type GameStoreType = {
  fen: string;
  playAs: Color;
  selectedSquare: Square | null;
  play: (move: NormalMove) => boolean;
  selectSquare: (square: Square) => void;
  unselectSquare: () => void;
};

export const useGameStore = create<GameStoreType>((set, get) => ({
  fen: fen.INITIAL_FEN,
  playAs: 'white',
  selectedSquare: null,
  play: (move) => {
    if (move.from === move.to) {
      return false;
    }

    const state = get();

    state.unselectSquare();

    const game = Chess.fromSetup(fen.parseFen(state.fen).unwrap()).unwrap();

    if (!game.isLegal(move)) {
      return false;
    }

    try {
      game.play(move);
    } catch {
      return false;
    }

    set({
      fen: fen.makeFen(game.toSetup()),
    });

    return true;
  },
  selectSquare: (square) => {
    set({ selectedSquare: square });
  },
  unselectSquare: () => {
    set({ selectedSquare: null });
  },
}));
