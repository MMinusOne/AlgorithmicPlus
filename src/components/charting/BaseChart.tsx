import { ChartingSeries } from "@/types";
import {
  createChart,
  LineSeries,
  CandlestickSeries,
  IChartApi,
  AreaSeries,
  BarSeries,
  HistogramSeries,
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

      switch (chartingSerie.chart_type) {
        case "ohlcv":
          const candlestickSeries = chart.addSeries(
            CandlestickSeries,
            {},
            paneIndex
          );
          candlestickSeries.setData(chartingSerie.data);
          candlestickSeries.priceScale().applyOptions({
            autoScale: false,
          });
          setStoredSeries((prev) => [...prev, candlestickSeries]);
          break;
        case "area":
          const areaSeries = chart.addSeries(AreaSeries, {}, paneIndex);
          areaSeries.setData(chartingSerie.data);
          setStoredSeries((prev) => [...prev, areaSeries]);
          break;
        case "bar":
          const barSeries = chart.addSeries(BarSeries, {}, paneIndex);
          barSeries.setData(chartingSerie.data);
          setStoredSeries((prev) => [...prev, barSeries]);
          break;
        case "histogram":
          const histogramSeries = chart.addSeries(
            HistogramSeries,
            {},
            paneIndex
          );
          histogramSeries.setData(chartingSerie.data);
          setStoredSeries((prev) => [...prev, histogramSeries]);
          break;
        case "line":
          const lineSeries = chart.addSeries(LineSeries, {}, paneIndex);
          lineSeries.setData(chartingSerie.data);
          setStoredSeries((prev) => [...prev, lineSeries]);
          break;
      }

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
