use crate::{
    library::providers::downloader::{
        DataType, DownloadData, Downloadable, MarketType, OHLCVJSONFileDataStructure, Source,
        SourceName,
    },
    utils::{classes::logger::LOGGER, date::parse_date_string_to_offsettime},
    APP_HANDLE,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{fs::File, io::Write, mem};
use tauri::Manager;
use yahoo_finance_api::{self as yahoo, YahooConnector};

const YAHOO_SYMBOLS_DATA: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/data/static/yahoo_symbols.json"
));

pub struct Yahoo {
    source_name: SourceName,
    source_url: String,
    timeframes: Vec<&'static str>,
}

#[async_trait]
impl Source for Yahoo {
    fn source_name(&self) -> SourceName {
        return self.source_name.clone();
    }

    fn source_url(&self) -> &str {
        return &self.source_url;
    }

    fn timeframes(&self) -> Vec<&str> {
        return self.timeframes.clone();
    }
    //TODO: change all download's to download_ohlcv
    async fn download(&self, download_data: DownloadData) -> Option<String> {
        if let Ok(yahoo_connector) = yahoo::YahooConnector::new() {
            let start_date = match parse_date_string_to_offsettime(&download_data.start_date) {
                Ok(date) => date,
                Err(_) => return None,
            };
            let end_date = match parse_date_string_to_offsettime(&download_data.end_date) {
                Ok(date) => date,
                Err(_) => return None,
            };
            match yahoo_connector
                .get_quote_history(&download_data.symbol, start_date, end_date)
                .await
            {
                Ok(yahoo_response) => {
                    match yahoo_response.quotes() {
                        Ok(yahoo_quotes) => {
                            let download_path = String::new();
                            let mut ohlcv_json_data = OHLCVJSONFileDataStructure {
                                symbol: download_data.symbol.clone(),
                                timeframe: download_data.timeframe,
                                start_date: start_date.unix_timestamp(),
                                end_date: end_date.unix_timestamp(),
                                timestamps: Vec::new(),
                                opens: Vec::new(),
                                highs: Vec::new(),
                                lows: Vec::new(),
                                closes: Vec::new(),
                                volumes: Vec::new(),
                            };

                            let download_id = uuid::Uuid::new_v4().to_string();

                            let app_handle =
                                match APP_HANDLE.get().ok_or("App handle is not initized") {
                                    Ok(app) => app,
                                    Err(error) => {
                                        LOGGER.error(&format!(
                                            "App handle is not initilized for Yahoo"
                                        ));
                                        return None;
                                    }
                                };

                            let app_data_dir = match app_handle.path().app_data_dir() {
                                Ok(dir) => dir,
                                Err(error) => {
                                    LOGGER.error(&format!("Couldn't get data dir for Yahoo"));
                                    return None;
                                }
                            };

                            let base_download_path = app_data_dir.join("raw/ohlcv");
                            let json_path = base_download_path.join(format!("{}.json", download_id));
                            let bin_path = base_download_path.join(format!("{}.bin", download_id));
                            
                            let mut binary_file = match File::create(bin_path) {
                                Ok(binary_file) => binary_file,
                                Err(error) => {
                                    LOGGER.error(&format!(
                                        "Couldnt create file for Yahoo symbol: {:?}, error: {:?}",
                                        download_data.symbol,
                                        error.to_string()
                                    ));
                                    return None;
                                }
                            };

                            for quote in yahoo_quotes {
                                let timestamp = quote.timestamp as u64;
                                let open = quote.open as f32;
                                let high = quote.high as f32;
                                let low = quote.low as f32;
                                let close = quote.close as f32;
                                let volume = quote.volume as u32;

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
                                    volume,
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
                                        LOGGER.error(&format!("Could not write to binary file Yahoo, symbol: {}, error: {}", download_data.symbol, error.to_string()));
                                    }
                                };
                            }

                            let ohlcv_data_string = match serde_json::to_string(&ohlcv_json_data) {
                                Ok(data) => data,
                                Err(error) => {
                                    LOGGER.error(&format!(
                                        "Couldn't serialize Yahoo symbol: {:?}, error: {:?}",
                                        download_data.symbol,
                                        error.to_string()
                                    ));

                                    return None;
                                }
                            };

                            match std::fs::write(&json_path, ohlcv_data_string) {
                                Ok(_) => {}
                                Err(error) => LOGGER.error(&format!(
                                    "Error writing to Yahoo symbol json path: {:?}, error: {}",
                                    json_path,
                                    error.to_string()
                                )),
                            };

                            return Some(download_path);
                        }

                        Err(error) => {
                            LOGGER.error(&format!(
                                "Couldn't get the quotes for Yahoo symbol: {:?}, error: {:?}",
                                download_data.symbol,
                                error.to_string()
                            ));

                            return None;
                        }
                    };
                }
                Err(error) => {
                    LOGGER.error(&format!(
                        "Error while downloading from Yahoo, symbol: {}, error: {:?}",
                        download_data.symbol,
                        error.to_string()
                    ));

                    return None;
                }
            };
        } else {
            return None;
        }
    }

    async fn get_downloadables(&self) -> Result<Vec<Downloadable>, Box<dyn std::error::Error>> {
        let mut downloadables: Vec<Downloadable> = vec![];

        let symbols: Vec<SymbolCell> = serde_json::from_str(YAHOO_SYMBOLS_DATA)?;

        for symbol in symbols {
            let symbol_downloadable = Downloadable {
                name: symbol.name,
                symbol: symbol.symbol,
                source_name: SourceName::YahooFinance,
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
            source_name: SourceName::YahooFinance,
            source_url: "https://finance.yahoo.com/".to_string(),
            timeframes: vec!["1d"],
        };
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SymbolCell {
    name: String,
    symbol: String,
}

#[repr(C, packed)]
pub struct OHLCVCandleObject {
    timestamp: u64,
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    volume: u32,
}
