use crate::{user::library::ITechnicalIndicator, utils::classes::charting::ChartingData};
use num_traits::{FromPrimitive, Num, Signed};

pub struct TR<T: Num + Copy + FromPrimitive + Ord + Signed> {
    name: String,
    description: String,
    current_high: T,
    current_low: T,
    previous_close: Option<T>,
}

impl<T: Num + Copy + FromPrimitive + Ord + Signed> ITechnicalIndicator<T> for TR<T> {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn description(&self) -> &str {
        return &self.description;
    }

    fn allocate(&mut self, data: T) {
        self.current_high = self.current_high.max(data);
        self.current_low = self.current_low.min(data);
        self.previous_close = Some(data);
    }

    fn get_data(&mut self) -> Option<T> {
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

    fn render(&mut self) -> Option<Vec<ChartingData>> {
        let charting_data: Vec<ChartingData> = vec![];

        return Some(charting_data);
    }
}

impl<T: Num + Copy + FromPrimitive + Ord + Signed> TR<T> {
    fn new() -> Self {
        return Self {
            name: "True Average".into(),
            description: "The Average Rage".into(),
            current_high: T::from_u8(0).unwrap(),
            current_low: T::from_u8(0).unwrap(),
            previous_close: None,
        };
    }
}
