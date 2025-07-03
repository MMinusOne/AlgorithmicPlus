use async_trait::async_trait;
use reqwest::get;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const BINANCE_SYMBOLS_DATA: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/data/misc/binance_symbols.json"
));


use crate::{
    library::providers::downloader::{DataType, Downloadable, MarketType, Source, SourceName},
    utils::classes::logger::LOGGER,
};

pub struct Binance {
    source_name: String,
    source_url: String,
    timeframes: Vec<&'static str>,
}

#[async_trait]
impl Source for Binance {
    fn source_name(&self) -> &str {
        return &self.source_name;
    }

    fn source_url(&self) -> &str {
        return &self.source_url;
    }

    fn timeframes(&self) -> Vec<&'static str> {
        return self.timeframes.clone();
    }

    async fn download(&self) -> Option<String> {
        return None;
    }

    async fn get_downloadables(&self) -> Result<Vec<Downloadable>, Box<dyn std::error::Error>> {
        let mut downloadables: Vec<Downloadable> = vec![];

        let symbols: Vec<String> = serde_json::from_str(BINANCE_SYMBOLS_DATA)?;

        for symbol in symbols {
            let symbol_downloadable = Downloadable {
                name: symbol.to_string(),
                symbol: symbol.to_string(),
                source: SourceName::Binance,
                market_type: MarketType::Crypto
            };

            downloadables.push(symbol_downloadable);
        }

        return Ok(downloadables);
    }
}

impl Binance {
    pub fn new() -> Self {
        return Self {
            source_name: "Binance".to_string(),
            source_url: "https://binance.com".to_string(),
            timeframes: vec![
                "1s", "1m", "5m", "10m", "15m", "30m", "45m", "1h", "2h", "3h", "4h", "12h", "1d",
                "1W", "1M",
            ],
        };
    }
}
