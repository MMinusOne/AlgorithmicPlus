import BaseChart from "@/components/charting/BaseChart";
import { SidebarData } from "@/types";
import { IChartApi } from "lightweight-charts";
import { MutableRefObject } from "react";

export default function ChartingContent({
  sidebarData,
  chartRef,
}: {
  sidebarData: SidebarData;
  chartRef: MutableRefObject<IChartApi | undefined>;
}) {
  return (
    <>
      <div className={`w-full h-full p-4`}>
        <BaseChart
          chartingData={sidebarData.chartingData}
          chartApiRef={chartRef}
        />
      </div>
    </>
  );
}
