import { useDownloadDialogState } from "@/lib/state/downloads";
import { MarketDataType, SourceInfo } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
interface FormData {
  marketDataTypes: MarketDataType[];
  timeframe: string | null;
}

export default function Download({
  onDownloadStart,
}: {
  onDownloadStart: () => void;
}) {
  const {
    selectedDownloadables,
    selectedDataTypes,
    selectedEndDate,
    selectedStartDate,
    selectedTimeframe,
    setSelectedDataTypes,
    setSelectedEndDate,
    setSelectedStartDate,
    setSelectedTimeframe,
  } = useDownloadDialogState();

  const [timeframeAvailability, setTimeframeAvailability] = useState<
    { timeframe: string; sources: SourceInfo[] }[]
  >([]);

  const MAX_YEARS_DOWNLOAD = 20;
  const MIN_DATE = new Date(new Date().getFullYear() - MAX_YEARS_DOWNLOAD, 0, 1)
    .toISOString()
    .split("T")[0];
  const MAX_DATE = new Date(Date.now() - 24 * 60 * 60 * 1000)
    .toISOString()
    .split("T")[0];

  useEffect(() => {
    const getExchangesData = async () => {
      const sources: SourceInfo[] = await invoke("get_sources_info");
      const allTimeframes = [
        ...new Set(sources.map((source) => source.timeframes).flat(2)),
      ];
      const timeframeAvailabilityTemp: {
        timeframe: string;
        sources: SourceInfo[];
      }[] = [];

      for (const timeframe of allTimeframes) {
        const availableSources = sources.filter((s) =>
          s.timeframes.includes(timeframe)
        );

        timeframeAvailabilityTemp.push({
          timeframe,
          sources: availableSources,
        });
      }

      setTimeframeAvailability(timeframeAvailabilityTemp);
    };

    getExchangesData();
  }, []);

  useEffect(() => {
    console.log(selectedDataTypes);
  }, [selectedDataTypes]);

  return (
    <>
      <div className="p-2 h-[500px] bg-base-200 flex">
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

                    const isAvailable =
                      timeframeAvailability
                        .find((t) => t.timeframe === selectedTimeframe)
                        ?.sources.some(
                          (s) => s.exchange_name === downloadable.source
                        ) ?? false;

                    return (
                      <tr
                        className={`hover:bg-base-300 ${
                          !isAvailable && "line-through bg-base-100"
                        }`}
                      >
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

        <div className="h-full w-1/2 p-2 flex flex-col justify-between">
          <div>
            <span className="text-lg font-semibold">Download</span>

            <fieldset className="fieldset flex-row flex-wrap flex gap-2 rounded-box w-64 p-4">
              <legend className="fieldset-legend font-semibold">
                Market Data Types
              </legend>

              {Object.keys(MarketDataType).map((marketDataKey) => {
                const marketDataValue =
                  MarketDataType[marketDataKey as keyof typeof MarketDataType];

                if (
                  ["Economics"].includes(
                    marketDataKey as keyof typeof MarketDataType
                  )
                )
                  return;

                return (
                  <label className="label">
                    <input
                      checked={selectedDataTypes.includes(marketDataValue)}
                      onChange={(e) => {
                        if (e.currentTarget.checked) {
                          setSelectedDataTypes([
                            ...selectedDataTypes,
                            marketDataValue,
                          ]);
                        } else {
                          setSelectedDataTypes(
                            selectedDataTypes.filter(
                              (dataType) => dataType !== marketDataValue
                            )
                          );
                        }
                      }}
                      type="checkbox"
                      className={`checkbox checkbox-sm`}
                    />
                    {marketDataValue}
                  </label>
                );
              })}

              <legend className="fieldset-legend font-semibold">
                Data Timeframe
              </legend>

              <select
                onChange={(e) => {
                  const selectedTimeframe = e.currentTarget.value;
                  setSelectedTimeframe(selectedTimeframe);
                }}
                className="select"
              >
                {timeframeAvailability.map((timeframe) => {
                  return (
                    <option defaultChecked={false} value={timeframe.timeframe}>
                      {timeframe.timeframe}
                    </option>
                  );
                })}
              </select>

              <legend className="fieldset-legend font-semibold">
                Data Start Date
              </legend>
              <input
                type="date"
                className="input"
                min={MIN_DATE}
                max={MAX_DATE}
                value={selectedStartDate}
                onInput={(e) => {
                  setSelectedStartDate(e.currentTarget.value);
                }}
              />
              <legend className="fieldset-legend font-semibold">
                Data End Date
              </legend>
              <input
                type="date"
                className="input"
                min={MIN_DATE}
                max={MAX_DATE}
                value={selectedEndDate}
                onInput={(e) => {
                  setSelectedEndDate(e.currentTarget.value);
                }}
              />
            </fieldset>
          </div>

          <button
            onClick={onDownloadStart}
            disabled={selectedDataTypes.length === 0 ? true : false}
            className={`btn btn-secondary`}
          >
            Download
          </button>
        </div>
      </div>
    </>
  );
}
