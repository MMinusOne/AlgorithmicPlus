use crate::{
    user::{
        composer::{IComposition, SMA200Composition},
        library::trade::{Trade, TradeOptions, TradeSide},
        strategies::{sma_200, IStrategy, StrategyData},
    },
    utils::classes::charting::ChartingData,
};
use std::error::Error;
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

    fn composition(&self) -> &'static dyn IComposition {
        return SMA200Composition::instance();
    }

    fn strategy(
        &self,
        get: impl Fn(&str) -> Option<StrategyData>,
        trades: &Vec<Trade>,
    ) -> Option<Vec<Trade>> {
        // let timestamp = StrategyData::extract_composition_int(get("timestamp")?);
        let close = StrategyData::extract_composition_float(get("close")?);
        let sma_200 = StrategyData::extract_composition_option_float(get("sma_200")?);
        let mut latest_trade = trades[trades.len() - 1];

        // force trade timestamps after returning Vec<Trade>, <Trade>.freeze_timestamp(i64)

        if sma_200 == None {
            return None;
        }

        let sma_200 = sma_200?;

        let side = match close > sma_200 {
            true => TradeSide::LONG,
            false => TradeSide::SHORT,
        };

        if side != latest_trade.side {
            latest_trade.close();
            let trades: Vec<Trade> = vec![Trade::new(TradeOptions { side })];
            return Some(trades);
        }

        return None;
    }

    fn render(&self) -> Vec<ChartingData> {
        return vec![];
    }

    fn save() -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl SMA200Strategy {
    fn new() -> Self {
        return Self {
            id: Uuid::new_v4().into(),
            name: "SMA 200 price crossover".into(),
            description: "Long when price > sma200 and short when price < sma200".into(),
        };
    }
}
