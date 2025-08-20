use crate::{
    library::engines::OptimizationStrategy,
    user::{
        composer::{CompositionDataType, IComposition},
        library::trade::Trade,
    },
    utils::classes::charting::ChartingData,
};
use std::error::Error;
use std::time::Duration;

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

pub struct TradeManager {}

impl TradeManager {}

impl TradeManager {
    pub fn new() -> Self {
        return Self {};
    }
}

pub struct BacktestResult {
    trades: Vec<Trade>,
    performance_time: Duration,
    trade_manager: TradeManager,
    sharpe: i8,
}

impl BacktestResult {
    pub fn trade_manager(&self) -> &TradeManager {
        return &self.trade_manager;
    }

    pub fn backtest_ended(&self) {}
}

impl BacktestResult {
    pub fn new() -> Self {
        return Self {
            performance_time: Duration::new(0, 0),
            trades: Vec::new(),
            trade_manager: TradeManager::new(),
            sharpe: 0,
        };
    }
}

struct StrategyManager {
    results: BacktestResult,
    trades: Vec<Trade>,
}

impl StrategyManager {}

pub trait IStrategy: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn composition(&self) -> &'static dyn IComposition;
    // fn optimizables(&self) -> HashMap<&'static str, (OptimizationData, InjectableState)> {
    //     return HashMap::new();
    // }
    // fn optimization_target(&self, backtest_result: BacktestResult) -> i16 {
    //     return backtest_result.sharpe as i16;
    // }
    // fn wfo(&self, optimizer: OptimizationStrategy) {}
    // fn optimized_backtest(&self, optimizer: OptimizationStrategy) {}
    fn backtest(&self) -> Result<BacktestResult, Box<dyn Error>>;
    fn render(&self) -> Vec<ChartingData>;
    fn save(&self) -> Result<(), Box<dyn Error>>;
}

pub mod sma_200;
pub use sma_200::SMA200Strategy;
