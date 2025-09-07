use crate::{
    user::library::{
        standard_deviation::StandardDeviation,
        sma::SMA,
        IInjectable
    },
    utils::classes::charting::{ChartingData, LineChartingData, LineData},
};
use std::error::Error;

pub struct BollingerBands {
    name: String,
    description: String,
    period: usize,
    sma: SMA,
    std_dev: StandardDeviation,
    upper_band: Vec<f32>,
    lower_band: Vec<f32>,
}

impl IInjectable<f32, (f32, f32)> for BollingerBands {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn allocate(&mut self, price: f32) {
        self.sma.allocate(price);
        self.std_dev.allocate(price);

        if let Some(sma_value) = self.sma.get_data() {
            if let Some(std_dev_value) = self.std_dev.get_data() {
                let std_dev: f32 = 1.0;
                let upper = sma_value + std_dev * std_dev_value;
                let lower = sma_value - std_dev * std_dev_value;

                self.upper_band.push(upper);
                self.lower_band.push(lower);
            } else {
                self.upper_band.push(f32::NAN);
                self.lower_band.push(f32::NAN);
            }
        } else {
            self.upper_band.push(f32::NAN);
            self.lower_band.push(f32::NAN);
        }
    }

    fn get_data(&mut self) -> Option<(f32, f32)> {
        if self.upper_band.is_empty() || self.lower_band.is_empty() {
            return None;
        }
        let upper = *self.upper_band.last().unwrap();

        if upper.is_nan() {
            return None;
        }

        let lower = *self.lower_band.last().unwrap();

        if lower.is_nan() {
            return None;
        }

        Some((upper, lower))
    }

    fn render(&self, timestamps: Vec<i64>) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        let mut upper_line_data: Vec<Option<LineData>> = vec![];
        let mut lower_line_data: Vec<Option<LineData>> = vec![];

        for (i, timestamp) in timestamps.iter().enumerate() {
            upper_line_data.push(Some(LineData {
                time: timestamp.clone(),
                value: self.upper_band.get(i).cloned().unwrap_or(f32::NAN),
                color: Some("red".into()),
            }));
            lower_line_data.push(Some(LineData {
                time: timestamp.clone(),
                value: self.lower_band.get(i).cloned().unwrap_or(f32::NAN),
                color: Some("green".into()),
            }));
        }

        let charting_data: Vec<ChartingData> = vec![
            ChartingData::LineChartingData(LineChartingData {
                chart_type: "line".into(),
                height: None,
                data: upper_line_data,
                pane: Some(0),
                title: Some(format!("Upper Bollinger Band")),
            }),
            ChartingData::LineChartingData(LineChartingData {
                chart_type: "line".into(),
                height: None,
                data: lower_line_data,
                pane: Some(0),
                title: Some(format!("Lower Bollinger Band")),
            }),
        ];

        Ok(charting_data)
    }
}

impl BollingerBands {
    pub fn get_stddev(&mut self, n: i8) -> Option<f32> {
        let (std_dev, _) = self.get_data()?;
        Some(std_dev * n as f32)
    }

    pub fn new(period: usize) -> Self {
        Self {
            name: "Bollinger Bands".into(),
            description: format!("Bollinger Bands with a period of {}", period),
            period,
            sma: SMA::new(period),
            std_dev: StandardDeviation::new(),
            upper_band: Vec::new(),
            lower_band: Vec::new(),
        }
    }
}
