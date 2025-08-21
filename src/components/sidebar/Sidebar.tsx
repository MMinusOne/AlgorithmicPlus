import { FaPlus } from "react-icons/fa6";
import { useDialogState } from "../../lib/state/dialogs";
import {
  Dialog,
  StaticResource,
  SelectedItemType,
  CompositionMetadata,
  StrategyMetadata,
} from "../../types";
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useSidebarState } from "@/lib/state/sidebar";

//TODO: tooltip when clicking on a selecteable item: properties, delete, etc...

export default function Sidebar() {
  const dialogState = useDialogState();
  const sidebarState = useSidebarState();

  useEffect(() => {
    const getStaticResources = async () => {
      const staticResources = await invoke<StaticResource[]>(
        "get_static_resources"
      );

      const compositions = await invoke<CompositionMetadata[]>(
        "get_compositions"
      );

      const strategies = await invoke<StrategyMetadata[]>("get_strategies");

      sidebarState.setStaticResources(staticResources);
      sidebarState.setCompositionMetadatas(compositions);
      sidebarState.setStrategiesMetadatas(strategies);
      sidebarState.setIsLoading(false);
    };

    getStaticResources();
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
                {sidebarState.strategiesMetadatas.map((strategyMetadata) => {
                  return (
                    <li
                      onClick={() => {
                        if (
                          sidebarState?.selectedItem?.id !== strategyMetadata.id
                        ) {
                          sidebarState.setSelectedItem({
                            itemType: SelectedItemType.Backtest,
                            id: strategyMetadata.id,
                          });
                        }
                      }}
                    >
                      <a className="truncate" title="Item">
                        <span className="truncate">
                          {strategyMetadata.name}
                        </span>
                      </a>
                    </li>
                  );
                })}
              </ul>
            </details>
            <details open>
              <SidebarSummary>data compositions</SidebarSummary>
              <ul>
                {sidebarState.compositionMetadatas.map(
                  (compositionMetadata) => {
                    return (
                      <li
                        onClick={() => {
                          if (
                            sidebarState?.selectedItem?.id !==
                            compositionMetadata.id
                          ) {
                            sidebarState.setSelectedItem({
                              itemType: SelectedItemType.Composition,
                              id: compositionMetadata.id,
                            });
                          }
                        }}
                      >
                        <a className="truncate" title="Item">
                          <span className="truncate">
                            {compositionMetadata.name}
                          </span>
                        </a>
                      </li>
                    );
                  }
                )}
              </ul>
            </details>
            <details open>
              <SidebarSummary>static resources</SidebarSummary>
              <ul>
                {sidebarState.staticResources.map((staticResource) => {
                  return (
                    <li
                      onClick={() => {
                        if (
                          sidebarState?.selectedItem?.id !== staticResource.id
                        ) {
                          sidebarState.setSelectedItem({
                            itemType: SelectedItemType.RawData,
                            id: staticResource.id,
                          });
                        }
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
