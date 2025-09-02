import { useSidebarState } from "@/lib/state/sidebar";
import ChartingContent from "../sidebar/content/ChartingContent";
import { useEffect, useState } from "react";
import { BacktestDataResponse, ChartingSeries } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import BaseChart from "../charting/BaseChart";

export default function BacktestContent() {
  const { selectedItem } = useSidebarState();

  const [chartingData, setChartingData] = useState<ChartingSeries[]>();

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
    </div>
  );
}
