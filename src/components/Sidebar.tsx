import { FaPlus } from "react-icons/fa6";
import { useDialogState } from "../lib/state/dialogs";
import { Dialog } from "../types";

export default function Sidebar() {
  const dialogState = useDialogState();

  return (
    <>
      <div className={`flex flex-col h-full`}>
        <div className="h-20 w-full bg-base-300 flex flex-col p-1">
          <div className="flex items-center justify-end w-full">
            <button
              onClick={() => {
                dialogState.addActiveDialog(Dialog.Download);
              }}
              className="btn btn-square btn-primary btn-sm"
            >
              <FaPlus />
            </button>
          </div>
        </div>
        <ul className={`menu bg-base-200 rounded-box h-full w-full`}>
          <li>
            <details open>
              <SidebarSummary>strategies</SidebarSummary>
              <ul>
                <li>
                  <a className="truncate" title="Item">
                    <span className="truncate">Item</span>
                  </a>
                </li>
                <li>
                  <a className="truncate" title="Item">
                    <span className="truncate">Item</span>
                  </a>
                </li>
              </ul>
            </details>
            <details open>
              <SidebarSummary>data study (composer)</SidebarSummary>
              <ul>
                <li>
                  <a className="truncate" title="Item">
                    <span className="truncate">Item</span>
                  </a>
                </li>
                <li>
                  <a className="truncate" title="Item">
                    <span className="truncate">Item</span>
                  </a>
                </li>
              </ul>
            </details>
            <details open>
              <SidebarSummary>data</SidebarSummary>
              <ul>
                <li>
                  <a className="truncate" title="Item">
                    <span className="truncate">Item</span>
                  </a>
                </li>
                <li>
                  <a className="truncate" title="Item">
                    <span className="truncate">Item</span>
                  </a>
                </li>
              </ul>
            </details>
          </li>
        </ul>
      </div>
    </>
  );
}

function SidebarSummary({ children }: { children: React.ReactNode }) {
  return (
    <summary className="capitalize font-medium truncate">{children}</summary>
  );
}
