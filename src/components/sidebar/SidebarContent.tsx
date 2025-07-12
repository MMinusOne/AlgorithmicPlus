import { useSidebarState } from "@/lib/state/sidebar";
import BaseChart from "../charting/BaseChart";
import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  SelectedItemType,
  RawDataResponse,
  NewsData,
  ChartingSeries,
} from "@/types";
import { IChartApi } from "lightweight-charts";

interface SidebarData {
  newsData: NewsData[];
  chartingData: ChartingSeries[];
}

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
    const panes = chartRef.current.panes()[1];
    panes.setHeight(1000);
   }, [chartRef])

  return (
    <>
      <div
        className={`w-full flex items-center justify-center ${
          sidebarData.newsData.length === 0 ? "h-[1000px]" : "h-[800px]"
        }`}
      >
        {sidebarData.chartingData.length !== 0 ? (
          <BaseChart chartingData={sidebarData.chartingData} chartApiRef={chartRef}/>
        ) : null}
      </div>
    </>
  );
}
