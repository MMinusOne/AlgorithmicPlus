use crate::{user::library::IInjectable, utils::classes::charting::ChartingData};
use std::error::Error;

pub struct TR {
    name: String,
    description: String,
    current_high: Option<f32>,
    current_low: Option<f32>,
    current_close: Option<f32>,
    previous_close: Option<f32>,
}

impl IInjectable<(f32, f32, f32), f32> for TR {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn description(&self) -> &str {
        return &self.description;
    }

    fn allocate(&mut self, (high, low, close): (f32, f32, f32)) {
        self.current_high = Some(high);
        self.current_low = Some(low);
        self.current_close = Some(close);
    }

    fn get_data(&mut self) -> Option<f32> {
        if let (Some(high), Some(low), Some(close)) = (self.current_high, self.current_low, self.current_close) {

            let tr: f32 = if let Some(prev_close) = self.previous_close { 
                let hl = high - low;
                let hc = (high - prev_close).abs();
                let lc = (low - prev_close).abs();
                hl.max(hc).max(lc)
            }else {
                high - low
            };

            self.previous_close = Some(close);
            self.current_high = None;
            self.current_low = None;
            self.current_close = None;

            Some(tr)
        }else { 
            None
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
            name: "True Range".into(),
            description: "The True Range".into(),
            current_high: None,
            current_low: None,
            current_close: None,
            previous_close: None,
        };
    }
}
