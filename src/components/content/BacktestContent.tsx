import { useSidebarState } from "@/lib/state/sidebar";
import ChartingContent from "../sidebar/content/ChartingContent";
import {
  ChangeEventHandler,
  DetailedHTMLProps,
  InputHTMLAttributes,
  useCallback,
  useEffect,
  useState,
} from "react";
import {
  BacktestDataResponse,
  BacktestResultDataResponse,
  ChartingSeries,
  Metric,
} from "@/types";
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
      let chartSeries: ChartingSeries[];
      switch (graphType) {
        case GraphType.FixedEquity:
          chartSeries = backtest.equity_growth_charting_data;
          break;
        case GraphType.PortfolioPercentage:
          chartSeries = backtest.portfolio_growth_data;
          break;
        case GraphType.TradePercentage:
          chartSeries = backtest.percentage_growth_data;
          break;
      }

      if (chartSeries) {
        // Maybe move this responsability to the backend
        for (const serie of chartSeries) {
          serie.data = serie.data.filter((e, i) => {
            return serie?.data[i + 1]?.time != e.time;
          });
          backtestsChartingData.push(serie);
        }
      }
    }

    setChartingData(backtestsChartingData);
  }, [backtestStrategy, graphType]);

  const setHighestSharpeDisplayOnly = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      if (e.currentTarget.checked) {
        let highestSharpeBacktest: BacktestResultDataResponse | null = null;

        for (const backtest of backtestStrategy?.backtests || []) {
          if (!highestSharpeBacktest) highestSharpeBacktest = backtest;
          const sharpeMetric = backtest.metrics.find(
            (m) => m.key == "SharpeRatio"
          );
          const comparasionSharpeMetric = highestSharpeBacktest.metrics.find(
            (m) => m.key == "SharpeRatio"
          );
          if (!sharpeMetric || !comparasionSharpeMetric) continue;

          const sharpe = Number(sharpeMetric.value);
          const comparasionSharpe = Number(comparasionSharpeMetric.value);

          if (sharpe > comparasionSharpe) highestSharpeBacktest = backtest;
        }

        const newChartingSeries: ChartingSeries[] = [];
        for (const backtest of backtestStrategy?.backtests || []) {
          let chartSeries;
          switch (graphType) {
            case GraphType.FixedEquity:
              chartSeries = backtest.equity_growth_charting_data;
              break;
            case GraphType.PortfolioPercentage:
              chartSeries = backtest.portfolio_growth_data;
              break;
            case GraphType.TradePercentage:
              chartSeries = backtest.percentage_growth_data;
              break;
          }
          for (const chartingSerie of chartSeries) {
            chartingSerie.hidden = backtest.id != highestSharpeBacktest?.id;
            newChartingSeries.push(chartingSerie);
          }
        }

        setChartingData(newChartingSeries);
      } else {
        const newChartingSeries: ChartingSeries[] = [];
        for (const backtest of backtestStrategy?.backtests || []) {
          let chartSeries;
          switch (graphType) {
            case GraphType.FixedEquity:
              chartSeries = backtest.equity_growth_charting_data;
              break;
            case GraphType.PortfolioPercentage:
              chartSeries = backtest.portfolio_growth_data;
              break;
            case GraphType.TradePercentage:
              chartSeries = backtest.percentage_growth_data;
              break;
          }
          for (const chartingSerie of chartSeries) {
            chartingSerie.hidden = false;
            newChartingSeries.push(chartingSerie);
          }
        }

        setChartingData(newChartingSeries);
      }
    },
    [backtestStrategy]
  );

  return (
    <div className="w-full h-screen overflow-hidden overflow-y-scroll flex flex-col gap-8">
      <div className="w-full h-full">
        <div className="h-[50px] w-full flex items-center justify-center p-4">
          <legend className="fieldset-legend">Login options</legend>
          {(chartingData?.length || 0) > 1 ? (
            <>
              <fieldset className="fieldset bg-base-100 border-base-300 rounded-box w-64 border p-4">
                <label className="label">
                  <input
                    onChange={setHighestSharpeDisplayOnly}
                    type="checkbox"
                    defaultChecked
                    className="checkbox"
                  />
                  Highest Sharpe
                </label>
              </fieldset>
            </>
          ) : null}
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
