use crate::{
    user::library::{trade::Trade, IInjectable},
    utils::classes::charting::ChartingData,
};
use num_traits::{FromPrimitive, Num};
use std::error::Error;

pub struct StandardDivation {
    name: String,
    description: String,
    mean: f32,
}

impl IInjectable<Trade, f32> for StandardDivation {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn description(&self) -> &str {
        return &self.description;
    }

    fn allocate(&mut self, data: Trade) {}

    fn get_data(&mut self) -> Option<f32> {
        // return Some(());
        None
    }

    fn render(&self, timestamps: Vec<i64>) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        let charting_data: Vec<ChartingData> = vec![];

        Ok(charting_data)
    }
}

impl StandardDivation {
    fn new() -> Self {
        return Self {
            name: "Standard Divation".into(),
            description: "Standard Divation.".into(),
            mean: 0 as f32,
        };
    }
}
