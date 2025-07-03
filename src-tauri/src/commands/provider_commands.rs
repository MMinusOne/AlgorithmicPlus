use std::sync::Arc;

use crate::library::providers::downloader::{DownloadData, Downloadable, Downloader, Source};
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

#[derive(Serialize, Deserialize, Clone)]
pub struct DownloadProgressResponse {
    pub download_id: String,
    pub download_progress: usize,
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
        println!("Executed");
        let download_id = Arc::clone(&download_id);
        move |download_progress: usize| {
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
