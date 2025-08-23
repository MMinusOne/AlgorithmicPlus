use crate::library::providers::downloader::{DownloadData, Downloadable, Downloader, SourceName};
use fuzzy_matcher::FuzzyMatcher;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::Emitter;
use uuid::Uuid;

lazy_static! {
    static ref DOWNLOADER: Downloader = Downloader::new();
}

#[tauri::command]
pub async fn get_downloadables() -> Result<Vec<Downloadable>, tauri::Error> {
    Ok(DOWNLOADER.get_downloadables().await)
}

#[derive(Serialize, Deserialize)]
pub struct SearchDownloadablesParams {
    query: String,
}

#[tauri::command]
pub async fn search_downloadables(
    params: SearchDownloadablesParams,
) -> Result<Vec<Downloadable>, tauri::Error> {
    let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();
    let query = params.query.to_lowercase();

    let downloadables = DOWNLOADER.get_downloadables().await;
    let relevant_downloadables: Vec<_> = downloadables
        .iter()
        .filter_map(|downloadable| {
            if downloadable.symbol.to_lowercase() == query {
                return Some((downloadable, i64::MAX));
            }

            if downloadable.symbol.starts_with(&query) {
                return Some((downloadable, i64::MAX - 1));
            }

            if downloadable.symbol.contains(&query) {
                return Some((downloadable, i64::MAX - 2));
            }

            return matcher
                .fuzzy_match(&downloadable.symbol, &params.query)
                .map(|score| (downloadable, score));
        })
        .collect();

    println!("{:?}", relevant_downloadables);

    Ok(relevant_downloadables
        .iter()
        .map(|tuple| tuple.0.clone())
        .collect())
}

#[derive(Serialize, Deserialize)]
pub struct SourceInfo {
    pub name: SourceName,
    pub url: &'static str,
    pub timeframes: Vec<&'static str>,
}

#[tauri::command]
pub fn get_sources_info() -> Result<Vec<SourceInfo>, tauri::Error> {
    let mut sources_info: Vec<SourceInfo> = vec![];

    for source in DOWNLOADER.sources.values() {
        sources_info.push(SourceInfo {
            name: source.source_name(),
            url: source.source_url(),
            timeframes: source.timeframes(),
        })
    }

    Ok(sources_info)
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
) -> Result<DownloadRequestResponse, tauri::Error> {
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

    Ok(response)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DownloadableTimeframePairAvailableRequestParams {
    downloadable: Downloadable,
    timeframe: String,
}

#[tauri::command]
pub async fn downloadable_timeframe_pair_available(
    params: DownloadableTimeframePairAvailableRequestParams,
) -> Result<bool, tauri::Error> {
    let source_name = params.downloadable.source_name.clone();

    match DOWNLOADER.sources.get(&source_name) {
        Some(source) => {
            let timeframes = source.timeframes();
            if timeframes.contains(&params.timeframe.as_str()) {
                return Ok(true);
            } else {
                return Ok(false);
            }
        }
        None => return Ok(false),
    }
}

#[tauri::command]
pub async fn get_available_sources_timeframes() -> Result<Vec<String>, tauri::Error> {
    let mut all_timeframes: Vec<String> = vec![];

    for source in DOWNLOADER.sources.values() {
        for timeframe in &source.timeframes() {
            if !all_timeframes.contains(&timeframe.to_string()) {
                all_timeframes.push(timeframe.to_string());
            }
        }
    }

    Ok(all_timeframes)
}
