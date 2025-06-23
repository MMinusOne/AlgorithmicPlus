import { FaMagnifyingGlass, FaX } from "react-icons/fa6";
import { useDialogState } from "../../../lib/state";
import { Dispatch, SetStateAction, useCallback, useEffect, useState } from "react";
import { Dialog, Downloadable, MarketDataType } from "../../../types";
import { throttle } from "lodash";
import { invoke } from "@tauri-apps/api/core";
import Loading from "../../Loading";

enum MarketType {
  Crypto = "Crypto",
  Stock = "Stock",
  Futures = "Futures",
}

export default function DownloadablesList() {
  const [isLoading, setIsLoading] = useState(true);
  const [currentMarketType, setCurrentMarketType] = useState<MarketType>(
    MarketType.Crypto
  );
  const [downloadables, setDownloadables] = useState<Downloadable[]>([]);
  const [displayedDownloadables, setDisplayedDownloadables] = useState<
    Downloadable[]
  >([]);
  const [selectedDownloadables, setSelectedDownloadables] = useState<
    Downloadable[]
  >([]);
  const [downloadablePage, setDownloadablePage] = useState(1);

  const handleSearch = useCallback(
    throttle((searchValue: string) => {
      const downloadablesSearch = downloadables.filter((downloadable) => {
        return (
          downloadable.name.toLowerCase().includes(searchValue.toLowerCase()) ||
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
  const DOWNLOAD_PAGE_ITEMS = 12;

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
        currentMarketType={currentMarketType}
        handleSearch={handleSearch}
        selectedDownloadables={selectedDownloadables}
        setCurrentMarketType={setCurrentMarketType}
      />

      <Loading isLoading={isLoading}>
        <div className="overflow-x-auto">
          <DownloadablesTable
            DOWNLOAD_PAGE_ITEMS={DOWNLOAD_PAGE_ITEMS}
            displayedDownloadables={displayedDownloadables}
            downloadablePage={downloadablePage}
            setSelectedDownloadables={setSelectedDownloadables}
          />
          <Pagination
            DOWNLOAD_PAGE_ITEMS={DOWNLOAD_PAGE_ITEMS}
            downloadablePage={downloadablePage}
            displayedDownloadables={displayedDownloadables}
            setDownloadablePage={setDownloadablePage}
          />
        </div>
      </Loading>
    </>
  );
}

export function NavigationBar({
  setCurrentMarketType,
  currentMarketType,
  handleSearch,
  selectedDownloadables,
}: {
  setCurrentMarketType: Dispatch<SetStateAction<MarketType>>;
  currentMarketType: MarketType;
  handleSearch: (query: string) => void;
  selectedDownloadables: Downloadable[];
}) {
  return (
    <>
      <ul className="menu menu-horizontal items-center w-full bg-base-200 p-0 flex justify-between">
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

        <li className="h-full ml-auto px-2 flex flex-row gap-2 justify-between">
          <label className="input input-xs w-40">
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
          <button
            disabled={selectedDownloadables.length < 0}
            aria-disabled={selectedDownloadables.length < 0}
            tabIndex={1}
            role="button"
            className={`btn btn-active btn-xs ${
              selectedDownloadables.length < 0 ? "btn-disabled" : ""
            }`}
          >
            Download
          </button>
        </li>
      </ul>
    </>
  );
}

export function DownloadablesTable({
  downloadablePage,
  DOWNLOAD_PAGE_ITEMS,
  displayedDownloadables,
  setSelectedDownloadables,
}: {
  downloadablePage: number;
  DOWNLOAD_PAGE_ITEMS: number;
  displayedDownloadables: Downloadable[];
  setSelectedDownloadables: Dispatch<SetStateAction<Downloadable[]>>;
}) {
  return (
    <>
      <table className="table table-xs">
        <thead>
          <tr>
            <th></th>
            <th></th>
            <th>Name</th>
            <th>Symbol</th>
            <th>Source</th>
            <th>Data Type</th>
          </tr>
        </thead>
        <tbody className="max-h-72 overflow-y-scroll">
          {displayedDownloadables.map((downloadable, downloadableIndex) => {
            const downloadNumber = downloadableIndex + 1;
            const downloadScaledIndex =
              downloadNumber + (downloadablePage - 1) * DOWNLOAD_PAGE_ITEMS;
            if (
              (downloadablePage - 1) * DOWNLOAD_PAGE_ITEMS <
                downloadScaledIndex &&
              downloadScaledIndex > downloadablePage * DOWNLOAD_PAGE_ITEMS
            )
              return;
            console.log(
              (downloadablePage - 1) * DOWNLOAD_PAGE_ITEMS,
              downloadNumber,
              downloadScaledIndex,
              downloadablePage * DOWNLOAD_PAGE_ITEMS
            );

            return (
              <tr className="hover:bg-base-300">
                <th>
                  <input
                    onChange={(e) => {
                      if (e.currentTarget.checked) {
                        setSelectedDownloadables((prev) => [...prev, downloadable]);
                      } else { 
                        setSelectedDownloadables((prev) => prev.filter((e) => e.symbol === downloadable.symbol));
                      }
                    }}
                    type="checkbox"
                    className="checkbox checkbox-primary"
                  />
                </th>
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
    </>
  );
}

export function Pagination({
  downloadablePage,
  setDownloadablePage,
  displayedDownloadables,
  DOWNLOAD_PAGE_ITEMS,
}: {
  downloadablePage: number;
  setDownloadablePage: Dispatch<SetStateAction<number>>;
  displayedDownloadables: Downloadable[];
  DOWNLOAD_PAGE_ITEMS: number;
}) {
  return (
    <>
      <div className="w-full flex items-center justify-center">
        <div className="join w-full">
          <button
            onClick={() =>
              downloadablePage > 1 && setDownloadablePage(downloadablePage - 1)
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
    </>
  );
}
