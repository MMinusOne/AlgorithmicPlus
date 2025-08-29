use crate::{
    library::engines::optimizers::Optimizer,
    user::{
        composer::CompositionDataType,
        strategies::{BacktestResult, IStrategy},
    },
};
use std::{
    collections::{btree_map::Range, HashMap},
    io::Error,
    ops::Range as OpsRange,
};

pub enum OptimizationKind {
    NUMERIC,
    CATEGORIC,
}

pub struct NumericOptimizationParameter {
    pub name: String,
    pub range: OpsRange<i64>,
}

pub struct CategoricOptimizationParameter {
    pub name: String,
    pub categories: Vec<String>,
}

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

pub struct OptimizedBacktestResult {
    backtest_result: BacktestResult,
    optimized_parameters: HashMap<&'static str, i64>,
    score: i16,
}

impl Optimizer for GridOptimizer {
    fn optimize(
        strategy: &Box<dyn IStrategy>,
        hyperparameters: Vec<OptimizationParameter>,
    ) -> Result<Vec<Box<OptimizedBacktestResult>>, Error> {
        let backtest_results: Vec<Box<OptimizedBacktestResult>> = vec![];

        let combinations = Self::generate_combinations(&hyperparameters);

        println!("{:?}", combinations);
        Ok(backtest_results)
    }
}

impl GridOptimizer {
    fn generate_combinations(
        hyperparameters: &Vec<OptimizationParameter>,
    ) -> Vec<HashMap<String, CompositionDataType>> {
        let mut combinations: Vec<HashMap<String, CompositionDataType>> = vec![];

        // Maybe reserve here

        for (parameter_index, parameter) in hyperparameters.iter().enumerate() {
            let numeric_parameter = OptimizationParameter::extract_numeric(parameter);

            for n in numeric_parameter.range.to_owned() {
                for other_hypermeter_index in (0..hyperparameters.len()).skip(parameter_index) {
                    let other_hyperparameter = &hyperparameters[other_hypermeter_index];
                    let other_numeric_parameter =
                        OptimizationParameter::extract_numeric(other_hyperparameter);

                    for other_n in other_numeric_parameter.range.to_owned() {
                        let mut combination: HashMap<String, CompositionDataType> = HashMap::new();

                        combination
                            .insert(numeric_parameter.name.clone(), CompositionDataType::Int(n));
                        combination.insert(
                            other_numeric_parameter.name.clone(),
                            CompositionDataType::Int(other_n),
                        );

                        combinations.push(combination);
                    }
                }
            }
        }

        println!("{:?}", combinations.len());

        return combinations;
    }
}
