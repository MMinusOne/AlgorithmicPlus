use crate::library::providers::sources::binance::OHLCVCandleObject;
use crate::user::composer::{CompositionDataType, IComposition};
use crate::user::static_resources::{self, crypto, us_equities, StaticResource};
use crate::utils::classes::charting::{ChartingData, LineChartingData, LineData};
use crate::utils::load_mmap::MmapManager;
use std::collections::HashMap;
use std::error::Error;
use std::sync::OnceLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct Universe {
    id: String,
    name: String,
    description: String,
    composition_fields: HashMap<&'static str, usize>,
    static_resources: HashMap<&'static str, StaticResource>,
}

impl IComposition for Universe {
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

    fn compose(&self) -> Result<Vec<Vec<CompositionDataType>>, Box<dyn Error>> {
        let mut composed_data: Vec<Vec<CompositionDataType>> = vec![];

        let mut asset_datas: Vec<MmapManager<OHLCVCandleObject>> = Vec::new();

        let mut oldest_resource: Option<&StaticResource> = None;
        let mut oldest_data: Option<MmapManager<OHLCVCandleObject>> = None;

        for (resource_name, resource) in &self.static_resources {
            let data = resource.load_ohlcv_mmap()?;

            if oldest_resource.is_none() {
                oldest_resource = Some(resource);
                oldest_data = Some(data.clone());
            } else {
                let oldest_resource_unwraped = oldest_resource.unwrap();
                if data.index(0).timestamp
                    < oldest_resource_unwraped
                        .load_ohlcv_mmap()
                        .unwrap()
                        .index(0)
                        .timestamp
                {
                    oldest_resource = Some(resource);
                    oldest_data = Some(data.clone());
                }
            }

            asset_datas.push(data.clone());
        }

        let oldest_data = oldest_data.unwrap();

        for base_candle in oldest_data.iter() {
            let base_timestamp = base_candle.timestamp;
            let mut data_vec: Vec<CompositionDataType> =
                vec![CompositionDataType::Int(base_timestamp)];

            for asset_data in &asset_datas {
                let candle = asset_data.iter().find(|c| c.timestamp == base_timestamp);
                if let Some(candle_data) = candle {
                    data_vec.push(CompositionDataType::OptionFloat(Some(candle_data.close)));
                } else {
                    data_vec.push(CompositionDataType::OptionFloat(None));
                }
            }

            composed_data.push(data_vec);
        }

        Ok(composed_data)
    }

    fn render(&self) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        let mut close_datas: Vec<Vec<Option<LineData>>> = vec![];

        let composed_data = self.compose()?;
        println!("{:?}", composed_data);
        let timestamp_position = self.composition_fields.get("timestamp").unwrap().clone();

        for (composition_field_name, close_field_position) in &self.composition_fields {
            if *composition_field_name == "timestamp" {
                continue;
            }

            let mut close_data: Vec<Option<LineData>> = vec![];

            for data_point in &composed_data {
                let timestamp = CompositionDataType::extract_int(&data_point[timestamp_position]);
                let close =
                    CompositionDataType::extract_option_float(&data_point[*close_field_position]);

                if close.is_some() {
                    close_data.push(Some(LineData {
                        time: timestamp,
                        value: close.unwrap(),
                        color: Some("blue".into()),
                    }));
                }
            }

            close_datas.push(close_data);
        }

        let mut charting_data: Vec<ChartingData> = vec![];

        for (pane, close_data) in close_datas.into_iter().enumerate() {
            charting_data.push(ChartingData::LineChartingData(LineChartingData {
                chart_type: "line".into(),
                height: None,
                data: close_data,
                pane: Some(pane as i8),
                title: Some("close".into()),
            }));
        }

        Ok(charting_data)
    }

    fn save(&self) -> Result<(), Box<dyn Error>> {
        // TODO: Save to file
        Ok(())
    }
}

impl Universe {
    pub fn instance() -> &'static Universe {
        static INSTANCE: OnceLock<Universe> = OnceLock::new();
        return INSTANCE.get_or_init(|| Universe::new());
    }

    pub fn new() -> Self {
        return Self {
            name: "Universe".into(),
            description: "Universe".into(),
            id: Uuid::new_v4().into(),
            composition_fields: HashMap::from([
                ("timestamp", 0),
                ("close_1", 1),
                ("close_2", 2),
                ("close_3", 3),
                ("close_4", 4),
                ("close_5", 5),
                ("close_6", 6),
                ("close_7", 7),
                ("close_8", 8),
                ("close_9", 9),
                ("close_10", 10),
                ("close_11", 11),
                ("close_12", 12),
                ("close_13", 13),
                ("close_14", 14),
                ("close_15", 15),
                ("close_16", 16),
            ]),
            static_resources: HashMap::from([
                (
                    "asset_1",
                    StaticResource::ohlcv_from(
                        "asset_1",
                        "raw/ohlcv/f0d65a7b-0037-4a79-9c98-c2c8a5738825",
                    ),
                ),
                (
                    "asset_2",
                    StaticResource::ohlcv_from(
                        "asset_2",
                        "raw/ohlcv/b8f1e3ba-2286-4575-9e1a-67356d0dd47e",
                    ),
                ),
                (
                    "asset_3",
                    StaticResource::ohlcv_from(
                        "asset_3",
                        "raw/ohlcv/e24bbea0-0871-45f5-9f75-343294542125",
                    ),
                ),
                (
                    "asset_4",
                    StaticResource::ohlcv_from(
                        "asset_4",
                        "raw/ohlcv/2f9c23c9-8088-401e-a23d-f16bddde6eeb",
                    ),
                ),
                (
                    "asset_5",
                    StaticResource::ohlcv_from(
                        "asset_5",
                        "raw/ohlcv/43eb836c-8bba-4688-beb3-0c8375a097c2",
                    ),
                ),
                (
                    "asset_6",
                    StaticResource::ohlcv_from(
                        "asset_6",
                        "raw/ohlcv/aba8daf4-cc3b-48f5-9e5f-6b4c378e5e05",
                    ),
                ),
                (
                    "asset_7",
                    StaticResource::ohlcv_from(
                        "asset_7",
                        "raw/ohlcv/5045de85-2f62-4afc-a9ee-dc69af9639e4",
                    ),
                ),
                (
                    "asset_8",
                    StaticResource::ohlcv_from(
                        "asset_8",
                        "raw/ohlcv/042ca4bb-3144-4b41-82e8-4c31aff72145",
                    ),
                ),
                (
                    "asset_9",
                    StaticResource::ohlcv_from(
                        "asset_9",
                        "raw/ohlcv/cbfb05e2-57f2-4a25-b713-f23bea6de87b",
                    ),
                ),
                (
                    "asset_10",
                    StaticResource::ohlcv_from(
                        "asset_10",
                        "raw/ohlcv/99f25064-0277-4886-9f31-d35060480352",
                    ),
                ),
                (
                    "asset_11",
                    StaticResource::ohlcv_from(
                        "asset_11",
                        "raw/ohlcv/c3d01894-996c-4e33-99cb-927240f4b181",
                    ),
                ),
                (
                    "asset_12",
                    StaticResource::ohlcv_from(
                        "asset_12",
                        "raw/ohlcv/4a81d9fc-2ec5-46f3-bf4a-40d1d2eb2ebe",
                    ),
                ),
                (
                    "asset_13",
                    StaticResource::ohlcv_from(
                        "asset_13",
                        "raw/ohlcv/2edcbd09-1490-4993-b48c-73776adc62d9",
                    ),
                ),
                (
                    "asset_14",
                    StaticResource::ohlcv_from(
                        "asset_14",
                        "raw/ohlcv/12b73b85-2890-4233-9c0b-394c6947f56c",
                    ),
                ),
                (
                    "asset_15",
                    StaticResource::ohlcv_from(
                        "asset_15",
                        "raw/ohlcv/d94a38d6-d54c-4c85-ace3-c09429124296",
                    ),
                ),
                (
                    "asset_16",
                    StaticResource::ohlcv_from(
                        "asset_16",
                        "raw/ohlcv/299cd419-dd6f-44a4-a150-2bb2239e2fd8",
                    ),
                ),
            ]),
        };
    }
}
