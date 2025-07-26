use crate::commands::ChartingData;

pub trait ITechnicalIndicator<T> { 
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn allocate(&mut self, data: T);
    fn get_data(&mut self) -> Option<T>;
    fn render(&mut self) -> Option<Vec<ChartingData>>;
}

pub mod sma;