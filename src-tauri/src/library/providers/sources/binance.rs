use std::{error::Error, fs::File, mem};

use crate::{
    library::providers::downloader::{
        DownloadData, Downloadable, MarketType, OHLCVJSONFileDataStructure, Source,
        SourceName,
    },
    utils::{classes::logger::LOGGER, date::parse_date_string_to_utc, paths::get_app_data_dir},
};
use async_trait::async_trait;
use reqwest;
use serde::{Deserialize, Serialize};
use std::io::Write;
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

    async fn download_ohlcv(&self, download_data: DownloadData) -> Result<(), Box<dyn Error>> {
        let start_timestamp = parse_date_string_to_utc(&download_data.start_date)?.timestamp();
        let end_timestamp = parse_date_string_to_utc(&download_data.end_date)?.timestamp(); 


        let mut ohlcv_json_data = OHLCVJSONFileDataStructure { 
            symbol: download_data.symbol.clone(),
            timeframe: download_data.timeframe.clone(), 
            start_timestamp,
            end_timestamp,
            timestamps: Vec::new(),
            opens: Vec::new(),
            highs: Vec::new(),
            lows: Vec::new(),
            closes: Vec::new(),
            volumes: Vec::new(),
        };

        let download_id = Uuid::new_v4().to_string();
        let app_data_dir = get_app_data_dir()?;

        let base_download_path = app_data_dir.join("raw/ohlcv");
        let json_path = base_download_path.join(format!("{}.json", download_id));
        let bin_path = base_download_path.join(format!("{}.bin", download_id));

        let mut binary_file = File::create(bin_path)?;
        
        loop {
            let mut timestamp = start_timestamp;
            if ohlcv_json_data.timestamps.len() != 0 { 
                timestamp = ohlcv_json_data.timestamps[ohlcv_json_data.timestamps.len()-1];
            }
            let request_url = 
            format!("https://fapi.binance.com/fapi/v1/klines?symbol={}&interval={}&limit=1000&startTime={}", 
            download_data.symbol, download_data.timeframe, timestamp * 1000
        ); 

        
        let raw_klines = reqwest::get(request_url).await?.json::<Vec<RawRequestKline>>().await?;
        
        LOGGER.success(&format!("Downloaded {:?} candles", raw_klines.len()));
        
            for raw_kline_index in 1..raw_klines.len() { 
                let raw_kline = &raw_klines[raw_kline_index];
                let timestamp = raw_kline.0 as i64 / 1000 as i64;
                let open = raw_kline.1.parse::<f32>()?;
                let high = raw_kline.2.parse::<f32>()?;
                let low = raw_kline.3.parse::<f32>()?;
                let close = raw_kline.4.parse::<f32>()?;
                let volume = raw_kline.5.parse::<f32>()?;

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
                    mem::size_of::<OHLCVCandleObject>()
                    )
                };

                binary_file.write_all(bytes)?;
            }

            if raw_klines.len() < 1000 {
                break;
            }
        }

        let ohlcv_data_string = serde_json::to_string(&ohlcv_json_data)?;
        std::fs::write(&json_path, ohlcv_data_string)?;

        Ok(())
    }

    async fn get_downloadables(&self) -> Result<Vec<Downloadable>, Box<dyn Error>> {
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

// I didn't copy this from ChatGPT, I copied this structure from the Binance documentation, Ill maintain my sanity from AI slop thank you very much
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
    pub timestamp: i64,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: f32,
}
