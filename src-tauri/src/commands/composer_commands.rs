use std::time::{Duration, Instant};

use crate::{
    user::composer::COMPOSITIONS,
    utils::classes::charting::{ChartingData, DataBlock},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CompositionMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[tauri::command]
pub fn get_compositions() -> Result<Vec<CompositionMetadata>, tauri::Error> {
    let mut composition_metadatas: Vec<CompositionMetadata> = vec![];

    for composition in &*COMPOSITIONS {
        composition_metadatas.push(CompositionMetadata {
            id: composition.id().into(),
            name: composition.name().into(),
            description: composition.description().into(),
        });
    }

    Ok(composition_metadatas)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositionData {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct CompositionDataResponse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub charting_data: Vec<ChartingData>,
    pub data_blocks: Vec<DataBlock>
}

#[tauri::command]
pub fn get_composition_data(
    data: GetCompositionData,
) -> Result<CompositionDataResponse, tauri::Error> {
    let mut data_response = CompositionDataResponse {
        name: None,
        description: None,
        charting_data: vec![],
        data_blocks: vec![]
    };

    let start = Instant::now();

    for composition in &*COMPOSITIONS {
        if composition.id() == data.id {
            data_response.name = Some(composition.name().into());
            data_response.description = Some(composition.description().into());
            let charting_data = composition.render().unwrap();
            for chart in charting_data {
                data_response.charting_data.push(chart);
            }
        }
    }

    Ok(data_response)
}
