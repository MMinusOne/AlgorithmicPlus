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

// Add more types the more I have 
export type StaticResource = OHLCVStaticResource;

export type OHLCVStaticResource = {
  id: string;
  name: string;
  symbol: string;
  timeframe: string;
  start_timestamp: string;
  end_timestamp: string;
};

export type ChartingSeries =
  | {
      chart_type: "ohlcv";
      height?: number;
      data: CandlestickData<Time>[];
    }
  | {
      chart_type: "line";
      height?: number;
      data: LineData<Time>[];
    }
  | {
      chart_type: "bar";
      height?: number;
      data: (WhitespaceData<Time> | BarData<Time>)[];
    }
  | {
      chart_type: "histogram";
      height?: number;
      data: HistogramData<Time>[];
    }
  | {
      chart_type: "area";
      height?: number;
      data: AreaData<Time>[];
    };

export interface NewsData {}

export enum SelectedItemType {
  RawData = "RAWDATA",
  Composition = "COMPOSITION",
  Backtest = "BACKTEST",
}

export interface RawDataResponse {
  symbol?: string;
  timeframe?: string;
  start_timestamp?: number;
  end_timestamp?: number;
  data_type: string;
  news_data: NewsData[];
  charting_data: ChartingSeries[];
}

export interface SidebarData {
  symbol?: string;
  timeframe?: string;
  startTimestamp?: number;
  endTimestamp?: number;
  dataType?: string;
  newsData: NewsData[];
  chartingData: ChartingSeries[];
}

export interface CompositionMetadata { 
  id: string;
  name: string;
  description: string;
}