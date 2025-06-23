use async_trait::async_trait;
use reqwest::get;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    library::providers::downloader::{DataType, Downloadable, MarketType, Source, SourceName},
    utils::classes::logger::LOGGER,
};

pub struct Binance {
    source_name: String,
    source_url: String,
    timeframes: Vec<&'static str>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExchangeInfo { 
    symbols: Vec<SymbolCell>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SymbolCell { 
    symbol: String
}

#[async_trait]
impl Source for Binance {
    fn source_name(&self) -> &str {
        return &self.source_name;
    }

    fn source_url(&self) -> &str {
        return &self.source_url;
    }

    fn timeframes(&self) -> Vec<&'static str> {
        return self.timeframes.clone();
    }

    async fn download(&self, symbol: String, timeframe: String, limit: i128) -> Option<String> {
        return Some("not done yet".to_string());
    }

    async fn get_downloadables(&self) -> Result<Vec<Downloadable>, Box<dyn std::error::Error>> {
        let mut downloadables: Vec<Downloadable> = vec![];

        match get("https://api.binance.com/api/v3/exchangeInfo").await {
            Ok(exchange_info_response) => match exchange_info_response.text().await {
                Ok(exchange_info_string) => {
                    match serde_json::from_str::<ExchangeInfo>(&exchange_info_string) {
                        Ok(exchange_info) => {
                            for symbol in exchange_info.symbols {
                                downloadables.push(Downloadable {
                                    name: symbol.symbol.clone(),
                                    symbol: symbol.symbol,
                                    source: SourceName::Binance,
                                    market_type: MarketType::Crypto,
                                    data_type: DataType::OHLCV,
                                });
                            }
                        }
                        Err(e) => LOGGER.error(&format!(
                            "Error serializing exchange_info Binance {}",
                            e.to_string()
                        )),
                    }
                }

                Err(e) => LOGGER.error(&format!(
                    "Error parsing test Binance exchangeInfo {}",
                    e.to_string()
                )),
            },
            Err(e) => LOGGER.error(&format!(
                "Error while getting the Binance exchangeInfo {}",
                e.to_string()
            )),
        }

        return Ok(downloadables);
    }
}

impl Binance {
    pub fn new() -> Self {
        return Self {
            source_name: "Binance".to_string(),
            source_url: "https://binance.com".to_string(),
            timeframes: vec![
                "1s", "1m", "5m", "10m", "15m", "30m", "45m", "1h", "2h", "3h", "4h", "12h", "1d",
                "1W", "1M",
            ],
        };
    }
}
