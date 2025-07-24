use crate::commands::ChartingData;

pub trait IComposition: Send + Sync { 
    fn id(&self) -> String;
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn render(&mut self) -> Option<Vec<ChartingData>>;
}