use crate::utils::classes::charting::{LineChartingData, LineData};
use crate::{user::library::IInjectable, utils::classes::charting::ChartingData};
use num_traits::ToPrimitive;
use std::collections::VecDeque;
use std::error::Error;

pub struct Renko {
    name: String,
    description: String,
    fixed_change_amount: f32,
    last_renko_price: Option<f32>,
}

impl IInjectable<f32, f32> for Renko {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn description(&self) -> &str {
        return &self.description;
    }

    fn allocate(&mut self, close_price: f32) {
        if self.last_renko_price.is_none() {
            self.last_renko_price = Some(close_price);
            return;
        }

        let last_renko_price = self.last_renko_price.unwrap();

        let change = close_price - last_renko_price;

        if (change).abs() > self.fixed_change_amount {
            self.last_renko_price = Some(close_price);
        }
    }

    fn get_data(&mut self) -> Option<f32> {
        return self.last_renko_price;
    }

    fn render(&self, timestamps: Vec<i64>) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        let charting_data: Vec<ChartingData> = vec![];
        Ok(charting_data)
    }
}

impl Renko {
    pub fn new(fixed_change_amount: f32) -> Self {
        return Self {
            name: "Renko".into(),
            description: "Renko".into(),
            fixed_change_amount,
            last_renko_price: None,
        };
    }
}
