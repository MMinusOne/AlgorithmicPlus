use crate::{
    library::engines::optimizers::Optimizer,
    user::strategies::{BacktestResult, IStrategy},
};
use std::io::Error;

pub enum OptimizationKind {
    NUMERIC,
    CATEGORIC,
}

struct NumericOptimizationParameter {}

struct CategoricOptimizationParameter {}

pub enum OptimizationParameter {
    NumericOpimizationParameter(NumericOptimizationParameter),
    CategoricOpimizationParameter(CategoricOptimizationParameter),
}

pub struct Grid {}

impl Optimizer for Grid {
    fn optimize<T>(&self, strategy: Box<dyn IStrategy>) -> Result<Vec<Box<BacktestResult>>, Error> {
        let backtest_results: Vec<Box<BacktestResult>> = vec![];

        Ok(backtest_results)
    }
}

impl Grid {
    fn new(optimization_parameters: Vec<OptimizationParameter>) -> Self {
        return Self {};
    }
}
