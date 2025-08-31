use crate::utils::classes::charting::{LineChartingData, LineData};
use crate::{user::library::IInjectable, utils::classes::charting::ChartingData};
use num_traits::{FromPrimitive, ToPrimitive};
use std::collections::VecDeque;
use std::error::Error;

pub struct SMA {
    name: String,
    description: String,
    period: usize,
    current_sum: f32,
    prices: VecDeque<f32>,
    data_values: Vec<f32>,
}

impl IInjectable<f32, f32> for SMA {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn description(&self) -> &str {
        return &self.description;
    }

    fn allocate(&mut self, data: f32) {
        if self.prices.len() == self.period {
            if let Some(last_value) = self.prices.pop_front() {
                self.current_sum = self.current_sum - last_value;
            }
        }

        self.prices.push_back(data);
        self.current_sum = self.current_sum + data;
    }

    fn get_data(&mut self) -> Option<f32> {
        if self.prices.len() < self.period {
            return None;
        }

        let sma = self.current_sum / self.period as f32;
        self.data_values.push(sma);
        return Some(sma);
    }

    fn render(&self, timestamps: Vec<i64>) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        let mut line_data: Vec<Option<LineData>> = vec![];

        for (i, timestamp) in timestamps.iter().enumerate() {
            line_data.push(Some(LineData {
                time: timestamp.clone(),
                value: self.data_values[i].to_f32().unwrap(),
                color: None,
            }));
        }

        let charting_data: Vec<ChartingData> =
            vec![ChartingData::LineChartingData(LineChartingData {
                chart_type: "line".into(),
                height: None,
                data: line_data,
                pane: Some(0),
                title: Some(format!("SMA {}", self.period))
            })];
        Ok(charting_data)
    }
}

impl SMA {
    pub fn new(period: usize) -> Self {
        return Self {
            name: "Simple Moving Average".into(),
            description: "The mean of the last {period} elements".into(),
            period,
            current_sum: 0 as f32,
            data_values: Vec::new(),
            prices: VecDeque::new(),
        };
    }
}
