import { create } from "zustand";
import { Dialog } from "../../types";

interface DialogState {
  activeDialogs: Dialog[];
  addActiveDialog: (dialog: Dialog) => void;
  removeActiveDialog: (dialog: Dialog) => void;
}

export const useDialogState = create<DialogState>((set) => {
  return {
    activeDialogs: [],
    addActiveDialog: (dialog: Dialog) =>
      set((state) => ({
        ...state,
        activeDialogs: !state.activeDialogs.includes(dialog)
          ? state.activeDialogs.concat(dialog)
          : state.activeDialogs,
      })),
    removeActiveDialog: (dialog: Dialog) =>
      set((state) => {
        const dialogIndex = state.activeDialogs.findIndex((d) => d === dialog);
        if (dialogIndex === -1) return state;
        const activeDialogs = [...state.activeDialogs];
        activeDialogs.splice(dialogIndex, 1);
        return {
          ...state,
          activeDialogs,
        };
      }),
  };
});
