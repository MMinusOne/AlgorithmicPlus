pub mod sma_200_composition;
pub use sma_200_composition::SMA200Composition;

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
}

pub trait IComposition: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn composition_fields(&self) -> HashMap<&'static str, usize>;
    fn compose(&self) -> Result<Vec<Box<[CompositionDataType]>>, Box<dyn Error>>;
    fn safe_compose(&mut self) -> Result<Vec<Box<[CompositionDataType]>>, Box<dyn Error>> {
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

    fn extract_int(&self, compsition_data: CompositionDataType) -> i64 {
        match compsition_data {
            CompositionDataType::Int(v) => v,
            _ => panic!("Invalid compsition type conversion."),
        }
    }

    fn extract_float(&self, compsition_data: CompositionDataType) -> f32 {
        match compsition_data {
            CompositionDataType::Float(v) => v,
            _ => panic!("Invalid compsition type conversion."),
        }
    }

    fn extract_option_float(&self, compsition_data: CompositionDataType) -> Option<f32> {
        match compsition_data {
            CompositionDataType::OptionFloat(v) => v,
            _ => panic!("Invalid compsition type conversion."),
        }
    }
}

pub static COMPOSITIONS: LazyLock<Vec<Box<dyn IComposition>>> =
    LazyLock::new(|| vec![Box::new(SMA200Composition::instance().clone())]);
