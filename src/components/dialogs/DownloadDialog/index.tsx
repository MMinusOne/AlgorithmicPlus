import Draggable from "react-draggable";
import DownloadablesList from "./DownloadablesList";
import { TopBar } from "../TopBar";
import { useDialogState } from "@/lib/state/dialogs";
import { Dialog, DownloadDialogMenu, MarketType } from "@/types";
import { useState } from "react";
import Download from "./Download";
import { useDownloadDialogState } from "@/lib/state/downloads";

export default function DownloadDialog() {
  const {
    setCurrentMarketType,
    setDisplayedDownloadables,
    setDownloadablePage,
    setIsLoading,
    setSelectedDownloadables,
    setCurrentMenu,
    currentMenu,
  } = useDownloadDialogState();

  const dialogState = useDialogState();

  const handleClose = () => {
    setCurrentMenu(DownloadDialogMenu.DownloadablesList);
    setCurrentMarketType(MarketType.Crypto);
    setDisplayedDownloadables([]);
    setDownloadablePage(1);
    setIsLoading(false);
    setSelectedDownloadables([]);
    dialogState.removeActiveDialog(Dialog.Download);
  };

  const onDownload = () => {
    setCurrentMenu(DownloadDialogMenu.Download);
  };

  const handleBack = () => {
    setCurrentMenu(DownloadDialogMenu.DownloadablesList);
  };

  return (
    dialogState.activeDialogs.includes(Dialog.Download) && (
      <>
        <dialog
          id="download-dialog"
          className="fixed z-50 flex items-center justify-center bg-[rgba(0,0,0,0.3)] backdrop-blur-[1px] w-full h-full"
        >
          <Draggable defaultClassNameDragging="cursor-grab">
            <div className="bg-white shadow-xl w-[600px] mx-4">
              <TopBar
                title="Download Menu"
                handleClose={handleClose}
                handleBack={
                  currentMenu !== DownloadDialogMenu.DownloadablesList
                    ? handleBack
                    : undefined
                }
              />
              {currentMenu === DownloadDialogMenu.DownloadablesList ? (
                <DownloadablesList onDownload={onDownload} />
              ) : currentMenu === DownloadDialogMenu.Download ? (
                <Download />
              ) : (
                <DownloadablesList onDownload={onDownload} />
              )}
            </div>
          </Draggable>
        </dialog>
      </>
    )
  );
}
