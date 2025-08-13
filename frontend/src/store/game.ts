import {
  Chess,
  fen,
  isNormal,
  SquareSet,
  type Color,
  type Move,
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
  fenStack: string[];
  backtrackStep: number;
  playAs: Color;
  selectedSquare: Square | null;
  promotingMove: Move | null;
  play: (move: Move) => MoveResult;
  selectSquare: (square: Square) => void;
  unselectSquare: () => void;
  resolvePromotion: (role: Role | null) => void;
  resetBoard: () => void;
  changeSide: () => void;
  prevPosition: () => void;
  nextPosition: () => void;
};

export const useGameStore = create<GameStoreType>((set, get) => ({
  fenStack: [fen.INITIAL_FEN],
  backtrackStep: 0,
  playAs: 'white' as Color,
  selectedSquare: null,
  promotingMove: null,
  play: (move) => {
    if (!isNormal(move) || move.from === move.to) {
      return MoveResult.Illegal;
    }

    const state = get();

    const game = Chess.fromSetup(
      fen
        .parseFen(
          state.fenStack[state.fenStack.length - 1 - state.backtrackStep],
        )
        .unwrap(),
    ).unwrap();

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

    const setup = game.toSetup();
    // set ep square bc the generated fen will be incorrect otherwise
    setup.epSquare = game.epSquare;

    set({
      fenStack: [
        ...state.fenStack.slice(0, state.fenStack.length - state.backtrackStep),
        fen.makeFen(setup),
      ],
      backtrackStep: 0,
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
      fenStack: [fen.INITIAL_FEN],
      backtrackStep: 0,
      selectedSquare: null,
    });
  },
  changeSide: () => {
    set((state) => ({
      playAs: state.playAs === 'white' ? 'black' : 'white',
    }));
  },
  nextPosition: () => {
    set((state) => ({
      backtrackStep: state.backtrackStep <= 0 ? 0 : state.backtrackStep - 1,
    }));
  },
  prevPosition: () => {
    set((state) => ({
      backtrackStep:
        state.backtrackStep >= state.fenStack.length - 1
          ? state.fenStack.length - 1
          : state.backtrackStep + 1,
    }));
  },
}));
