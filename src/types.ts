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
  OrderBook = "Order Book",
  BidAsk = "Bid/Ask",
  News = "News",
  Economics = "Economics",
}

export interface Downloadable {
  name: string;
  symbol: string;
  source: string;
  market_type: string;
  data_type: string;
}

export enum DownloadDialogMenu {
  DownloadablesList,
  Download,
}

export interface SourceInfo {
  source_name: string;
  source_url: string;
  timeframes: string[];
}
