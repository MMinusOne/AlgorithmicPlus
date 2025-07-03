use crate::{
    library::providers::downloader::{DataType, Downloadable, MarketType, Source, SourceName},
    utils::classes::logger::LOGGER,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, os::windows::fs::MetadataExt, time::SystemTime, time::UNIX_EPOCH};
use yahoo_finance::{history, Bar};

const YAHOO_SYMBOLS_DATA: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/data/misc/yahoo_symbols.json"
));

pub struct Yahoo {
    source_name: String,
    source_url: String,
    timeframes: Vec<&'static str>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SymbolCell {
    name: String,
    symbol: String,
}

#[derive(Serialize, Deserialize)]
struct SerializableBar {
    timestamp: i64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: Option<u64>,
}

impl From<Bar> for SerializableBar {
    fn from(bar: Bar) -> Self {
        SerializableBar {
            timestamp: bar.timestamp,
            open: bar.open,
            high: bar.high,
            low: bar.low,
            close: bar.close,
            volume: bar.volume,
        }
    }
}

#[async_trait]
impl Source for Yahoo {
    fn source_name(&self) -> &str {
        return &self.source_name;
    }

    fn source_url(&self) -> &str {
        return &self.source_url;
    }

    fn timeframes(&self) -> Vec<&str> {
        return self.timeframes.clone();
    }

    // fn format_raw_data(&self, data: Vec<Bar>) -> Vec<Vec<f64>> {
    //     let mut candles: Vec<Vec<f64>> = vec![];

    //     for candle in data {
    //      candles.push([candle.timestamp,
    //          candle.open,
    //           candle.high,
    //            candle.low,
    //             candle.close,
    //              candle.volume]);
    //     }

    //     return candles;
    // }

    async fn download(&self) -> Option<String> {
        return None;
    }

    async fn get_downloadables(&self) -> Result<Vec<Downloadable>, Box<dyn std::error::Error>> {
        let mut downloadables: Vec<Downloadable> = vec![];

        let symbols: Vec<SymbolCell> = serde_json::from_str(YAHOO_SYMBOLS_DATA)?;

        for symbol in symbols {
            let symbol_downloadable = Downloadable {
                name: symbol.name,
                symbol: symbol.symbol,
                source: SourceName::YahooFinance,
                market_type: MarketType::Stock,
            };

            downloadables.push(symbol_downloadable);
        }

        return Ok(downloadables);
    }
}

impl Yahoo {
    pub fn new() -> Self {
        return Self {
            source_name: "YahooFinance".to_string(),
            source_url: "https://finance.yahoo.com/".to_string(),
            timeframes: vec!["1d"],
        };
    }
}
