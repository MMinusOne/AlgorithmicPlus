pub mod sma_200_composition;
pub use sma_200_composition::SMA200Composition;
pub mod btc_eth_statarb;
pub use btc_eth_statarb::ETHBTCSTATARB;

use crate::utils::classes::charting::ChartingData;
use std::collections::HashMap;
use std::error::Error;
use std::marker::Copy;
use std::sync::LazyLock;

#[derive(Clone, Copy)]
pub enum CompositionDataType {
    Int(i64),
    Float(f32),
    OptionFloat(Option<f32>),
    Bool(bool),
}

impl CompositionDataType {
    pub fn extract_int(compsition_data: CompositionDataType) -> i64 {
        match compsition_data {
            Self::Int(v) => v,
            _ => panic!("Invalid compsition type conversion."),
        }
    }
    pub fn extract_float(compsition_data: CompositionDataType) -> f32 {
        match compsition_data {
            Self::Float(v) => v,
            _ => panic!("Invalid compsition type conversion."),
        }
    }
    pub fn extract_option_float(compsition_data: CompositionDataType) -> Option<f32> {
        match compsition_data {
            Self::OptionFloat(v) => v,
            _ => panic!("Invalid compsition type conversion."),
        }
    }
    pub fn extract_bool(compsition_data: CompositionDataType) -> bool {
        match compsition_data {
            Self::Bool(v) => v,
            _ => panic!("Invalid compsition type conversion."),
        }
    }
}

pub trait IComposition: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn composition_fields(&self) -> HashMap<&'static str, usize>;
    fn compose(&self) -> Result<Vec<Box<[CompositionDataType]>>, Box<dyn Error>>;
    fn safe_compose(&self) -> Result<Vec<Box<[CompositionDataType]>>, Box<dyn Error>> {
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
        Box::new(SMA200Composition::instance().clone()),
        Box::new(ETHBTCSTATARB::instance().clone()),
    ]
});
