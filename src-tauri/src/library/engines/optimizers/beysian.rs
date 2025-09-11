use crate::{
    library::engines::optimizers::{
        grid::{OptimizationParameter, OptimizedBacktestResult},
        Optimizer,
    },
    user::strategies::{IStrategy},
};
use std::io::Error;

pub struct BeysianGridOptimizer {}

impl Optimizer for BeysianGridOptimizer {
    fn optimize(
        strategy: &Box<dyn IStrategy>,
        hyperparameters: &[OptimizationParameter],
    ) -> Result<Vec<OptimizedBacktestResult>, Error> {
        let backtest_results: Vec<OptimizedBacktestResult> = vec![];

        Ok(backtest_results)
    }
}
