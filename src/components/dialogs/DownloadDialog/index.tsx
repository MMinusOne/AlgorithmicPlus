import Draggable from "react-draggable";
import DownloadablesList from "./DownloadablesList";
import { TopBar } from "../TopBar";
import { useDialogState } from "@/lib/state";
import { Dialog } from "@/types";
import { useState } from "react";
import IndividualDownload from "./IndividualDownload";
import GroupDownload from "./GroupDownload";

enum DownloadDialogMenu {
  DownloadablesList,
  GroupDownload,
  IndividualDownload,
}

export default function DownloadDialog() {
  const [currentMenu, setCurrentMenu] = useState<DownloadDialogMenu>(
    DownloadDialogMenu.DownloadablesList
  );

  const dialogState = useDialogState();
  
  const handleClose = () => {
    dialogState.removeActiveDialog(Dialog.Download);
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
              <TopBar title="Download Menu" handleClose={handleClose} />
              {currentMenu === DownloadDialogMenu.DownloadablesList ? (
                <DownloadablesList />
              ) : currentMenu === DownloadDialogMenu.IndividualDownload ? (
                <IndividualDownload />
              ) : currentMenu === DownloadDialogMenu.GroupDownload ? (
                <GroupDownload />
              ) : (
                <DownloadablesList />
              )}
            </div>
          </Draggable>
        </dialog>
      </>
    )
  );
}
