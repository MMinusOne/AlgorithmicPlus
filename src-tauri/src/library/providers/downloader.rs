use crate::{
    library::providers::sources::{binance::Binance, yahoo::{SymbolCell, Yahoo}},
    utils::classes::logger,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use yahoo_finance::Bar;

#[derive(Debug, Serialize, Deserialize)]
pub enum SourceName {
    YahooFinance,
    Binance,
    Okx,
    Marketaux,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MarketType {
    Crypto,
    Stock,
    Forex,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DataType {
    OHLCV,
    News,
    OrderFlow,
    BidAsk,
}

#[async_trait]
pub trait Source: Send + Sync {
    fn source_name(&self) -> &str;
    fn source_url(&self) -> &str;
    fn timeframes(&self) -> Vec<&str>;
    async fn download(&self, symbol: String, timeframe: String, limit: i128)
        -> Option<String>;
    // fn format_raw_data(&self, data: Vec<Bar>) -> Vec<Vec<f64>>;
    async fn get_downloadables(&self) -> Result<Vec<Downloadable>, Box<dyn std::error::Error>>;
}

pub struct Downloader {
    pub sources: Vec<Box<dyn Source>>,
}

impl Downloader {
    pub fn new() -> Self {
        let yahoo = Box::new(Yahoo::new());
        let binance = Box::new(Binance::new());
        return Self {
            sources: vec![yahoo, binance],
        };
    }
    pub async fn get_downloadables(&self) -> Vec<Downloadable> {
        let mut downloadables: Vec<Downloadable> = vec![];

        for source in &self.sources {
            match source.get_downloadables().await {
                Ok(source_downloadables) => downloadables.extend(source_downloadables),
                Err(err) => logger::LOGGER.error(&err.to_string()),
            }
        }

        return downloadables;
    }

    pub fn get_source(&self, source_name_or_url: &str) -> Option<&dyn Source> {
        for source in &self.sources {
            if source.source_name() == source_name_or_url
                || source.source_url() == source_name_or_url
            {
                return Some(source.as_ref());
            }
        }

        return None;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Downloadable {
    pub name: String,
    pub symbol: String,
    pub source: SourceName,
    pub market_type: MarketType,
    pub data_type: DataType,
}
