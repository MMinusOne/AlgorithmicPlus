use crate::{user::library::ITechnicalIndicator, utils::classes::charting::ChartingData};
use num_traits::{FromPrimitive, Num};
use std::collections::VecDeque;

struct SMA<T: Num + Copy + FromPrimitive> {
    name: String,
    description: String,
    period: usize,
    current_sum: T,
    values: VecDeque<T>,
}

impl<T: Num + Copy + FromPrimitive> ITechnicalIndicator<T> for SMA<T> {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn description(&self) -> &str {
        return &self.description;
    }

    fn allocate(&mut self, data: T) {
        if self.values.len() == self.period {
            if let Some(last_value) = self.values.pop_front() {
                self.current_sum = self.current_sum - last_value;
            }
        }

        self.values.push_back(data);
        self.current_sum = self.current_sum + data;
    }

    fn get_data(&mut self) -> Option<T> {
        if self.values.len() < self.period {
            return None;
        }

        let period = T::from_usize(self.period)?;
        let value = self.current_sum / period;

        return Some(value);
    }

    fn render(&mut self) -> Option<Vec<ChartingData>> {
        let charting_data: Vec<ChartingData> = vec![];

        return Some(charting_data);
    }
}

impl<T: Num + Copy + FromPrimitive> SMA<T> {
    fn new(period: usize) -> Self {
        return Self {
            name: "Simple Moving Average".into(),
            description: "The mean of the last {period} elements".into(),
            period,
            current_sum: T::zero(),
            values: VecDeque::new()
        };
    }
}
