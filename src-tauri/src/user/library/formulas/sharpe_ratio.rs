use crate::{
    user::library::{
        formulas::standard_deviation::{self, StandardDeviation},
        IInjectable,
    },
    utils::classes::charting::{ChartingData, LineChartingData, LineData},
};
use std::error::Error;

pub struct SharpeRatio {
    name: String,
    description: String,
    risk_free_rate: f32,
    std_dev: StandardDeviation,
    returns: Vec<f32>,
    timestamps: Vec<i64>,
    annualization_factor: f32,
}

impl IInjectable<f32, f32> for SharpeRatio {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn description(&self) -> &str {
        return &self.description;
    }

    fn allocate(&mut self, return_value: f32) {
        self.returns.push(return_value);
        let excess_return = return_value - self.risk_free_rate;
        self.std_dev.allocate(excess_return);
    }

    fn get_data(&mut self) -> Option<f32> {
        if self.std_dev.count() < 2 {
            return None;
        }

        let mean_excess_return = self.std_dev.sum() / (self.std_dev.count() as f32);

        let volatility = self.std_dev.get_data()?;

        if volatility == 0.0 {
            return None;
        }

        let sharpe_ratio = mean_excess_return / volatility;

        let annualized_sharpe = sharpe_ratio * self.annualization_factor.sqrt();

        Some(annualized_sharpe)
    }

    fn render(&self, timestamps: Vec<i64>) -> Result<Vec<ChartingData>, Box<dyn Error>> {
        if self.returns.len() < 2 {
            return Ok(vec![]);
        }

        let mut line_data: Vec<Option<LineData>> = vec![];

        // Calculate rolling Sharpe ratio
        for i in 2..self.returns.len() {
            let window_returns = &self.returns[..=i];
            let mean_return = window_returns.iter().sum::<f32>() / window_returns.len() as f32;

            // Calculate variance
            let variance = window_returns
                .iter()
                .map(|&r| (r - mean_return).powi(2))
                .sum::<f32>()
                / (window_returns.len() - 1) as f32;

            let std_dev = variance.sqrt();

            if std_dev > 0.0 {
                let excess_return = mean_return - self.risk_free_rate;
                let sharpe = (excess_return / std_dev) * self.annualization_factor.sqrt();

                line_data.push(Some(LineData {
                    time: timestamps.get(i).copied().unwrap_or(0),
                    value: sharpe,
                    color: Some("green".into()),
                }));
            }
        }

        let charting_data: Vec<ChartingData> =
            vec![ChartingData::LineChartingData(LineChartingData {
                chart_type: "line".into(),
                height: None,
                data: line_data,
                pane: Some(0),
                title: Some(format!(
                    "Sharpe Ratio (Rf={:.2}%)",
                    self.risk_free_rate * 100.0
                )),
            })];

        Ok(charting_data)
    }
}

impl SharpeRatio {
    pub fn new(risk_free_rate: Option<f32>) -> Self {
        return Self {
            name: "Sharpe Ratio".into(),
            description: "Risk-adjusted return metric (annualized)".into(),
            risk_free_rate: match risk_free_rate {
                Some(r) => r,
                None => 0.0,
            },
            std_dev: StandardDeviation::new(),
            returns: Vec::new(),
            timestamps: Vec::new(),
            annualization_factor: 252.0, // Assuming daily returns (252 trading days per year)
        };
    }

    /// Create a new Sharpe ratio calculator with custom annualization factor
    pub fn new_with_annualization(risk_free_rate: Option<f32>, annualization_factor: f32) -> Self {
        return Self {
            name: "Sharpe Ratio".into(),
            description: "Risk-adjusted return metric (annualized)".into(),
            risk_free_rate: match risk_free_rate {
                Some(r) => r,
                None => 0.0,
            },
            std_dev: StandardDeviation::new(),
            returns: Vec::new(),
            timestamps: Vec::new(),
            annualization_factor,
        };
    }

    /// Get the current risk-free rate
    pub fn risk_free_rate(&self) -> f32 {
        self.risk_free_rate
    }

    /// Set the risk-free rate
    pub fn set_risk_free_rate(&mut self, rate: f32) {
        self.risk_free_rate = rate;
    }

    /// Get the annualization factor
    pub fn annualization_factor(&self) -> f32 {
        self.annualization_factor
    }

    /// Set the annualization factor
    pub fn set_annualization_factor(&mut self, factor: f32) {
        self.annualization_factor = factor;
    }

    /// Get the number of observations
    pub fn count(&self) -> usize {
        self.returns.len()
    }

    /// Get all returns
    pub fn returns(&self) -> &[f32] {
        &self.returns
    }

    /// Clear all data
    pub fn clear(&mut self) {
        self.returns.clear();
        self.timestamps.clear();
        self.std_dev = StandardDeviation::new();
    }

    /// Calculate the mean return
    pub fn mean_return(&self) -> Option<f32> {
        if self.returns.is_empty() {
            return None;
        }
        Some(self.returns.iter().sum::<f32>() / self.returns.len() as f32)
    }

    /// Calculate the mean excess return
    pub fn mean_excess_return(&self) -> Option<f32> {
        self.mean_return().map(|mean| mean - self.risk_free_rate)
    }

    /// Calculate the volatility (standard deviation of returns)
    pub fn volatility(&self) -> Option<f32> {
        if self.returns.len() < 2 {
            return None;
        }

        let mean = self.returns.iter().sum::<f32>() / self.returns.len() as f32;
        let variance = self
            .returns
            .iter()
            .map(|&r| (r - mean).powi(2))
            .sum::<f32>()
            / (self.returns.len() - 1) as f32;

        Some(variance.sqrt())
    }

    /// Calculate the annualized volatility
    pub fn annualized_volatility(&self) -> Option<f32> {
        self.volatility()
            .map(|vol| vol * self.annualization_factor.sqrt())
    }
}
