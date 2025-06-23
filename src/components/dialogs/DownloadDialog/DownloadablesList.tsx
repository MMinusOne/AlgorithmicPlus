import { FaMagnifyingGlass, FaX } from "react-icons/fa6";
import { useDialogState } from "../../../lib/state";
import { useCallback, useEffect, useState } from "react";
import { Dialog, Downloadable, MarketDataType } from "../../../types";
import { throttle } from "lodash";
import { invoke } from "@tauri-apps/api/core";

export default function DownloadablesList() { 

    const [isLoading, setIsLoading] = useState(true);
    const [currentMarketType, setCurrentMarketType] = useState<
      "Crypto" | "Stock" | "Futures"
    >("Crypto");
    const [currentDataType] = useState<MarketDataType>(MarketDataType.OHLCV);
    const [downloadables, setDownloadables] = useState<Downloadable[]>([]);
    const [displayedDownloadables, setDisplayedDownloadables] = useState<
      Downloadable[]
    >([]);
    const [downloadablePage, setDownloadablePage] = useState(1);
    const DOWNLOAD_PAGE_ITEMS = 12;

    const handleSearch = useCallback(
      throttle((searchValue: string) => {
        const downloadablesSearch = downloadables.filter((downloadable) => {
          return (
            downloadable.name
              .toLowerCase()
              .includes(searchValue.toLowerCase()) ||
            downloadable.symbol
              .toLowerCase()
              .includes(searchValue.toLowerCase()) ||
            downloadable.source
              .toLowerCase()
              .includes(searchValue.toLowerCase()) ||
            downloadable.data_type
              .toLowerCase()
              .includes(searchValue.toLowerCase())
          );
        });

        setDisplayedDownloadables(downloadablesSearch);
        setDownloadablePage(1);
      }, 1000),
      [downloadables]
    );

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
        <ul className="menu menu-horizontal items-center w-full bg-base-200 p-0 flex justify-between">
          <div className="flex">
            <li
              onClick={() => {
                setCurrentMarketType("Crypto");
              }}
              className={` ${
                currentMarketType === "Crypto" ? "bg-base-300" : ""
              }`}
            >
              <a className={`rounded-none`}>Crypto</a>
            </li>
            <li
              onClick={() => {
                setCurrentMarketType("Stock");
              }}
              className={`${
                currentMarketType === "Stock" ? "bg-base-300" : ""
              }`}
            >
              <a className="rounded-none">Stocks</a>
            </li>
            <li
              onClick={() => {
                setCurrentMarketType("Futures");
              }}
              className={` ${
                currentMarketType === "Futures" ? "bg-base-300" : ""
              }`}
            >
              <a className="rounded-none">Futures</a>
            </li>
          </div>

          <li className="h-full ml-auto px-2">
            <label className="input input-xs">
              <FaMagnifyingGlass />
              <input
                type="search"
                className="grow"
                placeholder="Search"
                onChange={(e) => {
                  const { value: searchValue } = e.currentTarget;

                  handleSearch(searchValue);
                }}
              />
            </label>
          </li>
        </ul>

        {isLoading ? (
          <>
            <div className="w-full h-full grid place-items-center py-4">
              <span className="loading loading-spinner loading-xl"></span>
            </div>
          </>
        ) : (
          <>
            <div className="overflow-x-auto">
              <table className="table table-xs">
                <thead>
                  <tr>
                    <th></th>
                    <th>Name</th>
                    <th>Symbol</th>
                    <th>Source</th>
                    <th>Data Type</th>
                  </tr>
                </thead>
                <tbody className="max-h-72 overflow-y-scroll">
                  {displayedDownloadables
                    .slice(
                      downloadablePage * DOWNLOAD_PAGE_ITEMS,
                      downloadablePage * DOWNLOAD_PAGE_ITEMS +
                        DOWNLOAD_PAGE_ITEMS
                    )
                    .map((downloadable, downloadableIndex) => {
                      const downloadNumber = downloadableIndex + 1;

                      return (
                        <tr className="hover:bg-base-300">
                          <th>
                            {(downloadablePage - 1) * DOWNLOAD_PAGE_ITEMS +
                              downloadNumber}
                          </th>
                          <td>{downloadable.name}</td>
                          <td>{downloadable.symbol}</td>
                          <td>{downloadable.source}</td>
                          <td>{downloadable.data_type}</td>
                        </tr>
                      );
                    })}
                </tbody>
              </table>
              <div className="w-full flex items-center justify-center">
                <div className="join w-full">
                  <button
                    onClick={() =>
                      downloadablePage > 1 &&
                      setDownloadablePage(downloadablePage - 1)
                    }
                    className="join-item btn w-1/3"
                  >
                    «
                  </button>
                  <button className="join-item btn w-1/3">
                    Page {downloadablePage}
                  </button>
                  <button
                    onClick={() =>
                      downloadablePage <
                        Math.ceil(
                          displayedDownloadables.length / DOWNLOAD_PAGE_ITEMS
                        ) && setDownloadablePage(downloadablePage + 1)
                    }
                    className="join-item btn w-1/3"
                  >
                    »
                  </button>
                </div>
              </div>
            </div>
          </>
        )}
      </>
    );
}

export function DownloadablesTable() { 
    return (<></>)
}

export function Pagination() { 

}