import type { NormalMove } from 'chessops';
import { create } from 'zustand';

export type AnalyzerStoreType = {
  timeRange: {
    fromUnix: number;
    toUnix: number;
  };
  moveStatistics: { move: NormalMove; playRate: number; winRate: number }[];
  setTimeRange: (timeRange: AnalyzerStoreType['timeRange']) => void;
};

export const useAnalyzerStore = create<AnalyzerStoreType>((set) => ({
  timeRange: {
    fromUnix: Math.floor(new Date().getTime() / 1000),
    toUnix: Math.floor(new Date().getTime() / 1000),
  },
  moveStatistics: [
    {
      move: { from: 12, to: 28 },
      playRate: 0.8,
      winRate: 0.52,
    },
    { move: { from: 12, to: 20 }, playRate: 0.1, winRate: 0.37 },
    { move: { from: 17, to: 6 }, playRate: 0.1, winRate: 0.5 },
  ],
  setTimeRange: (timeRange) =>
    set({
      timeRange,
    }),
}));
