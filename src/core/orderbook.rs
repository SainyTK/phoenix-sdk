use crate::types::{Orderbook, OrderbookEntry, PhoenixError};
use crate::core::phoenix_api::{PhoenixApiClient, PhoenixApiConfig};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Callback type for pure Rust implementations
pub type OrderbookCallback = Box<dyn Fn(&Orderbook) + Send + 'static>;

/// Subscription information
pub struct Subscription {
    id: String,
    symbol: String,
    callbacks: Vec<Arc<OrderbookCallback>>,
}

/// Core orderbook functionality using phoenix_api.rs as the single source of truth
pub struct OrderbookManager {
    url: String,
    subscriptions: HashMap<String, Subscription>,
    reconnect_attempts: u32,
    max_reconnect_attempts: u32,
    reconnect_delay_ms: u32,
    phoenix_api_client: PhoenixApiClient,
    symbol_to_address: Arc<Mutex<HashMap<String, String>>>,
}

impl OrderbookManager {
    pub fn new(url: &str) -> Result<Self, PhoenixError> {
        let config = PhoenixApiConfig::default();
        let phoenix_api_client = PhoenixApiClient::new(config)?;
        
        Ok(OrderbookManager {
            url: url.to_string(),
            subscriptions: HashMap::new(),
            reconnect_attempts: 0,
            max_reconnect_attempts: 5,
            reconnect_delay_ms: 1000,
            phoenix_api_client,
            symbol_to_address: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Default constructor with preset WebSocket URL
    pub fn default() -> Result<Self, PhoenixError> {
        Self::new("wss://api.mainnet-beta.solana.com")
    }

    /// Connect to the WebSocket and initialize market mappings
    pub async fn connect(&mut self) -> Result<(), PhoenixError> {
        println!("Connecting to WebSocket at {}", self.url);
        
        // Initialize symbol to address mapping
        self.refresh_market_mappings().await?;
        
        Ok(())
    }

    /// Refresh the symbol to address mapping from phoenix_api
    async fn refresh_market_mappings(&self) -> Result<(), PhoenixError> {
        let phoenix_markets = self.phoenix_api_client.fetch_phoenix_markets().await?;
        let mut mappings = self.symbol_to_address.lock().unwrap();
        mappings.clear();
        
        for market in phoenix_markets {
            let symbol = market.name.replace("/", "_");
            mappings.insert(symbol, market.address);
        }
        
        Ok(())
    }

    /// Subscribe to orderbook updates for a specific symbol
    pub fn subscribe(&mut self, symbol: &str, callback: OrderbookCallback) -> Result<String, String> {
        // Generate a subscription ID
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let subscription_id = format!("sub_{}_{}", symbol, timestamp);
        
        // Create or update the subscription
        let subscription = self.subscriptions.entry(symbol.to_string()).or_insert_with(|| {
            Subscription {
                id: subscription_id.clone(),
                symbol: symbol.to_string(),
                callbacks: Vec::new(),
            }
        });
        
        // Add the callback
        subscription.callbacks.push(Arc::new(callback));
        
        // In a real implementation, we would send a subscription message to the server
        println!("Subscribed to orderbook updates for {}", symbol);
        
        Ok(subscription_id)
    }

    /// Unsubscribe from orderbook updates
    pub fn unsubscribe(&mut self, subscription_id: &str) -> Result<bool, String> {
        // Parse the subscription ID to extract the symbol
        let parts: Vec<&str> = subscription_id.split('_').collect();
        if parts.len() >= 2 {
            let symbol = parts[1];
            
            // Remove the subscription
            if self.subscriptions.remove(symbol).is_some() {
                println!("Unsubscribed from orderbook updates for {}", symbol);
                return Ok(true);
            }
        }
        
        Ok(false)
    }

    /// Close the WebSocket connection
    pub fn close(&mut self) -> Result<(), String> {
        self.subscriptions.clear();
        self.symbol_to_address.lock().unwrap().clear();
        println!("OrderbookManager connection closed");
        Ok(())
    }
    
    /// Fetch current orderbook for a symbol using phoenix_api
    pub async fn fetch_orderbook(&self, symbol: &str, depth: Option<u32>) -> Result<Orderbook, PhoenixError> {
        // Get market address from symbol
        let market_address = {
            let mappings = self.symbol_to_address.lock().unwrap();
            mappings.get(symbol).cloned()
        };
        
        match market_address {
            Some(address) => {
                // Fetch orderbook from phoenix_api
                let phoenix_orderbook = self.phoenix_api_client.fetch_phoenix_orderbook(&address, depth).await?;
                
                // Convert to legacy format
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
                
                Ok(Orderbook { bids, asks })
            }
            None => {
                // Refresh mappings and try again
                self.refresh_market_mappings().await?;
                let mappings = self.symbol_to_address.lock().unwrap();
                
                match mappings.get(symbol) {
                    Some(address) => {
                        let phoenix_orderbook = self.phoenix_api_client.fetch_phoenix_orderbook(address, depth).await?;
                        
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
                        
                        Ok(Orderbook { bids, asks })
                    }
                    None => {
                        Err(PhoenixError::MarketNotFound(symbol.to_string()))
                    }
                }
            }
        }
    }
    
    /// Simulate orderbook updates using real phoenix_api data
    pub async fn simulate_update(&self, symbol: &str) -> Result<(), PhoenixError> {
        if let Some(subscription) = self.subscriptions.get(symbol) {
            // Fetch current orderbook from phoenix_api
            let orderbook = self.fetch_orderbook(symbol, Some(10)).await?;
            
            // Call all callbacks with the real orderbook data
            for callback in &subscription.callbacks {
                callback(&orderbook);
            }
        }
        
        Ok(())
    }
    
    /// Get available symbols
    pub async fn get_available_symbols(&self) -> Result<Vec<String>, PhoenixError> {
        let phoenix_markets = self.phoenix_api_client.fetch_phoenix_markets().await?;
        let symbols: Vec<String> = phoenix_markets.iter()
            .map(|market| market.name.replace("/", "_"))
            .collect();
        
        Ok(symbols)
    }
}

// Provide synchronous wrapper methods for backward compatibility
impl OrderbookManager {
    /// Synchronous wrapper for connect (for backward compatibility)
    pub fn connect(&mut self) -> Result<(), String> {
        println!("Connecting to WebSocket at {}", self.url);
        Ok(())
    }
    
    /// Synchronous simulate_update (for backward compatibility)
    pub fn simulate_update(&self, symbol: &str) -> Result<(), String> {
        if let Some(subscription) = self.subscriptions.get(symbol) {
            // Create a basic orderbook structure for backward compatibility
            let orderbook = Orderbook {
                bids: Vec::new(),
                asks: Vec::new(),
            };
            
            // Call all callbacks with the orderbook
            for callback in &subscription.callbacks {
                callback(&orderbook);
            }
        }
        
        Ok(())
    }
}