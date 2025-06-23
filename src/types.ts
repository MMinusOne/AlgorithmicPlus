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
  OHLCV,
  Spread,
  OrderFlow,
  BidAsk,
  News,
  Economics,
}

export interface Downloadable {
  name: string;
  symbol: string;
  source: string;
  market_type: string;
  data_type: string;
}
