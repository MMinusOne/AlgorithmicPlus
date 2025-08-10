use crate::{
    library::engines::OptimizationStrategy,
    user::{
        composer::{CompositionDataType, IComposition},
        library::{trade::Trade, IInjectable, Injectable},
    },
    utils::classes::charting::ChartingData,
};
use std::error::Error;
use std::time::Duration;
use std::{collections::HashMap, time::Instant};

pub enum StrategyData {
    CompositionDataType(CompositionDataType),
    InjectableFloatData(Option<f32>),
}

impl StrategyData {
    pub fn extract_composition_int(strategy_data: StrategyData) -> i64 {
        match strategy_data {
            StrategyData::CompositionDataType((composition_data)) => {
                CompositionDataType::extract_int(composition_data)
            }

            _ => panic!("Invalid strategy type conversion."),
        }
    }
    pub fn extract_composition_float(strategy_data: StrategyData) -> f32 {
        match strategy_data {
            StrategyData::CompositionDataType((composition_data)) => {
                CompositionDataType::extract_float(composition_data)
            }

            _ => panic!("Invalid strategy type conversion."),
        }
    }
    pub fn extract_composition_option_float(strategy_data: StrategyData) -> Option<f32> {
        match strategy_data {
            StrategyData::CompositionDataType((composition_data)) => {
                CompositionDataType::extract_option_float(composition_data)
            }

            _ => panic!("Invalid strategy type conversion."),
        }
    }
}

pub struct BacktestResult {
    trades: Vec<Trade>,
    performance_time: Duration,
    sharpe: i8,
}

pub struct OptimizationData {
    pub start: i16,
    pub end: i16,
    pub interval: i16,
}

pub enum InjectableState {
    Initiated(Injectable),
    Template(Injectable),
}

#[derive(Clone, Copy)]
pub enum DATA_BELONGING {
    COMPOSITION,
    INJECTABLE,
}

pub trait IStrategy: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn state_index(&self) -> usize;
    fn increment_state_index(&mut self);
    fn composition(&self) -> &'static dyn IComposition;
    fn injectables(&mut self) -> HashMap<&'static str, (InjectableState, &'static str)> {
        return HashMap::new();
    }
    fn optimizables(&self) -> HashMap<&'static str, (OptimizationData, InjectableState)> {
        return HashMap::new();
    }
    fn optimization_target(&self, backtest_result: BacktestResult) -> i16 {
        return backtest_result.sharpe as i16;
    }
    fn get_data(
        &self,
        data_key: &str,
    ) -> Option<StrategyData> {
        
    }
    fn strategy(&self) -> Option<Vec<Trade>>;
    fn backtest(&mut self) -> Result<BacktestResult, Box<dyn Error>> {
        let start_instant = Instant::now();

        let mut backtest_result = BacktestResult {
            trades: Vec::new(),
            performance_time: start_instant.elapsed(),
            sharpe: 0,
        };

        let composition = self.composition();
        let composed_data = composition.safe_compose()?;

        let composition_fields = composition.composition_fields();
        let mut injectables = self.injectables();

        let mut data_fields: HashMap<&'static str, (DATA_BELONGING, usize)> = HashMap::new();

        for (composition_field_key, composition_field_index) in composition_fields.iter() {
            data_fields.insert(
                composition_field_key,
                (DATA_BELONGING::COMPOSITION, *composition_field_index),
            );
        }

        for (injectable_index, injectable) in injectables.iter().enumerate() {
            data_fields.insert(injectable.0, (DATA_BELONGING::INJECTABLE, injectable_index));
        }

        for point in composed_data.iter() {
            // let trades = self.strategy(get_fn, &backtest_result.trades);

            for (injectable_name, (injectable_state, injectable_index)) in injectables.iter() {}
        }

        backtest_result.performance_time = start_instant.elapsed();
        Ok(backtest_result)
    }
    fn wfo(&self, optimizer: OptimizationStrategy) {}
    fn optimized_backtest(&self, optimizer: OptimizationStrategy) {}
    fn render(&self) -> Vec<ChartingData>;
    fn save() -> Result<(), Box<dyn Error>>;
}

pub mod sma_200;
pub use sma_200::SMA200Strategy;
