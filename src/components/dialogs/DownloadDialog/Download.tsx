import { useDownloadDialogState } from "@/lib/state/downloads";
import { MarketDataType, SourceInfo } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

export default function Download() {
  const { selectedDownloadables } = useDownloadDialogState();
  const [exchangesData, setExchangesData] = useState<SourceInfo[]>([]);

  useEffect(() => {
    const getExchangesData = async () => {
      const exchangesData: SourceInfo[] = await invoke("get_sources_info");
      console.log(exchangesData);
      setExchangesData(exchangesData);
    };

    getExchangesData();
  }, []);

  return (
    <>
      <div className="p-2 h-[400px] bg-base-200 flex">
        <div className="h-full w-1/2 p-2 flex justify-center">
          <div className="overflow-x-auto">
            <table className="table table-xs">
              <thead>
                <tr>
                  <th></th>
                  <th>Name</th>
                  <th>Symbol</th>
                  <th>Source</th>
                </tr>
              </thead>
              <tbody className="max-h-72 overflow-y-scroll">
                {selectedDownloadables.map(
                  (downloadable, downloadableIndex) => {
                    const downloadNumber = downloadableIndex + 1;

                    return (
                      <tr className="hover:bg-base-300">
                        <th>{downloadNumber}</th>
                        <td className="truncate max-w-80 w-80">
                          {downloadable.name}
                        </td>
                        <td>{downloadable.symbol}</td>
                        <td>{downloadable.source}</td>
                      </tr>
                    );
                  }
                )}
              </tbody>
            </table>
          </div>
        </div>

        <div className="h-full w-1/2 p-2 flex flex-col">
          <span className="text-lg font-semibold">Download</span>

          <fieldset className="fieldset flex-row flex-wrap flex gap-4 rounded-box w-64 p-4">
            <legend className="fieldset-legend font-semibold">
              Market Data Types
            </legend>

            {Object.keys(MarketDataType).map((marketDataKey) => {
              const marketDataValue =
                MarketDataType[marketDataKey as keyof typeof MarketDataType];

              return (
                <label className="label">
                  <input type="checkbox" className="checkbox checkbox-sm" />
                  {marketDataValue}
                </label>
              );
            })}
          </fieldset>

          <select defaultValue="Timeframe" className="select">
            <option disabled={true}>Pick a timeframe</option>
            
          </select>
        </div>
      </div>
    </>
  );
}
