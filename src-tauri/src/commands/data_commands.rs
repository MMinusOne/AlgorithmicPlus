use crate::{
    user::static_resources::STATIC_RESOURCES,
    utils::classes::charting::{ChartingData, DataBlock},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DownloadMetadata {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub timeframe: String,
    pub start_timestamp: i64,
    pub end_timestamp: i64,
    pub data_type: String,
}

#[tauri::command]
pub async fn get_static_resources() -> Result<Vec<DownloadMetadata>, tauri::Error> {
    let mut metadatas: Vec<DownloadMetadata> = vec![];

    for static_resource in &*STATIC_RESOURCES {
        if static_resource.data_type() == "OHLCV" {
            if let Ok(raw_data) = static_resource.load_ohlcv_metadata() {
                metadatas.push(DownloadMetadata {
                    id: static_resource.id().into(),
                    name: static_resource.name().into(),
                    symbol: raw_data.symbol,
                    timeframe: raw_data.timeframe,
                    start_timestamp: raw_data.start_timestamp,
                    end_timestamp: raw_data.end_timestamp,
                    data_type: static_resource.data_type().into(),
                });
            }
        } else {
            continue;
        }
    }

    Ok(metadatas)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetRawDataParams {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RawDataResponse {
    pub symbol: Option<String>,
    pub timeframe: Option<String>,
    pub start_timestamp: Option<i64>,
    pub end_timestamp: Option<i64>,
    pub charting_data: Vec<ChartingData>,
    pub data_blocks: Vec<DataBlock>,
}

#[tauri::command]
pub async fn get_static_resource_data(
    data: GetRawDataParams,
) -> Result<RawDataResponse, tauri::Error> {
    let mut data_response = RawDataResponse {
        symbol: None,
        timeframe: None,
        start_timestamp: None,
        end_timestamp: None,
        charting_data: vec![],
        data_blocks: vec![],
    };

    for static_resource in &*STATIC_RESOURCES {
        if static_resource.id() == data.id {
            if static_resource.data_type() == "OHLCV" {
                if let Ok(raw_metadata) = static_resource.load_ohlcv_metadata() {
                    data_response.symbol = Some(raw_metadata.symbol);
                    data_response.timeframe = Some(raw_metadata.timeframe);
                    data_response.start_timestamp = Some(raw_metadata.start_timestamp);

                    if let Ok(charting_data) = static_resource.render() {
                        for chart in charting_data {
                            data_response.charting_data.push(chart);
                        }
                    }
                }
            } else {
            }
        } else {
            // Add more cases later on
            continue;
        }
    }

    Ok(data_response)
}
