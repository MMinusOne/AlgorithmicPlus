import { FaPlus } from "react-icons/fa6";
import { useDialogState } from "../../lib/state/dialogs";
import { Dialog, DownloadedMetadata, SelectedItemType } from "../../types";
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useSidebarState } from "@/lib/state/sidebar";

export default function Sidebar() {
  const dialogState = useDialogState();
  const sidebarState = useSidebarState();

  useEffect(() => {
    const getDownloadMetadata = async () => {
      const downloadedMetadatas = await invoke<DownloadedMetadata[]>(
        "get_downloaded_metadatas"
      );

      sidebarState.setDownloadedMetadata(downloadedMetadatas);
      sidebarState.setIsLoading(false);
    };

    getDownloadMetadata();
  }, []);

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
              <SidebarSummary>data study (story composer)</SidebarSummary>
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
              <SidebarSummary>data stories</SidebarSummary>
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
                {sidebarState.downloadedMetadatas.map((downloadedMetadata) => {
                  const { id, symbol, timeframe, start_date, end_date } =
                    downloadedMetadata;

                  return (
                    <li
                      onClick={() => {
                        sidebarState.setSelectedItem({
                          type: SelectedItemType.RawData,
                          id,
                        });
                      }}
                    >
                      <a
                        className={`truncate ${
                          sidebarState.selectedItem?.id ===
                          downloadedMetadata.id
                            ? "bg-base-300"
                            : ""
                        }`}
                        title="Item"
                      >
                        <span className="truncate">
                          {symbol}_{timeframe}_{start_date}_{end_date}
                        </span>
                      </a>
                    </li>
                  );
                })}
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
