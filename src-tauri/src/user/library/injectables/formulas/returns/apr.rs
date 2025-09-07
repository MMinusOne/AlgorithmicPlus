use crate::user::library::IInjectable;
use crate::utils::classes::charting::ChartingData;
use std::error::Error;

pub struct APR {
    name: String,
    description: String,
    sum: f32,
    count: usize,
    annualization_factor: f32,
}

impl IInjectable<f32, f32> for APR {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn description(&self) -> &str {
        return &self.description;
    }

    fn allocate(&mut self, return_value: f32) {
        self.sum += return_value;
        self.count += 1;
    }

    fn get_data(&mut self) -> Option<f32> {
        if self.count == 0 {
            return None;
        }

        let mean_return = self.sum / self.count as f32;
        let apr = (1.0 + mean_return).powf(self.annualization_factor) - 1.0;

        Some(apr)
    }

    fn render(&self, timestamps: Vec<i64>) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        let charting_data: Vec<ChartingData> = vec![];

        Ok(charting_data)
    }
}

impl APR {
    pub fn new() -> Self {
        return Self {
            name: "APR".into(),
            description: "Annual Percentage Rate - annualized return rate".into(),
            sum: 0.0,
            count: 0,
            annualization_factor: 252.0,
        };
    }
}
