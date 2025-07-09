import { create } from 'zustand';

export type AnalyzerStoreType = {
  timeRange: {
    fromUnix: number;
    toUnix: number;
  };
  setTimeRange: (timeRange: AnalyzerStoreType['timeRange']) => void;
};

export const useAnalyzerStore = create<AnalyzerStoreType>((set, get) => ({
  timeRange: {
    fromUnix: Math.floor(new Date().getTime() / 1000),
    toUnix: Math.floor(new Date().getTime() / 1000),
  },
  setTimeRange: (timeRange) =>
    set({
      timeRange,
    }),
}));
