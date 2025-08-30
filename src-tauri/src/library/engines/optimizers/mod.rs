use std::io::Error;
use async_trait::async_trait;
use crate::{
    library::engines::optimizers::grid::{OptimizationParameter, OptimizedBacktestResult},
    user::strategies::{BacktestResult, IStrategy},
};

#[async_trait]
pub trait Optimizer {
     async fn optimize(
        strategy: &Box<dyn IStrategy>,
        hyperparameters:&[OptimizationParameter],
    ) -> Result<Vec<Box<OptimizedBacktestResult>>, Error>;
}

pub mod beysian;
pub mod genetic;
pub mod grid;
