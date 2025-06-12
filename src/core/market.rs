use crate::types::{MarketSymbol, Orderbook};
use std::collections::HashMap;

/// Core market functionality without any WASM dependencies
pub struct PhoenixMarket {
    api_base_url: String,
}

impl PhoenixMarket {
    pub fn new(api_base_url: &str) -> Self {
        PhoenixMarket {
            api_base_url: api_base_url.to_string(),
        }
    }
    
    /// Default constructor with preset API URL
    pub fn default() -> Self {
        PhoenixMarket::new("https://api.mainnet-beta.solana.com")
    }
    
    /// Fetch available market symbols
    pub fn fetch_market_symbols(&self) -> Result<Vec<MarketSymbol>, String> {
        // In a real implementation, this would call the Phoenix API to get markets
        // For now, we'll return some sample data based on the Phoenix API documentation
        
        let markets = vec![
            MarketSymbol {
                id: 1,
                symbol: "SOL_USDC".to_string(),
                info: r#"{"pubkey": "4DoNfFBfF7UokCC2FQzriy7yHK6DY6NVdYpuekQ5pRgg"}"#.to_string(),
            },
            MarketSymbol {
                id: 2,
                symbol: "BTC_USDC".to_string(),
                info: r#"{"pubkey": "8JnSiuvQq3BVXgFQ89t6GBwnR4T4eqwcW1KEvWDFUMdd"}"#.to_string(),
            },
        ];
        
        Ok(markets)
    }
    
    /// Fetch orderbooks for specified symbols
    pub fn fetch_order_books(&self, symbols: &[String], limit: Option<u32>) -> Result<HashMap<String, Orderbook>, String> {
        let mut orderbooks = std::collections::HashMap::new();
        
        // Add sample data for requested symbols
        for symbol in symbols {
            let orderbook = match symbol.as_str() {
                "SOL_USDC" => {
                    // Example orderbook data for SOL/USDC
                    Orderbook {
                        bids: vec![
                            crate::types::OrderbookEntry { price: 19.9240, size: 214.3670 },
                            crate::types::OrderbookEntry { price: 19.9090, size: 53.5920 },
                            crate::types::OrderbookEntry { price: 19.8860, size: 46.8330 },
                        ],
                        asks: vec![
                            crate::types::OrderbookEntry { price: 19.9710, size: 37.4840 },
                            crate::types::OrderbookEntry { price: 19.9860, size: 62.3450 },
                            crate::types::OrderbookEntry { price: 20.0000, size: 105.7230 },
                        ],
                    }
                },
                "BTC_USDC" => {
                    // Example orderbook data for BTC/USDC
                    Orderbook {
                        bids: vec![
                            crate::types::OrderbookEntry { price: 28500.00, size: 0.5210 },
                            crate::types::OrderbookEntry { price: 28450.00, size: 1.2340 },
                            crate::types::OrderbookEntry { price: 28400.00, size: 0.8970 },
                        ],
                        asks: vec![
                            crate::types::OrderbookEntry { price: 28550.00, size: 0.7650 },
                            crate::types::OrderbookEntry { price: 28600.00, size: 1.5430 },
                            crate::types::OrderbookEntry { price: 28650.00, size: 0.9870 },
                        ],
                    }
                },
                _ => {
                    // Empty orderbook for unknown symbols
                    Orderbook {
                        bids: Vec::new(),
                        asks: Vec::new(),
                    }
                }
            };
            
            // Apply limit if specified
            let orderbook = if let Some(limit_val) = limit {
                let limit_val = limit_val as usize;
                Orderbook {
                    bids: orderbook.bids.into_iter().take(limit_val).collect(),
                    asks: orderbook.asks.into_iter().take(limit_val).collect(),
                }
            } else {
                orderbook
            };
            
            orderbooks.insert(symbol.clone(), orderbook);
        }
        
        Ok(orderbooks)
    }
}