use crate::{
    library::engines::OptimizationStrategy,
    user::{
        composer::{CompositionDataType, IComposition},
        library::trade::Trade,
    },
    utils::classes::charting::ChartingData,
};
use std::{cell::RefCell, rc::Rc, sync::LazyLock, time::Duration};
use std::{collections::HashMap, error::Error};

pub enum StrategyData {
    CompositionDataType(CompositionDataType),
    InjectableFloatData(Option<f32>),
}

impl StrategyData {
    pub fn extract_composition_int(strategy_data: StrategyData) -> i64 {
        match strategy_data {
            StrategyData::CompositionDataType((composition_data)) => {
                CompositionDataType::extract_int(composition_data)
            }

            _ => panic!("Invalid strategy type conversion."),
        }
    }
    pub fn extract_composition_float(strategy_data: StrategyData) -> f32 {
        match strategy_data {
            StrategyData::CompositionDataType((composition_data)) => {
                CompositionDataType::extract_float(composition_data)
            }

            _ => panic!("Invalid strategy type conversion."),
        }
    }
    pub fn extract_composition_option_float(strategy_data: StrategyData) -> Option<f32> {
        match strategy_data {
            StrategyData::CompositionDataType((composition_data)) => {
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
    initial_capital: u16,
    available_capital: u16,
    performance_time: Duration,
    trades: Vec<Trade>,
    metrics: HashMap<Metric, f32>,
}

impl BacktestManager {
    pub fn initial_capital(&self) -> u16 {
        return self.initial_capital;
    }

    pub fn available_capital(&self) -> u16 {
        return self.available_capital;
    }

    pub fn update_price(&mut self, timestamp: i64, price: f32) {
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
        let allocation = trade.capital_allocation().unwrap().to_owned();
        if self.available_capital() >= allocation {
            trade.freeze_open_timestamp(self.current_timestamp.unwrap());
            trade.freeze_open_price(self.current_price.unwrap());
            self.reduce_available_capital(allocation);
            self.trades.push(*trade);
        }
    }

    pub fn close_trade(&mut self, trade: &mut Trade) {
        let allocation = trade.capital_allocation().unwrap();
        let current_price = self.current_price.unwrap();
        let current_timestamp = self.current_timestamp.unwrap();
        trade.freeze_close_price(current_price);
        trade.freeze_close_timestamp(current_timestamp);
        trade.close();
        println!("{:?}", trade);
        self.add_available_capital(allocation);
    }

    fn reduce_available_capital(&mut self, reduce_capital: u16) {
        self.available_capital -= reduce_capital;
    }

    fn add_available_capital(&mut self, add_capital: u16) {
        self.available_capital += add_capital;
    }

    // OPEN, CLOSE, DEDUCES AND ADDS BACKTEST MANGER CAPITAL ALLOC
    pub fn backtest_ended(&mut self) {
     
    }
}

impl BacktestManager {
    pub fn new(options: BacktestOptions) -> Self {
        return Self {
            performance_time: Duration::new(0, 0),
            initial_capital: options.initial_capital,
            available_capital: options.initial_capital,
            metrics: HashMap::new(),
            current_price: None,
            current_timestamp: None,
            trades: Vec::new()
            //record_metrics: Vec<Metric>
        };
    }
}

struct BacktestOptions {
    pub initial_capital: u16,
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
    fn backtest(&self) -> Result<BacktestManager, Box<dyn Error>>;
    fn render(&self) -> Vec<ChartingData>;
    fn save(&self) -> Result<(), Box<dyn Error>>;
}

pub mod sma_200;
pub use sma_200::SMA200Strategy;

pub static STRATEGIES: LazyLock<Vec<Box<dyn IStrategy>>> =
    LazyLock::new(|| vec![Box::new(SMA200Strategy::new())]);
