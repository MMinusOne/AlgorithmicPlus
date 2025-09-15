use crate::{user::strategies::Trade, utils::classes::charting::ChartingData};
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

pub mod injectables;
pub use injectables::*;
