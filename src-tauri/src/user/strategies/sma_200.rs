use crate::{
    user::{
        composer::{CompositionDataType, IComposition, SMA200Composition},
        library::{
            trade::{Trade, TradeOptions, TradeSide},
            IInjectable,
        },
        strategies::{BacktestManager, BacktestResult, IStrategy},
    },
    utils::classes::charting::{ChartingData, LineChartingData, LineData},
};
use std::{error::Error, vec};
use uuid::Uuid;

pub struct SMA200Strategy {
    id: String,
    name: String,
    description: String,
}

impl IStrategy for SMA200Strategy {
    fn id(&self) -> &str {
        return &self.id;
    }

    fn name(&self) -> &str {
        return &self.name;
    }

    fn description(&self) -> &str {
        return &self.description;
    }

    fn backtest(&self) -> Result<BacktestResult, Box<dyn Error>> {
        let mut backtest_manager = BacktestManager::new(super::BacktestOptions {
            initial_capital: 1_000.0,
        });

        let composition: &'static dyn IComposition = self.composition();
        let composition_data = composition.compose()?;

        let timestamp_position = composition.get_composition_field_position("timestamp");
        let close_position = composition.get_composition_field_position("close");
        let sma_200_position = composition.get_composition_field_position("sma_200");

        for composition_point in &composition_data {
            let timestamp = CompositionDataType::extract_int(composition_point[timestamp_position]);
            let close = CompositionDataType::extract_float(composition_point[close_position]);
            let sma_200 =
                CompositionDataType::extract_option_float(composition_point[sma_200_position]);

            // maybe let the backtest manager handle that
            backtest_manager.update_price(timestamp, close);

            if sma_200.is_none() {
                continue;
            }

            let sma_200 = sma_200.unwrap();

            let latest_trade = backtest_manager.get_last_trade();

            let side = match close > sma_200 {
                true => TradeSide::LONG,
                false => TradeSide::SHORT,
            };

            let mut new_trade = Trade::new(TradeOptions {
                side: side,
                capital_allocation: Some(backtest_manager.initial_capital()),
                leverage: Some(1.0),
            });

            if !latest_trade.is_none() {
                let mut latest_trade = latest_trade.unwrap();

                if !latest_trade.is_closed() && latest_trade.side() != side {
                    backtest_manager.close_trade(&mut latest_trade);
                    backtest_manager.open_trade(&mut new_trade);
                    continue;
                }
            }

            backtest_manager.open_trade(&mut new_trade);
        }

        let backtest_result = backtest_manager.backtest_ended();
        Ok(backtest_result)
    }

    fn composition(&self) -> &'static dyn IComposition {
        return SMA200Composition::instance();
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
            title: Some("Portfolio equity growth backtest".into()),
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
            title: Some("Portfolio percentage growth backtest".into()),
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
            title: Some("Portfolio percentage growth backtest".into()),
        }));

        return charting_data;
    }

    fn save(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl SMA200Strategy {
    pub fn new() -> Self {
        return Self {
            id: Uuid::new_v4().into(),
            name: "SMA 200 price crossover".into(),
            description: "Long when price > sma200 and short when price < sma200".into(),
        };
    }
}
