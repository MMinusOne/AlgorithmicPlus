use crate::{
    library::engines::optimizers::grid::{OptimizationParameter, OptimizedBacktestResult},
    user::strategies::{BacktestResult, IStrategy},
};
use async_trait::async_trait;
use std::io::Error;

pub trait Optimizer {
    fn optimize(
        strategy: &Box<dyn IStrategy>,
        hyperparameters: &[OptimizationParameter],
    ) -> Result<Vec<OptimizedBacktestResult>, Error>;
}

pub mod beysian;
pub mod genetic;
pub mod grid;
