use std::fs::File;
use std::thread;

use crate::user::composer::CompositionDataType;
use crate::user::strategies::{Metric, STRATEGIES};
use crate::utils::classes::charting::{ChartingData, DataBlock};
use crate::utils::paths::get_app_data_dir;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Serialize, Deserialize, Clone)]
pub struct MetricPair {
    key: Metric,
    value: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OptimizationParameterPair {
    key: String,
    value: CompositionDataType,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BacktestResultResponse {
    pub id: String,
    pub equity_growth_charting_data: Vec<ChartingData>,
    pub portfolio_growth_data: Vec<ChartingData>,
    pub percentage_growth_data: Vec<ChartingData>,
    pub data_blocks: Vec<DataBlock>,
    pub metrics: Vec<MetricPair>,
    pub parameters: Vec<OptimizationParameterPair>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BacktestStrategyResponse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub backtests: Vec<BacktestResultResponse>,
}

#[tauri::command]
pub fn backtest_strategy(
    params: BacktestStrategyParams,
) -> Result<BacktestStrategyResponse, tauri::Error> {
    let mut data_response = BacktestStrategyResponse {
        name: None,
        description: None,
        backtests: Vec::new(),
    };

    let strategy = (&*STRATEGIES)
        .into_iter()
        .find(|strategy| strategy.id() == params.id)
        .unwrap();

    data_response.name = Some(strategy.name().into());
    data_response.description = Some(strategy.description().into());

    let optimization = strategy.optimize();

    if optimization.is_none() {
        let backtest_result = strategy.backtest(None).unwrap();

        let portfolio_growth_data = strategy.render_portfolio_percentage_growth(&backtest_result);
        let percentage_growth_data = strategy.render_percentage_growth(&backtest_result);
        let equity_growth_charting_data = strategy.render_equity_growth(&backtest_result);
        let mut metrics = Vec::new();

        for (key, value) in backtest_result.metrics() {
            metrics.push(MetricPair {
                key: key.clone(),
                value: value.to_owned(),
            });
        }

        data_response.backtests.push(BacktestResultResponse {
            id: Uuid::new_v4().to_string(),
            equity_growth_charting_data: equity_growth_charting_data,
            portfolio_growth_data: portfolio_growth_data,
            percentage_growth_data: percentage_growth_data,
            data_blocks: vec![],
            metrics: metrics,
            parameters: Vec::new(),
        })
    } else {
        for optimized_backtest_result in optimization.unwrap() {
            let portfolio_growth_data = strategy
                .render_portfolio_percentage_growth(&optimized_backtest_result.backtest_result);
            let percentage_growth_data =
                strategy.render_percentage_growth(&optimized_backtest_result.backtest_result);
            let equity_growth_charting_data =
                strategy.render_equity_growth(&optimized_backtest_result.backtest_result);
            let mut metrics = Vec::new();

            for (key, value) in optimized_backtest_result.backtest_result.metrics() {
                metrics.push(MetricPair {
                    key: key.clone(),
                    value: value.to_owned(),
                });
            }

            let mut optimized_parameters_pairs: Vec<OptimizationParameterPair> = vec![];

            for (key, value) in &optimized_backtest_result.optimized_parameters {
                optimized_parameters_pairs.push(OptimizationParameterPair {
                    key: key.to_owned(),
                    value: value.to_owned(),
                })
            }

            data_response.backtests.push(BacktestResultResponse {
                id: Uuid::new_v4().to_string(),
                equity_growth_charting_data: equity_growth_charting_data,
                portfolio_growth_data: portfolio_growth_data,
                percentage_growth_data: percentage_growth_data,
                data_blocks: vec![],
                metrics: metrics,
                parameters: optimized_parameters_pairs,
            })
        }
    }

    let data_response_clone = data_response.clone();

    thread::spawn(move || {
        let app_data_dir = get_app_data_dir().unwrap();
        let base_download_path = app_data_dir.join("saves");
        let save_id = format!(
            "{}_{}_{}",
            strategy.name(),
            strategy.composition().name(),
            Utc::now().to_string().replace(":", "_")
        );
        eprintln!("{:?}", save_id);
        let json_path = base_download_path.join(format!("{}.json", save_id));
        let ohlcv_data_string = serde_json::to_string(&data_response_clone).unwrap();
        std::fs::write(&json_path, ohlcv_data_string).unwrap();
    })
    .join()
    .unwrap();

    Ok(data_response)
}

// #[tauri::command]
// pub async fn optimize_strategy() -> Result<_, _> {
//     Ok(())
// }
