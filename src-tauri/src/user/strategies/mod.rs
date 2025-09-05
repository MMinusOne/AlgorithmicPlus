use crate::{
    user::{
        composer::{CompositionDataType, IComposition},
        library::{formulas::sharpe_ratio::SharpeRatio, trade::Trade, IInjectable},
    },
    utils::classes::charting::ChartingData,
};
use std::{collections::HashMap, error::Error};
use std::{sync::LazyLock, time::Instant};
pub mod double_sma_optimize_strategy;
pub mod sma_200_strategy;
pub mod sma_optimizable_period_strategy;

pub enum StrategyData {
    CompositionDataType(CompositionDataType),
    InjectableFloatData(Option<f32>),
}

impl StrategyData {
    pub fn extract_composition_int(strategy_data: StrategyData) -> i64 {
        match strategy_data {
            StrategyData::CompositionDataType(composition_data) => {
                CompositionDataType::extract_int(&composition_data)
            }

            _ => panic!("Invalid strategy type conversion."),
        }
    }
    pub fn extract_composition_float(strategy_data: StrategyData) -> f32 {
        match strategy_data {
            StrategyData::CompositionDataType(composition_data) => {
                CompositionDataType::extract_float(&composition_data)
            }

            _ => panic!("Invalid strategy type conversion."),
        }
    }
    pub fn extract_composition_option_float(strategy_data: StrategyData) -> Option<f32> {
        match strategy_data {
            StrategyData::CompositionDataType(composition_data) => {
                CompositionDataType::extract_option_float(&composition_data)
            }

            _ => panic!("Invalid strategy type conversion."),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Metric {
    PerformanceTime,

    // Consistency
    StabilityRatio,

    // Market comparasion
    Alpha,
    Beta,
    TimingRatio,

    // Portfolio Activity
    TurnoverRate,

    // Return,
    APR,
    TotalReturns,
    CumulativeReturns,
    CGAR,
    IntervalReturns,

    // Risk
    MaxDrawdown,
    PainIndex,
    StandardDeviation,

    // Risk Adjusted Returns
    BurkeRatio,
    CalmarRatio,
    KappaRatio,
    RarmddRatio,
    SortinoRatio,
    SterlingRatio,
    TreynorRatio,
    SharpeRatio,

    // Trade analysis
    AverageTradeDuration,
    ConsecutiveWinsLoses,
    ProfitFactor,
    RecoveryFactor,
    RiskRewardRatio,
    WinRate,
}

// MAKE TRADE MANAGER WRAPPER TO GIVE BACKTEST MANAGER AND HANDLE CAPITAL ALLOCATION
#[derive(Clone, Debug)]
pub struct BacktestManager {
    current_timestamp: Option<i64>,
    current_price: Option<f32>,
    initial_capital: f32,
    available_capital: f32,
    trades: Vec<Trade>,
    computational_metrics: HashMap<Metric, f32>,
    instant: Instant,
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
        if let Some(existing_trade) = self.trades.iter_mut().find(|t| t.id() == trade.id()) {
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

    // OPEN, CLOSE, DEDUCES AND ADDS BACKTEST MANGER CAPITAL ALLOC
    pub fn backtest_ended(&mut self) -> BacktestResult {
        for trade in self.trades.clone().iter_mut() {
            self.close_trade(trade);
        }

        self.backtest_ended = true;
        self.computational_metrics.insert(
            Metric::PerformanceTime,
            self.instant.elapsed().as_secs_f32(),
        );
        return BacktestResult::from(self.to_owned());
    }
}

impl BacktestManager {
    pub fn new(options: BacktestOptions) -> Self {
        let computational_metrics: HashMap<Metric, f32> = HashMap::new();

        return Self {
            initial_capital: options.initial_capital,
            available_capital: options.initial_capital,
            current_price: None,
            current_timestamp: None,
            trades: Vec::new(),
            computational_metrics,
            instant: Instant::now(),
            backtest_ended: false,
        };
    }
}

pub struct BacktestOptions {
    pub initial_capital: f32,
}
#[derive(Debug)]
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
        let mut metrics: HashMap<Metric, f32> = HashMap::new();

        let mut sharpe = SharpeRatio::new(Some(0.0));

        let mut valid_trades: Vec<Trade> = vec![];

        for trade in backtest_manager.trades() {
            if !trade.is_closed() {
                continue;
            }

            valid_trades.push(trade.to_owned());
            sharpe.allocate(trade.pl_ratio());
        }

        let sharpe = sharpe.get_data().unwrap();
        metrics.insert(Metric::SharpeRatio, sharpe);
        metrics.insert(
            Metric::PerformanceTime,
            backtest_manager
                .computational_metrics
                .get(&Metric::PerformanceTime)
                .unwrap()
                .to_owned(),
        );

        return Self {
            initial_capital: backtest_manager.initial_capital(),
            growth_capital: backtest_manager.available_capital(),
            trades: valid_trades,
            metrics,
        };
    }
}

pub trait IStrategy: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn composition(&self) -> &'static dyn IComposition;
    // fn wfo(&self, optimizer: OptimizationStrategy) {}
    fn optimization_target(&self, backtest_result: &BacktestResult) -> f32 {
        let sharpe = backtest_result
            .metrics()
            .get(&Metric::SharpeRatio)
            .unwrap()
            .to_owned();

        return sharpe;
    }
    fn backtest(
        &self,
        optimization_map: Option<&HashMap<String, CompositionDataType>>,
    ) -> Result<BacktestResult, Box<dyn Error>>;
    fn composed_data(&self) -> Vec<Box<[CompositionDataType]>>;
    fn render_equity_growth(&self, backtest: &BacktestResult) -> Vec<ChartingData>;
    fn render_percentage_growth(&self, backtest: &BacktestResult) -> Vec<ChartingData>;
    fn render_portfolio_percentage_growth(&self, backtest: &BacktestResult) -> Vec<ChartingData>;
    fn save(&self) -> Result<(), Box<dyn Error>>;
}

// pub mod sma_200;
use serde::{Deserialize, Serialize};
// pub use sma_200::SMA200Strategy;

pub static STRATEGIES: LazyLock<Vec<Box<dyn IStrategy>>> = LazyLock::new(|| {
    vec![
        Box::new(sma_200_strategy::Sma200Strategy::new()),
        Box::new(sma_optimizable_period_strategy::SmaOptimizablePeriodStrategy::new()),
        Box::new(double_sma_optimize_strategy::DoubleSmaOptimizablePeriodStrategy::new()),
    ]
});
