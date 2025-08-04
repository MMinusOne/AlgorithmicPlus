import { ChartingSeries } from "@/types";
import {
  createChart,
  LineSeries,
  CandlestickSeries,
  IChartApi,
  AreaSeries,
  BarSeries,
  HistogramSeries,
  LineWidth,
} from "lightweight-charts";
import { MutableRefObject, useEffect, useRef, useState } from "react";

export default function BaseChart({
  chartingData,
  chartApiRef,
}: {
  chartingData: ChartingSeries[];
  chartApiRef?: MutableRefObject<IChartApi | undefined>;
}) {
  const [storedSeries, setStoredSeries] = useState<any[]>([]);
  const chartContainer = useRef<HTMLDivElement | null>(null);
  chartApiRef = chartApiRef || useRef<IChartApi>();

  useEffect(() => {
    const priceFormatter = Intl.NumberFormat("en", {
      notation: "compact",
    });

    if (!chartContainer.current) return;
    const chart = createChart(chartContainer.current, {
      autoSize: true,
      localization: {
        priceFormatter: (price: number) => {
          if (Math.abs(price) > 10_000) {
            return priceFormatter.format(price);
          } else {
            return price.toFixed(3);
          }
        },
      },

      grid: { horzLines: { visible: true } },
      layout: {
        attributionLogo: false,
        panes: {
          enableResize: true,
        },
      },
    });

    chartApiRef.current = chart;
  }, [chartContainer]);

  useEffect(() => {
    if (!chartContainer.current || !chartApiRef.current) return;
    const chart = chartApiRef.current;

    for (const storedSerieIndex in storedSeries) {
      if (storedSeries[storedSerieIndex]) {
        chart.removeSeries(storedSeries[storedSerieIndex]);
      }

      setStoredSeries([]);
    }

    for (const chartingSerieIndex in chartingData || []) {
      const chartingSerie = chartingData[chartingSerieIndex];
      //@ts-ignore
      const paneIndex: number = Number.isNaN(chartingSerie.pane)
        ? Number(chartingSerieIndex)
        : chartingSerie.pane;

      const SeriesTypes = {
        ohlcv: CandlestickSeries,
        area: AreaSeries,
        bar: BarSeries,
        histogram: HistogramSeries,
        line: LineSeries,
      };

      const series = chart.addSeries(
        SeriesTypes[chartingSerie.chart_type],
        {
          title: chartingSerie.title,
          lineWidth: 0.75 as LineWidth,
        },
        paneIndex
      );

      series.setData(chartingSerie.data);
      series.priceScale().applyOptions({
        autoScale: false,
      });

      setStoredSeries((prev) => [...prev, series]);

      if (chartingSerie.height) {
        const pane = chart.panes()[paneIndex];
        pane.setHeight(chartingSerie.height);
      }
    }

    chart.timeScale().fitContent();
  }, [chartContainer, chartingData, chartApiRef]);

  return (
    <>
      <div ref={chartContainer} className="w-full h-full" />
    </>
  );
}
