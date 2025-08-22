use crate::{
    user::{
        composer::{CompositionDataType, IComposition, SMA200Composition},
        library::{
            technical_indicators::{sma, SMA, TR},
            trade::{Trade, TradeOptions, TradeSide},
            IInjectable, Injectable,
        },
        strategies::{BacktestManager, IStrategy, StrategyData},
    },
    utils::classes::charting::ChartingData,
};
use std::{collections::HashMap, error::Error, vec};
use uuid::Uuid;
use std::rc::Rc;

pub struct SMA200Strategy {
    id: String,
    name: String,
    description: String,
    state_index: usize,
}

impl SMA200Strategy {
    fn strategy(&self) -> Option<Vec<Trade>> {
        return None;
    }
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

    fn backtest(&self) -> Result<BacktestManager, Box<dyn Error>> {
        let mut backtest_manager = BacktestManager::new(super::BacktestOptions {
            initial_capital: 1_000,
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

            let side = match close > sma_200 {
                true => TradeSide::LONG,
                false => TradeSide::SHORT,
            };

            if let Some(mut latest_trade) = backtest_manager.get_last_trade() {
                if latest_trade.side() != side {
                    backtest_manager.close_trade(&mut latest_trade);
                }
            }

            let capital_allocation = backtest_manager.available_capital().to_owned();

            let mut trade = Trade::new(TradeOptions {
                side,
                capital_allocation: Some(capital_allocation),
                leverage: None,
            });

            backtest_manager.open_trade(&mut trade);
        }

        backtest_manager.backtest_ended();
        Ok(backtest_manager)
    }

    fn composition(&self) -> &'static dyn IComposition {
        return SMA200Composition::instance();
    }

    fn render(&self) -> Vec<ChartingData> {
        return vec![];
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
            state_index: 0,
        };
    }
}
