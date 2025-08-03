use crate::user::composer::{CompositionDataType, IComposition};
use crate::user::library::technical_indicators::{sma, SMA};
use crate::user::library::ITechnicalIndicator;
use crate::user::static_resources::ohlcv::ETHUSDT;
use crate::user::static_resources::StaticResource;
use crate::utils::classes::charting::{CandlestickData, ChartingData, LineChartingData, LineData};
use std::collections::HashMap;
use std::error::Error;
use std::sync::OnceLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct SMA200Composition {
    id: String,
    name: String,
    description: String,
    composition_fields: HashMap<&'static str, usize>,
    static_resources: HashMap<&'static str, StaticResource>,
}

impl IComposition for SMA200Composition {
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
        let mut sma = SMA::<f32>::new(200);

        composed_data.reserve(ethusdt_data.len());

        for candle in ethusdt_data.iter() {
            let timestamp = candle.timestamp;
            let close = candle.close;
            let current_sma = sma.get_data();

            let data = Box::new([
                CompositionDataType::Int(timestamp),
                CompositionDataType::Float(close),
                CompositionDataType::OptionFloat(current_sma),
            ]);

            composed_data.push(data);

            sma.allocate(close);
        }

        Ok(composed_data)
    }

    fn render(&self) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        let mut close_data: Vec<Option<LineData>> = vec![];
        let mut sma_data: Vec<Option<LineData>> = vec![];

        let composed_data = self.compose()?;
        let timestamp_position = self.composition_fields.get("timestamp").unwrap().clone();
        let close_position = self.composition_fields.get("close").unwrap().clone();
        let sma_200_position = self.composition_fields.get("sma_200").unwrap().clone();

        for data_point in composed_data.into_iter() {
            let timestamp = self.extract_int(data_point[timestamp_position]);
            let close = self.extract_float(data_point[close_position]);
            let sma_200 = self.extract_option_float(data_point[sma_200_position]);

            close_data.push(Some(LineData {
                time: timestamp,
                value: close,
                color: Some("blue".into()),
            }));

            match sma_200 {
                Some(value) => sma_data.push(Some(LineData {
                    time: timestamp,
                    value,
                    color: Some("red".into()),
                })),
                None => {}
            }
        }

        let charting_data: Vec<ChartingData> = vec![
            ChartingData::LineChartingData(LineChartingData {
                chart_type: "line".into(),
                height: None,
                data: close_data,
                pane: Some(0),
            }),
            ChartingData::LineChartingData(LineChartingData {
                chart_type: "line".into(),
                height: None,
                data: sma_data,
                pane: Some(0),
            }),
        ];

        Ok(charting_data)
    }

    fn save(&self) -> Result<(), Box<dyn Error>> {
        // TODO: Save to file
        Ok(())
    }
}

impl SMA200Composition {
    pub fn instance() -> &'static SMA200Composition {
        static INSTANCE: OnceLock<SMA200Composition> = OnceLock::new();
        return INSTANCE.get_or_init(|| SMA200Composition::new());
    }

    pub fn new() -> Self {
        return Self {
            name: "SMA 200 Composition".into(),
            description: "The composition for the SMA 200 strategy".into(),
            id: Uuid::new_v4().into(),
            composition_fields: HashMap::from([("timestamp", 0), ("close", 1), ("sma_200", 2)]),
            static_resources: HashMap::from([(
                "ETHUSDT",
                StaticResource::OHLCVDataType(ETHUSDT::instance()),
            )]),
        };
    }
}
