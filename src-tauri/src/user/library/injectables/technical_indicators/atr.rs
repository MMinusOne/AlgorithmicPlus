use crate::{
    user::library::{tr::TR, IInjectable},
    utils::classes::charting::ChartingData,
};
use num_traits::FromPrimitive;
use std::{collections::VecDeque, error::Error};

pub struct ATR {
    name: String,
    description: String,
    period: usize,
    tr: TR,
    tr_values: VecDeque<f32>,
    current_atr: Option<f32>,
    is_initialized: bool,
}

impl IInjectable<(f32, f32, f32), f32> for ATR {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn description(&self) -> &str {
        return &self.description;
    }

    fn allocate(&mut self, (high, low, close): (f32, f32, f32)) {
        self.tr.allocate((high, low, close));

        if let Some(tr_value) = self.tr.get_data() {
            if !self.is_initialized {
                self.tr_values.push_back(tr_value);

                if self.tr_values.len() > self.period {
                    self.tr_values.pop_front();
                }

                if self.tr_values.len() == self.period {
                    let sum: f32 = self.tr_values.iter().sum();
                    self.current_atr = Some(sum / self.period as f32);
                    self.is_initialized = true;
                    self.tr_values.clear();
                }
            } else {
                if let Some(prev_atr) = self.current_atr {
                    let period_f32 = self.period as f32;
                    let new_atr = ((period_f32 - 1.0) * prev_atr + tr_value) / period_f32;
                    self.current_atr = Some(new_atr)
                }
            }
        }
    }

    fn get_data(&mut self) -> Option<f32> {
        if self.is_initialized {
            self.current_atr
        } else {
            None
        }
    }

    fn render(&self, timestamps: Vec<i64>) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        let charting_data: Vec<ChartingData> = vec![];

        Ok(charting_data)
    }
}

impl ATR {
    pub fn new(period: usize) -> Self {
        return Self {
            name: "ATR".into(),
            description: "Average True Range.".into(),
            is_initialized: false,
            current_atr: None,
            period: period.max(1),
            tr_values: VecDeque::with_capacity(period),
            tr: TR::new(),
        };
    }
}
