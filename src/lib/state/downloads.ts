import { Downloadable, DownloadDialogMenu, MarketType } from "@/types";
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
  currentMenu: DownloadDialogMenu,
  setCurrentMenu: (menu: DownloadDialogMenu) => void;
  DOWNLOAD_PAGE_ITEMS: number;
}

export const useDownloadDialogState = create<DownloadDialogState>((set) => ({
  displayedDownloadables: [],
  isLoading: true,
  downloadablePage: 1,
  DOWNLOAD_PAGE_ITEMS: 12,
  currentMarketType: MarketType.Crypto,
  downloadables: [],
  selectedDownloadables: [],
  currentMenu: DownloadDialogMenu.DownloadablesList,
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
}));
