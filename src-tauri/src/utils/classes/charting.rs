use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ChartingData {
    CandlestickChartingData(CandlestickChartingData),
    LineChartingData(LineChartingData),
    BarChartingData(BarChartingData),
    HistogramChartingData(HistogramChartingData),
    AreaChartingData(AreaChartingData),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CandlestickChartingData {
    pub chart_type: String,
    pub height: Option<i16>,
    pub data: Vec<CandlestickData>,
    pub pane: Option<i8>,
    pub title: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct LineChartingData {
    pub chart_type:  String,
    pub height: Option<i16>,
    pub data: Vec<Option<LineData>>,
    pub pane: Option<i8>,
    pub title: Option<String>
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct LineData {
    pub time: i64,
    pub value: f32,
    pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BarChartingData {}

#[derive(Serialize, Deserialize, Clone)]
pub struct HistogramChartingData {
    pub chart_type: String,
    pub height: Option<i16>,
    pub data: Vec<HistogramData>,
    pub pane: Option<i8>,
    pub title: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HistogramData {
    pub time: i64,
    pub value: f32,
    pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AreaChartingData {}

#[derive(Serialize, Deserialize, Clone)]
pub struct DataBlock {}

// Maybe in the future move everything to generics
