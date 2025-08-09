use serde::{Deserialize, Serialize};
use std::marker::Copy;

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum TradeSide {
    LONG,
    SHORT,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Trade {
    pub timestamp: Option<i64>,
    // pub capital: u16,
    pub open_price: Option<f32>,
    pub close_price: Option<f32>,
    pub side: TradeSide,
    pub is_closed: bool,
    pub pl: f32,
}

impl Trade {
    pub fn freeze_timestamp(&mut self, timestamp: i64) {
        if self.timestamp == None {
            self.timestamp = Some(timestamp)
        };
    }

    pub fn freeze_open(&mut self, open_price: f32) {
        if self.open_price != None {
            self.open_price = Some(open_price);
        };
    }

    pub fn freeze_close(&mut self, close_price: f32) {
        if self.close_price == None {
            self.close_price = Some(close_price)
        };
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }

    pub fn pl(&mut self) -> f32 {
        if self.open_price != None && self.close_price != None {
            let pl = self.open_price.unwrap() / self.close_price.unwrap();
            self.pl = pl;
            return pl;
        } else {
            return self.pl;
        }
    }

    pub fn new(trade_options: TradeOptions) -> Self {
        return Self {
            timestamp: None,
            open_price: None,
            close_price: None,
            side: trade_options.side,
            is_closed: false,
            // capital: trade_options.capital,
            pl: 0 as f32,
        };
    }
}

pub struct TradeOptions {
    pub side: TradeSide,
}
