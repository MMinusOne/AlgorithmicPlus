use std::sync::Arc;

use crate::{
    library::providers::downloader::{
        DownloadData, Downloadable, Downloader, OHLCVJSONFileDataStructure, Source, SourceName,
    },
    APP_HANDLE,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tauri::{Emitter, Manager};
use uuid::Uuid;
// use uuid::Uuid;

lazy_static! {
    static ref DOWNLOADER: Downloader = Downloader::new();
}

#[tauri::command]
pub async fn get_downloadables() -> Vec<Downloadable> {
    return DOWNLOADER.get_downloadables().await;
}

#[derive(Serialize, Deserialize)]
pub struct SourceInfo {
    pub name: SourceName,
    pub url: &'static str,
    pub timeframes: Vec<&'static str>,
}

#[tauri::command]
pub fn get_sources_info() -> Vec<SourceInfo> {
    let mut sources_info: Vec<SourceInfo> = vec![];

    for source in DOWNLOADER.sources.values() {
        sources_info.push(SourceInfo {
            name: source.source_name(),
            url: source.source_url(),
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

#[derive(Serialize, Deserialize, Clone)]
pub struct DownloadProgressResponse {
    pub download_id: String,
    pub download_progress: f32,
}

#[tauri::command]
pub async fn download_request(
    app: tauri::AppHandle,
    data: DownloadRequestParams,
) -> DownloadRequestResponse {
    let download_id = Uuid::new_v4().to_string();
    let status = "OK".to_string();
    //TODO: make it download the data, normilization feature for charts
    let response = DownloadRequestResponse {
        download_id: download_id.clone(),
        status,
    };
    let mut download_datas: Vec<DownloadData> = vec![];

    for downloadable in data.downloadables {
        let download_data = DownloadData {
            symbol: downloadable.symbol,
            source_name: downloadable.source_name,
            timeframe: data.timeframe.clone(),
            data_types: data.data_types.clone(),
            start_date: data.start_date.clone(),
            end_date: data.end_date.clone(),
        };

        download_datas.push(download_data);
    }

    let app_handle = app.clone();
    let download_id = Arc::new(download_id);

    let on_progress = {
        let download_id = Arc::clone(&download_id);
        move |download_progress: f32| {
            app_handle
                .emit(
                    "download_progress",
                    DownloadProgressResponse {
                        download_id: (*download_id).clone(),
                        download_progress,
                    },
                )
                .unwrap();
        }
    };

    DOWNLOADER
        .download_all(download_datas, None, Some(on_progress))
        .await;

    return response;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DownloadableTimeframePairAvailableRequestData {
    downloadable: Downloadable,
    timeframe: String,
}

#[tauri::command]
pub async fn downloadable_timeframe_pair_available(
    data: DownloadableTimeframePairAvailableRequestData,
) -> bool {
    let source_name = data.downloadable.source_name.clone();

    match DOWNLOADER.sources.get(&source_name) {
        Some(source) => {
            let timeframes = source.timeframes();
            if timeframes.contains(&data.timeframe.as_str()) {
                return true;
            } else {
                return false;
            }
        }
        None => {
            return false;
        }
    }
}

#[tauri::command]
pub async fn get_available_sources_timeframes() -> Vec<String> {
    let mut all_timeframes: Vec<String> = vec![];

    for source in DOWNLOADER.sources.values() {
        for timeframe in &source.timeframes() {
            if !all_timeframes.contains(&timeframe.to_string()) {
                all_timeframes.push(timeframe.to_string());
            }
        }
    }

    return all_timeframes;
}



