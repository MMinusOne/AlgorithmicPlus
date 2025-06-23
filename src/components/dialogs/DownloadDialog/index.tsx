import Draggable from "react-draggable";
import DownloadablesList from "./DownloadablesList";
import { TopBar } from "./TopBar";
import { useDialogState } from "../../../lib/state";
import { Dialog } from "../../../types";

enum DialogMenu { 
  DownloadablesList, 
  IndividualDownload,
  
}

export default function DownloadDialog() {
  const [currentMenu, setCurrentMenu] = useState<DialogMenu>(DialogMenu.DownloadablesList);
  const handleClose = () => {
    dialogState.removeActiveDialog(Dialog.Download);
  };

  const dialogState = useDialogState();

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
              <DownloadablesList />
            </div>
          </Draggable>
        </dialog>
      </>
    )
  );
}
