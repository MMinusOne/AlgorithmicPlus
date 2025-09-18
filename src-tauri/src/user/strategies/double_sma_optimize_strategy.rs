use super::{BacktestManager, BacktestResult, IStrategy, Trade, TradeOptions, TradeSide};
use crate::library::engines::optimizers::Optimizer;
use crate::{
    library::engines::optimizers::grid::{
        GridOptimizer, NumericOptimizationParameter, OptimizationParameter, OptimizedBacktestResult,
    },
    user::{
        composer::{
            eth_standalone_4h_4y_composition::ETH_STANDALONE_4H_4Y, CompositionDataType,
            IComposition,
        },
        library::{sma::SMA, IInjectable},
    },
    utils::classes::charting::{ChartingData, LineChartingData, LineData},
};
use std::collections::HashMap;
use std::{error::Error, vec};
use uuid::Uuid;

#[derive(Clone)]
pub struct DoubleSmaOptimizablePeriodStrategy {
    id: String,
    name: String,
    description: String,
    composition_data: Option<Vec<Vec<CompositionDataType>>>,
}

impl IStrategy for DoubleSmaOptimizablePeriodStrategy {
    fn id(&self) -> &str {
        return &self.id;
    }

    fn name(&self) -> &str {
        return &self.name;
    }

    fn description(&self) -> &str {
        return &self.description;
    }

    fn optimize(&self) -> Option<Vec<OptimizedBacktestResult>> {
        let optimization_parameters = [
            OptimizationParameter::Numeric(NumericOptimizationParameter {
                name: "sma_short_period".into(),
                start: 10.0,
                end: 100.0,
                step: 15.0,
            }),
            OptimizationParameter::Numeric(NumericOptimizationParameter {
                name: "sma_long_period".into(),
                start: 100.0,
                end: 200.0,
                step: 15.0,
            }),
        ];

        let strategy: Box<dyn IStrategy> = Box::new(self.clone());

        let optimization_results =
            GridOptimizer::optimize(&strategy, &optimization_parameters).unwrap_or(Vec::new());

        Some(optimization_results)
    }

    fn backtest(
        &self,
        optimization_map: Option<&HashMap<String, CompositionDataType>>,
    ) -> Result<BacktestResult, Box<dyn Error>> {
        let mut backtest_manager = BacktestManager::new(super::BacktestOptions {
            initial_capital: 1_000.0,
            fees: 0.001
        });

        let composition: &'static dyn IComposition = self.composition();
        let composition_data = self.composed_data();

        if optimization_map.is_none() {
            let backtest_result = backtest_manager.backtest_end();
            return Ok(backtest_result);
        }

        let sma_short_comp = optimization_map
            .unwrap()
            .get("sma_short_period")
            .unwrap()
            .to_owned();
        let sma_short_period = CompositionDataType::extract_usize(&sma_short_comp);

        let sma_long_comp = optimization_map
            .unwrap()
            .get("sma_long_period")
            .unwrap()
            .to_owned();
        let sma_long_period = CompositionDataType::extract_usize(&sma_long_comp);

        let timestamp_position = composition.get_composition_field_position("timestamp");
        let close_position = composition.get_composition_field_position("close");
        let mut sma_short_injectable = SMA::new(sma_short_period);
        let mut sma_long_injectable = SMA::new(sma_long_period);

        let mut latest_trade: Option<Trade> = None;

        for composition_point in &composition_data {
            if backtest_manager.backtest_ended {
                continue;
            }

            let timestamp =
                CompositionDataType::extract_i64(&composition_point[timestamp_position]);
            let close = CompositionDataType::extract_f32(&composition_point[close_position]);

            backtest_manager.update_price(timestamp, close);

            sma_short_injectable.allocate(close);
            sma_long_injectable.allocate(close);

            let sma_short = sma_short_injectable.get_data();
            let sma_long = sma_long_injectable.get_data();

            if sma_short.is_none() || sma_long.is_none() {
                continue;
            }

            let sma_short_value = sma_short.unwrap();
            let sma_long_value = sma_long.unwrap();

            let side = if sma_short_value > sma_long_value {
                TradeSide::LONG
            } else {
                TradeSide::SHORT
            };

            if let Some(ref mut trade) = latest_trade {
                if !trade.is_closed() && trade.side() != side {
                    backtest_manager.close_trade(trade);
                    latest_trade = None;
                }
            }

            if latest_trade.is_none() {
                let trade_allocation = backtest_manager.available_capital() * 0.30;

                let mut new_trade = Trade::new(TradeOptions {
                    side,
                    capital_allocation: Some(trade_allocation),
                    leverage: Some(1.0),
                });
                backtest_manager.open_trade(&mut new_trade);
                latest_trade = Some(new_trade);
            }
        }

        let backtest_result = backtest_manager.backtest_end();
        Ok(backtest_result)
    }

    fn composition(&self) -> &'static dyn IComposition {
        return ETH_STANDALONE_4H_4Y::instance();
    }

    fn optimization_target(&self, backtest_result: &BacktestResult) -> f32 {
        if let (Some(sharpe_ratio), Some(ratio_return)) = (
            backtest_result.metrics.get(&super::Metric::SharpeRatio),
            backtest_result
                .metrics
                .get(&super::Metric::TotalRatioReturn),
        ) {
            return (sharpe_ratio * 0.8) + (ratio_return * 0.2);
        } else {
            return -1.0;
        }
    }

    fn composed_data(&self) -> Vec<Vec<CompositionDataType>> {
        if self.composition_data.is_some() {
            return self.composition_data.as_ref().unwrap().to_vec();
        }

        return self.composition().compose().unwrap();
    }

    fn render_equity_growth(&self, backtest_result: &BacktestResult) -> Vec<ChartingData> {
        let mut charting_data: Vec<ChartingData> = Vec::new();

        let mut line_data: Vec<Option<LineData>> = vec![];
        let mut cumulative_returns: f32 = 0.0;

        for trade in backtest_result.trades() {
            cumulative_returns += trade.pl_fixed();

            line_data.push(Some(LineData {
                time: trade.close_timestamp().unwrap(),
                value: cumulative_returns,
                color: None,
            }));
        }

        charting_data.push(ChartingData::LineChartingData(LineChartingData {
            chart_type: "line".into(),
            height: None,
            data: line_data,
            pane: None,
            title: None,
        }));

        return charting_data;
    }

    fn render_percentage_growth(&self, backtest_result: &BacktestResult) -> Vec<ChartingData> {
        let mut charting_data: Vec<ChartingData> = Vec::new();

        let mut line_data: Vec<Option<LineData>> = vec![];
        let mut cumulative_returns: f32 = 0.0;

        for trade in backtest_result.trades() {
            cumulative_returns += trade.pl_ratio();

            line_data.push(Some(LineData {
                time: trade.close_timestamp().unwrap(),
                value: cumulative_returns,
                color: None,
            }));
        }

        charting_data.push(ChartingData::LineChartingData(LineChartingData {
            chart_type: "line".into(),
            height: None,
            data: line_data,
            pane: None,
            title: None,
        }));

        return charting_data;
    }

    fn render_portfolio_percentage_growth(
        &self,
        backtest_result: &BacktestResult,
    ) -> Vec<ChartingData> {
        let mut charting_data: Vec<ChartingData> = Vec::new();

        let mut line_data: Vec<Option<LineData>> = vec![];
        let mut cumulative_returns: f32 = 0.0;

        for trade in backtest_result.trades() {
            cumulative_returns += trade.pl_portfolio();

            line_data.push(Some(LineData {
                time: trade.close_timestamp().unwrap(),
                value: cumulative_returns,
                color: None,
            }));
        }

        charting_data.push(ChartingData::LineChartingData(LineChartingData {
            chart_type: "line".into(),
            height: None,
            data: line_data,
            pane: None,
            title: None,
        }));

        return charting_data;
    }

    fn save(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl DoubleSmaOptimizablePeriodStrategy {
    pub fn new() -> Self {
        let mut strategy = Self {
            id: Uuid::new_v4().into(),
            name: "Double SMA optimizable period price crossover".into(),
            description: "Long when sma(short_period) > sma(long_period) and vice-versa where short_period and long_period are optimizable".into(),
            composition_data: None,
        };

        strategy.composition_data = Some(strategy.composition().compose().unwrap());

        return strategy;
    }
}
