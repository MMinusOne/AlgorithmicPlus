use std::{fs::File, mem};

use crate::{
    library::providers::downloader::{
        DataType, DownloadData, Downloadable, MarketType, OHLCVJSONFileDataStructure, Source,
        SourceName,
    },
    utils::{classes::logger::LOGGER, date::parse_date_string_to_utc}, APP_HANDLE,
};
use async_recursion::async_recursion;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest;
use serde::{Deserialize, Serialize};
use tauri::http::request;
use std::io::Write;
use uuid::Uuid;
use tauri::Manager;

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
    ) -> Result<(), Box<dyn std::error::Error>> {
        let start_timestamp = match parse_date_string_to_utc(&download_data.start_date) {
            Ok(timestamp) => timestamp,
            Err(error) => {
                LOGGER.error(&format!(
                    "Error formatting start timestamp Binance error: {}",
                    error.to_string()
                ));
                return Err(error.into());
            }
        }
        .timestamp()
            * 1000;
        let end_timestamp = match parse_date_string_to_utc(&download_data.end_date) {
            Ok(timestamp) => timestamp,
            Err(error) => {
                LOGGER.error(&format!(
                    "Error formatting end timestamp Binance error: {}",
                    error.to_string()
                ));
                return Err(error.into());
            }
        }
        .timestamp()
            * 1000;

        let base_url = format!(
            "https://fapi.binance.com/fapi/v1/klines?symbol={}&interval={}&limit={}",
            download_data.symbol, download_data.timeframe, "1000"
        );

        let mut ohlcv_json_data = OHLCVJSONFileDataStructure {
            symbol: download_data.symbol.clone(),
            timeframe: download_data.timeframe,
            start_date: start_timestamp,
            end_date: end_timestamp,
            timestamps: Vec::new(),
            opens: Vec::new(),
            highs: Vec::new(),
            lows: Vec::new(),
            closes: Vec::new(),
            volumes: Vec::new(),
        };

        #[async_recursion]
        async fn download_next_segments(
            current_timestamp: i64,
            start_timestamp: i64,
            end_timestamp: i64,
            base_url: &str,
            ohlcv_json_data: &mut OHLCVJSONFileDataStructure,
        ) {
            if current_timestamp >= end_timestamp {
                return;
            }

            let request_url = format!(
                "{}&{}",
                base_url,
                format!("startTime={}", current_timestamp)
            );

            let data = match reqwest::get(request_url).await {
                Ok(response) => match response.status().is_success() {
                    true => response,
                    false => {
                        LOGGER.error(&"Error making Binance request error, bad status code");
                        return ();
                    }
                },
                Err(error) => {
                    LOGGER.error(&format!(
                        "Error making Binance request error: {}",
                        error.to_string()
                    ));
                    return ();
                }
            };

            let json_data: Vec<RawRequestKline> = match data.json().await {
                Ok(json) => json,
                Err(error) => {
                    LOGGER.error(&format!(
                        "Error making Binance request error: {}",
                        error.to_string()
                    ));
                    return ();
                }
            };

            let app_handle = match APP_HANDLE.get().ok_or("App handle is not initized") {
                Ok(app) => app,
                Err(error) => {
                    LOGGER.error(&format!("App handle is not initilized for Yahoo"));
                    return ();
                }
            };

            let app_data_dir = match app_handle.path().app_data_dir() {
                Ok(dir) => dir,
                Err(error) => {
                    LOGGER.error(&format!("Couldn't get data dir for Yahoo"));
                    return ();
                }
            };

            let download_id = Uuid::new_v4().to_string();
            let base_download_path = app_data_dir.join("raw/ohlcv");
            let json_path = base_download_path.join(format!("{}.json", download_id));
            let bin_path = base_download_path.join(format!("{}.bin", download_id));

            let mut binary_file = match File::create(bin_path) {
                Ok(binary_file) => binary_file,
                Err(error) => {
                    LOGGER.error(&format!(
                        "Couldnt create file for Binance, error: {:?}",
                        error.to_string()
                    ));
                    return ();
                }
            };

            for candle in &json_data {
                let timestamp: i64 = candle.0 as i64;
                let open: f32 = candle.1.parse().expect("Failed to parse open price");
                let high: f32 = candle.2.parse().expect("Failed to parse high price");
                let low: f32 = candle.3.parse().expect("Failed to parse low price");
                let close: f32 = candle.4.parse().expect("Failed to parse close price");
                let volume: f32 = candle.5.parse().expect("Failed to parse volume");

                if timestamp <= end_timestamp {
                    ohlcv_json_data.timestamps.push(timestamp);
                    ohlcv_json_data.opens.push(open);
                    ohlcv_json_data.highs.push(high);
                    ohlcv_json_data.lows.push(low);
                    ohlcv_json_data.closes.push(close);
                    ohlcv_json_data.volumes.push(volume);
                    
                    let candle = OHLCVCandleObject {
                        timestamp, 
                        open,
                        high,
                        low,
                        close,
                        volume
                    };

                    let bytes = unsafe {
                        std::slice::from_raw_parts(
                            &candle as *const OHLCVCandleObject as *const u8,
                            mem::size_of::<OHLCVCandleObject>(),
                        )
                    };

                    match binary_file.write_all(bytes) {
                        Ok(_) => {}
                        Err(error) => {
                            LOGGER.error(&format!("Could not write to binary file Binance, error: {}", error.to_string()));
                        }
                    };
                } else {
                    ohlcv_json_data.start_date = ohlcv_json_data.start_date / 1000;
                    ohlcv_json_data.end_date = ohlcv_json_data.end_date / 1000;
                    let ohlcv_data_string = match serde_json::to_string(ohlcv_json_data) {
                        Ok(data) => data,
                        Err(error) => {
                            LOGGER.error(&format!("Failed serializing Binance data"));
                            return ();
                        }
                    };
                    std::fs::write(&json_path, ohlcv_data_string).expect("Couldn't write JSON file");
                    break;
                }
            }

            download_next_segments(
                (json_data[json_data.len() - 1].0 as i64) as i64,
                start_timestamp,
                end_timestamp,
                base_url,
                ohlcv_json_data,
            )
            .await;
            return ();
        }

        download_next_segments(
            start_timestamp,
            start_timestamp,
            end_timestamp,
            &base_url,
            &mut ohlcv_json_data,
        )
        .await;

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
