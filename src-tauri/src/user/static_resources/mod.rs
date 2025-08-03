pub mod ohlcv;
use crate::library::providers::downloader::{OHLCVJSONFileDataStructure, OHLCVMetaData};
use crate::library::providers::sources::binance::OHLCVCandleObject;
use crate::user::static_resources::ohlcv::ethusdt::ETHUSDT;
use crate::utils::classes::charting::{
    CandlestickChartingData, CandlestickData, ChartingData, HistogramChartingData, HistogramData,
};
use crate::utils::load_mmap::{load_mmap, MmapManager};
use serde::Deserialize;
use std::error::Error;
use std::path::PathBuf;
use std::sync::LazyLock;

pub trait IStaticResource<T: for<'de> Deserialize<'de>>: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn load_path(&self) -> PathBuf;
    fn render(&self) -> Option<Vec<ChartingData>> {
        return None;
    }
}

pub type OHLCVData = OHLCVJSONFileDataStructure;

#[derive(Clone)]
pub enum StaticResource {
    OHLCVDataType(&'static dyn IStaticResource<OHLCVData>),
}

impl StaticResource {
    // Add more types the more types there is
    pub fn id(&self) -> &str {
        match self {
            StaticResource::OHLCVDataType(resource) => resource.id(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            StaticResource::OHLCVDataType(resource) => resource.name(),
        }
    }

    pub fn load_path(&self) -> PathBuf {
        match self {
            StaticResource::OHLCVDataType(resource) => resource.load_path(),
        }
    }

    pub fn data_type(&self) -> &str {
        match self {
            StaticResource::OHLCVDataType(_resource) => "OHLCV",
        }
    }

    pub fn render(&self) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        match self {
            StaticResource::OHLCVDataType(_resource) => {
                let mut candlestick_data: Vec<CandlestickData> = vec![];
                let mut volume_data: Vec<HistogramData> = vec![];

                let mmap_data: MmapManager<OHLCVCandleObject> = self.load_ohlcv_mmap().unwrap();

                candlestick_data.reserve(mmap_data.len());

                for candle in mmap_data.iter() {
                    let timestamp: i64 = candle.timestamp;
                    let open: f32 = candle.open as f32;
                    let high: f32 = candle.high as f32;
                    let low: f32 = candle.low as f32;
                    let close: f32 = candle.close as f32;
                    let volume: f32 = candle.volume as f32;

                    candlestick_data.push(CandlestickData {
                        open,
                        high,
                        low,
                        close,
                        time: timestamp,
                        border_color: None,
                        color: None,
                        wick_color: None,
                    });

                    volume_data.push(HistogramData {
                        time: timestamp,
                        value: volume,
                        color: None,
                    });
                }

                let candlestick_chart = CandlestickChartingData {
                    chart_type: "ohlcv".into(),
                    height: Some(800),
                    data: candlestick_data,
                };

                let volume_chart = HistogramChartingData {
                    chart_type: "histogram".into(),
                    height: Some(200),
                    data: volume_data,
                };

                let charting_data: Vec<ChartingData> = vec![
                    ChartingData::CandlestickChartingData(candlestick_chart),
                    ChartingData::HistogramChartingData(volume_chart),
                ];

                Ok(charting_data)
            }
        }
    }

    // OHLCV methods
    pub fn load_ohlcv_metadata(&self) -> Result<OHLCVMetaData, Box<dyn Error>> {
        match self {
            StaticResource::OHLCVDataType(_resource) => {
                let mut path = self.load_path();
                path.set_extension("json");
                let file_data_string = std::fs::read_to_string(path)?;
                let data: OHLCVMetaData = serde_json::from_str::<OHLCVMetaData>(&file_data_string)?;

                Ok(data)
            }
            _ => Err("Wrong resource type".into()),
        }
    }

    pub fn load_ohlcv_json(&self) -> Result<OHLCVData, Box<dyn Error>> {
        match self {
            StaticResource::OHLCVDataType(_resource) => {
                let mut path = self.load_path();
                path.set_extension("json");
                let file_data_string = std::fs::read_to_string(path)?;
                let data = serde_json::from_str(&file_data_string)?;

                Ok(data)
            }
            _ => Err("Wrong resource type".into()),
        }
    }

    pub fn load_ohlcv_mmap(&self) -> Result<MmapManager<OHLCVCandleObject>, Box<dyn Error>> {
        match self {
            StaticResource::OHLCVDataType(_resource) => {
                let mut path = self.load_path();
                path.set_extension("bin");
                let data = load_mmap::<OHLCVCandleObject>(path).unwrap();
                Ok(data)
            }
        }
    }

    // Add more types as it goes on: load_news, etc...
}

pub static STATIC_RESOURCES: LazyLock<Vec<StaticResource>> =
    LazyLock::new(|| vec![StaticResource::OHLCVDataType(ETHUSDT::instance())]);
