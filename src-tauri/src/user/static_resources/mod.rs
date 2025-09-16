pub mod crypto;
pub mod us_equities;

use crate::library::providers::downloader::{OHLCVJSONFileDataStructure, OHLCVMetaData};
use crate::library::providers::sources::binance::OHLCVCandleObject;
use crate::utils::classes::charting::{
    CandlestickChartingData, CandlestickData, ChartingData, HistogramChartingData, HistogramData,
};
use crate::utils::load_mmap::{load_mmap, MmapManager};
use crate::utils::paths::join_app_data_dir;
use serde::Deserialize;
use std::error::Error;
use std::path::PathBuf;
use std::sync::{LazyLock, OnceLock};
use uuid::Uuid;

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
    pub fn from(name: &str, path: &str) -> StaticResource {
        pub struct FromStaticResource {
            id: String,
            name: String,
            load_path: PathBuf,
        }

        impl IStaticResource<OHLCVData> for FromStaticResource {
            fn id(&self) -> &str {
                return &self.id;
            }

            fn name(&self) -> &str {
                return &self.name;
            }

            fn load_path(&self) -> PathBuf {
                return self.load_path.clone();
            }
        }

        impl FromStaticResource {
            pub fn instance(name: &str, path: &str) -> &'static FromStaticResource {
                static INSTANCE: OnceLock<FromStaticResource> = OnceLock::new();
                INSTANCE.get_or_init(|| FromStaticResource {
                    id: Uuid::new_v4().to_string(),
                    name: name.to_string(),
                    load_path: join_app_data_dir(path).unwrap(),
                })
            }
        }

        let static_resource = StaticResource::OHLCVDataType(FromStaticResource::instance(name, path));

        return static_resource;
    }

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
                let mut candlestick_data: Vec<Option<CandlestickData>> = vec![];
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

                    candlestick_data.push(Some(CandlestickData {
                        open,
                        high,
                        low,
                        close,
                        time: timestamp,
                        border_color: None,
                        color: None,
                        wick_color: None,
                    }));

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
                    pane: Some(0),
                    title: Some(format!("{:?} OHLC", self.name())),
                };

                let volume_chart = HistogramChartingData {
                    chart_type: "histogram".into(),
                    height: Some(200),
                    data: volume_data,
                    pane: Some(1),
                    title: Some("Volume".into()),
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

pub static STATIC_RESOURCES: LazyLock<Vec<StaticResource>> = LazyLock::new(|| {
    vec![
        // Crypto assets
        StaticResource::OHLCVDataType(
            crypto::ethusdt_01_01_2021_06_15_2025_15m::ETHUSDT_4YEARS_15M::instance(),
        ),
        StaticResource::OHLCVDataType(
            crypto::ethusdt_01_01_2021_06_15_2025_4h::ETHUSDT_4YEARS_4H::instance(),
        ),
        StaticResource::OHLCVDataType(
            crypto::solusdt_01_01_2021_06_15_2025_15m::SOLUSDT_4YEARS_15M::instance(),
        ),
        StaticResource::OHLCVDataType(
            crypto::btcusdt_01_01_2021_06_15_2025_15m::BTCUSDT_4YEARS_15M::instance(),
        ),
        StaticResource::OHLCVDataType(
            crypto::btcusdt_01_01_2021_06_15_2025_4h::BTCUSDT_4YEARS_4H::instance(),
        ),
        StaticResource::OHLCVDataType(
            crypto::eth_usdt_01_01_2018_06_15_2025_1m::ETHUSDT_7YEARS_1M::instance(),
        ),
        // US equity assets
        StaticResource::OHLCVDataType(
            us_equities::aapl_01_01_2010_06_15_2025_1d::AAPL_15YEARS_1D::instance(),
        ),
        StaticResource::OHLCVDataType(
            us_equities::nvda_01_01_2010_06_15_2025_1d::NVDA_15YEARS_1D::instance(),
        ),
        StaticResource::OHLCVDataType(
            us_equities::msft_01_01_2010_06_15_2025_1d::MSFT_15YEARS_1D::instance(),
        ),
        StaticResource::OHLCVDataType(
            us_equities::tsla_01_01_2010_06_15_2025_1d::TSLA_15YEARS_1D::instance(),
        ),
        StaticResource::OHLCVDataType(
            us_equities::spy_01_01_2010_06_15_2025_1d::SPY_15YEARS_1D::instance(),
        ),
        StaticResource::OHLCVDataType(
            us_equities::wmt_01_01_2010_06_15_2025_1d::WMT_15YEARS_1D::instance(),
        ),
    ]
});
