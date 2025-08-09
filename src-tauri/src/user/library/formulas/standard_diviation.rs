use crate::{user::library::IInjectable, utils::classes::charting::ChartingData};
use std::error::Error;

/**
 * Mean = (2+4+6+8+10)/5 = 6
Squared differences: (2-6)²=16, (4-6)²=4, (6-6)²=0, (8-6)²=4, (10-6)²=16
Sum = 16+4+0+4+16 = 40
Population: σ = √(40/5) = √8 = 2.83
Sample: s = √(40/4) = √10 = 3.16
 */

pub struct StandardDivation {
    name: String,
    description: String,
    sum: f32,
    sum_squared: f32,
    count: usize,
}

impl IInjectable<f32, f32> for StandardDivation {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn description(&self) -> &str {
        return &self.description;
    }

    fn allocate(&mut self, pl: f32) {
        self.sum += pl;
        self.sum_squared += pl * pl;
        self.count += 1;
    }

    fn get_data(&mut self) -> Option<f32> {
        if self.count < 2 {
            return None;
        }

        let n = self.count as f32;
        let mean = self.sum / n;

        let variance = (self.sum_squared - n * mean * mean) / (n - 1.0);
        let std_dev = variance.sqrt();

        Some(std_dev)
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
            sum: 0 as f32,
            sum_squared: 0 as f32,
            count: 0,
        };
    }
}
