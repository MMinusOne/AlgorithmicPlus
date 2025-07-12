import Loading from "@/components/Loading";
import { useDownloadDialogState } from "@/lib/state/downloads";
import { Downloadable, MarketDataType, SourceInfo } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

const MAX_YEARS_DOWNLOAD = 20;
const MIN_DATE = new Date(new Date().getFullYear() - MAX_YEARS_DOWNLOAD, 0, 1)
  .toISOString()
  .split("T")[0];
const MAX_DATE = new Date(Date.now() - 24 * 60 * 60 * 1000)
  .toISOString()
  .split("T")[0];

export default function Download({
  onDownloadStart,
}: {
  onDownloadStart: () => void;
}) {
  const {
    selectedDataTypes,
    selectedEndDate,
    selectedStartDate,
    setSelectedDataTypes,
    setSelectedEndDate,
    setSelectedStartDate,
    setSelectedTimeframe,
    selectedTimeframe,
  } = useDownloadDialogState();

  const [availableTimeframes, setAvailableTimeframes] = useState<string[]>([]);

  useEffect(() => {
    const getAvailableTimeframes = async () => {
      const timeframes = await invoke<string[]>(
        "get_available_sources_timeframes"
      );

      setAvailableTimeframes(timeframes);
    };

    getAvailableTimeframes();
  }, []);

  return (
    <>
      <div className="p-2 h-[500px] bg-base-200 flex">
        <div className="h-full w-1/2 p-2 flex justify-center">
          <div className="overflow-x-auto">
            <DownloadList />
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
                value={selectedTimeframe}
                className="select"
              >
                {availableTimeframes.map((timeframe) => {
                  return (
                    <option defaultChecked={false} value={timeframe}>
                      {timeframe}
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

export function DownloadList() {
  const {
    selectedDownloadables,
    selectedTimeframe,
    setAvailableSelectedDownloadables,
    availableSelectedDownloadables,
  } = useDownloadDialogState();

  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const checkAvailabilities = async () => {
      setIsLoading(true);
      let availableDownloadabes: Downloadable[] = [];

      for (const downloadable of selectedDownloadables) {
        const isAvailable = await invoke(
          "downloadable_timeframe_pair_available",
          { data: { downloadable, timeframe: selectedTimeframe } }
        );

        if (isAvailable) {
          availableDownloadabes.push(downloadable);
        }
      }

      setAvailableSelectedDownloadables(availableDownloadabes);
      setIsLoading(false);
    };

    checkAvailabilities();
  }, [selectedDownloadables, selectedTimeframe]);

  return (
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
        <Loading isLoading={isLoading}>
          {selectedDownloadables.map((downloadable, downloadableIndex) => {
            const downloadNumber = downloadableIndex + 1;
            const isAvailalble = availableSelectedDownloadables.some(
              (d) => d.name === downloadable.name
            );

            return (
              <tr
                className={`hover:bg-base-300 ${
                  !isAvailalble && "line-through bg-base-100"
                }`}
              >
                <th>{downloadNumber}</th>
                <td className="truncate max-w-80 w-80">{downloadable.name}</td>
                <td>{downloadable.symbol}</td>
                <td>{downloadable.source_name}</td>
              </tr>
            );
          })}
        </Loading>
      </tbody>
    </table>
  );
}
