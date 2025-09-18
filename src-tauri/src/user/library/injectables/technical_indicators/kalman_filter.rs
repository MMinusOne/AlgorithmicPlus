use crate::{
    user::library::IInjectable,
    utils::classes::charting::{ChartingData, LineChartingData, LineData},
};
use std::error::Error;

pub struct KalmanFilter {
    name: String,
    description: String,
    q_noise: f32,
    r_noise: f32,
    x_est: Option<f32>,
    p_est: Option<f32>,
    data_values: Vec<f32>,
    is_initialized: bool,
}

impl IInjectable<f32, f32> for KalmanFilter {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn allocate(&mut self, close_price: f32) {
        if !self.is_initialized {
            self.x_est = Some(close_price);
            self.p_est = Some(1.0);
            self.is_initialized = true;
            self.data_values.push(close_price);
            return;
        }

        let x_pred = self.x_est.unwrap();
        let p_pred = self.p_est.unwrap() + self.q_noise;

        let k_gain = p_pred / (p_pred + self.r_noise); // Kalman Gain
        let x_est_new = x_pred + k_gain * (close_price - x_pred);
        let p_est_new = (1.0 - k_gain) * p_pred;

        self.x_est = Some(x_est_new);
        self.p_est = Some(p_est_new);
        self.data_values.push(x_est_new);
    }

    fn get_data(&mut self) -> Option<f32> {
        self.x_est
    }

    fn render(&self, timestamps: Vec<i64>) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        let mut line_data: Vec<Option<LineData>> = vec![];

        for (i, timestamp) in timestamps.iter().enumerate() {
            if i < self.data_values.len() {
                line_data.push(Some(LineData {
                    time: timestamp.clone(),
                    value: self.data_values[i],
                    color: Some("aqua".into()),
                }));
            }
        }

        let charting_data: Vec<ChartingData> =
            vec![ChartingData::LineChartingData(LineChartingData {
                chart_type: "line".into(),
                height: None,
                data: line_data,
                pane: Some(0),
                title: Some("Kalman Filter".into()),
            })];
        Ok(charting_data)
    }
}

impl KalmanFilter {
    pub fn new(q_noise: f32, r_noise: f32) -> Self {
        Self {
            name: "Kalman Filter".into(),
            description: "Algorithmic Kalman Filter for price smoothing and trend estimation"
                .into(),
            q_noise,
            r_noise,
            x_est: None,
            p_est: None,
            data_values: Vec::new(),
            is_initialized: false,
        }
    }
}
