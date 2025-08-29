use std::io::Error;

use crate::{
    library::engines::optimizers::{grid::{OptimizationParameter, OptimizedBacktestResult}, Optimizer},
    user::strategies::{BacktestResult, IStrategy},
};

pub struct BeysianGridOptimizer {}

impl Optimizer for BeysianGridOptimizer {
     fn optimize(
         strategy: &Box<dyn IStrategy>,
        hyperparameters: Vec<OptimizationParameter>,
    ) -> Result<Vec<Box<OptimizedBacktestResult>>, Error> {
        let backtest_results: Vec<Box<OptimizedBacktestResult>> = vec![];

        Ok(backtest_results)
    }
}
