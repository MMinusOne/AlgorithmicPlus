// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod commands;
/**
 * TODO:
 * - Make ALL the metric formulas in utils/formulas
 * - Make the data downloader at lib/downloader.rs, allow to data
 *  from multiple sources and assets, example: Binance, Yahoo
 *  and to download: OHLCV, News, Order Flow
 * - Make the normal backtester at lib/backtest.rs, supporting
 *  concurrency options, data story, etc...
 * - Based on that make the WFO tester with its options
 */
mod library;
mod utils;

use crate::library::providers::{
    downloader::{Downloader, Source},
    sources::binance::Binance,
};
use commands::*;
use serde::{Deserialize, Serialize};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    return tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_downloadables,
            get_sources_info,
            download_request
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
