import { DownloadedMetadata, SelectedItemType } from "@/types";
import { create } from "zustand";

interface SelectedItem { 
  type: SelectedItemType,
  id: string
}

interface SidebarState {
  isLoading: boolean;
  downloadedMetadatas: DownloadedMetadata[];
  selectedItem: SelectedItem | null;

  setDownloadedMetadata: (downloadedMetada: DownloadedMetadata[]) => void;
  setIsLoading: (isLoading: boolean) => void;
  setSelectedItem: (item: SelectedItem) => void;
}

export const useSidebarState = create<SidebarState>((set) => ({
  isLoading: true,
  downloadedMetadatas: [],
  selectedItem: null,

  setDownloadedMetadata: (downloadedMetadatas: DownloadedMetadata[]) =>
    set((state) => ({ ...state, downloadedMetadatas })),
  setIsLoading: (isLoading: boolean) =>
    set((state) => ({ ...state, isLoading })),
  setSelectedItem: (selectedItem: SelectedItem) =>
    set((state) => ({ ...state, selectedItem })),
}));
