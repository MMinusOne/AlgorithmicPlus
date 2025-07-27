import { StaticResource, SelectedItemType } from "@/types";
import { create } from "zustand";

interface SelectedItem {
  itemType: SelectedItemType;
  id: string;
}

interface SidebarState {
  isLoading: boolean;
  staticResources: StaticResource[];
  selectedItem: SelectedItem | null;

  setStaticResources: (staticResource: StaticResource[]) => void;
  setIsLoading: (isLoading: boolean) => void;
  setSelectedItem: (item: SelectedItem) => void;
}

export const useSidebarState = create<SidebarState>((set) => ({
  isLoading: true,
  staticResources: [],
  selectedItem: null,

  setStaticResources: (staticResources: StaticResource[]) =>
    set((state) => ({ ...state, staticResources })),
  setIsLoading: (isLoading: boolean) =>
    set((state) => ({ ...state, isLoading })),
  setSelectedItem: (selectedItem: SelectedItem) =>
    set((state) => ({ ...state, selectedItem })),
}));
