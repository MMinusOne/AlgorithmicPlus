use std::error::Error;

use num_traits::{FromPrimitive, Num};

use crate::{user::library::ITechnicalIndicator, utils::classes::charting::ChartingData};

pub struct ATR<T: Num + Copy + FromPrimitive> {
    name: String,
    description: String,
    period: usize,
    current_tr: T,
    previous_atr: T,
}

impl<T: Num + Copy + FromPrimitive> ITechnicalIndicator<T> for ATR<T> {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn description(&self) -> &str {
        return &self.description;
    }

    fn allocate(&mut self, data: T) {
        self.current_tr = data;
    }

    fn get_data(&mut self) -> Option<T> {
        if self.period == 0 {
            return None;
        }

        let period = T::from_usize(self.period)?;
        let atr = (self.previous_atr * (period - T::from_u8(1)?) + self.current_tr) / period;
        self.previous_atr = atr;
        return Some(atr);
    }

    fn render(&self, timestamps: Vec<i64>) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        let charting_data: Vec<ChartingData> = vec![];

        Ok(charting_data)
    }
}

impl<T: Num + Copy + FromPrimitive> ATR<T> {
    fn new(period: usize, current_tr: T) -> Self {
        return Self {
            name: "ATR".into(),
            description: "Average True Range.".into(),
            period,
            current_tr: T::from_u8(0).unwrap(),
            previous_atr: T::from_u8(0).unwrap(),
        };
    }
}
