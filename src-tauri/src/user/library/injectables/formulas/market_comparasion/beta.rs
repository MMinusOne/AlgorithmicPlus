use crate::{
    user::library::{IInjectable},
    utils::classes::charting::{ChartingData, LineChartingData, LineData},
};
use std::error::Error;

pub struct Beta {
    name: String,
    description: String,
    count: usize,
    sum_asset: f32,
    sum_market: f32,
    sum_asset_sq: f32,
    sum_market_sq: f32,
    sum_asset_market: f32,
    asset_returns: Vec<f32>,
    market_returns: Vec<f32>,
}

impl IInjectable<(f32, f32), f32> for Beta {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn allocate(&mut self, returns: (f32, f32)) {
        let (asset_return, market_return) = returns;
        
        self.asset_returns.push(asset_return);
        self.market_returns.push(market_return);
        
        self.count += 1;
        self.sum_asset += asset_return;
        self.sum_market += market_return;
        self.sum_asset_sq += asset_return * asset_return;
        self.sum_market_sq += market_return * market_return;
        self.sum_asset_market += asset_return * market_return;
    }

    fn get_data(&mut self) -> Option<f32> {
        if self.count < 2 {
            return None;
        }

        let n = self.count as f32;
        
        let mean_asset = self.sum_asset / n;
        let mean_market = self.sum_market / n;
        
        let covariance = (self.sum_asset_market - n * mean_asset * mean_market) / (n - 1.0);
        let market_variance = (self.sum_market_sq - n * mean_market * mean_market) / (n - 1.0);
        
        if market_variance <= 0.0 {
            return None;
        }
        
        Some(covariance / market_variance)
    }

    fn render(&self, timestamps: Vec<i64>) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        if self.asset_returns.len() < 2 {
            return Ok(vec![]);
        }

        let mut line_data: Vec<Option<LineData>> = vec![];

        for i in 2..=self.asset_returns.len() {
            let window_asset = &self.asset_returns[..i];
            let window_market = &self.market_returns[..i];
            
            let n = i as f32;
            let mean_asset = window_asset.iter().sum::<f32>() / n;
            let mean_market = window_market.iter().sum::<f32>() / n;
            
            let covariance = window_asset
                .iter()
                .zip(window_market.iter())
                .map(|(&a, &m)| (a - mean_asset) * (m - mean_market))
                .sum::<f32>()
                / (n - 1.0);
            
            let market_variance = window_market
                .iter()
                .map(|&m| (m - mean_market).powi(2))
                .sum::<f32>()
                / (n - 1.0);

            if market_variance > 0.0 {
                let beta = covariance / market_variance;
                
                line_data.push(Some(LineData {
                    time: timestamps.get(i - 1).copied().unwrap_or(0),
                    value: beta,
                    color: Some(if beta > 1.0 { "red" } else { "blue" }.into()),
                }));
            }
        }

        let charting_data = vec![ChartingData::LineChartingData(LineChartingData {
            chart_type: "line".into(),
            height: None,
            data: line_data,
            pane: Some(0),
            title: Some("Beta (Market Sensitivity)".into()),
        })];

        Ok(charting_data)
    }
}

impl Beta {
    pub fn new() -> Self {
        Self {
            name: "Beta".into(),
            description: "Market sensitivity measure".into(),
            count: 0,
            sum_asset: 0.0,
            sum_market: 0.0,
            sum_asset_sq: 0.0,
            sum_market_sq: 0.0,
            sum_asset_market: 0.0,
            asset_returns: Vec::new(),
            market_returns: Vec::new(),
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn asset_returns(&self) -> &[f32] {
        &self.asset_returns
    }

    pub fn market_returns(&self) -> &[f32] {
        &self.market_returns
    }

    pub fn correlation(&self) -> Option<f32> {
        if self.count < 2 {
            return None;
        }

        let n = self.count as f32;
        let mean_asset = self.sum_asset / n;
        let mean_market = self.sum_market / n;
        
        let covariance = (self.sum_asset_market - n * mean_asset * mean_market) / (n - 1.0);
        let asset_variance = (self.sum_asset_sq - n * mean_asset * mean_asset) / (n - 1.0);
        let market_variance = (self.sum_market_sq - n * mean_market * mean_market) / (n - 1.0);
        
        let denominator = (asset_variance * market_variance).sqrt();
        if denominator <= 0.0 {
            return None;
        }
        
        Some(covariance / denominator)
    }

    pub fn covariance(&self) -> Option<f32> {
        if self.count < 2 {
            return None;
        }

        let n = self.count as f32;
        let mean_asset = self.sum_asset / n;
        let mean_market = self.sum_market / n;
        
        Some((self.sum_asset_market - n * mean_asset * mean_market) / (n - 1.0))
    }

    pub fn market_variance(&self) -> Option<f32> {
        if self.count < 2 {
            return None;
        }

        let n = self.count as f32;
        let mean_market = self.sum_market / n;
        
        Some((self.sum_market_sq - n * mean_market * mean_market) / (n - 1.0))
    }

    pub fn asset_variance(&self) -> Option<f32> {
        if self.count < 2 {
            return None;
        }

        let n = self.count as f32;
        let mean_asset = self.sum_asset / n;
        
        Some((self.sum_asset_sq - n * mean_asset * mean_asset) / (n - 1.0))
    }

    pub fn r_squared(&self) -> Option<f32> {
        self.correlation().map(|corr| corr * corr)
    }
}