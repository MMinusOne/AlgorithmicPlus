use crate::user::strategies::IStrategy;
use crate::user::strategies::{SMA200Strategy, STRATEGIES};
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

pub struct BacktestStrategyParams {
    pub id: String,
}

pub struct BacktestStrategyResponse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub charting_data: Vec<ChartingData>,
    pub data_blocks: Vec<DataBlock>,
}

#[tauri::command]
pub async fn backtest_strategy(
    params: BacktestStrategyParams,
) -> Result<BacktestStrategyResponse, tauri::Error> {
    let mut data_response = BacktestStrategyResponse {
        name: None,
        description: None,
        charting_data: vec![],
        data_blocks: vec![],
    };

    for strategy in &*STRATEGIES {
        if strategy.id() == params.id {
            data_response.name = Some(strategy.name().into());
            data_response.description = Some(strategy.description().into());
        }
    }

    Ok(data_response)
}

// #[tauri::command]
// pub async fn optimize_strategy() -> Result<_, _> {
//     Ok(())
// }
