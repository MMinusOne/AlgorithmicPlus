import { FaPlus } from "react-icons/fa6";
import { useDialogState } from "../../lib/state/dialogs";
import { Dialog, StaticResource, SelectedItemType } from "../../types";
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useSidebarState } from "@/lib/state/sidebar";

export default function Sidebar() {
  const dialogState = useDialogState();
  const sidebarState = useSidebarState();

  useEffect(() => {
    const getDownloadMetadata = async () => {
      const staticResources = await invoke<StaticResource[]>(
        "get_static_resources"
      );

      sidebarState.setStaticResources(staticResources);
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
              <SidebarSummary>static resources</SidebarSummary>
              <ul>
                {sidebarState.staticResources.map((staticResource) => {
                  return (
                    <li
                      onClick={() => {
                        sidebarState.setSelectedItem({
                          itemType: SelectedItemType.RawData,
                          id: staticResource.id,
                        });
                      }}
                    >
                      <a
                        className={`truncate ${
                          sidebarState.selectedItem?.id === staticResource.id
                            ? "bg-base-300"
                            : ""
                        }`}
                        title="Item"
                      >
                        <span className="truncate">{staticResource.name}</span>
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
