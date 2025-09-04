use crate::{user::library::trade::Trade, utils::classes::charting::ChartingData};
use serde::Deserialize;
use std::error::Error;

pub trait IInjectable<Input: for<'de> Deserialize<'de>, Output: for<'de> Deserialize<'de>>:
    Send + Sync
{
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn allocate(&mut self, data: Input);
    fn get_data(&mut self) -> Option<Output>;
    fn render(&self, timestamps: Vec<i64>) -> Result<Vec<ChartingData>, Box<dyn Error>>;
}

pub mod formulas;
pub mod technical_indicators;
pub mod trade;

#[derive(Clone)]
pub enum Injectable {
    FloatNumericType(&'static dyn IInjectable<f32, f32>),
    TupleFloatNumericType(&'static dyn IInjectable<f32, (f32, f32)>),
    IntegerNumericType(&'static dyn IInjectable<i64, f32>),
    TradeNumericType(&'static dyn IInjectable<Trade, f32>),
    // Add any custom types if needed
}

impl Injectable {
    pub fn extract_float_numeric(
        injectable_type: Injectable,
    ) -> &'static dyn IInjectable<f32, f32> {
        match injectable_type {
            Self::FloatNumericType(v) => v,
            _ => panic!("Invalid compsition type conversion."),
        }
    }
    pub fn extract_integer_numeric(
        injectable_type: Injectable,
    ) -> &'static dyn IInjectable<i64, f32> {
        match injectable_type {
            Self::IntegerNumericType(v) => v,
            _ => panic!("Invalid compsition type conversion."),
        }
    }
    pub fn extract_trade_numeric(
        injectable_type: Injectable,
    ) -> &'static dyn IInjectable<Trade, f32> {
        match injectable_type {
            Self::TradeNumericType(v) => v,
            _ => panic!("Invalid compsition type conversion."),
        }
    }
}
