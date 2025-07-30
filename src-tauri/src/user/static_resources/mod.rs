mod ohlcv;
use crate::library::providers::downloader::OHLCVJSONFileDataStructure;
use crate::user::static_resources::ohlcv::nflx::NFLX;
use crate::utils::classes::charting::{
    CandlestickChartingData, CandlestickData, ChartingData, HistogramChartingData, HistogramData,
};
use serde::Deserialize;
use std::error::Error;
use std::path::PathBuf;
use std::sync::LazyLock;

pub trait IStaticResource<T: for<'de> Deserialize<'de>>: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn load_path(&self) -> PathBuf;
    fn data_type(&self) -> &str;
    fn render(&self) -> Option<Vec<ChartingData>> {
        return None;
    }
}

pub type OHLCVData = OHLCVJSONFileDataStructure;

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

    pub fn render(&self) -> Option<Vec<ChartingData>> {
        match self {
            StaticResource::OHLCVDataType(resource) => {
                let mut candlestick_data: Vec<CandlestickData> = vec![];
                let mut volume_data: Vec<HistogramData> = vec![];

                if let Ok(data) = self.load() {
                    let data = OHLCVData::from(data);
                    let size = data.timestamps.len();

                    for i in 0..size {
                        let timestamp: i64 = data.timestamps[i] as i64;
                        let open: f32 = data.opens[i] as f32;
                        let high: f32 = data.highs[i] as f32;
                        let low: f32 = data.lows[i] as f32;
                        let close: f32 = data.closes[i] as f32;
                        let volume: f32 = data.volumes[i] as f32;

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

                return Some(charting_data);
            }
        }
    }

    pub fn load(&self) -> Result<OHLCVData, Box<dyn Error>> {
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

    // Add more types as it goes on: load_news, etc...
}

pub static STATIC_RESOURCES: LazyLock<Vec<StaticResource>> =
    LazyLock::new(|| vec![StaticResource::OHLCVDataType(NFLX::instance())]);
