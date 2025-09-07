use crate::user::composer::{CompositionDataType, IComposition};
use crate::user::library::bollinger_bands::BollingerBands;
use crate::user::library::IInjectable;
use crate::user::static_resources::{crypto, StaticResource};
use crate::utils::classes::charting::{ChartingData, LineChartingData, LineData};
use crate::utils::formulas::processing::normalize::normalize_inline;
use std::collections::HashMap;
use std::error::Error;
use std::sync::OnceLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct BTC_ETH_STATARB_4H_4Y {
    id: String,
    name: String,
    description: String,
    composition_fields: HashMap<&'static str, usize>,
    static_resources: HashMap<&'static str, StaticResource>,
}

impl IComposition for BTC_ETH_STATARB_4H_4Y {
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

        // Get the static resource data (OHLCV)
        let btcusdt_resource = self.static_resources.get("BTCUSDT").unwrap();
        let ethusdt_resource = self.static_resources.get("ETHUSDT").unwrap();

        let ethusdt_data = ethusdt_resource.load_ohlcv_mmap()?;
        let btcusdt_data = btcusdt_resource.load_ohlcv_mmap()?;

        // Make arrays for only what is relevant (timestamps and closes, NOT OPEN HIGH LOW)
        let mut timestamps: Vec<i64> = vec![];
        let mut btc_normalized_closes: Vec<f32> = vec![];
        let mut eth_normalized_closes: Vec<f32> = vec![];

        let size = ethusdt_data.len().min(btcusdt_data.len());

        for index in 0..size {
            let timestamp = btcusdt_data.index(index).timestamp;
            let btc_close = btcusdt_data.index(index).close;
            let eth_close = ethusdt_data.index(index).close;
            // Push the raw data to the array
            btc_normalized_closes.push(btc_close);
            eth_normalized_closes.push(eth_close);
            timestamps.push(timestamp);
        }

        // Normilize the array of numbers
        normalize_inline::<f32>(&mut btc_normalized_closes);
        normalize_inline::<f32>(&mut eth_normalized_closes);

        let mut bollinger_bands_injectable = BollingerBands::new(100);

        for index in 0..size {
            let timestamp = timestamps[index];
            let btc_normalized_close = btc_normalized_closes[index];
            let eth_normalized_close = eth_normalized_closes[index];
            let stationary_asset_price = 0.5 * btc_normalized_close - 0.5 * eth_normalized_close;

            bollinger_bands_injectable.allocate(stationary_asset_price);

            let bounds = bollinger_bands_injectable.get_data();

            let mut upper_bound: Option<f32> = None;
            let mut lower_bound: Option<f32> = None;

            match bounds {
                Some((u_b, l_b)) => {
                    upper_bound = Some(u_b);
                    lower_bound = Some(l_b);
                }
                None => {}
            }

            // Push the data
            let data = Box::new([
                CompositionDataType::Int(timestamp),
                CompositionDataType::Float(btc_normalized_close),
                CompositionDataType::Float(eth_normalized_close),
                CompositionDataType::OptionFloat(upper_bound),
                CompositionDataType::OptionFloat(lower_bound),
                CompositionDataType::Float(stationary_asset_price),
            ]);

            composed_data.push(data);
        }

        Ok(composed_data)
    }

    fn render(&self) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        let mut btc_close_data: Vec<Option<LineData>> = vec![];
        let mut eth_close_data: Vec<Option<LineData>> = vec![];
        let mut upper_bound_data: Vec<Option<LineData>> = vec![];
        let mut lower_bound_data: Vec<Option<LineData>> = vec![];
        let mut new_asset: Vec<Option<LineData>> = vec![];

        let composed_data = self.compose()?;

        let timestamp_position = self.composition_fields.get("timestamp").unwrap().to_owned();
        let btc_close_position = self.composition_fields.get("btc_close").unwrap().to_owned();
        let eth_close_position = self.composition_fields.get("eth_close").unwrap().to_owned();
        let std_upper_bound_position = self
            .composition_fields
            .get("stddev_upperbound")
            .unwrap()
            .to_owned();
        let std_lower_bound_position = self
            .composition_fields
            .get("stddev_lowerbound")
            .unwrap()
            .to_owned();
        let stationary_asset_position = self
            .composition_fields
            .get("stationary_asset")
            .unwrap()
            .to_owned();

        for data_point in composed_data.into_iter() {
            let timestamp = CompositionDataType::extract_int(&data_point[timestamp_position]);
            let btc_close = CompositionDataType::extract_float(&data_point[btc_close_position]);
            let eth_close = CompositionDataType::extract_float(&data_point[eth_close_position]);
            let std_upper_bound =
                CompositionDataType::extract_option_float(&data_point[std_upper_bound_position]);
            let std_lower_bound =
                CompositionDataType::extract_option_float(&data_point[std_lower_bound_position]);
            let stationary_asset =
                CompositionDataType::extract_float(&data_point[stationary_asset_position]);

            btc_close_data.push(Some(LineData {
                time: timestamp,
                value: btc_close,
                color: Some("orange".into()),
            }));

            eth_close_data.push(Some(LineData {
                time: timestamp,
                value: eth_close,
                color: Some("blue".into()),
            }));

            match std_upper_bound {
                Some(value) => upper_bound_data.push(Some(LineData {
                    time: timestamp,
                    value,
                    color: Some("red".into()),
                })),
                None => {}
            }

            match std_lower_bound {
                Some(value) => lower_bound_data.push(Some(LineData {
                    time: timestamp,
                    value,
                    color: Some("green".into()),
                })),
                None => {}
            }

            new_asset.push(Some(LineData {
                time: timestamp,
                value: stationary_asset,
                color: Some("black".into()),
            }));
        }

        let charting_data: Vec<ChartingData> = vec![
            ChartingData::LineChartingData(LineChartingData {
                chart_type: "line".into(),
                height: None,
                data: btc_close_data,
                pane: Some(0),
                title: Some("BTC NORMALIZED CLOSE".into()),
            }),
            ChartingData::LineChartingData(LineChartingData {
                chart_type: "line".into(),
                height: None,
                data: new_asset,
                pane: Some(1),
                title: Some("New Asset".into()),
            }),
            ChartingData::LineChartingData(LineChartingData {
                chart_type: "line".into(),
                height: None,
                data: upper_bound_data,
                pane: Some(1),
                title: Some("Upper bounds".into()),
            }),
            ChartingData::LineChartingData(LineChartingData {
                chart_type: "line".into(),
                height: None,
                data: lower_bound_data,
                pane: Some(1),
                title: Some("Lower bounds".into()),
            }),
            ChartingData::LineChartingData(LineChartingData {
                chart_type: "line".into(),
                height: None,
                data: eth_close_data,
                pane: Some(2),
                title: Some("ETH NORMALIZED CLOSE".into()),
            }),
        ];

        Ok(charting_data)
    }

    fn save(&self) -> Result<(), Box<dyn Error>> {
        // TODO: Save to file
        Ok(())
    }
}

impl BTC_ETH_STATARB_4H_4Y {
    pub fn instance() -> &'static BTC_ETH_STATARB_4H_4Y {
        static INSTANCE: OnceLock<BTC_ETH_STATARB_4H_4Y> = OnceLock::new();
        return INSTANCE.get_or_init(|| BTC_ETH_STATARB_4H_4Y::new());
    }

    pub fn new() -> Self {
        return Self {
            name: "BTC ETH STAT ARB".into(),
            description: "The composition for statistical arbitrage between eth and btc half/half (no co-efficient optimization)".into(),
            id: Uuid::new_v4().into(),
            composition_fields: HashMap::from([("timestamp", 0), ("btc_close", 1), ("eth_close", 2), ("stddev_upperbound", 3), ("stddev_lowerbound", 4), ("stationary_asset", 5)]),
            static_resources: HashMap::from([
                (
                    "BTCUSDT",
                    StaticResource::OHLCVDataType(crypto::btcusdt_01_01_2021_06_15_2025_4h::BTCUSDT_4YEARS_4H::instance()),
                ),
                (
                    "ETHUSDT",
                    StaticResource::OHLCVDataType(crypto::ethusdt_01_01_2021_06_15_2025_4h::ETHUSDT_4YEARS_4H::instance()),
                ),
            ]),
        };
    }
}
