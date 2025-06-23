use crate::{library::providers::downloader::{DataType, Downloadable, MarketType, Source, SourceName}, utils::classes::logger::LOGGER};
use serde::{Deserialize, Serialize};
use serde_json::{json};
use std::{collections::HashMap, os::windows::fs::MetadataExt, time::UNIX_EPOCH, time::SystemTime};
use yahoo_finance::{history, Bar};
use async_trait::async_trait;


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

    async fn download(&self, symbol: String, timeframe: String, limit: i128) -> Option<String> {
        let download_path = format!(
            "{}/src/data/data-stories/ohlcv/{}.json", 
            env!("CARGO_MANIFEST_DIR"), 
            symbol
        );
    
        let result = history::retrieve(&symbol)
            .await
            .map_err(|error| {
                let error_msg = format!(
                    "Error while downloading symbol: {}, timeframe: {}, limit: {}, from YahooFinance, error: {}",
                    symbol, timeframe, limit, error
                );
                LOGGER.error(&error_msg);
            })
            .and_then(|data| {
                let serializable_data: Vec<SerializableBar> = data
                    .into_iter()
                    .map(SerializableBar::from)
                    .collect();
    
                serde_json::to_string(&serializable_data)
                    .map_err(|error| {
                        LOGGER.error(&format!("Error serializing data: {}", error));
                    })
            })
            .and_then(|serialized_data| {
                std::fs::write(&download_path, &serialized_data)
                    .map_err(|error| {
                        LOGGER.error(&format!("Error writing file: {}", error));
                    })
                    .map(|_| download_path.clone())
            });
    
        return result.ok()
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
                data_type: DataType::OHLCV
            };

            downloadables.push(symbol_downloadable);
        }

        return Ok(downloadables);
    }
}

impl Yahoo {
    pub fn new() -> Self {
        return Self {
            source_name: "Yahoo Finance".to_string(),
            source_url: "https://finance.yahoo.com/".to_string(),
            timeframes: vec!["1d"]
        };
    }
}
