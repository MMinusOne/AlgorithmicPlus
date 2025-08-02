//  TODO: make it blinky, add error sound

import Draggable from "react-draggable";
import { TopBar } from "./TopBar";
import { useDialogState } from "@/lib/state/dialogs";
import { Dialog } from "@/types";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

enum ErrorType {
  Unknown,
  Panic,
  BacktestError,
  WFOError,
  GridOptimizationError,
  BesyainOptimizationError,
  FetchDownloadablesError,
  DownloadDownloadablesError,
  FormulaError,
}

interface EngineError {
  id: number;
  type: ErrorType;
  message: string;
}

export default function ErrorDialog() {
  const dialogState = useDialogState();
  const [error, setError] = useState<EngineError>();
  const handleClose = () => {
    dialogState.removeActiveDialog(Dialog.Error);
  };

  useEffect(() => {
    listen("error", (event) => {
      setError(event.payload as EngineError);
      dialogState.addActiveDialog(Dialog.Error);
    });
  }, []);

  return (
    dialogState.activeDialogs.includes(Dialog.Error) && (
      <>
        <dialog
          id="download-dialog"
          className="fixed z-50 flex items-center justify-center bg-[rgba(0,0,0,0.3)] backdrop-blur-[1px] w-full h-full"
        >
          <Draggable defaultClassNameDragging="cursor-grab">
            <div className="bg-white shadow-xl w-[600px] mx-4">
              <TopBar title="Error Menu" handleClose={handleClose} />

              <div>{error?.message}</div>
            </div>
          </Draggable>
        </dialog>
      </>
    )
  );
}
