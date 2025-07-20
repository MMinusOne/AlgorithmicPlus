use crate::{library::providers::downloader::OHLCVJSONFileDataStructure, APP_HANDLE};
use serde::{Deserialize, Serialize};
use tauri::Manager;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize)]
pub struct DownloadMetadata {
    pub symbol: String,
    pub timeframe: String,
    pub start_date: i64,
    pub end_date: i64,
    pub data_type: String,
    pub id: String,
}

#[tauri::command]
pub async fn get_downloaded_metadatas() -> Vec<DownloadMetadata> {
    let mut metadatas: Vec<DownloadMetadata> = vec![];

    let app_data_dir = APP_HANDLE.get().expect("Couldn't get app handle");
    let data_path = app_data_dir
        .path()
        .app_data_dir()
        .expect("Couldn't get app_data_dir");
    let raw_data_path = data_path.join("raw");

    let file_paths: Vec<String> = WalkDir::new(raw_data_path)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok().expect("Couldn't get entry");
            let is_file = entry.file_type().is_file();
            let file_path = entry.path().display().to_string();

            if is_file && file_path.ends_with(".json") {
                return Some(file_path);
            } else {
                return None;
            }
        })
        .collect();

    for file_path in file_paths {
        let dir_parts: Vec<&str> = file_path.rsplitn(3, "\\").collect();
        let file = dir_parts[0];
        let data_type = dir_parts[1];
        let file_parts: Vec<&str> = file.rsplitn(2, ".").collect();
        let file_id = file_parts[1];
        let file_data_string = std::fs::read_to_string(&file_path).expect("Couldn't read file");
        let file_data: OHLCVJSONFileDataStructure =
            serde_json::from_str(&file_data_string).expect("Couldn't serialize file");
        metadatas.push(DownloadMetadata {
            symbol: file_data.symbol,
            timeframe: file_data.timeframe,
            start_date: file_data.start_date,
            end_date: file_data.end_date,
            data_type: data_type.to_string(),
            id: file_id.to_string(),
        });
    }

    return metadatas;
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetRawDataParams {
    pub id: String,
    pub item_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct RawDataResponse {
    pub symbol: Option<String>,
    pub timeframe: Option<String>,
    pub start_timestamp: Option<i64>,
    pub end_timestamp: Option<i64>, 
    pub news_data: Vec<NewsData>,
    pub charting_data: Vec<ChartingData>,
}

#[tauri::command]
pub async fn get_raw_data(data: GetRawDataParams) -> RawDataResponse {
    println!("{:?}", data);
    let mut raw_data = RawDataResponse {
        symbol: None,
        timeframe: None, 
        start_timestamp: None, 
        end_timestamp: None,
        charting_data: vec![],
        news_data: vec![],
    };

    let app_data_dir = APP_HANDLE.get().expect("Couldn't get app handle");
    let data_path = app_data_dir
        .path()
        .app_data_dir()
        .expect("Couldn't get app_data_dir");

    if data.item_type == "raw_data" {
        let raw_data_path = data_path.join("raw");

        for path_result in WalkDir::new(raw_data_path) {
            let path_entry = path_result.ok().expect("Couldn't get path");

            if !path_entry.file_type().is_file() {
                continue;
            }

            let path_display = path_entry.path().display().to_string();
            let file_path_parts: Vec<&str> = path_display.rsplitn(2, "\\").collect();
            let file_path = file_path_parts[0];
            let file_name_parts: Vec<&str> = file_path.rsplitn(2, ".").collect();
            let file_id = file_name_parts[1];
            let file_extension = file_name_parts[0];

            if file_id == data.id && file_extension == "json" {
                let file_data_string =
                    std::fs::read_to_string(path_display).expect("Couldn't read file");
                let file_data_json: OHLCVJSONFileDataStructure =
                    serde_json::from_str(&file_data_string).expect("Couldn't serialize JSON");
                let mut candlestick_data: Vec<CandlestickData> = vec![];
                let mut volume_data: Vec<HistogramData> = vec![];

                raw_data.symbol = Some(file_data_json.symbol);
                raw_data.timeframe = Some(file_data_json.timeframe);
                raw_data.start_timestamp = Some(file_data_json.timestamps[0]);
                raw_data.end_timestamp = Some(file_data_json.timestamps[file_data_json.timestamps.len() - 1]);

                for index in 0..file_data_json.timestamps.len() {
                    let timestamp = file_data_json.timestamps[index];
                    let open = file_data_json.opens[index];
                    let high = file_data_json.highs[index];
                    let low = file_data_json.lows[index];
                    let close = file_data_json.closes[index];
                    let volume = file_data_json.volumes[index];

                    candlestick_data.push(CandlestickData {
                        close: close,
                        high: high,
                        low: low,
                        open: open,
                        time: timestamp,
                        border_color: None,
                        color: None,
                        wick_color: None,
                    });

                    volume_data.push(HistogramData {
                        value: volume,
                        time: timestamp,
                        color: None,
                    });
                }

                raw_data
                    .charting_data
                    .push(ChartingData::CandlestickChartingData(
                        CandlestickChartingData {
                            chart_type: "ohlcv".into(),
                            data: candlestick_data,
                            height: Some(900),
                        },
                    ));
                raw_data
                    .charting_data
                    .push(ChartingData::HistogramChartingData(HistogramChartingData {
                        chart_type: "histogram".into(),
                        data: volume_data,
                        height: Some(150),
                    }));
                //TODO: send volume histogram
                break;
            }
        }
    }

    return raw_data;
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChartingData {
    CandlestickChartingData(CandlestickChartingData),
    LineChartingData(LineChartingData),
    BarChartingData(BarChartingData),
    HistogramChartingData(HistogramChartingData),
    AreaChartingData(AreaChartingData),
}

#[derive(Serialize, Deserialize)]
pub struct CandlestickChartingData {
    pub chart_type: String,
    pub height: Option<i16>,
    pub data: Vec<CandlestickData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CandlestickData {
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    time: i64,
    border_color: Option<String>,
    color: Option<String>,
    wick_color: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LineChartingData {}

#[derive(Serialize, Deserialize)]
pub struct BarChartingData {}

#[derive(Serialize, Deserialize)]
pub struct HistogramChartingData {
    pub chart_type: String,
    pub height: Option<i16>,
    pub data: Vec<HistogramData>,
}

#[derive(Serialize, Deserialize)]
pub struct HistogramData {
    pub time: i64,
    pub value: f32,
    pub color: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AreaChartingData {}

#[derive(Serialize, Deserialize)]
pub struct NewsData {}

// Maybe in the future move everything to generics
