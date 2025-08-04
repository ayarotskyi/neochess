import {
  Chess,
  fen,
  SquareSet,
  type Color,
  type NormalMove,
  type Role,
  type Square,
} from 'chessops';
import { create } from 'zustand';

export enum MoveResult {
  Success = 'Success',
  Illegal = 'Illegal',
  Promotion = 'Promotion',
}

export type GameStoreType = {
  fen: string;
  playAs: Color;
  selectedSquare: Square | null;
  promotingMove: NormalMove | null;
  play: (move: NormalMove) => MoveResult;
  selectSquare: (square: Square) => void;
  unselectSquare: () => void;
  resolvePromotion: (role: Role | null) => void;
  resetBoard: () => void;
  changeSide: () => void;
};

export const useGameStore = create<GameStoreType>((set, get) => ({
  fen: fen.INITIAL_FEN,
  playAs: 'white' as Color,
  selectedSquare: null,
  promotingMove: null,
  play: (move) => {
    if (move.from === move.to) {
      return MoveResult.Illegal;
    }

    const state = get();

    const game = Chess.fromSetup(fen.parseFen(state.fen).unwrap()).unwrap();

    if (
      game.board.pawn.has(move.from) &&
      SquareSet.backranks().has(move.to) &&
      !move.promotion
    ) {
      const dests = game.dests(move.from);
      if (dests.has(move.to)) {
        set({
          promotingMove: move,
        });
        return MoveResult.Promotion;
      } else {
        return MoveResult.Illegal;
      }
    }

    if (!game.isLegal(move)) {
      return MoveResult.Illegal;
    }

    try {
      game.play(move);
    } catch {
      return MoveResult.Illegal;
    }

    set({
      fen: fen.makeFen(game.toSetup()),
    });

    return MoveResult.Success;
  },
  selectSquare: (square) => {
    set({ selectedSquare: square });
  },
  unselectSquare: () => {
    set({ selectedSquare: null });
  },
  resolvePromotion: (role) => {
    const state = get();
    if (state.promotingMove === null) {
      return;
    }

    if (role === null) {
      set({
        promotingMove: null,
      });
      return;
    }

    const result = state.play({ ...state.promotingMove, promotion: role });

    if (result === MoveResult.Success) {
      set({
        promotingMove: null,
      });
      state.unselectSquare();
    }
  },
  resetBoard: () => {
    set({
      fen: fen.INITIAL_FEN,
    });
  },
  changeSide: () => {
    const side = get().playAs;

    set({
      playAs: side === 'white' ? 'black' : 'white',
    });
  },
}));
