import { FaMagnifyingGlass } from "react-icons/fa6";
import { useCallback, useEffect, useMemo, useState } from "react";
import { Downloadable, MarketType } from "@/types";
import { throttle } from "lodash";
import { invoke } from "@tauri-apps/api/core";
import Loading from "@/components/Loading";
import { useDownloadDialogState } from "@/lib/state/downloads";
import Fuse from "fuse.js";

export default function DownloadablesList({
  onDownload,
}: {
  onDownload: () => void;
}) {
  const {
    isLoading,
    setIsLoading,
    currentMarketType,
    downloadables,
    setDownloadables,
    setDisplayedDownloadables,
    displayedDownloadables,
    setDownloadablePage,
  } = useDownloadDialogState();

  const [downloadableSearchQuery, setDownloadableSearchQuery] = useState("");

  const handleSearch = useCallback(
    async (searchValue: string) => {
      setDownloadableSearchQuery(searchValue);
    },
    [downloadables]
  );

  useEffect(() => {
    const searchDownloadables = async () => {
      const searchResults = await invoke<Downloadable[]>(
        "search_downloadables",
        {
          params: {
            query: downloadableSearchQuery,
          },
        }
      );

      setDisplayedDownloadables([...searchResults]);
    };

    searchDownloadables();
    setDownloadablePage(1);
  }, [downloadableSearchQuery]);

  useEffect(() => {
    const getDownloadables = async () => {
      const downloadables: Downloadable[] = await invoke("get_downloadables");
      setDownloadables(downloadables);
      setIsLoading(false);
    };

    getDownloadables();
  }, []);

  useEffect(() => {
    setDisplayedDownloadables(
      downloadables.filter((downloadable) => {
        return downloadable.market_type === currentMarketType;
      })
    );
  }, [downloadables, currentMarketType]);

  useEffect(() => {
    setDownloadablePage(1);
  }, [currentMarketType]);

  return (
    <>
      <NavigationBar
        onDownload={onDownload}
        handleSearch={handleSearch}
        downloadableSearchQuery={downloadableSearchQuery}
      />

      <Loading isLoading={isLoading}>
        <div className="overflow-x-auto">
          <DownloadablesTable />
          <Pagination />
        </div>
      </Loading>
    </>
  );
}

export function NavigationBar({
  onDownload,
  handleSearch,
  downloadableSearchQuery,
}: {
  onDownload: () => void;
  handleSearch: (searchValue: string) => void;
  downloadableSearchQuery: string;
}) {
  const { currentMarketType, setCurrentMarketType, selectedDownloadables } =
    useDownloadDialogState();

  return (
    <>
      <ul className="flex justify-between items-center bg-base-200 p-0 w-full menu menu-horizontal">
        <div className="flex">
          <li
            onClick={() => {
              setCurrentMarketType(MarketType.Crypto);
            }}
            className={` ${
              currentMarketType === MarketType.Crypto ? "bg-base-300" : ""
            }`}
          >
            <a className={`rounded-none`}>Crypto</a>
          </li>
          <li
            onClick={() => {
              setCurrentMarketType(MarketType.Stock);
            }}
            className={`${
              currentMarketType === MarketType.Stock ? "bg-base-300" : ""
            }`}
          >
            <a className="rounded-none">Stocks</a>
          </li>
          <li
            onClick={() => {
              setCurrentMarketType(MarketType.Futures);
            }}
            className={` ${
              currentMarketType === "Futures" ? "bg-base-300" : ""
            }`}
          >
            <a className="rounded-none">Futures</a>
          </li>
        </div>

        <li className="flex flex-row justify-between gap-2 ml-auto px-2 h-full">
          <label className="w-40 input input-xs">
            <FaMagnifyingGlass />
            <input
              type="search"
              className="grow"
              placeholder="Search"
              value={downloadableSearchQuery}
              onChange={(e) => {
                const { value: searchValue } = e.currentTarget;

                handleSearch(searchValue);
              }}
            />
          </label>
          <button
            disabled={selectedDownloadables.length < 0}
            aria-disabled={selectedDownloadables.length < 0}
            tabIndex={1}
            role="button"
            className={`btn btn-active btn-xs ${
              selectedDownloadables.length === 0 ? "btn-disabled" : ""
            }`}
            onClick={onDownload}
          >
            Download
          </button>
        </li>
      </ul>
    </>
  );
}

export function DownloadablesTable() {
  const {
    displayedDownloadables,
    selectedDownloadables,
    setSelectedDownloadables,
    downloadablePage,
    DOWNLOAD_PAGE_ITEMS,
  } = useDownloadDialogState();

  return (
    <>
      <table className="table table-xs bg-base-200">
        <thead>
          <tr>
            <th></th>
            <th></th>
            <th>Name</th>
            <th>Symbol</th>
            <th>Source</th>
          </tr>
        </thead>
        <tbody className="max-h-72 overflow-y-scroll">
          {displayedDownloadables
            .slice(
              (downloadablePage - 1) * DOWNLOAD_PAGE_ITEMS,
              (downloadablePage - 1) * DOWNLOAD_PAGE_ITEMS + DOWNLOAD_PAGE_ITEMS
            )
            .map((downloadable, downloadableIndex) => {
              const downloadNumber = downloadableIndex + 1;
              const isSelected = selectedDownloadables.find(
                (selectedDownloadable) =>
                  selectedDownloadable.symbol === downloadable.symbol
              );

              return (
                <tr
                  onClick={() => {
                    if (!isSelected) {
                      setSelectedDownloadables([
                        ...selectedDownloadables,
                        downloadable,
                      ]);
                    } else {
                      setSelectedDownloadables(
                        selectedDownloadables.filter(
                          (e) => e.symbol !== downloadable.symbol
                        )
                      );
                    }
                  }}
                  className="hover:bg-base-300"
                >
                  <th>
                    <input
                      onChange={(e) => {
                        if (e.currentTarget.checked) {
                          setSelectedDownloadables([
                            ...selectedDownloadables,
                            downloadable,
                          ]);
                        } else {
                          setSelectedDownloadables(
                            selectedDownloadables.filter(
                              (e) => e.symbol !== downloadable.symbol
                            )
                          );
                        }
                      }}
                      checked={isSelected ? true : false}
                      type="checkbox"
                      className="transition-none animate-none checkbox checkbox-primary"
                    />
                  </th>
                  <th>
                    {(downloadablePage - 1) * DOWNLOAD_PAGE_ITEMS +
                      downloadNumber}
                  </th>
                  <td className="w-80 max-w-80 truncate">
                    {downloadable.name}
                  </td>
                  <td>{downloadable.symbol}</td>
                  <td>{downloadable.source_name}</td>
                </tr>
              );
            })}
        </tbody>
      </table>
    </>
  );
}

export function Pagination() {
  const {
    displayedDownloadables,
    downloadablePage,
    setDownloadablePage,
    DOWNLOAD_PAGE_ITEMS,
  } = useDownloadDialogState();
  return (
    <>
      <div className="flex justify-center items-center w-full">
        <div className="w-full join">
          <button
            onClick={() =>
              downloadablePage > 1 && setDownloadablePage(downloadablePage - 1)
            }
            className="w-1/3 join-item btn"
          >
            «
          </button>
          <button className="w-1/3 join-item btn">
            Page {downloadablePage}
          </button>
          <button
            onClick={() => {
              if (
                downloadablePage <
                Math.ceil(displayedDownloadables.length / DOWNLOAD_PAGE_ITEMS)
              ) {
                setDownloadablePage(downloadablePage + 1);
              }
            }}
            className="w-1/3 join-item btn"
          >
            »
          </button>
        </div>
      </div>
    </>
  );
}
