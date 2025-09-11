use crate::{
    library::providers::sources::{binance::Binance, yahoo::Yahoo},
    DEFAULT_THREAD_COUNT,
};
use async_trait::async_trait;
use futures::{
    stream::{self, StreamExt},
};
use num_traits::AsPrimitive;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
    },
};
use tokio::sync::Semaphore;

pub struct Downloader {
    pub sources: HashMap<SourceName, Box<dyn Source>>,
}

impl Downloader {
    pub fn new() -> Self {
        let mut sources_map: HashMap<SourceName, Box<dyn Source>> = HashMap::new();

        let yahoo = Box::new(Yahoo::new());
        let binance = Box::new(Binance::new());

        sources_map.insert(yahoo.source_name(), yahoo);
        sources_map.insert(binance.source_name(), binance);

        return Self {
            sources: sources_map,
        };
    }

    pub async fn get_downloadables(&self) -> Vec<Downloadable> {
        let mut downloadables: Vec<Downloadable> = vec![];

        for source in self.sources.values() {
            let source_downloadables = source.get_downloadables().await.unwrap();
            downloadables.extend(source_downloadables);
        }

        return downloadables;
    }

    pub async fn download_all<F, T>(
        &self,
        download_datas: Vec<DownloadData>,
        thread_limit: Option<usize>,
        on_progress: Option<F>,
    ) where
        F: Fn(T) + Send + Sync + 'static,
        T: 'static + Copy + Send + Sync,
        f32: num_traits::AsPrimitive<T>,
    {
        let thread_count: usize = thread_limit.unwrap_or(
            std::thread::available_parallelism()
                .map(|n: std::num::NonZero<usize>| n.get() as usize)
                .unwrap_or(DEFAULT_THREAD_COUNT),
        );
        let total_count = download_datas.len();
        let completed_count = Arc::new(Mutex::new(0usize));
        let semaphore = Arc::new(Semaphore::new(thread_count));

        let tasks: Vec<_> = stream::iter(download_datas)
            .map(|download_data| {
                let semaphore = semaphore.clone();
                let completed_count = completed_count.clone();
                let on_progress = on_progress.as_ref();

                return async move {
                    let _permit = semaphore.acquire().await.unwrap();

                    for download_data_type in &download_data.data_types {
                        match download_data_type.as_str() {
                            "OHLCV" => self.download_ohlcv(&download_data).await,

                            "bidask" => self.download_bidask(&download_data).await,

                            "news" => {}

                            _ => {}
                        }
                    }

                    if let Some(p_callback) = on_progress {
                        let mut count = completed_count.lock().unwrap();
                        *count += 1;
                        let progress: T = ((*count * 100) as f32 / total_count as f32).as_();
                        p_callback(progress);
                    }
                };
            })
            .buffer_unordered(thread_count)
            .collect()
            .await;
    }

    pub async fn download_ohlcv(&self, download_data: &DownloadData) {
        match self.sources.get(&download_data.source_name) {
            Some(source) => {
                let ohlcv_download_path = source.download_ohlcv(download_data).await;
            }
            None => {}
        };
    }

    pub async fn download_bidask(&self, download_data: &DownloadData) {
        match self.sources.get(&download_data.source_name) {
            Some(source) => {
                let ohlcv_download_path = source.download_bidask(download_data).await;
            }
            _ => {}
        }
    }
}

#[derive(Clone, Debug)]
pub struct DownloadData {
    pub symbol: String,
    pub timeframe: String,
    pub data_types: Vec<String>,
    pub source_name: SourceName,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Downloadable {
    pub name: String,
    pub symbol: String,
    pub source_name: SourceName,
    pub market_type: MarketType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OHLCVDownloadJSONFileStructure {
    pub timestamps: Vec<u64>,
    pub opens: Vec<f32>,
    pub highs: Vec<f32>,
    pub lows: Vec<u64>,
    pub closes: Vec<f32>,
    pub volumes: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OHLCVMetaData {
    pub symbol: String,
    pub timeframe: String,
    pub start_timestamp: i64,
    pub end_timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OHLCVJSONFileDataStructure {
    pub symbol: String,
    pub timeframe: String,
    pub start_timestamp: i64,
    pub end_timestamp: i64,

    pub timestamps: Vec<i64>,
    pub opens: Vec<f32>,
    pub highs: Vec<f32>,
    pub lows: Vec<f32>,
    pub closes: Vec<f32>,
    pub volumes: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum SourceName {
    YahooFinance,
    Binance,
    Marketaux,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    fn source_name(&self) -> SourceName;
    fn source_url(&self) -> &str;
    fn timeframes(&self) -> Vec<&str>;
    async fn download_ohlcv(
        &self,
        download_data: &DownloadData,
    ) -> Result<(), Box<dyn std::error::Error>>;
    async fn download_bidask(
        &self,
        download_data: &DownloadData,
    ) -> Result<(), Box<dyn std::error::Error>>;
    // fn format_raw_data(&self, data: Vec<>) -> Vec<Vec<String>>;
    async fn get_downloadables(&self) -> Result<Vec<Downloadable>, Box<dyn std::error::Error>>;
}
