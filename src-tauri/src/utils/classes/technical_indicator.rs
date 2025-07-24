use crate::commands::ChartingData;

pub trait ITechnicalIndicator { 
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn allocate<T>(&mut self, candle: T);
    fn get_data<T>(&mut self) -> Option<T>;
    fn render(&mut self) -> Option<Vec<ChartingData>>;
}