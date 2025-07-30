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
mod commands;
mod library;
mod user;
mod utils;

use commands::*;
use once_cell::sync::OnceCell;
use tauri::AppHandle;
use tauri::Manager;

static APP_HANDLE: OnceCell<AppHandle> = OnceCell::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    return tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle();
            APP_HANDLE.set(app_handle.clone()).unwrap();

            let app_data_dir = app.path().app_data_dir()?;

            let directories: Vec<&str> = vec![
                "raw",
                "raw/ohlcv",
                "raw/bidask",
                "raw/news",
                "results",
                "stories",
            ];

            for directory in directories {
                let full_path = app_data_dir.join(directory);
                std::fs::create_dir_all(&full_path).unwrap();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_downloadables,
            get_sources_info,
            download_request,
            downloadable_timeframe_pair_available,
            get_available_sources_timeframes,
            get_static_resources,
            get_raw_data,
            // get_compositions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
