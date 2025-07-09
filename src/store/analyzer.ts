import type { NormalMove } from 'chessops';
import { create } from 'zustand';
import { immer } from 'zustand/middleware/immer';

export type MoveStat = {
  move: NormalMove;
  playRate: number;
  winRate: number;
  hovered: boolean;
};

export type AnalyzerStoreType = {
  timeRange: {
    fromUnix: number;
    toUnix: number;
  };
  moveStatistics: MoveStat[];
  setTimeRange: (timeRange: AnalyzerStoreType['timeRange']) => void;
  setHovered: (statIndex: number, hovered: boolean) => void;
};

export const useAnalyzerStore = create<AnalyzerStoreType>()(
  immer((set) => ({
    timeRange: {
      fromUnix: Math.floor(new Date().getTime() / 1000),
      toUnix: Math.floor(new Date().getTime() / 1000),
    },
    moveStatistics: [
      {
        move: { from: 12, to: 28 },
        playRate: 0.8,
        winRate: 0.52,
        hovered: false,
      },
      {
        move: { from: 12, to: 20 },
        playRate: 0.1,
        winRate: 0.37,
        hovered: false,
      },
      {
        move: { from: 17, to: 6 },
        playRate: 0.1,
        winRate: 0.5,
        hovered: false,
      },
    ],
    setTimeRange: (timeRange) =>
      set({
        timeRange,
      }),
    setHovered: (statIndex, hovered) => {
      set((state) => {
        if (statIndex < 0 || statIndex >= state.moveStatistics.length) {
          return;
        }

        state.moveStatistics[statIndex].hovered = hovered;
      });
    },
  })),
);
