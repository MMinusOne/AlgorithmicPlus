use super::{BacktestManager, BacktestResult, IStrategy, Trade, TradeOptions, TradeSide};
use crate::{
    library::engines::optimizers::{
        grid::{
            GridOptimizer, NumericOptimizationParameter, OptimizationParameter,
            OptimizedBacktestResult,
        },
        Optimizer,
    },
    user::{
        composer::{
            eth_standalone_4h_4y_composition::ETH_STANDALONE_4H_4Y, CompositionDataType,
            IComposition,
        },
        library::{renko::Renko, sma::SMA, IInjectable},
    },
    utils::classes::charting::{ChartingData, LineChartingData, LineData},
};
use chrono::format::Numeric;
use std::collections::HashMap;
use std::marker::Copy;
use std::{error::Error, vec};
use uuid::Uuid;

#[derive(Clone)]
pub struct SmaRenkoOptimizablePeriodStrategy {
    id: String,
    name: String,
    description: String,
    composition_data: Option<Vec<Vec<CompositionDataType>>>,
}

impl IStrategy for SmaRenkoOptimizablePeriodStrategy {
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
                name: "sma_period".into(),
                start: 10.0,
                end: 200.0,
                step: 30.0,
            }),
            OptimizationParameter::Numeric(NumericOptimizationParameter {
                name: "renko_change".into(),
                start: 10.0,
                end: 400.0,
                step: 30.0,
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
        });

        let composition: &'static dyn IComposition = self.composition();
        let composition_data = self.composed_data();

        if optimization_map.is_none() {
            let backtest_result = backtest_manager.backtest_end();
            return Ok(backtest_result);
        }

        let sma_comp = optimization_map
            .unwrap()
            .get("sma_period")
            .unwrap()
            .to_owned();
        let sma_period = CompositionDataType::extract_usize(&sma_comp);
        let renko_change_comp = optimization_map
            .unwrap()
            .get("renko_change")
            .unwrap()
            .to_owned();
        let renko_change = CompositionDataType::extract_usize(&renko_change_comp);

        let timestamp_position = composition.get_composition_field_position("timestamp");
        let close_position = composition.get_composition_field_position("close");
        let mut sma_injectable = SMA::new(sma_period);
        let mut renko_injectable = Renko::new(renko_change as f32);

        let mut latest_trade: Option<Trade> = None;

        for composition_point in &composition_data {
            if backtest_manager.backtest_ended {
                break;
            }

            let timestamp =
                CompositionDataType::extract_i64(&composition_point[timestamp_position]);
            let close = CompositionDataType::extract_f32(&composition_point[close_position]);

            backtest_manager.update_price(timestamp, close);

            renko_injectable.allocate(close);
            sma_injectable.allocate(close);

            let sma = sma_injectable.get_data();
            let renko = renko_injectable.get_data();

            if sma.is_none() || renko.is_none() {
                continue;
            }

            let sma_value = sma.unwrap();
            let renko_value = renko.unwrap();

            let side = if renko_value > sma_value {
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
                let trade_allocation = backtest_manager.available_capital() * 0.7;
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

            if trade.close_timestamp.is_none() {
                break;
            }

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

impl SmaRenkoOptimizablePeriodStrategy {
    pub fn new() -> Self {
        let mut strategy = Self {
            id: Uuid::new_v4().into(),
            name: "SMA Renko optimizable period price crossover".into(),
            description: "Long when renko(price) > sma(period) and short when renko(price) < sma(period) where period is optimizable".into(),
            composition_data: None,
        };

        strategy.composition_data = Some(strategy.composition().compose().unwrap());

        return strategy;
    }
}
