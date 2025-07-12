import {
  Downloadable,
  DownloadDialogMenu,
  MarketDataType,
  MarketType,
} from "@/types";
import { create } from "zustand";

interface UserDownloadState {
  downloadables: Downloadable[];
  setDownloadables: (downloadables: Downloadable[]) => void;
}

export const useUserDownloadState = create<UserDownloadState>((set) => ({
  downloadables: [],
  setDownloadables: (downloadables) =>
    set((state) => ({ ...state, downloadables })),
}));

interface DownloadDialogState {
  isLoading: boolean;
  setIsLoading: (isLoading: boolean) => void;
  displayedDownloadables: Downloadable[];
  setDisplayedDownloadables: (downloadables: Downloadable[]) => void;
  downloadablePage: number;
  setDownloadablePage: (downloadPage: number) => void;
  currentMarketType: MarketType;
  setCurrentMarketType: (marketType: MarketType) => void;
  downloadables: Downloadable[];
  setDownloadables: (downloadables: Downloadable[]) => void;
  selectedDownloadables: Downloadable[];
  setSelectedDownloadables: (downloadables: Downloadable[]) => void;
  currentMenu: DownloadDialogMenu;
  setCurrentMenu: (menu: DownloadDialogMenu) => void;
  selectedTimeframe: string;
  setSelectedTimeframe: (timeframe: string) => void;
  selectedStartDate: string;
  setSelectedStartDate: (startDate: string) => void;
  selectedEndDate: string;
  setSelectedEndDate: (endDate: string) => void;
  selectedDataTypes: MarketDataType[];
  setSelectedDataTypes: (dataTypes: MarketDataType[]) => void;
  availableSelectedDownloadables: Downloadable[];
  setAvailableSelectedDownloadables: (downloadadables: Downloadable[]) => void;
  DOWNLOAD_PAGE_ITEMS: number;
}

export const useDownloadDialogState = create<DownloadDialogState>((set) => ({
  displayedDownloadables: [],
  isLoading: true,
  downloadablePage: 1,
  DOWNLOAD_PAGE_ITEMS: 12,
  currentMarketType: MarketType.Crypto,
  downloadables: [],
  currentMenu: DownloadDialogMenu.DownloadablesList,
  selectedDownloadables: [],
  selectedTimeframe: "1h",
  selectedDataTypes: [MarketDataType.OHLCV],
  selectedStartDate: "2025-05-15",
  selectedEndDate: "2025-06-15",
  availableSelectedDownloadables: [],

  setDisplayedDownloadables: (downloadables: Downloadable[]) =>
    set((state) => ({ ...state, displayedDownloadables: downloadables })),
  setDownloadablePage: (downloadablePage: number) =>
    set((state) => ({ ...state, downloadablePage })),
  setIsLoading: (isLoading: boolean) =>
    set((state) => ({ ...state, isLoading })),
  setCurrentMarketType: (marketType: MarketType) =>
    set((state) => ({ ...state, currentMarketType: marketType })),
  setDownloadables: (downloadables: Downloadable[]) =>
    set((state) => ({ ...state, downloadables })),
  setSelectedDownloadables: (downloadables: Downloadable[]) =>
    set((state) => ({ ...state, selectedDownloadables: downloadables })),
  setCurrentMenu: (currentMenu: DownloadDialogMenu) =>
    set((state) => ({ ...state, currentMenu: currentMenu })),
  setSelectedDataTypes: (selectedDataTypes: MarketDataType[]) =>
    set((state) => ({ ...state, selectedDataTypes })),
  setSelectedEndDate: (selectedEndDate: string) =>
    set((state) => ({ ...state, selectedEndDate })),
  setSelectedStartDate: (selectedStartDate: string) =>
    set((state) => ({ ...state, selectedStartDate })),
  setSelectedTimeframe: (selectedTimeframe: string) =>
    set((state) => ({ ...state, selectedTimeframe })),
  setAvailableSelectedDownloadables: (
    availableSelectedDownloadables: Downloadable[]
  ) => set((state) => ({ ...state, availableSelectedDownloadables })),
}));
