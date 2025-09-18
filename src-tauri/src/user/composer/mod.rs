pub mod btc_eth_statarb_4h_4y_composition;
pub mod eth_hlc_standalone_4h_4y;
pub mod eth_sma_200_4h_4y_composition;
pub mod eth_standalone_4h_4y_composition;
pub mod testing_composition;
use serde::{Deserialize, Serialize};
use crate::user::composer::{
    btc_eth_statarb_4h_4y_composition::BTC_ETH_STATARB_4H_4Y,
    eth_hlc_standalone_4h_4y::ETH_HLC_STANDALONE_4H_4Y,
    eth_sma_200_4h_4y_composition::ETH_SMA_200_4H_4Y,
    eth_standalone_4h_4y_composition::ETH_STANDALONE_4H_4Y,
    testing_composition::TESTING_COMPOSITION,
};
use crate::utils::classes::charting::ChartingData;
use std::collections::HashMap;
use std::error::Error;
use std::sync::LazyLock;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CompositionDataType {
    I64(i64),
    F32(f32),
    OptionF32(Option<f32>),
    Bool(bool),
    Usize(usize),
    String(String),
}

impl CompositionDataType {
    pub fn extract_i64(compsition_data: &CompositionDataType) -> i64 {
        match compsition_data {
            Self::I64(v) => v.clone(),
            _ => panic!("Invalid compsition type conversion."),
        }
    }
    pub fn extract_f32(compsition_data: &CompositionDataType) -> f32 {
        match compsition_data {
            Self::F32(v) => v.clone(),
            _ => panic!("Invalid compsition type conversion."),
        }
    }
    pub fn extract_option_f32(compsition_data: &CompositionDataType) -> Option<f32> {
        match compsition_data {
            Self::OptionF32(v) => v.clone(),
            _ => panic!("Invalid compsition type conversion."),
        }
    }
    pub fn extract_bool(compsition_data: &CompositionDataType) -> bool {
        match compsition_data {
            Self::Bool(v) => v.clone(),
            _ => panic!("Invalid compsition type conversion."),
        }
    }

    pub fn extract_string(compsition_data: &CompositionDataType) -> String {
        match compsition_data {
            Self::String(v) => v.clone(),
            _ => panic!("Invalid compsition type conversion."),
        }
    }

    pub fn extract_usize(compsition_data: &CompositionDataType) -> usize {
        match compsition_data {
            Self::Usize(v) => v.clone(),
            _ => panic!("Invalid compsition type conversion."),
        }
    }
}

pub trait IComposition: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn composition_fields(&self) -> HashMap<&'static str, usize>;
    fn get_composition_field_position(&self, field_name: &str) -> usize {
        return self
            .composition_fields()
            .get(field_name)
            .unwrap()
            .to_owned();
    }
    fn compose(&self) -> Result<Vec<Vec<CompositionDataType>>, Box<dyn Error>>;
    fn safe_compose(&self) -> Result<Vec<Vec<CompositionDataType>>, Box<dyn Error>> {
        let composition_data = self.compose()?;
        let composition_fields = self.composition_fields();
        let composition_fields_length = composition_fields.len();

        for data_point in &composition_data {
            if data_point.len() != composition_fields_length {
                return Err("Data point not as long as composition fields".into());
            }
        }

        Ok(composition_data)
    }
    fn render(&self) -> Result<Vec<ChartingData>, Box<dyn Error>>;
    fn save(&self) -> Result<(), Box<dyn Error>>;
}

pub static COMPOSITIONS: LazyLock<Vec<Box<dyn IComposition>>> = LazyLock::new(|| {
    vec![
        Box::new(ETH_SMA_200_4H_4Y::instance().clone()),
        Box::new(BTC_ETH_STATARB_4H_4Y::instance().clone()),
        Box::new(ETH_STANDALONE_4H_4Y::instance().clone()),
        Box::new(ETH_HLC_STANDALONE_4H_4Y::instance().clone()),
        Box::new(TESTING_COMPOSITION::instance().clone()),
    ]
});
