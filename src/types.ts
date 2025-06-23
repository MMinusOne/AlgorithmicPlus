export enum Dialog {
  Environment,
  Download,
  Error,
}

export enum MarketType {
  Crypto,
  Stock,
  Futures,
}

export enum MarketDataType {
  OHLCV = "OHLCV",
  Spread = "Spread",
  OrderFlow = "Order Flow", 
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
