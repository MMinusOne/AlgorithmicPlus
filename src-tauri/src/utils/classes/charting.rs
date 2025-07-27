use serde::{Deserialize, Serialize};


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
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub time: i64,
    pub border_color: Option<String>,
    pub color: Option<String>,
    pub wick_color: Option<String>,
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
pub struct DataBlock {}

// Maybe in the future move everything to generics
