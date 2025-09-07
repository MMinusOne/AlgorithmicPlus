use crate::{user::library::IInjectable, utils::classes::charting::ChartingData};
use std::error::Error;

pub struct StandardDeviation {
    name: String,
    description: String,
    sum: f32,
    sum_squared: f32,
    count: usize,
}

impl StandardDeviation {
    pub fn sum(&self) -> f32 {
        return self.sum;
    }

    pub fn sum_squared(&self) -> f32 {
        return self.sum_squared;
    }

    pub fn count(&self) -> usize {
        return self.count;
    }
}

impl IInjectable<f32, f32> for StandardDeviation {
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

        let variance = (self.sum_squared - (self.sum * self.sum) / n) / (n - 1.0);
        let std_dev = variance.sqrt();

        Some(std_dev)
    }

    fn render(&self, timestamps: Vec<i64>) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        let charting_data: Vec<ChartingData> = vec![];

        Ok(charting_data)
    }
}

impl StandardDeviation {
    pub fn new() -> Self {
        return Self {
            name: "Standard Deviation".into(),
            description: "Standard Deviation calculation.".into(),
            sum: 0 as f32,
            sum_squared: 0 as f32,
            count: 0,
        };
    }
}
