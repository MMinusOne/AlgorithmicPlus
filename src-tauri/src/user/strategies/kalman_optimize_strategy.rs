use super::{BacktestManager, BacktestResult, IStrategy, Trade, TradeOptions, TradeSide};
use crate::library::engines::optimizers::Optimizer;
use crate::user::composer::eth_standalone_4h_4y_composition::ETH_STANDALONE_4H_4Y;
use crate::user::library::kalman_filter::KalmanFilter;
use crate::{
    library::engines::optimizers::grid::{
        GridOptimizer, NumericOptimizationParameter, OptimizationParameter, OptimizedBacktestResult,
    },
    user::{
        composer::{CompositionDataType, IComposition},
        library::IInjectable,
    },
    utils::classes::charting::{ChartingData, LineChartingData, LineData},
};
use std::collections::HashMap;
use std::{error::Error, vec};
use uuid::Uuid;

#[derive(Clone)]
pub struct KalmanOptimizeableStrategy {
    id: String,
    name: String,
    description: String,
    composition_data: Option<Vec<Vec<CompositionDataType>>>,
}

impl IStrategy for KalmanOptimizeableStrategy {
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
                name: "q_noise".into(),
                start: 0.1,
                end: 2.0,
                step: 0.5,
            }),
            OptimizationParameter::Numeric(NumericOptimizationParameter {
                name: "r_noise".into(),
                start: 1.0,
                end: 50.0,
                step: 15.0,
            }),
            OptimizationParameter::Numeric(NumericOptimizationParameter {
                name: "capital_ratio".into(),
                start: 0.1,
                end: 0.9,
                step: 0.4,
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

        let q_noise_comp = optimization_map
            .unwrap()
            .get("q_noise")
            .unwrap_or(&CompositionDataType::F32(0.1))
            .to_owned();
        let r_noise_comp = optimization_map
            .unwrap()
            .get("r_noise")
            .unwrap_or(&CompositionDataType::F32(1.0))
            .to_owned();
        let capital_ratio_comp = optimization_map
            .unwrap()
            .get("capital_ratio")
            .unwrap_or(&CompositionDataType::F32(0.30))
            .to_owned();

        let q_noise = CompositionDataType::extract_f32(&q_noise_comp);
        let r_noise = CompositionDataType::extract_usize(&r_noise_comp) as f32;
        let capital_ratio = CompositionDataType::extract_f32(&capital_ratio_comp);
        let timestamp_position = composition.get_composition_field_position("timestamp");
        let close_position = composition.get_composition_field_position("close");
        let mut kalman_injectable = KalmanFilter::new(q_noise, r_noise);

        let mut latest_trade: Option<Trade> = None;
        let mut prev_kalman_value: Option<f32> = None;

        for composition_point in &composition_data {
            if backtest_manager.backtest_ended {
                continue;
            }

            let timestamp =
                CompositionDataType::extract_i64(&composition_point[timestamp_position]);
            let close = CompositionDataType::extract_f32(&composition_point[close_position]);

            backtest_manager.update_price(timestamp, close);

            kalman_injectable.allocate(close);

            let kalman_value = kalman_injectable.get_data();
            if kalman_value.is_none() {
                continue;
            }

            let kalman_value = kalman_value.unwrap();

            if prev_kalman_value.is_none() {
                prev_kalman_value = Some(kalman_value);
                continue;
            }

            let prev_kalman = prev_kalman_value.unwrap();

            let side = if kalman_value > prev_kalman {
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

            let trade_allocation = backtest_manager.available_capital() * capital_ratio;

            if latest_trade.is_none() {
                let mut new_trade = Trade::new(TradeOptions {
                    side,
                    capital_allocation: Some(trade_allocation),
                    leverage: Some(1.0),
                });
                backtest_manager.open_trade(&mut new_trade);
                latest_trade = Some(new_trade);
            }

            prev_kalman_value = Some(kalman_value);
        }

        let backtest_result = backtest_manager.backtest_end();
        Ok(backtest_result)
    }

    fn composition(&self) -> &'static dyn IComposition {
        return ETH_STANDALONE_4H_4Y::instance();
    }

    fn optimization_target(&self, backtest_result: &BacktestResult) -> f32 {
        if let Some(sharpe_ratio) = backtest_result.metrics.get(&super::Metric::SharpeRatio) {
            return sharpe_ratio.to_owned();
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

impl KalmanOptimizeableStrategy {
    pub fn new() -> Self {
        let mut strategy = Self {
            id: Uuid::new_v4().into(),
            name: "Kalman".into(),
            description: "Kalman".into(),
            composition_data: None,
        };

        strategy.composition_data = Some(strategy.composition().compose().unwrap());

        return strategy;
    }
}
