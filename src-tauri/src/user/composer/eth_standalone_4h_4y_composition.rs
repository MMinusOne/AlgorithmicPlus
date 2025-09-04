use crate::user::composer::{CompositionDataType, IComposition};
use crate::user::library::technical_indicators::SMA;
use crate::user::library::IInjectable;
use crate::user::static_resources::{crypto, StaticResource};
use crate::utils::classes::charting::{ChartingData, LineChartingData, LineData};
use std::collections::HashMap;
use std::error::Error;
use std::sync::OnceLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct ETH_STANDALONE_4H_4Y {
    id: String,
    name: String,
    description: String,
    composition_fields: HashMap<&'static str, usize>,
    static_resources: HashMap<&'static str, StaticResource>,
}

impl IComposition for ETH_STANDALONE_4H_4Y {
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
            let close = candle.close;

            let data = Box::new([
                CompositionDataType::Int(timestamp),
                CompositionDataType::Float(close),
            ]);

            composed_data.push(data);
        }

        Ok(composed_data)
    }

    fn render(&self) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        let mut close_data: Vec<Option<LineData>> = vec![];
        let mut sma_data: Vec<Option<LineData>> = vec![];

        let composed_data = self.compose()?;

        let timestamp_position = self.composition_fields.get("timestamp").unwrap().clone();
        let close_position = self.composition_fields.get("close").unwrap().clone();

        for data_point in composed_data.into_iter() {
            let timestamp = CompositionDataType::extract_int(&data_point[timestamp_position]);
            let close = CompositionDataType::extract_float(&data_point[close_position]);

            close_data.push(Some(LineData {
                time: timestamp,
                value: close,
                color: Some("blue".into()),
            }));
        }

        let charting_data: Vec<ChartingData> =
            vec![ChartingData::LineChartingData(LineChartingData {
                chart_type: "line".into(),
                height: None,
                data: close_data,
                pane: Some(0),
                title: Some("ETHUSDT close".into()),
            })];

        Ok(charting_data)
    }

    fn save(&self) -> Result<(), Box<dyn Error>> {
        // TODO: Save to file
        Ok(())
    }
}

impl ETH_STANDALONE_4H_4Y {
    pub fn instance() -> &'static ETH_STANDALONE_4H_4Y {
        static INSTANCE: OnceLock<ETH_STANDALONE_4H_4Y> = OnceLock::new();
        return INSTANCE.get_or_init(|| ETH_STANDALONE_4H_4Y::new());
    }

    pub fn new() -> Self {
        return Self {
            name: "SMA 200 Composition".into(),
            description: "The composition for the SMA 200 strategy".into(),
            id: Uuid::new_v4().into(),
            composition_fields: HashMap::from([("timestamp", 0), ("close", 1)]),
            static_resources: HashMap::from([(
                "ETHUSDT",
                StaticResource::OHLCVDataType(
                    crypto::ethusdt_01_01_2021_06_15_2025_4h::ETHUSDT_4YEARS_4H::instance(),
                ),
            )]),
        };
    }
}
