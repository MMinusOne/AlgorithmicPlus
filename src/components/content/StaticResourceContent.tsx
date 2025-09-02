import { useSidebarState } from "@/lib/state/sidebar";
import ChartingContent from "../sidebar/content/ChartingContent";
import { ChartingSeries, RawDataResponse } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import BaseChart from "../charting/BaseChart";

export default function StaticResourceContent() {
  const { selectedItem } = useSidebarState();

  const [chartingData, setChartingData] = useState<ChartingSeries[]>();

  useEffect(() => {
    const getStaticResource = async () => {
      const rawData = await invoke<RawDataResponse>(
        "get_static_resource_data",
        {
          params: {
            id: selectedItem?.id,
          },
        }
      );

      setChartingData(rawData.charting_data);
    };

    getStaticResource();
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
