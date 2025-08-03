use crate::utils::classes::charting::{LineChartingData, LineData};
use crate::{user::library::ITechnicalIndicator, utils::classes::charting::ChartingData};
use num_traits::{FromPrimitive, Num, ToPrimitive};
use std::collections::VecDeque;
use std::error::Error;

pub struct SMA<T: Num + Copy + FromPrimitive + ToPrimitive> {
    name: String,
    description: String,
    period: usize,
    current_sum: T,
    prices: VecDeque<T>,
    data_values: Vec<T>,
}

impl<T: Num + Copy + FromPrimitive + ToPrimitive> ITechnicalIndicator<T> for SMA<T> {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn description(&self) -> &str {
        return &self.description;
    }

    fn allocate(&mut self, data: T) {
        if self.prices.len() == self.period {
            if let Some(last_value) = self.prices.pop_front() {
                self.current_sum = self.current_sum - last_value;
            }
        }

        self.prices.push_back(data);
        self.current_sum = self.current_sum + data;
    }

    fn get_data(&mut self) -> Option<T> {
        if self.prices.len() < self.period {
            return None;
        }

        let period = T::from_usize(self.period)?;
        let sma = self.current_sum / period;

        self.data_values.push(sma);

        return Some(sma);
    }

    fn render(&self, timestamps: Vec<i64>) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        let mut line_data: Vec<LineData> = vec![];

        for (i, timestamp) in timestamps.iter().enumerate() {
            line_data.push(LineData {
                time: timestamp.clone(),
                value: self.data_values[i].to_f32().unwrap(),
                color: None,
            });
        }

        let charting_data: Vec<ChartingData> =
            vec![ChartingData::LineChartingData(LineChartingData {
                chart_type: "line".into(),
                height: None,
                data: line_data,
            })];
        Ok(charting_data)
    }
}

impl<T: Num + Copy + FromPrimitive + ToPrimitive> SMA<T> {
    pub fn new(period: usize) -> Self {
        return Self {
            name: "Simple Moving Average".into(),
            description: "The mean of the last {period} elements".into(),
            period,
            current_sum: T::zero(),
            data_values: Vec::new(),
            prices: VecDeque::new(),
        };
    }
}
