use crate::{
    user::{
        composer::{CompositionDataType, IComposition},
        library::trade::Trade,
    },
    utils::classes::charting::ChartingData,
};
use std::sync::LazyLock;
use std::{collections::HashMap, error::Error};

pub enum StrategyData {
    CompositionDataType(CompositionDataType),
    InjectableFloatData(Option<f32>),
}

impl StrategyData {
    pub fn extract_composition_int(strategy_data: StrategyData) -> i64 {
        match strategy_data {
            StrategyData::CompositionDataType(composition_data) => {
                CompositionDataType::extract_int(composition_data)
            }

            _ => panic!("Invalid strategy type conversion."),
        }
    }
    pub fn extract_composition_float(strategy_data: StrategyData) -> f32 {
        match strategy_data {
            StrategyData::CompositionDataType(composition_data) => {
                CompositionDataType::extract_float(composition_data)
            }

            _ => panic!("Invalid strategy type conversion."),
        }
    }
    pub fn extract_composition_option_float(strategy_data: StrategyData) -> Option<f32> {
        match strategy_data {
            StrategyData::CompositionDataType(composition_data) => {
                CompositionDataType::extract_option_float(composition_data)
            }

            _ => panic!("Invalid strategy type conversion."),
        }
    }
}

#[derive(Clone, Debug)]
enum Metric {
    Sharpe,
}

// MAKE TRADE MANAGER WRAPPER TO GIVE BACKTEST MANAGER AND HANDLE CAPITAL ALLOCATION
#[derive(Clone, Debug)]
pub struct BacktestManager {
    current_timestamp: Option<i64>,
    current_price: Option<f32>,
    initial_capital: f32,
    available_capital: f32,
    trades: Vec<Trade>,
    metrics: HashMap<Metric, f32>,
    backtest_ended: bool,
}

impl BacktestManager {
    pub fn initial_capital(&self) -> f32 {
        return self.initial_capital;
    }

    pub fn available_capital(&self) -> f32 {
        return self.available_capital;
    }

    pub fn update_price(&mut self, timestamp: i64, price: f32) {
        if self.backtest_ended {
            return;
        }
        self.current_timestamp = Some(timestamp);
        self.current_price = Some(price);
    }

    pub fn get_last_trade(&self) -> Option<Trade> {
        if self.trades.len() == 0 {
            return None;
        }

        let trade = self.trades[self.trades.len() - 1];
        return Some(trade.clone());
    }

    pub fn open_trade(&mut self, trade: &mut Trade) {
        if self.backtest_ended {
            return;
        }
        let allocation = trade.capital_allocation().unwrap().to_owned();
        if self.available_capital() >= allocation {
            trade.freeze_open_timestamp(self.current_timestamp.unwrap());
            trade.freeze_open_price(self.current_price.unwrap());
            self.adjust_available_capital(-allocation);
            self.trades.push(*trade);
        }
    }

    pub fn close_trade(&mut self, trade: &mut Trade) {
        if self.backtest_ended {
            return;
        }
        let current_price = self.current_price.unwrap();
        let current_timestamp = self.current_timestamp.unwrap();
        if let Some(existing_trade) = self
            .trades
            .iter_mut()
            .find(|t| t.id().to_string() == trade.id().to_string())
        {
            existing_trade.close(current_price, current_timestamp);
            self.adjust_available_capital(
                trade.capital_allocation().unwrap() as f32 + trade.pl_fixed(),
            );
        }
    }

    fn adjust_available_capital(&mut self, change: f32) {
        if self.backtest_ended {
            return;
        }
        self.available_capital += change;
    }

    pub fn trades(&self) -> &Vec<Trade> {
        return &self.trades;
    }

    pub fn metrics(&self) -> &HashMap<Metric, f32> {
        return &self.metrics;
    }

    // OPEN, CLOSE, DEDUCES AND ADDS BACKTEST MANGER CAPITAL ALLOC
    pub fn backtest_ended(&mut self) -> BacktestResult {
        self.backtest_ended = true;
        return BacktestResult::from(self.to_owned());
    }
}

impl BacktestManager {
    pub fn new(options: BacktestOptions) -> Self {
        return Self {
            initial_capital: options.initial_capital,
            available_capital: options.initial_capital,
            metrics: HashMap::new(),
            current_price: None,
            current_timestamp: None,
            trades: Vec::new(),
            //record_metrics: Vec<Metric>
            backtest_ended: false,
        };
    }
}

pub struct BacktestOptions {
    pub initial_capital: f32,
}

pub struct BacktestResult {
    initial_capital: f32,
    growth_capital: f32,
    trades: Vec<Trade>,
    metrics: HashMap<Metric, f32>,
}

impl BacktestResult {
    pub fn initial_capital(&self) -> f32 {
        return self.initial_capital;
    }

    pub fn growth_capital(&self) -> f32 {
        return self.growth_capital;
    }

    pub fn trades(&self) -> Vec<Trade> {
        return self.trades.clone();
    }

    pub fn metrics(&self) -> HashMap<Metric, f32> {
        return self.metrics.clone();
    }
}

impl BacktestResult {
    pub fn from(backtest_manager: BacktestManager) -> Self {
        return Self {
            initial_capital: backtest_manager.initial_capital(),
            growth_capital: backtest_manager.available_capital(),
            trades: backtest_manager
                .trades()
                .to_vec()
                .into_iter()
                .filter(|trade| trade.is_closed())
                .collect(),
            metrics: backtest_manager.metrics().to_owned(),
        };
    }
}

pub trait IStrategy: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn composition(&self) -> &'static dyn IComposition;
    // fn optimizables(&self) -> HashMap<&'static str, (OptimizationData, InjectableState)> {
    //     return HashMap::new();
    // }
    // fn optimization_target(&self, backtest_result: BacktestManager) -> i16 {
    //     return backtest_result.sharpe as i16;
    // }
    // fn wfo(&self, optimizer: OptimizationStrategy) {}
    // fn optimized_backtest(&self, optimizer: OptimizationStrategy) {}
    fn backtest(&self) -> Result<BacktestResult, Box<dyn Error>>;
    fn render_equity_growth(&self, backtest: &BacktestResult) -> Vec<ChartingData>;
    fn render_percentage_growth(&self, backtest: &BacktestResult) -> Vec<ChartingData>;
    fn render_portfolio_percentage_growth(&self, backtest: &BacktestResult) -> Vec<ChartingData>;
    fn save(&self) -> Result<(), Box<dyn Error>>;
}

pub mod sma_200;
pub use sma_200::SMA200Strategy;

pub static STRATEGIES: LazyLock<Vec<Box<dyn IStrategy>>> =
    LazyLock::new(|| vec![Box::new(SMA200Strategy::new())]);
