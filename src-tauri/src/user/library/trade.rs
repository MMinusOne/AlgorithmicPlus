use serde::{Deserialize, Serialize};
use std::marker::Copy;

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum TradeSide {
    LONG,
    SHORT,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Trade {
    open_timestamp: Option<i64>,
    close_timestamp: Option<i64>,
    capital_allocation: Option<u16>,
    open_price: Option<f32>,
    close_price: Option<f32>,
    side: TradeSide,
    is_closed: bool,
    pl: f32,
}

impl Trade {
    pub fn freeze_open_timestamp(&mut self, timestamp: i64) {
        if self.open_timestamp == None {
            self.open_timestamp = Some(timestamp)
        };
    }

    pub fn freeze_open_price(&mut self, open_price: f32) {
        if self.open_price != None {
            self.open_price = Some(open_price);
        };
    }

    pub fn freeze_close_timestamp(&mut self, timestamp: i64) {
        if self.close_timestamp == None {
            self.close_timestamp = Some(timestamp)
        };
    }

    pub fn freeze_close_price(&mut self, close_price: f32) {
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

    pub fn is_closed(&self) -> bool {
        return self.is_closed;
    }

    pub fn side(&self) -> TradeSide {
        return self.side;
    }

    pub fn open_price(&self) -> Option<f32> {
        return self.open_price;
    }

    pub fn close_price(&self) -> Option<f32> {
        return self.close_price;
    }

    pub fn capital_allocation(&self) -> Option<u16> {
        return self.capital_allocation;
    }
}

impl Trade {
    pub fn new(trade_options: TradeOptions) -> Self {
        return Self {
            open_timestamp: None,
            close_timestamp: None,
            open_price: None,
            close_price: None,
            side: trade_options.side,
            is_closed: false,
            capital_allocation: trade_options.capital_allocation,
            pl: 0 as f32,
        };
    }
}

// Probably make builder pattern
pub struct TradeOptions {
    pub side: TradeSide,
    pub capital_allocation: Option<u16>,
}