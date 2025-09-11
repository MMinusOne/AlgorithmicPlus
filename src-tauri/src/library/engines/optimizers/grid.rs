use crate::library::engines::optimizers::async_trait;
use crate::user::strategies::Metric;
use crate::{
    library::engines::optimizers::Optimizer,
    user::{
        composer::CompositionDataType,
        strategies::{BacktestResult, IStrategy},
    },
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Error, ops::Range as OpsRange};

pub enum OptimizationKind {
    NUMERIC,
    CATEGORIC,
}

#[derive(Serialize, Deserialize)]
pub struct NumericOptimizationParameter {
    pub name: String,
    pub start: usize,
    pub end: usize,
    pub step: usize,
}

#[derive(Serialize, Deserialize)]
pub struct CategoricOptimizationParameter {
    pub name: String,
    pub categories: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub enum OptimizationParameter {
    Numeric(NumericOptimizationParameter),
    Categoric(CategoricOptimizationParameter),
}

impl OptimizationParameter {
    pub fn extract_numeric(
        optimization_parameter: &OptimizationParameter,
    ) -> &NumericOptimizationParameter {
        match optimization_parameter {
            OptimizationParameter::Numeric(p) => p,
            _ => panic!("Wrong optimization parameter type"),
        }
    }

    pub fn extract_categoric(
        optimization_parameter: &OptimizationParameter,
    ) -> &CategoricOptimizationParameter {
        match optimization_parameter {
            OptimizationParameter::Categoric(p) => p,
            _ => panic!("Wrong optimization parameter type"),
        }
    }
}

pub struct GridOptimizer {}

#[derive(Debug)]
pub struct OptimizedBacktestResult {
    pub backtest_result: BacktestResult,
    pub optimized_parameters: HashMap<String, CompositionDataType>,
    pub score: f32,
}

impl Optimizer for GridOptimizer {
    fn optimize(
        strategy: &Box<dyn IStrategy>,
        hyperparameters: &[OptimizationParameter],
    ) -> Result<Vec<OptimizedBacktestResult>, Error> {
        let combinations = Self::generate_combinations(hyperparameters);

        let backtest_results: Vec<OptimizedBacktestResult> = combinations
            .into_par_iter()
            .filter_map(|combination| {
                println!("{:?}", combination);
                strategy
                    .backtest(Some(&combination))
                    .map(|backtest_result| {
                        let score = strategy.optimization_target(&backtest_result);
                        OptimizedBacktestResult {
                            backtest_result,
                            optimized_parameters: combination,
                            score,
                        }
                    })
                    .ok()
            })
            .collect();

        for b in &backtest_results {
            println!("{:?}", b.backtest_result.metrics());
        }

        Ok(backtest_results)
    }
}

impl GridOptimizer {
    fn generate_combinations(
        hyperparameters: &[OptimizationParameter],
    ) -> Vec<HashMap<String, CompositionDataType>> {
        let numeric_params: Vec<&NumericOptimizationParameter> = hyperparameters
            .iter()
            .map(|param| OptimizationParameter::extract_numeric(param))
            .collect();

        let total_combinations = numeric_params
            .iter()
            .map(|param| param.end - param.start)
            .product();

        let mut combinations = Vec::with_capacity(total_combinations);

        Self::generate_recursive(&numeric_params, 0, &mut HashMap::new(), &mut combinations);

        return combinations;
    }

    fn generate_recursive(
        numeric_params: &Vec<&NumericOptimizationParameter>,
        param_index: usize,
        current_combination: &mut HashMap<String, CompositionDataType>,
        combinations: &mut Vec<HashMap<String, CompositionDataType>>,
    ) {
        if param_index == numeric_params.len() {
            combinations.push(current_combination.clone());
            return;
        }

        let current_param = numeric_params[param_index];

        for value in (current_param.start..current_param.end).step_by(current_param.step) {
            let composition_usize = CompositionDataType::Usize(value);
            current_combination.insert(current_param.name.clone(), composition_usize);

            Self::generate_recursive(
                numeric_params,
                param_index + 1,
                current_combination,
                combinations,
            );
        }

        current_combination.remove(&current_param.name);
    }
}
