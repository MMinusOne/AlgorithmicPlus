import {
  AreaData,
  BarData,
  CandlestickData,
  HistogramData,
  LineData,
  Time,
  WhitespaceData,
} from "lightweight-charts";

export enum Dialog {
  Environment,
  Download,
  Warning,
  Error,
}

export enum MarketType {
  Crypto = "Crypto",
  Stock = "Stock",
  Futures = "Futures",
}

export enum MarketDataType {
  OHLCV = "OHLCV",
  BidAsk = "Bid/Ask",
  News = "News",
  Economics = "Economics",
}

export interface Downloadable {
  name: string;
  symbol: string;
  source_name: string;
  market_type: string;
}

export enum DownloadDialogMenu {
  DownloadablesList,
  Download,
  DownloadProgress,
}

export interface SourceInfo {
  name: string;
  url: string;
  timeframes: string[];
}

export interface DownloadedMetadata {
  symbol: string;
  timeframe: string;
  start_date: string;
  end_date: string;
  id: string;
}

export type ChartingSeries =
  | {
      chart_type: "ohlcv";
      data: (CandlestickData<Time>)[];
    }
  | {
      chart_type: "line";
      data: (LineData<Time>)[];
    }
  | {
      chart_type: "bar";
      data: (WhitespaceData<Time> | BarData<Time>)[];
    }
  | {
      chart_type: "histogram";
      data: (HistogramData<Time>)[];
    }
  | {
      chart_type: "area";
      data: (AreaData<Time>)[];
    };

export interface NewsData {}

export enum SelectedItemType {
  RawData = "raw_data",
}

export interface RawDataResponse {
  news_data: NewsData[];
  charting_data: ChartingSeries[];
}