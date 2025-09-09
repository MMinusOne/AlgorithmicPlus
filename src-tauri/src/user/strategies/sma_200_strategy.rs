use super::{BacktestManager, BacktestResult, IStrategy, Trade, TradeOptions, TradeSide};
use crate::{
    library::engines::optimizers::grid::OptimizedBacktestResult,
    user::composer::{
        eth_sma_200_4h_4y_composition::ETH_SMA_200_4H_4Y, CompositionDataType, IComposition,
    },
    utils::classes::charting::{ChartingData, LineChartingData, LineData},
};
use std::collections::HashMap;
use std::{error::Error, vec};
use uuid::Uuid;

pub struct Sma200Strategy {
    id: String,
    name: String,
    description: String,
    composition_data: Option<Vec<Box<[CompositionDataType]>>>,
}

impl IStrategy for Sma200Strategy {
    fn id(&self) -> &str {
        return &self.id;
    }

    fn name(&self) -> &str {
        return &self.name;
    }

    fn description(&self) -> &str {
        return &self.description;
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

        let timestamp_position = composition.get_composition_field_position("timestamp");
        let close_position = composition.get_composition_field_position("close");
        let sma_position = composition.get_composition_field_position("sma_200");

        let mut latest_trade: Option<Trade> = None;

        for composition_point in &composition_data {
            let timestamp =
                CompositionDataType::extract_int(&composition_point[timestamp_position]);
            let close = CompositionDataType::extract_float(&composition_point[close_position]);
            let sma = CompositionDataType::extract_option_float(&composition_point[sma_position]);
            backtest_manager.update_price(timestamp, close);

            if sma.is_none() {
                continue;
            }

            let sma_value = sma.unwrap();
            let side = if close > sma_value {
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
                let available_capital = backtest_manager.available_capital();
                let mut new_trade = Trade::new(TradeOptions {
                    side,
                    capital_allocation: Some(available_capital),
                    leverage: Some(1.0),
                });
                backtest_manager.open_trade(&mut new_trade);
                latest_trade = Some(new_trade);
            }
        }

        let backtest_result = backtest_manager.backtest_ended();
        Ok(backtest_result)
    }

    fn optimize(&self) -> Option<Vec<OptimizedBacktestResult>> {
        None
    }

    fn composition(&self) -> &'static dyn IComposition {
        return ETH_SMA_200_4H_4Y::instance();
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

impl Sma200Strategy {
    pub fn new() -> Self {
        let mut strategy = Self {
            id: Uuid::new_v4().into(),
            name: "SMA 200 price crossover".into(),
            description: "Long when price > sma200 and short when price < sma200".into(),
            composition_data: None,
        };

        strategy.composition_data = Some(strategy.composition().compose().unwrap());

        return strategy;
    }
}
