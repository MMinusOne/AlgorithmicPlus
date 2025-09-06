use std::cmp::Ordering;
use std::collections::HashMap;

use crate::library::engines::optimizers::grid::{
    GridOptimizer, NumericOptimizationParameter, OptimizationParameter,
};
use crate::library::engines::optimizers::Optimizer;
use crate::user::strategies::{IStrategy, Metric, STRATEGIES};
use crate::utils::classes::charting::{ChartingData, DataBlock};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StrategyMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[tauri::command]
pub async fn get_strategies() -> Result<Vec<StrategyMetadata>, String> {
    let mut strategies_metadatas: Vec<StrategyMetadata> = vec![];

    for strategy_metadata in STRATEGIES.iter() {
        strategies_metadatas.push(StrategyMetadata {
            id: strategy_metadata.id().into(),
            name: strategy_metadata.name().into(),
            description: strategy_metadata.description().into(),
        });
    }

    Ok(strategies_metadatas)
}

#[derive(Serialize, Deserialize)]
pub struct BacktestStrategyParams {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct MetricPair {
    key: Metric,
    value: f32,
}

#[derive(Serialize, Deserialize)]
pub struct BacktestStrategyResponse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub equity_growth_charting_data: Vec<ChartingData>,
    pub portfolio_growth_data: Vec<ChartingData>,
    pub percentage_growth_data: Vec<ChartingData>,
    pub data_blocks: Vec<DataBlock>,
    pub metrics: Vec<MetricPair>,
}

#[tauri::command]
pub fn backtest_strategy(
    params: BacktestStrategyParams,
) -> Result<BacktestStrategyResponse, tauri::Error> {
    let mut data_response = BacktestStrategyResponse {
        name: None,
        description: None,
        equity_growth_charting_data: Vec::new(),
        portfolio_growth_data: Vec::new(),
        percentage_growth_data: Vec::new(),
        data_blocks: Vec::new(),
        metrics: Vec::new(),
    };

    let strategy = (&*STRATEGIES)
        .into_iter()
        .find(|strategy| strategy.id() == params.id)
        .unwrap();

    data_response.name = Some(strategy.name().into());
    data_response.description = Some(strategy.description().into());

    let parameters = [
        OptimizationParameter::Numeric(NumericOptimizationParameter {
            name: "sma_short_period".into(),
            range: 10..100,
        }),
        OptimizationParameter::Numeric(NumericOptimizationParameter {
            name: "sma_long_period".into(),
            range: 100..200,
        }),
    ];

    // let backtest_result = GridOptimizer::optimize(strategy, &parameters);
    println!("Executing strategy {:?}", strategy.name());

    let mut backtest_results = strategy.backtest(None).unwrap();
    // let mut backtest_results = GridOptimizer::optimize(strategy, &parameters).unwrap();

    for trade in backtest_results.trades() {
        println!(
            "
        ============
        Capital: {:?}
        PL Portfolio {:?}
        PL Ratio {:?}
        Entry Price {:?}
        Exit Price {:?}
        ============
        ",
            trade.capital_allocation(),
            trade.pl_portfolio(),
            trade.pl_ratio(),
            trade.open_price(),
            trade.close_price()
        );
    }

    // backtest_results.sort_by(|a, b| {
    //     let backtest_results_a = a.backtest_result.metrics();
    //     let sharpe_a = backtest_results_a.get(&Metric::SharpeRatio).unwrap();
    //     let backtest_results_b = b.backtest_result.metrics();
    //     let sharpe_b = backtest_results_b.get(&Metric::SharpeRatio).unwrap();

    //     if sharpe_a < sharpe_b {
    //         Ordering::Less
    //     } else if sharpe_a > sharpe_b {
    //         Ordering::Greater
    //     } else {
    //         Ordering::Equal
    //     }
    // });

    // for backtest in backtest_results {
    //     println!(
    //         "
    //     ================================
    //     Parameters: {:?}
    //     Metrics: {:?}
    //     ================================
    //     ",
    //         backtest.optimized_parameters,
    //         backtest.backtest_result.metrics()
    //     );
    // }

    // for (metric_key, metric_value) in backtest_result.metrics() {
    //     data_response.metrics.push(MetricPair {
    //         key: metric_key,
    //         value: metric_value,
    //     });
    // }

    //TODO: make a render(...) function so there isnt a need to loop thrice
    // data_response.equity_growth_charting_data = strategy.render_equity_growth(&backtest_result);
    // data_response.percentage_growth_data = strategy.render_percentage_growth(&backtest_result);
    // data_response.portfolio_growth_data =
    //     strategy.render_portfolio_percentage_growth(&backtest_result);

    // println!("Metrics {:?}", backtest_result.metrics());
    Ok(data_response)
}

// #[tauri::command]
// pub async fn optimize_strategy() -> Result<_, _> {
//     Ok(())
// }
