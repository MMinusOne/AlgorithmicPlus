import { useSidebarState } from "@/lib/state/sidebar";
import BaseChart from "../charting/BaseChart";
import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  SelectedItemType,
  RawDataResponse,
  NewsData,
  ChartingSeries,
  SidebarData,
  CompositionDataResponse,
} from "@/types";
import { IChartApi } from "lightweight-charts";
import ChartingContent from "./content/ChartingContent";
import NewsContent from "./content/NewsContent";

export default function SidebarContent() {
  const { selectedItem } = useSidebarState();
  // In the future there will be more than raw data responses
  const [sidebarData, setSidebarData] = useState<SidebarData>({
    newsData: [],
    chartingData: [],
  });

  const chartRef = useRef<IChartApi>();

  useEffect(() => {
    const getSidebarData = async () => {
      switch (selectedItem?.itemType) {
        case SelectedItemType.RawData:
          const rawData = await invoke<RawDataResponse>(
            "get_static_resource_data",
            {
              data: {
                id: selectedItem.id,
              },
            }
          );

          setSidebarData({
            symbol: rawData.symbol,
            timeframe: rawData.timeframe,
            startTimestamp: rawData.start_timestamp,
            endTimestamp: rawData.end_timestamp,
            chartingData: rawData.charting_data,
            newsData: rawData.news_data,
          });
          break;

        case SelectedItemType.Composition:
          const compositionData = await invoke<CompositionDataResponse>(
            "get_composition_data",
            {
              data: {
                id: selectedItem.id,
              },
            }
          );
          console.log(compositionData);
          setSidebarData({
            name: compositionData.name,
            description: compositionData.description,
            chartingData: compositionData.charting_data,
            newsData: compositionData.news_data,
          });
          break;
      }
    };

    getSidebarData();
  }, [selectedItem]);

  useEffect(() => {
    if (!chartRef.current) return;
  }, [chartRef]);

  //TODO: Display download informaton and not just chart (symbol name, downloaded at, start timestamp, end timestamp, download size, download location)

  if (sidebarData.chartingData.length > 0) {
    return (
      <div className="w-full h-screen overflow-hidden overflow-y-scroll">
        <ChartingContent sidebarData={sidebarData} chartRef={chartRef} />
        {/* <div className="h-[300px] w-full">
          <div className="flex flex-col">
            <span>Download Symbol: {sidebarData.symbol}</span>
            <span>Timeframe: {sidebarData.timeframe}</span>
            <span>Data Type: {sidebarData.dataType}</span>
            <span>
              Start Date: {new Date(sidebarData.startTimestamp!).getUTCDate()}
            </span>
            <span>
              End Date: {new Date(sidebarData.endTimestamp!).getUTCDate()}
            </span>
          </div>
        </div> */}
      </div>
    );
  }

  if (sidebarData.newsData.length > 0) {
    return <NewsContent />;
  }
}
