use std::io::Error;

use crate::{
    library::engines::optimizers::Optimizer,
    user::strategies::{BacktestResult, IStrategy},
};

pub struct Beysian {}

impl Optimizer for Beysian {
    fn optimize<T>(&self, strategy: Box<dyn IStrategy>) -> Result<Vec<Box<BacktestResult>>, Error> {
        let backtest_results: Vec<Box<BacktestResult>> = vec![];

        Ok(backtest_results)
    }
}
