use crate::{user::library::IInjectable, utils::classes::charting::ChartingData};
use num_traits::{FromPrimitive, Signed};
use std::error::Error;

pub struct TR {
    name: String,
    description: String,
    current_high: f32,
    current_low: f32,
    previous_close: Option<f32>,
}

impl IInjectable<f32, f32> for TR {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn description(&self) -> &str {
        return &self.description;
    }

    fn allocate(&mut self, data: f32) {
        self.current_high = self.current_high.max(data);
        self.current_low = self.current_low.min(data);
        self.previous_close = Some(data);
    }

    fn get_data(&mut self) -> Option<f32> {
        if let Some(prev_close) = self.previous_close {
            let delta_high_low = self.current_high - self.current_low;
            let delta_high_close = self.current_high - prev_close;
            let delta_low_close = self.current_low - prev_close;

            let tr = delta_high_low
                .max(delta_high_close.abs())
                .max(delta_low_close.abs());

            return Some(tr);
        } else {
            return None;
        }
    }

    fn render(&self, timestamps: Vec<i64>) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        let charting_data: Vec<ChartingData> = vec![];

        Ok(charting_data)
    }
}

impl TR {
    pub fn new() -> Self {
        return Self {
            name: "True Average".into(),
            description: "The Average Rage".into(),
            current_high: f32::from_u8(0).unwrap(),
            current_low: f32::from_u8(0).unwrap(),
            previous_close: None,
        };
    }
}
