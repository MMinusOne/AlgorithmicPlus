use crate::user::composer::{CompositionDataType, IComposition};
use crate::user::library::sma::SMA;
use crate::user::library::IInjectable;
use crate::user::static_resources::{crypto, us_equities, StaticResource};
use crate::utils::classes::charting::{
    CandlestickChartingData, CandlestickData, ChartingData, LineChartingData, LineData,
};
use std::collections::HashMap;
use std::error::Error;
use std::sync::OnceLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct ETH_HLC_STANDALONE_4H_4Y {
    id: String,
    name: String,
    description: String,
    composition_fields: HashMap<&'static str, usize>,
    static_resources: HashMap<&'static str, StaticResource>,
}

impl IComposition for ETH_HLC_STANDALONE_4H_4Y {
    fn id(&self) -> &str {
        return &self.id;
    }

    fn name(&self) -> &str {
        return &self.name;
    }

    fn description(&self) -> &str {
        return &self.description;
    }

    fn composition_fields(&self) -> HashMap<&'static str, usize> {
        return self.composition_fields.clone();
    }

    fn compose(&self) -> Result<Vec<Box<[CompositionDataType]>>, Box<dyn Error>> {
        let mut composed_data: Vec<Box<[CompositionDataType]>> = vec![];

        let ethusdt_resource = self.static_resources.get("ETHUSDT").unwrap();
        let ethusdt_data = ethusdt_resource.load_ohlcv_mmap()?;

        composed_data.reserve(ethusdt_data.len());

        for candle in ethusdt_data.iter() {
            let timestamp = candle.timestamp;

            let data = Box::new([
                CompositionDataType::Int(timestamp),
                CompositionDataType::Float(candle.high),
                CompositionDataType::Float(candle.low),
                CompositionDataType::Float(candle.close),
            ]);

            composed_data.push(data);
        }

        Ok(composed_data)
    }

    fn render(&self) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        let mut hlc_data: Vec<Option<CandlestickData>> = vec![];
        let mut sma_data: Vec<Option<LineData>> = vec![];

        let composed_data = self.compose()?;

        let timestamp_position = self.composition_fields.get("timestamp").unwrap().clone();
        let high_position = self.composition_fields.get("high").unwrap().clone();
        let low_position = self.composition_fields.get("low").unwrap().clone();
        let close_position = self.composition_fields.get("close").unwrap().clone();
        let sma_200_position = self.composition_fields.get("sma_200").unwrap().clone();

        for data_point in composed_data.into_iter() {
            let timestamp = CompositionDataType::extract_int(&data_point[timestamp_position]);
            let high = CompositionDataType::extract_float(&data_point[high_position]);
            let low = CompositionDataType::extract_float(&data_point[low_position]);
            let close = CompositionDataType::extract_float(&data_point[close_position]);
            let sma_200 = CompositionDataType::extract_option_float(&data_point[sma_200_position]);

            hlc_data.push(Some(CandlestickData {
                time: timestamp,
                open: high,
                high: high,
                low: low,
                close: close,
                wick_color: None,
                border_color: None,
                color: Some("blue".into()),
            }));

            if sma_200.is_some() {
                sma_data.push(Some(LineData {
                    time: timestamp,
                    value: sma_200.unwrap(),
                    color: Some("red".into()),
                }))
            }
        }

        let charting_data: Vec<ChartingData> = vec![
            ChartingData::CandlestickChartingData(CandlestickChartingData {
                chart_type: "ohlcv".into(),
                height: None,
                data: hlc_data,
                pane: None,
                title: Some("ETHUSDT close".into()),
            }),
            ChartingData::LineChartingData(LineChartingData {
                chart_type: "line".into(),
                height: None,
                data: sma_data,
                pane: None,
                title: Some("SMA 200".into()),
            }),
        ];

        Ok(charting_data)
    }

    fn save(&self) -> Result<(), Box<dyn Error>> {
        // TODO: Save to file
        Ok(())
    }
}

impl ETH_HLC_STANDALONE_4H_4Y {
    pub fn instance() -> &'static ETH_HLC_STANDALONE_4H_4Y {
        static INSTANCE: OnceLock<ETH_HLC_STANDALONE_4H_4Y> = OnceLock::new();
        return INSTANCE.get_or_init(|| ETH_HLC_STANDALONE_4H_4Y::new());
    }

    pub fn new() -> Self {
        return Self {
            name: "ETH HLC Standalone composition".into(),
            description: "ETH HLC Standalone".into(),
            id: Uuid::new_v4().into(),
            composition_fields: HashMap::from([
                ("timestamp", 0),
                ("high", 1),
                ("low", 2),
                ("close", 3),
            ]),
            static_resources: HashMap::from([(
                "ETHUSDT",
                StaticResource::OHLCVDataType(
                    // crypto::ethusdt_01_01_2021_06_15_2025_4h::ETHUSDT_4YEARS_4H::instance(),
                    crypto::ethusdt_01_01_2021_06_15_2025_15m::ETHUSDT_4YEARS_15M::instance()
                ),
            )]),
        };
    }
}
