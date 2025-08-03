import { StaticResource, SelectedItemType, CompositionMetadata } from "@/types";
import { create } from "zustand";

interface SelectedItem {
  itemType: SelectedItemType;
  id: string;
}

interface SidebarState {
  isLoading: boolean;
  staticResources: StaticResource[];
  compositionMetadatas: CompositionMetadata[];
  selectedItem: SelectedItem | null;

  setStaticResources: (staticResource: StaticResource[]) => void;
  setIsLoading: (isLoading: boolean) => void;
  setCompositionMetadatas: (compositionMetadatas: CompositionMetadata[]) => void;
  setSelectedItem: (item: SelectedItem) => void;
}

export const useSidebarState = create<SidebarState>((set) => ({
  isLoading: true,
  staticResources: [],
  compositionMetadatas: [],
  selectedItem: null,

  setStaticResources: (staticResources: StaticResource[]) =>
    set((state) => ({ ...state, staticResources })),
  setCompositionMetadatas: (compositionMetadatas: CompositionMetadata[]) =>
    set((state) => ({ ...state, compositionMetadatas })),
  setIsLoading: (isLoading: boolean) =>
    set((state) => ({ ...state, isLoading })),
  setSelectedItem: (selectedItem: SelectedItem) =>
    set((state) => ({ ...state, selectedItem })),
}));
