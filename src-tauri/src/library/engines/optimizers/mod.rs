use std::io::Error;

use crate::user::strategies::{BacktestResult, IStrategy};

pub trait Optimizer {
    fn optimize<T>(&self, strategy: Box<dyn IStrategy>) -> Result<Vec<Box<BacktestResult>>, Error>;
}

pub mod beysian;
pub mod genetic;
pub mod grid;
