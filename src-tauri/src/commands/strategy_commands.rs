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

#[derive(Serialize, Deserialize)]
pub struct BacktestStrategyParams {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct BacktestStrategyResponse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub equity_growth_charting_data: Vec<ChartingData>,
    pub portfolio_growth_data: Vec<ChartingData>,
    pub percentage_ratio_data: Vec<ChartingData>,
    pub data_blocks: Vec<DataBlock>,
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
        percentage_ratio_data: Vec::new(),
        data_blocks: vec![],
    };

    let strategy = (&*STRATEGIES)
        .into_iter()
        .find(|strategy| strategy.id() == params.id)
        .unwrap();

    data_response.name = Some(strategy.name().into());
    data_response.description = Some(strategy.description().into());

    let backtest_result = strategy.backtest().unwrap();

    data_response.equity_growth_charting_data = strategy.render_equity_growth(&backtest_result);
    data_response.equity_growth_charting_data = strategy.render_percentage_growth(&backtest_result);
    data_response.portfolio_growth_data = strategy.render_portfolio_percentage_growth(&backtest_result);

    Ok(data_response)
}

// #[tauri::command]
// pub async fn optimize_strategy() -> Result<_, _> {
//     Ok(())
// }
