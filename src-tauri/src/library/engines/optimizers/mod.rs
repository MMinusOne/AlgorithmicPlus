use std::io::Error;

use crate::{
    library::engines::optimizers::grid::{OptimizationParameter, OptimizedBacktestResult},
    user::strategies::{BacktestResult, IStrategy},
};

pub trait Optimizer {
     fn optimize(
        strategy: &Box<dyn IStrategy>,
        hyperparameters: Vec<OptimizationParameter>,
    ) -> Result<Vec<Box<OptimizedBacktestResult>>, Error>;
}

pub mod beysian;
pub mod genetic;
pub mod grid;
