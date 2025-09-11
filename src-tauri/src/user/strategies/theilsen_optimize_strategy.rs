use super::{BacktestManager, BacktestResult, IStrategy, Trade, TradeOptions, TradeSide};
use crate::library::engines::optimizers::Optimizer;
use crate::{
    library::engines::optimizers::grid::{
        GridOptimizer, NumericOptimizationParameter, OptimizationParameter, OptimizedBacktestResult,
    },
    user::{
        composer::{
            eth_hlc_standalone_4h_4y::ETH_HLC_STANDALONE_4H_4Y, CompositionDataType, IComposition,
        },
        library::{sma::SMA, theilsen::TheilSen, IInjectable},
    },
    utils::classes::charting::{ChartingData, LineChartingData, LineData},
};
use std::collections::HashMap;
use std::{error::Error, vec};
use uuid::Uuid;

#[derive(Clone)]
pub struct TheilSenOptimizeableStrategy {
    id: String,
    name: String,
    description: String,
    composition_data: Option<Vec<Box<[CompositionDataType]>>>,
}

impl IStrategy for TheilSenOptimizeableStrategy {
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
        let optimization_parameters = [OptimizationParameter::Numeric(
            NumericOptimizationParameter {
                name: "theilsen_window_length".into(),
                start: 10,
                end: 200,
                step: 5,
            },
        )];

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

        let theilsen_window_length_comp = optimization_map
            .unwrap()
            .get("theilsen_window_length")
            .unwrap()
            .to_owned();

        let theilsen_window_length =
            CompositionDataType::extract_usize(theilsen_window_length_comp);

        let timestamp_position = composition.get_composition_field_position("timestamp");
        let high_position = composition.get_composition_field_position("high");
        let low_position = composition.get_composition_field_position("low");
        let close_position = composition.get_composition_field_position("close");
        let mut theilsen_injectable = TheilSen::new(Some(theilsen_window_length), None, None);

        let mut latest_trade: Option<Trade> = None;
        let mut prev_theilsen_value: Option<f32> = None;

        for composition_point in &composition_data {
            if backtest_manager.backtest_ended {
                continue;
            }

            let timestamp =
                CompositionDataType::extract_int(&composition_point[timestamp_position]);
            let high = CompositionDataType::extract_float(&composition_point[high_position]);
            let low = CompositionDataType::extract_float(&composition_point[low_position]);
            let close = CompositionDataType::extract_float(&composition_point[close_position]);

            backtest_manager.update_price(timestamp, close);

            theilsen_injectable.allocate((high, low, close));

            let theilsen_value = theilsen_injectable.get_data();

            if theilsen_value.is_none() {
                continue;
            }

            let theilsen_value = theilsen_value.unwrap();

            if prev_theilsen_value.is_none() {
                prev_theilsen_value = Some(theilsen_value);
                continue;
            }

            let prev_theilsen = prev_theilsen_value.unwrap();

            let side = if theilsen_value > prev_theilsen {
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

            let portfolio_value = backtest_manager.current_portfolio_value();

            if portfolio_value == 0.0 {
                break;
            }

            let trade_allocation = backtest_manager.available_capital() * 0.30;

            if latest_trade.is_none() {
                let mut new_trade = Trade::new(TradeOptions {
                    side,
                    capital_allocation: Some(trade_allocation),
                    leverage: Some(1.0),
                });
                backtest_manager.open_trade(&mut new_trade);
                latest_trade = Some(new_trade);
            }

            prev_theilsen_value = Some(theilsen_value);
        }

        let backtest_result = backtest_manager.backtest_end();
        Ok(backtest_result)
    }

    fn composition(&self) -> &'static dyn IComposition {
        return ETH_HLC_STANDALONE_4H_4Y::instance();
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

    fn composed_data(&self) -> Vec<Box<[CompositionDataType]>> {
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

impl TheilSenOptimizeableStrategy {
    pub fn new() -> Self {
        let mut strategy = Self {
            id: Uuid::new_v4().into(),
            name: "Theilsen".into(),
            description: "Theilsen".into(),
            composition_data: None,
        };

        strategy.composition_data = Some(strategy.composition().compose().unwrap());

        return strategy;
    }
}
