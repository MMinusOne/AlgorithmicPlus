use crate::user::composer::{CompositionDataType, IComposition};
use crate::user::library::kalman_filter::KalmanFilter;
use crate::user::library::sma::SMA;
use crate::user::library::theilsen::TheilSen;
use crate::user::library::IInjectable;
use crate::user::static_resources::{crypto, StaticResource};
use crate::utils::classes::charting::{ChartingData, LineChartingData, LineData};
use std::collections::HashMap;
use std::error::Error;
use std::sync::OnceLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct TESTING_COMPOSITION {
    id: String,
    name: String,
    description: String,
    composition_fields: HashMap<&'static str, usize>,
    static_resources: HashMap<&'static str, StaticResource>,
}

impl IComposition for TESTING_COMPOSITION {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn composition_fields(&self) -> HashMap<&'static str, usize> {
        self.composition_fields.clone()
    }

    fn compose(&self) -> Result<Vec<Box<[CompositionDataType]>>, Box<dyn Error>> {
        let mut composed_data: Vec<Box<[CompositionDataType]>> = vec![];

        let ethusdt_resource = self.static_resources.get("ETHUSDT").unwrap();
        let ethusdt_data = ethusdt_resource.load_ohlcv_mmap()?;

        let mut kalman_filter = KalmanFilter::default();

        composed_data.reserve(ethusdt_data.len());

        for candle in ethusdt_data.iter() {
            let timestamp = candle.timestamp;
            let close = candle.close;

            kalman_filter.allocate(close);
            let current_kalman = kalman_filter.get_data();

            let data = Box::new([
                CompositionDataType::Int(timestamp),
                CompositionDataType::Float(close),
                CompositionDataType::OptionFloat(current_kalman),
            ]);

            composed_data.push(data);
        }

        Ok(composed_data)
    }

    fn render(&self) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        let mut close_data: Vec<Option<LineData>> = vec![];
        let mut kalman_data: Vec<Option<LineData>> = vec![];
        let mut kalman_trend_data: Vec<Option<LineData>> = vec![];

        let composed_data = self.compose()?;

        let timestamp_position = self.composition_fields.get("timestamp").unwrap().clone();
        let close_position = self.composition_fields.get("close").unwrap().clone();
        let kalman_position = self
            .composition_fields
            .get("kalman_filter")
            .unwrap()
            .clone();

        let mut previous_kalman: Option<f32> = None;

        for data_point in composed_data.into_iter() {
            let timestamp = CompositionDataType::extract_int(&data_point[timestamp_position]);
            let close = CompositionDataType::extract_float(&data_point[close_position]);
            let kalman_value =
                CompositionDataType::extract_option_float(&data_point[kalman_position]);

            close_data.push(Some(LineData {
                time: timestamp,
                value: close,
                color: Some("blue".into()),
            }));

            if let Some(value) = kalman_value {
                kalman_data.push(Some(LineData {
                    time: timestamp,
                    value,
                    color: Some("aqua".into()),
                }));

                let trend_color = if let Some(prev_value) = previous_kalman {
                    if value > prev_value {
                        "green"
                    } else {
                        "red"
                    }
                } else {
                    "aqua"
                };

                kalman_trend_data.push(Some(LineData {
                    time: timestamp,
                    value,
                    color: Some(trend_color.into()),
                }));

                previous_kalman = Some(value);
            }
        }

        let charting_data: Vec<ChartingData> = vec![
            ChartingData::LineChartingData(LineChartingData {
                chart_type: "line".into(),
                height: None,
                data: close_data,
                pane: Some(0),
                title: Some("ETHUSDT Close".into()),
            }),
            ChartingData::LineChartingData(LineChartingData {
                chart_type: "line".into(),
                height: None,
                data: kalman_data,
                pane: Some(0),
                title: Some("Kalman Filter".into()),
            }),
            ChartingData::LineChartingData(LineChartingData {
                chart_type: "line".into(),
                height: None,
                data: kalman_trend_data,
                pane: Some(0),
                title: Some("Kalman Filter Trend".into()),
            }),
        ];

        Ok(charting_data)
    }

    fn save(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl TESTING_COMPOSITION {
    pub fn instance() -> &'static TESTING_COMPOSITION {
        static INSTANCE: OnceLock<TESTING_COMPOSITION> = OnceLock::new();
        return INSTANCE.get_or_init(|| TESTING_COMPOSITION::new());
    }

    pub fn new() -> Self {
        return Self {
            name: "Testing composition".into(),
            description: "Testing composition".into(),
            id: Uuid::new_v4().into(),
            composition_fields: HashMap::from([
                ("timestamp", 0),
                ("close", 1),
                ("kalman_filter", 2),
            ]),
            static_resources: HashMap::from([(
                "ETHUSDT",
                StaticResource::OHLCVDataType(
                    // crypto::ethusdt_01_01_2021_06_15_2025_4h::ETHUSDT_4YEARS_4H::instance(),
                    crypto::ethusdt_01_01_2021_06_15_2025_15m::ETHUSDT_4YEARS_15M::instance(),
                ),
            )]),
        };
    }
}
