use serde::{Deserialize, Serialize};
use std::marker::Copy;

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum TradeSide {
    LONG,
    SHORT,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Trade {
    open_timestamp: Option<i64>,
    close_timestamp: Option<i64>,
    capital_allocation: Option<u16>,
    open_price: Option<f32>,
    close_price: Option<f32>,
    leverage: f32,
    side: TradeSide,
    is_closed: bool,
    pl: f32,
}

impl Trade {
    pub fn freeze_open_timestamp(&mut self, timestamp: i64) {
        if self.open_timestamp.is_none() {
            self.open_timestamp = Some(timestamp)
        };
    }

    pub fn freeze_open_price(&mut self, open_price: f32) {
        if self.open_price != None {
            self.open_price = Some(open_price);
        };
    }

    pub fn freeze_close_timestamp(&mut self, timestamp: i64) {
        if self.close_timestamp.is_none() {
            self.close_timestamp = Some(timestamp)
        };
    }

    pub fn freeze_close_price(&mut self, close_price: f32) {
        if self.close_price.is_none() {
            self.close_price = Some(close_price)
        };
    }

    pub fn close(&mut self) {
        self.pl();
        self.is_closed = true;
    }

    pub fn pl(&mut self) -> f32 {
        if !self.open_price.is_none() && !self.close_price.is_none() {
            let open_price = self.open_price.unwrap();
            let close_price = self.close_price.unwrap();
            let leverage = self.leverage;

            let unleveraged_pl = match self.side {
                TradeSide::LONG => close_price / open_price,
                TradeSide::SHORT => open_price / close_price,
            };

            let pl = unleveraged_pl * leverage;
            self.pl = pl;
            return self.pl;
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

    pub fn leverage(&self) -> f32 {
        return self.leverage;
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
            capital_allocation: trade_options.capital_allocation,
            leverage: match trade_options.leverage {
                Some(l) => l,
                None => 1.0,
            },
            side: trade_options.side,
            is_closed: false,
            pl: 0 as f32,
        };
    }
}

// Probably make builder pattern
pub struct TradeOptions {
    pub side: TradeSide,
    pub capital_allocation: Option<u16>,
    pub leverage: Option<f32>,
}
