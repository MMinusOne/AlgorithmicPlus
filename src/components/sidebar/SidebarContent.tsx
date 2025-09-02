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
  BacktestDataResponse,
} from "@/types";
import { IChartApi } from "lightweight-charts";
import ChartingContent from "./content/ChartingContent";
import NewsContent from "./content/NewsContent";
import StaticResourceContent from "../content/StaticResourceContent";
import CompositionContent from "../content/CompositionContent";
import BacktestContent from "../content/BacktestContent";

export default function SidebarContent() {
  const { selectedItem } = useSidebarState();

  //TODO: Display download informaton and not just chart (symbol name, downloaded at, start timestamp, end timestamp, download size, download location), news, metrics

  if (selectedItem?.itemType == SelectedItemType.RawData) {
    return <StaticResourceContent />;
  }

  if (selectedItem?.itemType == SelectedItemType.Composition) {
    return <CompositionContent />;
  }

  if (selectedItem?.itemType == SelectedItemType.Backtest) {
    return <BacktestContent />;
  }
}
