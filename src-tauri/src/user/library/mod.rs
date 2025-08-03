use crate::utils::classes::charting::ChartingData;
use std::error::Error;

pub trait ITechnicalIndicator<T> { 
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn allocate(&mut self, data: T);
    fn get_data(&mut self) -> Option<T>;
    fn render(&self, timestamps: Vec<i64>) -> Result<Vec<ChartingData>, Box<dyn Error>>;
}

pub mod technical_indicators;