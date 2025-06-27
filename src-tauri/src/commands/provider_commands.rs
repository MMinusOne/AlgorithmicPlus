use std::any;

use crate::library::providers::downloader::{Downloadable, Downloader, Source};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref DOWNLOADER: Downloader = Downloader::new();
}

#[tauri::command]
pub async fn get_downloadables() -> Vec<Downloadable> {
    return DOWNLOADER.get_downloadables().await;
}

#[derive(Serialize, Deserialize)]
pub struct SourceInfo {
    pub exchange_name: &'static str,
    pub exchange_url: &'static str,
    pub timeframes: Vec<&'static str>,
}

#[tauri::command]
pub fn get_sources_info() -> Vec<SourceInfo> {
    let mut sources_info: Vec<SourceInfo> = vec![];

    for source in &DOWNLOADER.sources {
        sources_info.push(SourceInfo {
            exchange_name: source.source_name(),
            exchange_url: source.source_url(),
            timeframes: source.timeframes(),
        })
    }

    return sources_info;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadRequestParams {
    pub downloadables: Vec<Downloadable>,
    pub data_types: Vec<String>,
    pub timeframe: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Serialize, Deserialize)]
pub struct DownloadRequestResponse {
    pub download_id: String,
    pub status: String,
}

#[tauri::command]
pub async fn download_request(data: DownloadRequestParams) -> DownloadRequestResponse {
    //TODO: make it download the data
    let response = DownloadRequestResponse {
        download_id: "hi".to_string(),
        status: "OK".to_string(),
    };

    println!("{:?}", data);

    return response;
}
