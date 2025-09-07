use crate::{
    user::library::{atr::ATR, IInjectable},
    utils::classes::charting::ChartingData,
};
use std::collections::VecDeque;
use std::error::Error;

pub struct TheilSen {
    name: String,
    description: String,
    window_length: usize,
    response: f32,
    atr_length: usize,
    atr_multiplier: f32,
    close_history: VecDeque<f32>,
    baseline: Option<f32>,
    atr: ATR,
}

impl IInjectable<(f32, f32, f32), f32> for TheilSen {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn description(&self) -> &str {
        return &self.description;
    }

    fn allocate(&mut self, (high, low, close): (f32, f32, f32)) {
        self.atr.allocate((high, low, close));

        self.close_history.push_back(close);

        if self.close_history.len() > self.window_length + 1 {
            self.close_history.pop_front();
        }

        if self.close_history.len() < self.window_length + 1 {
            return;
        }

        let mut slopes = Vec::new();
        let current_price = close;
        let current_idx = self.close_history.len() - 1;

        for i in 1..=self.window_length {
            if let Some(historical_price) = self.close_history.get(current_idx - i) {
                let slope = (current_price - historical_price) / i as f32;
                slopes.push(slope);
            }
        }

        if slopes.is_empty() {
            return;
        }

        slopes.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let median_slope = if slopes.len() % 2 == 1 {
            slopes[slopes.len() / 2]
        } else {
            let mid = slopes.len() / 2;
            (slopes[mid - 1] + slopes[mid]) / 2.0
        };

        let slope_cap = self.calculate_slope_cap();
        let capped_slope = median_slope.clamp(-slope_cap, slope_cap);

        let previous_baseline = self.baseline.unwrap_or(current_price);
        self.baseline = Some(previous_baseline + self.response * capped_slope);
    }

    fn get_data(&mut self) -> Option<f32> {
        self.baseline
    }

    fn render(&self, timestamps: Vec<i64>) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        let charting_data: Vec<ChartingData> = vec![];

        Ok(charting_data)
    }
}

impl TheilSen {
    pub fn new(
        window_length: Option<usize>,
        atr_length: Option<usize>,
        atr_multiplier: Option<f32>,
    ) -> Self {
        let window_length = window_length.unwrap_or(2).max(2);
        let atr_length = atr_length.unwrap_or(14);

        return Self {
            name: "TheilSen".into(),
            description: "Theil-Sen median-slope line filter with ATR-based slope capping".into(),
            window_length: window_length,
            response: 0.97,
            atr_length: atr_length,
            atr_multiplier: atr_multiplier.unwrap_or(0.5),
            close_history: VecDeque::with_capacity(window_length + 1 as usize),
            baseline: None,
            atr: ATR::new(atr_length),
        };
    }

    fn calculate_slope_cap(&mut self) -> f32 {
        if let Some(atr_value) = self.atr.get_data() {
            atr_value * self.atr_multiplier
        } else {
            0.001
        }
    }
}
