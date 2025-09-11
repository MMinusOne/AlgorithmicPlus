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
    cached_atr: Option<f32>,
    slopes_buffer: Vec<f32>,
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

        if let Some(new_atr) = self.atr.get_data() {
            self.cached_atr = Some(new_atr);
        }

        self.close_history.push_back(close);

        if self.close_history.len() > self.window_length + 1 {
            self.close_history.pop_front();
        }

        if self.close_history.len() < self.window_length + 1 {
            return;
        }

        self.slopes_buffer.clear();
        let current_price = close;

        for (i, &historical_price) in self
            .close_history
            .iter()
            .rev()
            .skip(1)
            .take(self.window_length)
            .enumerate()
        {
            let slope = (current_price - historical_price) / (i + 1) as f32;
            self.slopes_buffer.push(slope);
        }

        if self.slopes_buffer.is_empty() {
            return;
        }

        let median_slope = self.quick_median();

        let slope_cap = self.cached_atr.unwrap_or(0.001) * self.atr_multiplier;
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
            cached_atr: None,
            slopes_buffer: Vec::with_capacity(window_length),
        };
    }

    fn quick_median(&mut self) -> f32 {
        let len = self.slopes_buffer.len();
        if len == 0 {
            return 0.0;
        }

        let mid = len / 2;

        // Use select_nth_unstable_by for efficient median finding
        self.slopes_buffer
            .select_nth_unstable_by(mid, |a, b| a.partial_cmp(b).unwrap());

        if len % 2 == 1 {
            // Odd length: return middle element
            self.slopes_buffer[mid]
        } else {
            // Even length: return average of two middle elements
            let left_mid = mid - 1;
            self.slopes_buffer
                .select_nth_unstable_by(left_mid, |a, b| a.partial_cmp(b).unwrap());
            (self.slopes_buffer[left_mid] + self.slopes_buffer[mid]) / 2.0
        }
    }

    fn calculate_slope_cap(&mut self) -> f32 {
        if let Some(atr_value) = self.atr.get_data() {
            atr_value * self.atr_multiplier
        } else {
            0.001
        }
    }
}
