import { useSidebarState } from "@/lib/state/sidebar";
import ChartingContent from "../sidebar/content/ChartingContent";
import { ChartingSeries, CompositionDataResponse } from "@/types";
import { useEffect, useState } from "react";
import BaseChart from "../charting/BaseChart";
import { invoke } from "@tauri-apps/api/core";

export default function CompositionContent() {
  const { selectedItem } = useSidebarState();

  const [chartingData, setChartingData] = useState<ChartingSeries[]>();

  useEffect(() => {
    const getCompositionData = async () => {
      const compositionData = await invoke<CompositionDataResponse>(
        "get_composition_data",
        {
          params: {
            id: selectedItem?.id,
          },
        }
      );
      setChartingData(compositionData.charting_data);
    };

    getCompositionData();
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
