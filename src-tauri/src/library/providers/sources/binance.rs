use std::{error::Error, fs::File, mem};

use crate::{
    library::providers::downloader::{
        DataType, DownloadData, Downloadable, MarketType, OHLCVJSONFileDataStructure, Source,
        SourceName,
    },
    utils::{classes::logger::LOGGER, date::parse_date_string_to_utc},
    APP_HANDLE,
};
use async_recursion::async_recursion;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest;
use serde::{Deserialize, Serialize};
use std::io::Write;
use tauri::http::request;
use tauri::Manager;
use uuid::Uuid;

const BINANCE_SYMBOLS_DATA: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/data/static/binance_symbols.json"
));

pub struct Binance {
    source_name: SourceName,
    source_url: String,
    timeframes: Vec<&'static str>,
}
//TODO: fix the download command
#[async_trait]
impl Source for Binance {
    fn source_name(&self) -> SourceName {
        return self.source_name.clone();
    }

    fn source_url(&self) -> &str {
        return &self.source_url;
    }

    fn timeframes(&self) -> Vec<&'static str> {
        return self.timeframes.clone();
    }

    async fn download_ohlcv(
        &self,
        download_data: DownloadData,
    ) -> Result<(), Box<dyn Error>> {
        //TODO

        Ok(())
    }

    async fn get_downloadables(&self) -> Result<Vec<Downloadable>, Box<dyn std::error::Error>> {
        let mut downloadables: Vec<Downloadable> = vec![];

        let symbols: Vec<String> = serde_json::from_str(BINANCE_SYMBOLS_DATA)?;

        for symbol in symbols {
            let symbol_downloadable = Downloadable {
                name: symbol.to_string(),
                symbol: symbol.to_string(),
                source_name: SourceName::Binance,
                market_type: MarketType::Crypto,
            };

            downloadables.push(symbol_downloadable);
        }

        return Ok(downloadables);
    }
}

impl Binance {
    pub fn new() -> Self {
        return Self {
            source_name: SourceName::Binance,
            source_url: "https://binance.com".to_string(),
            timeframes: vec![
                "1s", "1m", "5m", "10m", "15m", "30m", "45m", "1h", "2h", "3h", "4h", "12h", "1d",
                "1W", "1M",
            ],
        };
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct RawRequestKline(
    u64,    // Timestamp
    String, // Open price
    String, // High price
    String, // Low price
    String, // Close price
    String, // Volume
    u64,    // Close Timestamp
    String, // Quote Asset Volume
    u32,    // Number of Trades
    String, // Taker Buy Base Asset Volume
    String, // Taker Buy Quote Asset Volume
    String, // Ignore (can be a String)
);

#[repr(C, packed)]
pub struct OHLCVCandleObject {
    timestamp: i64,
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    volume: f32,
}
