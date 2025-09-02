import { useSidebarState } from "@/lib/state/sidebar";
import ChartingContent from "../sidebar/content/ChartingContent";
import { useEffect, useState } from "react";
import { BacktestDataResponse, ChartingSeries, Metric } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import BaseChart from "../charting/BaseChart";

export default function BacktestContent() {
  const { selectedItem } = useSidebarState();

  const [chartingData, setChartingData] = useState<ChartingSeries[]>();
  const [metrics, setMetrics] = useState<Metric[]>([]);

  useEffect(() => {
    const getBacktestData = async () => {
      const backtestStrategyData = await invoke<BacktestDataResponse>(
        "backtest_strategy",
        {
          params: {
            id: selectedItem?.id,
          },
        }
      );

      // setChartingData(backtestStrategyData);
      setMetrics(backtestStrategyData.metrics);
    };

    getBacktestData();
  }, [selectedItem]);

  return (
    <div className="w-full h-screen overflow-hidden overflow-y-scroll">
      <div className={`w-full h-full p-4`}>
        {chartingData !== undefined ? (
          <BaseChart chartingData={chartingData} />
        ) : null}
      </div>

      <div className="w-full h-[200px]">
        <div className="p-4 flex flex-col">
          <span className="font-semibold text-2xl">Metrics</span>
          <div className="p-4">
            <div className="overflow-x-auto">
              <table className="table table-xs">
                <thead>
                  <tr>
                    <th></th>
                    <th>Metric</th>
                    <th>Value</th>
                  </tr>
                </thead>
                <tbody>
                  {metrics.map((metric, metricIndex) => {
                    return (
                      <tr key={metric.key}>
                        <th>{metricIndex}</th>
                        <td>{metric.key}</td>
                        <td>{metric.value}</td>
                      </tr>
                    );
                  })}
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
