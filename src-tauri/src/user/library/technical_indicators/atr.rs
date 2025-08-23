use std::error::Error;
use num_traits::FromPrimitive;
use crate::{user::library::IInjectable, utils::classes::charting::ChartingData};

pub struct ATR {
    name: String,
    description: String,
    period: usize,
    current_tr: f32,
    previous_atr: f32,
}

impl IInjectable<f32, f32> for ATR {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn description(&self) -> &str {
        return &self.description;
    }

    fn allocate(&mut self, data: f32) {
        self.current_tr = data;
    }

    fn get_data(&mut self) -> Option<f32> {
        if self.period == 0 {
            return None;
        }

        let period = f32::from_usize(self.period)?;
        let atr = (self.previous_atr * (period - f32::from_u8(1)?) + self.current_tr) / period;
        self.previous_atr = atr;
        return Some(atr);
    }

    fn render(&self, timestamps: Vec<i64>) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        let charting_data: Vec<ChartingData> = vec![];

        Ok(charting_data)
    }
}

impl ATR {
    fn new(period: usize) -> Self {
        return Self {
            name: "ATR".into(),
            description: "Average True Range.".into(),
            period,
            current_tr: 0 as f32,
            previous_atr: 0 as f32,
        };
    }
}
