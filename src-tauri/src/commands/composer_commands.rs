use crate::user::composer::COMPOSITIONS;
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

    for story in &*COMPOSITIONS {
        composition_metadatas.push(CompositionMetadata {
            id: story.id().into(),
            name: story.name().into(),
            description: story.description().into(),
        });
    }

    Ok(composition_metadatas)
}

#[tauri::command]
pub fn get_composition_by_id() {}
