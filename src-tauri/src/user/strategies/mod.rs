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

#[derive(Clone)]
enum Metric {
    Sharpe,
}

// MAKE TRADE MANAGER WRAPPER TO GIVE BACKTEST MANAGER AND HANDLE CAPITAL ALLOCATION
#[derive(Clone)]
pub struct BacktestManager {
    trades: Vec<Trade>,
    initial_capital: u16,
    available_capital: u16,
    performance_time: Duration,
    trade_manager: Option<TradeManager>,
    metrics: HashMap<Metric, f32>,
}

impl BacktestManager {
    pub fn initial_capital(&self) -> u16 {
        return self.initial_capital;
    }

    pub fn available_capital(&self) -> u16 {
        return self.available_capital;
    }

    pub fn trade_manager(&mut self) -> TradeManager {
        if self.trade_manager.is_none() {
            let backtest_manager_self = Rc::new(RefCell::new(self.clone()));
            self.trade_manager = Some(TradeManager::new(backtest_manager_self));
        }

        return self.trade_manager.take().unwrap().clone();
    }

    pub fn reduce_available_capital(&mut self, reduce_capital: u16) {
        self.available_capital -= reduce_capital;
    }

    // OPEN, CLOSE, DEDUCES AND ADDS BACKTEST MANGER CAPITAL ALLOC
    pub fn backtest_ended(&self) {}
}

impl BacktestManager {
    pub fn new(options: BacktestOptions) -> Self {
        return Self {
            performance_time: Duration::new(0, 0),
            trades: Vec::new(),
            trade_manager: None,
            initial_capital: options.initial_capital,
            available_capital: options.initial_capital,
            metrics: HashMap::new(),
            //record_metrics: Vec<Metric>
        };
    }
}

struct BacktestOptions {
    pub initial_capital: u16,
}

#[derive(Clone)]
pub struct TradeManager {
    backtest_manager: Rc<RefCell<BacktestManager>>,
    current_timestamp: Option<i64>,
    current_price: Option<f32>,
    trades: Vec<Trade>,
}

impl TradeManager {
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

    pub fn open_trade(&self, trade: &mut Trade) {
        // check if backtest_manager has the allocation needed to open the trade
        let mut backtest_manager = self.backtest_manager.borrow_mut();
        let allocation = trade.capital_allocation().unwrap();
        if backtest_manager.available_capital() >= allocation {
            trade.freeze_open_timestamp(self.current_timestamp.unwrap());
            trade.freeze_open_price(self.current_price.unwrap());
            backtest_manager.reduce_available_capital(allocation);
        }
    }
}

impl TradeManager {
    pub fn new(backtest_manager: Rc<RefCell<BacktestManager>>) -> Self {
        return Self {
            backtest_manager,
            current_timestamp: None,
            current_price: None,
            trades: Vec::new(),
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
    fn backtest(&self) -> Result<BacktestManager, Box<dyn Error>>;
    fn render(&self) -> Vec<ChartingData>;
    fn save(&self) -> Result<(), Box<dyn Error>>;
}

pub mod sma_200;
pub use sma_200::SMA200Strategy;

pub static STRATEGIES: LazyLock<Vec<Box<dyn IStrategy>>> = LazyLock::new(|| vec![
    Box::new(SMA200Strategy::new())
]);
