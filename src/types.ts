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
  DownloadProgress,
}

export interface SourceInfo {
  exchange_name: string;
  exchange_url: string;
  timeframes: string[];
}
