use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use crate::{
    library::providers::sources::{
        binance::Binance,
        yahoo::{SymbolCell, Yahoo},
    },
    utils::classes::logger::{self, LOGGER},
};
use async_trait::async_trait;
use futures::stream::{self, StreamExt};
use serde::{Deserialize, Serialize};

pub struct Downloader {
    pub sources: Vec<Box<dyn Source>>,
}

impl Downloader {
    pub fn new() -> Self {
        let yahoo = Box::new(Yahoo::new());
        let binance = Box::new(Binance::new());
        return Self {
            sources: vec![yahoo, binance],
        };
    }

    pub async fn get_downloadables(&self) -> Vec<Downloadable> {
        let mut downloadables: Vec<Downloadable> = vec![];

        for source in &self.sources {
            match source.get_downloadables().await {
                Ok(source_downloadables) => downloadables.extend(source_downloadables),
                Err(err) => logger::LOGGER.error(&err.to_string()),
            }
        }

        return downloadables;
    }

    pub async fn download_all<F>(
        &self,
        download_datas: Vec<DownloadData>,
        thread_limit: Option<u8>,
        on_progress: Option<F>,
    ) -> Option<Vec<String>>
    where
        F: Fn(usize) + Send + Sync + 'static,
    {
        let thread_count = thread_limit.unwrap_or(8);
        let total_count = download_datas.len();
        let completed_count = Arc::new(AtomicUsize::new(0));

        let results: Vec<Option<String>> = stream::iter(download_datas)
            .map(|download_data| {
                let counter = Arc::clone(&completed_count);
                let on_progress = &on_progress;
                
                return async move {
                    let result = self.download(download_data).await;
                    let current = counter.fetch_add(1, Ordering::Relaxed) + 1;
                    let progress = current * 100 / total_count;

                    if let Some(progress_callback) = on_progress {
                        progress_callback(progress);
                    }

                    return result;
                };
            })
            .buffer_unordered(thread_count as usize)
            .collect()
            .await;

        let successful_results: Vec<String> = results
            .into_iter()
            .filter_map(|result| {
                if let Some(value) = result {
                    return Some(value);
                } else {
                    LOGGER.warning(&format!("Couldn't download a symbol"));
                    return None;
                }
            })
            .collect();

        if successful_results.is_empty() {
            return None;
        } else {
            return Some(successful_results);
        }
    }

    pub async fn download(&self, download_data: DownloadData) -> Option<String> {
        //TODO: make it download the data
        // Dont bother editing this, this will just download the data and return string
        return None;
    }

    pub fn get_source(&self, source_name_or_url: &str) -> Option<&dyn Source> {
        for source in &self.sources {
            if source.source_name() == source_name_or_url
                || source.source_url() == source_name_or_url
            {
                return Some(source.as_ref());
            }
        }

        return None;
    }
}

#[derive(Clone)]
pub struct DownloadData {
    pub symbol: String,
    pub timeframe: String,
    pub data_types: Vec<String>,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Downloadable {
    pub name: String,
    pub symbol: String,
    pub source: SourceName,
    pub market_type: MarketType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SourceName {
    YahooFinance,
    Binance,
    Okx,
    Marketaux,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MarketType {
    Crypto,
    Stock,
    Forex,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DataType {
    OHLCV,
    News,
    OrderFlow,
    BidAsk,
}

#[async_trait]
pub trait Source: Send + Sync {
    fn source_name(&self) -> &str;
    fn source_url(&self) -> &str;
    fn timeframes(&self) -> Vec<&str>;
    async fn download(&self) -> Option<String>;
    // fn format_raw_data(&self, data: Vec<>) -> Vec<Vec<String>>;
    async fn get_downloadables(&self) -> Result<Vec<Downloadable>, Box<dyn std::error::Error>>;
}
