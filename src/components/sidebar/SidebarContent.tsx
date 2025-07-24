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
      switch (selectedItem?.type) {
        case SelectedItemType.RawData:
          const data = await invoke<RawDataResponse>("get_raw_data", {
            data: {
              itemType: selectedItem.type,
              id: selectedItem.id,
            },
          });

          setSidebarData({
            symbol: data.symbol,
            timeframe: data.timeframe,
            dataType: data.data_type,
            startTimestamp: data.start_timestamp,
            endTimestamp: data.end_timestamp,
            chartingData: data.charting_data,
            newsData: data.news_data,
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
