use crate::library::providers::downloader::{Downloadable, Downloader, Source};
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};

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
    pub timeframes: Vec<&'static str>
}

#[tauri::command]
pub async fn get_sources_info() -> Vec<SourceInfo> { 
  let mut sources_info: Vec<SourceInfo> = vec![];

  for source in &DOWNLOADER.sources { 
    sources_info.push(SourceInfo { 
    exchange_name: source.source_name(),
    exchange_url: source.source_url(),
    timeframes: source.timeframes()
   })
  }

  return sources_info;
}