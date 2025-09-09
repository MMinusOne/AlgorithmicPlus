import { useSidebarState } from "@/lib/state/sidebar";
import ChartingContent from "../sidebar/content/ChartingContent";
import { useEffect, useState } from "react";
import { BacktestDataResponse, ChartingSeries, Metric } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import BaseChart from "../charting/BaseChart";

enum GraphType {
  FixedEquity = "FixedEquity",
  TradePercentage = "TradePercentage",
  PortfolioPercentage = "PortfolioPercentage",
}

export default function BacktestContent() {
  const { selectedItem } = useSidebarState();

  const [graphType, setGraphType] = useState<GraphType>(
    GraphType.PortfolioPercentage
  );
  const [chartingData, setChartingData] = useState<ChartingSeries[]>();
  const [metrics, setMetrics] = useState<Metric[]>([]);
  const [backtestStrategy, setBacktestStrategy] =
    useState<BacktestDataResponse>();

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

      setBacktestStrategy(backtestStrategyData);
    };

    getBacktestData();
  }, [selectedItem]);

  useEffect(() => {
    if (!backtestStrategy) return;

    const backtestsChartingData: ChartingSeries[] = [];
    
    for (const backtest of backtestStrategy.backtests) {
  
      switch (graphType) {
        case GraphType.FixedEquity:
          backtestsChartingData.push(...backtest.equity_growth_charting_data);
          break;
        case GraphType.PortfolioPercentage:
          backtestsChartingData.push(...backtest.portfolio_growth_data);
          break;
        case GraphType.TradePercentage:
          backtestsChartingData.push(...backtest.percentage_growth_data);
          break;
      }
    }

    setChartingData(backtestsChartingData);
  }, [backtestStrategy, graphType]);

  return (
    <div className="w-full h-screen overflow-hidden overflow-y-scroll flex flex-col gap-8">
      <div className="w-full h-full">
        <div className="h-[50px] w-full flex items-center justify-center p-4">
          <select
            onChange={(e) => {
              setGraphType(e.currentTarget.value as GraphType);
            }}
            value={graphType}
            className="select select-xs"
          >
            {Object.values(GraphType).map((graph) => {
              return <option value={graph}>{graph}</option>;
            })}
          </select>
        </div>
        <div className={`p-4 w-full h-full`}>
          {chartingData !== undefined ? (
            <BaseChart chartingData={chartingData} />
          ) : null}
        </div>
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
