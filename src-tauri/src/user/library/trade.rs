use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Trade {
    pub open_price: f32,
    pub close_price: f32,
    pub pl: f32,
}