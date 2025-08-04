import type { Move } from 'chessops';
import { create } from 'zustand';
import { immer } from 'zustand/middleware/immer';

export type MoveStat = {
  move: Move;
  playRate: number;
  winRate: number;
  hovered: boolean;
};

export type AnalyzerStoreType = {
  timeRange: {
    fromUnix?: number;
    toUnix?: number;
  };
  moveStatistics?: MoveStat[];
  setTimeRange: (timeRange: AnalyzerStoreType['timeRange']) => void;
  setHovered: (statIndex: number, hovered: boolean) => void;
  updateStats: (stats: Omit<MoveStat, 'hovered'>[] | null) => void;
};

export const useAnalyzerStore = create<AnalyzerStoreType>()(
  immer((set) => ({
    timeRange: {
      fromUnix: undefined,
      toUnix: undefined,
    },
    moveStatistics: undefined,
    setTimeRange: (timeRange) =>
      set({
        timeRange,
      }),
    setHovered: (statIndex, hovered) => {
      set((state) => {
        if (
          !state.moveStatistics ||
          statIndex < 0 ||
          statIndex >= state.moveStatistics.length
        ) {
          return;
        }

        state.moveStatistics[statIndex].hovered = hovered;
      });
    },
    updateStats: (stats) => {
      set({
        moveStatistics: !stats
          ? undefined
          : stats.map((stat) => ({
              ...stat,
              hovered: false,
            })),
      });
    },
  })),
);
