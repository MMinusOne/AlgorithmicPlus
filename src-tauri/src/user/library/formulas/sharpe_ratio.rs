use crate::{
    user::library::{
        formulas::standard_diviation::{self, StandardDeviation},
        IInjectable,
    },
    utils::classes::charting::ChartingData,
};
use std::error::Error;

pub struct SharpeRatio {
    name: String,
    description: String,
    risk_free_rate: f32,
    std_dev: StandardDeviation,
}

impl IInjectable<f32, f32> for SharpeRatio {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn description(&self) -> &str {
        return &self.description;
    }

    fn allocate(&mut self, return_value: f32) {
        self.std_dev.allocate(return_value);
    }

    fn get_data(&mut self) -> Option<f32> {
        if self.std_dev.count() < 2 {
            return None;
        }

        let mean_return = self.std_dev.sum() / (self.std_dev.count() as f32);
        let volatility = self.std_dev.get_data()?;

        if volatility == 0.0 {
            return None;
        }

        let excess_return = mean_return - self.risk_free_rate;
        let sharpe_ratio = excess_return / volatility;

        Some(sharpe_ratio)
    }

    fn render(&self, timestamps: Vec<i64>) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        let charting_data: Vec<ChartingData> = vec![];

        Ok(charting_data)
    }
}

impl SharpeRatio {
    fn new(risk_free_rate: Option<f32>) -> Self {
        return Self {
            name: "Sharpe Ratio".into(),
            description: "Risk-adjusted return metric".into(),
            risk_free_rate: match risk_free_rate {
                Some(r) => r,
                None => 0.0,
            },
            std_dev: StandardDeviation::new(),
        };
    }
}
