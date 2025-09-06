use serde::{Deserialize, Serialize};
use std::marker::Copy;
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum TradeSide {
    LONG,
    SHORT,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Trade {
    id: Uuid,
    open_timestamp: Option<i64>,
    close_timestamp: Option<i64>,
    capital_allocation: Option<f32>,
    open_price: Option<f32>,
    close_price: Option<f32>,
    leverage: f32,
    side: TradeSide,
    is_closed: bool,
    pl_ratio: f32,
    pl_fixed: f32,
    pl_portfolio: f32,
}

impl Trade {
    pub fn id(&self) -> Uuid {
        return self.id;
    }

    pub fn freeze_open_timestamp(&mut self, timestamp: i64) {
        if self.open_timestamp.is_none() {
            self.open_timestamp = Some(timestamp)
        };
    }

    pub fn freeze_open_price(&mut self, open_price: f32) {
        if self.open_price.is_none() {
            self.open_price = Some(open_price);
        };
    }

    pub fn open_timestamp(&self) -> Option<i64> {
        return self.open_timestamp;
    }

    pub fn close_timestamp(&self) -> Option<i64> {
        return self.close_timestamp;
    }

    pub fn close(&mut self, close_price: f32, close_timestamp: i64) {
        if !self.is_closed {
            self.close_price = Some(close_price);
            self.close_timestamp = Some(close_timestamp);
            // Only calculate P&L if we have all required data
            if self.open_price.is_some() && self.capital_allocation.is_some() {
                let open_price = self.open_price.unwrap();
                let leverage = self.leverage;
                let trade_allocation = self.capital_allocation().unwrap() as f32;

                let unleveraged_pl = match self.side {
                    TradeSide::LONG => (close_price - open_price) / open_price,
                    TradeSide::SHORT => (open_price - close_price) / open_price,
                };

                self.pl_ratio = unleveraged_pl * leverage * 100.0;
                self.pl_fixed = unleveraged_pl * leverage * trade_allocation;
                self.pl_portfolio = (self.pl_fixed / trade_allocation) * 100.0;
            }
            self.is_closed = true;
        }
    }

    pub fn pl_portfolio(&self) -> f32 {
        if !self.open_price.is_none()
            && !self.close_price.is_none()
            && !self.capital_allocation.is_none()
        {
            let fixed_pl = self.pl_fixed();
            let trade_allocation = self.capital_allocation().unwrap() as f32;

            let portfolio_pl = (fixed_pl / trade_allocation) * 100.0;
            return portfolio_pl;
        } else {
            return self.pl_portfolio;
        }
    }

    pub fn pl_fixed(&self) -> f32 {
        if self.open_price.is_some()
            && self.close_price.is_some()
            && self.capital_allocation.is_some()
        {
            let open_price = self.open_price.unwrap();
            let close_price = self.close_price.unwrap();
            let leverage = self.leverage;
            let trade_allocation = self.capital_allocation().unwrap() as f32;

            let unleveraged_pl = match self.side {
                TradeSide::LONG => (close_price - open_price) / open_price,
                TradeSide::SHORT => (open_price - close_price) / open_price,
            };

            let pl_fixed = unleveraged_pl * leverage * trade_allocation;
            return pl_fixed;
        } else {
            return self.pl_fixed;
        }
    }

    pub fn pl_ratio(&self) -> f32 {
        if !self.open_price.is_none() && !self.close_price.is_none() {
            let open_price = self.open_price.unwrap();
            let close_price = self.close_price.unwrap();
            let leverage = self.leverage;

            let unleveraged_pl = match self.side {
                TradeSide::LONG => (close_price - open_price) / open_price,
                TradeSide::SHORT => (open_price - close_price) / open_price,
            };

            let pl_ratio = unleveraged_pl * leverage * 100.0;
            return pl_ratio;
        } else {
            return self.pl_ratio;
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

    pub fn capital_allocation(&self) -> Option<f32> {
        return self.capital_allocation;
    }
}

impl Trade {
    pub fn new(trade_options: TradeOptions) -> Self {
        return Self {
            id: Uuid::new_v4(),
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
            pl_ratio: 0 as f32,
            pl_fixed: 0 as f32,
            pl_portfolio: 0 as f32,
        };
    }
}

// Probably make builder pattern
#[derive(Debug)]
pub struct TradeOptions {
    pub side: TradeSide,
    pub capital_allocation: Option<f32>,
    pub leverage: Option<f32>,
}
