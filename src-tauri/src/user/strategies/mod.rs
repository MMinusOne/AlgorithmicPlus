use crate::{
    user::{
        composer::{CompositionDataType, IComposition},
        library::{
            injectables::formulas::{
                apr::APR,
                consecutive_wins_losses::{self, ConsecutiveWinsLosses},
                sharpe_ratio::SharpeRatio,
                standard_deviation::StandardDeviation,
            },
            IInjectable,
        },
    },
    utils::classes::charting::ChartingData,
};
use std::{collections::HashMap, error::Error};
use std::{sync::LazyLock, time::Instant};
pub mod double_sma_optimize_strategy;
pub mod sma_200_strategy;
pub mod sma_optimizable_period_strategy;

use serde::{Deserialize, Serialize};
use std::marker::Copy;
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum TradeSide {
    LONG,
    SHORT,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Trade {
    id: Uuid,
    open_timestamp: Option<i64>,
    close_timestamp: Option<i64>,
    capital_allocation: Option<f32>,
    open_price: Option<f32>,
    close_price: Option<f32>,
    leverage: f32,
    side: TradeSide,
    is_closed: bool,
    pl_ratio: f32,
    pl_fixed: f32,
    pl_portfolio: f32,
    portfolio_value_at_open: Option<f32>,
}

impl Trade {
    pub fn id(&self) -> Uuid {
        return self.id;
    }

    pub fn freeze_open_timestamp(&mut self, timestamp: i64) {
        if self.open_timestamp.is_none() {
            self.open_timestamp = Some(timestamp)
        };
    }

    pub fn freeze_open_price(&mut self, open_price: f32) {
        if self.open_price.is_none() {
            self.open_price = Some(open_price);
        };
    }

    pub fn freeze_portfolio_value_at_open(&mut self, portfolio_value_at_open: f32) {
        if self.portfolio_value_at_open.is_none() {
            self.portfolio_value_at_open = Some(portfolio_value_at_open);
        }
    }

    pub fn open_timestamp(&self) -> Option<i64> {
        return self.open_timestamp;
    }

    pub fn close_timestamp(&self) -> Option<i64> {
        return self.close_timestamp;
    }

    pub fn close(&mut self, close_price: f32, close_timestamp: i64) {
        if !self.is_closed {
            self.close_price = Some(close_price);
            self.close_timestamp = Some(close_timestamp);
            // Only calculate P&L if we have all required data
            if self.open_price.is_some()
                && self.capital_allocation.is_some()
                && self.portfolio_value_at_open.is_some()
            {
                let open_price = self.open_price.unwrap();
                let portfolio_value_at_open = self.portfolio_value_at_open.unwrap();
                let leverage = self.leverage;
                let trade_allocation = self.capital_allocation().unwrap() as f32;

                let price_change_ratio = match self.side {
                    TradeSide::LONG => (close_price - open_price) / open_price,
                    TradeSide::SHORT => (open_price - close_price) / open_price,
                };

                self.pl_ratio = price_change_ratio * leverage * 100.0;
                self.pl_fixed = price_change_ratio * leverage * trade_allocation;
                self.pl_portfolio = (self.pl_fixed / portfolio_value_at_open) * 100.0;
            }
            self.is_closed = true;
        }
    }

    pub fn pl_portfolio(&self) -> f32 {
        return self.pl_portfolio;
    }

    pub fn pl_fixed(&self) -> f32 {
        return self.pl_fixed;
    }

    pub fn pl_unrealized_fixed(&self, current_price: Option<f32>) -> f32 {
        let open_price = self.open_price.unwrap();
        if current_price.is_none() {
            return 0.0;
        }

        let trade_allocation = self.capital_allocation.unwrap();
        let current_price = current_price.unwrap();

        let leverage = self.leverage;

        let price_change_ratio = match self.side {
            TradeSide::LONG => (current_price - open_price) / open_price,
            TradeSide::SHORT => (open_price - current_price) / open_price,
        };

        return price_change_ratio * leverage * trade_allocation;
    }

    pub fn pl_ratio(&self) -> f32 {
        return self.pl_ratio;
    }

    pub fn is_closed(&self) -> bool {
        return self.is_closed;
    }

    pub fn side(&self) -> TradeSide {
        return self.side;
    }

    pub fn open_price(&self) -> Option<f32> {
        return self.open_price;
    }

    pub fn close_price(&self) -> Option<f32> {
        return self.close_price;
    }

    pub fn leverage(&self) -> f32 {
        return self.leverage;
    }

    pub fn capital_allocation(&self) -> Option<f32> {
        return self.capital_allocation;
    }

    pub fn new(trade_options: TradeOptions) -> Self {
        return Self {
            id: Uuid::new_v4(),
            open_timestamp: None,
            close_timestamp: None,
            open_price: None,
            close_price: None,
            capital_allocation: trade_options.capital_allocation,
            portfolio_value_at_open: None,
            leverage: match trade_options.leverage {
                Some(l) => l,
                None => 1.0,
            },
            side: trade_options.side,
            is_closed: false,
            pl_ratio: 0 as f32,
            pl_fixed: 0 as f32,
            pl_portfolio: 0 as f32,
        };
    }
}

// Probably make builder pattern
#[derive(Debug)]
pub struct TradeOptions {
    pub side: TradeSide,
    pub capital_allocation: Option<f32>,
    pub leverage: Option<f32>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Metric {
    //Computational
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
    TotalDollarReturn,
    TotalRatioReturn,
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
    MostConsecutiveWins,
    MostConsecutiveLosses,
    ProfitFactor,
    RecoveryFactor,
    RiskRewardRatio,
    WinRate,
}

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

    pub fn current_portfolio_value(&self) -> f32 {
        let mut total_value = self.available_capital;

        if let Some(current_price) = self.current_price {
            for trade in &self.trades {
                if !trade.is_closed() {
                    total_value += trade.pl_unrealized_fixed(Some(current_price));
                }
            }
        }

        total_value
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
            trade.freeze_portfolio_value_at_open(self.current_portfolio_value());
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

            let fixed_pl = existing_trade.pl_fixed();
            let trade_capital_allocation = existing_trade.capital_allocation().unwrap();

            let capital_returned = trade_capital_allocation + fixed_pl;
            self.adjust_available_capital(capital_returned);
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

    pub fn backtest_ended(&mut self) -> BacktestResult {
        for trade in &mut self.trades.clone().iter_mut() {
            if !trade.is_closed() {
                self.close_trade(trade);
            }
        }

        self.backtest_ended = true;
        self.computational_metrics.insert(
            Metric::PerformanceTime,
            self.instant.elapsed().as_secs_f32(),
        );
        return BacktestResult::from(self.to_owned());
    }

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

    pub fn trades(&self) -> &Vec<Trade> {
        return &self.trades;
    }

    pub fn metrics(&self) -> &HashMap<Metric, f32> {
        return &self.metrics;
    }
}

impl BacktestResult {
    pub fn from(backtest_manager: BacktestManager) -> Self {
        let mut metrics: HashMap<Metric, f32> = HashMap::new();

        // Maybe compute these on backtest_manager side as to have an O(1) metrics compute solution
        let mut sharpe = SharpeRatio::new(Some(0.0));
        let mut standard_deviation = StandardDeviation::new();
        let mut apr = APR::new();
        let mut consecutive_wins_losses = ConsecutiveWinsLosses::new();

        let mut valid_trades: Vec<Trade> = vec![];

        for trade in backtest_manager.trades() {
            if !trade.is_closed() {
                continue;
            }

            valid_trades.push(trade.to_owned());

            let pl_portfolio = trade.pl_portfolio();
            let pl_ratio = trade.pl_ratio();

            apr.allocate(pl_portfolio / 100.0);
            sharpe.allocate(pl_portfolio);
            standard_deviation.allocate(pl_portfolio);
            consecutive_wins_losses.allocate(pl_ratio);
        }

        let sharpe = sharpe.get_data().unwrap_or(0.0);
        let standard_deviation = standard_deviation.get_data().unwrap_or(0.0);
        let apr = apr.get_data().unwrap_or(0.0);
        let consecutive_wins_losses = consecutive_wins_losses.get_data().unwrap_or((0,0));

        let performance_time = backtest_manager
            .computational_metrics
            .get(&Metric::PerformanceTime)
            .unwrap()
            .to_owned();
        let total_dollar_returns = backtest_manager.available_capital();
        let total_ratio_returns =
            (total_dollar_returns / backtest_manager.initial_capital()) * 100.0;

        // Maybe expand upon metrics with MetricType
        metrics.insert(Metric::StandardDeviation, standard_deviation);
        metrics.insert(Metric::SharpeRatio, sharpe);
        metrics.insert(Metric::PerformanceTime, performance_time);
        metrics.insert(Metric::TotalDollarReturn, total_dollar_returns);
        metrics.insert(Metric::TotalRatioReturn, total_ratio_returns);
        metrics.insert(Metric::APR, apr);
        metrics.insert(
            Metric::MostConsecutiveWins,
            consecutive_wins_losses.0 as f32,
        );
        metrics.insert(
            Metric::MostConsecutiveLosses,
            consecutive_wins_losses.1 as f32,
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

pub static STRATEGIES: LazyLock<Vec<Box<dyn IStrategy>>> = LazyLock::new(|| {
    vec![
        Box::new(sma_200_strategy::Sma200Strategy::new()),
        Box::new(sma_optimizable_period_strategy::SmaOptimizablePeriodStrategy::new()),
        Box::new(double_sma_optimize_strategy::DoubleSmaOptimizablePeriodStrategy::new()),
    ]
});
