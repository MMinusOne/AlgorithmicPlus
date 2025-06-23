use crate::library::providers::downloader::{Downloadable, Downloader};
use lazy_static::lazy_static;

lazy_static! {
    static ref DOWNLOADER: Downloader = Downloader::new();
}

#[tauri::command]
pub async fn get_downloadables() -> Vec<Downloadable> { 
   return DOWNLOADER.get_downloadables().await;
}