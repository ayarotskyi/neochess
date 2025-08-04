import { create } from 'zustand';

export type PositionStore = {
  statistics: {
    totalGames: number;
    wins: number;
    avgOpponentElo: number;
    lastPlayedUnix: number;
  } | null;
  setStatistics: (value: PositionStore['statistics']) => void;
};

export const usePositionStore = create<PositionStore>((set) => ({
  statistics: null,
  setStatistics: (value) => {
    set({
      statistics: value,
    });
  },
}));
