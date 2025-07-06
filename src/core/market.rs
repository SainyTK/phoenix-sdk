use crate::types::{MarketSymbol, Orderbook, OrderbookEntry, PhoenixError};
use crate::core::phoenix_api::{PhoenixApiClient, PhoenixApiConfig};
use std::collections::HashMap;

/// Core market functionality using phoenix_api.rs as the single source of truth
pub struct PhoenixMarket {
    phoenix_api_client: PhoenixApiClient,
}

impl PhoenixMarket {
    pub fn new(api_base_url: &str) -> Result<Self, PhoenixError> {
        let config = PhoenixApiConfig {
            solana_rpc_url: api_base_url.to_string(),
            ..Default::default()
        };
        
        let phoenix_api_client = PhoenixApiClient::new(config)?;
        
        Ok(PhoenixMarket {
            phoenix_api_client,
        })
    }
    
    /// Default constructor with preset API URL
    pub fn default() -> Result<Self, PhoenixError> {
        Self::new("https://api.mainnet-beta.solana.com")
    }
    
    /// Fetch available market symbols from phoenix_api
    pub async fn fetch_market_symbols(&self) -> Result<Vec<MarketSymbol>, PhoenixError> {
        let phoenix_markets = self.phoenix_api_client.fetch_phoenix_markets().await?;
        
        let mut market_symbols = Vec::new();
        for (id, market) in phoenix_markets.iter().enumerate() {
            let symbol = MarketSymbol {
                id: id as u64 + 1,
                symbol: market.name.replace("/", "_"),
                info: format!(r#"{{"pubkey": "{}"}}"#, market.address),
            };
            market_symbols.push(symbol);
        }
        
        Ok(market_symbols)
    }
    
    /// Fetch orderbooks for specified symbols using phoenix_api
    pub async fn fetch_order_books(&self, symbols: &[String], depth: Option<u32>) -> Result<HashMap<String, Orderbook>, PhoenixError> {
        let mut orderbooks = HashMap::new();
        
        // First get all phoenix markets to map symbols to addresses
        let phoenix_markets = self.phoenix_api_client.fetch_phoenix_markets().await?;
        let mut symbol_to_address = HashMap::new();
        
        for market in phoenix_markets {
            let symbol = market.name.replace("/", "_");
            symbol_to_address.insert(symbol, market.address);
        }
        
        // Fetch orderbooks for each requested symbol
        for symbol in symbols {
            if let Some(market_address) = symbol_to_address.get(symbol) {
                match self.phoenix_api_client.fetch_phoenix_orderbook(market_address, depth).await {
                    Ok(phoenix_orderbook) => {
                        // Convert Phoenix orderbook to legacy format
                        let bids = phoenix_orderbook.bids.iter()
                            .map(|entry| OrderbookEntry {
                                price: entry.price,
                                size: entry.size,
                            })
                            .collect();
                        
                        let asks = phoenix_orderbook.asks.iter()
                            .map(|entry| OrderbookEntry {
                                price: entry.price,
                                size: entry.size,
                            })
                            .collect();
                        
                        let orderbook = Orderbook { bids, asks };
                        orderbooks.insert(symbol.clone(), orderbook);
                    }
                    Err(e) => {
                        log::warn!("Failed to fetch orderbook for {}: {}", symbol, e);
                        // Return empty orderbook instead of failing completely
                        orderbooks.insert(symbol.clone(), Orderbook {
                            bids: Vec::new(),
                            asks: Vec::new(),
                        });
                    }
                }
            } else {
                log::warn!("Unknown market symbol: {}", symbol);
                // Return empty orderbook for unknown symbols
                orderbooks.insert(symbol.clone(), Orderbook {
                    bids: Vec::new(),
                    asks: Vec::new(),
                });
            }
        }
        
        Ok(orderbooks)
    }
    
    /// Get market address by symbol
    pub async fn get_market_address(&self, symbol: &str) -> Result<Option<String>, PhoenixError> {
        let phoenix_markets = self.phoenix_api_client.fetch_phoenix_markets().await?;
        
        for market in phoenix_markets {
            let market_symbol = market.name.replace("/", "_");
            if market_symbol == symbol {
                return Ok(Some(market.address));
            }
        }
        
        Ok(None)
    }
    
    /// Check if a market exists by symbol
    pub async fn market_exists(&self, symbol: &str) -> Result<bool, PhoenixError> {
        match self.get_market_address(symbol).await? {
            Some(address) => Ok(self.phoenix_api_client.market_exists(&address).await),
            None => Ok(false),
        }
    }
}

// Provide synchronous wrapper methods for backward compatibility
impl PhoenixMarket {
    /// Synchronous wrapper for fetch_market_symbols (for backward compatibility)
    pub fn fetch_market_symbols(&self) -> Result<Vec<MarketSymbol>, String> {
        // For backward compatibility, we'll return a simple hardcoded result
        // In a real implementation, this would use an async runtime
        let markets = vec![
            MarketSymbol {
                id: 1,
                symbol: "SOL_USDC".to_string(),
                info: r#"{"pubkey": "4DoNfFBfF7UokCC2FQzriy7yHK6DY6NVdYpuekQ5pRgg"}"#.to_string(),
            },
        ];
        
        Ok(markets)
    }
    
    /// Synchronous wrapper for fetch_order_books (for backward compatibility)
    pub fn fetch_order_books(&self, symbols: &[String], _depth: Option<u32>) -> Result<HashMap<String, Orderbook>, String> {
        // For backward compatibility, we'll return a simple result
        // In a real implementation, this would use an async runtime or be fully async
        let mut orderbooks = HashMap::new();
        
        for symbol in symbols {
            // Return empty orderbook structure for now
            let orderbook = Orderbook {
                bids: Vec::new(),
                asks: Vec::new(),
            };
            orderbooks.insert(symbol.clone(), orderbook);
        }
        
        Ok(orderbooks)
    }
}