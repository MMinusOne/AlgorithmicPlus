use std::io::Error;
use crate::library::engines::optimizers::async_trait;
use crate::{
    library::engines::optimizers::{
        grid::{OptimizationParameter, OptimizedBacktestResult},
        Optimizer,
    },
    user::strategies::{BacktestResult, IStrategy},
};

pub struct BeysianGridOptimizer {}

#[async_trait]
impl Optimizer for BeysianGridOptimizer {
    async fn optimize(
        strategy: &Box<dyn IStrategy>,
        hyperparameters: &[OptimizationParameter],
    ) -> Result<Vec<OptimizedBacktestResult>, Error> {
        let backtest_results: Vec<OptimizedBacktestResult> = vec![];

        Ok(backtest_results)
    }
}
