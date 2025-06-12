use crate::types::{Orderbook, OrderbookEntry};
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

/// Core orderbook functionality without any WASM dependencies
pub struct OrderbookManager {
    url: String,
    subscriptions: HashMap<String, Subscription>,
    reconnect_attempts: u32,
    max_reconnect_attempts: u32,
    reconnect_delay_ms: u32,
}

impl OrderbookManager {
    pub fn new(url: &str) -> Self {
        OrderbookManager {
            url: url.to_string(),
            subscriptions: HashMap::new(),
            reconnect_attempts: 0,
            max_reconnect_attempts: 5,
            reconnect_delay_ms: 1000,
        }
    }

    /// Default constructor with preset WebSocket URL
    pub fn default() -> Self {
        OrderbookManager::new("wss://api.mainnet-beta.solana.com")
    }

    /// Connect to the WebSocket in a Rust-native way
    pub fn connect(&mut self) -> Result<(), String> {
        // In a real implementation, this would use a Rust WebSocket client
        // like tungstenite to connect
        println!("Connecting to WebSocket at {}", self.url);
        Ok(())
    }

    /// Subscribe to orderbook updates
    pub fn subscribe(&mut self, symbol: &str, callback: OrderbookCallback) -> Result<String, String> {
        self.connect()?;
        
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
        println!("Subscribed to {}", symbol);
        
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
                println!("Unsubscribed from {}", symbol);
                return Ok(true);
            }
        }
        
        Ok(false)
    }

    /// Close the WebSocket connection
    pub fn close(&mut self) -> Result<(), String> {
        self.subscriptions.clear();
        println!("WebSocket connection closed");
        Ok(())
    }
    
    /// Simulate orderbook updates (for testing)
    pub fn simulate_update(&self, symbol: &str) -> Result<(), String> {
        if let Some(subscription) = self.subscriptions.get(symbol) {
            // Create a simulated orderbook update
            let mut bids = Vec::new();
            let mut asks = Vec::new();
            
            // Generate some random bid/ask data
            let base_bid = match symbol {
                "SOL_USDC" => 19.92,
                "BTC_USDC" => 28500.00,
                _ => 100.0,
            };
            
            let base_ask = match symbol {
                "SOL_USDC" => 19.97,
                "BTC_USDC" => 28550.00,
                _ => 101.0,
            };
            
            // Generate 3 simulated bids with pseudo-random values
            for i in 0..3 {
                let price_offset = ((i as f64 * 0.037) % 0.1) - 0.05;
                let size = (i as f64 * 1.5) % 5.0 + 1.0;
                bids.push(OrderbookEntry {
                    price: base_bid - (i as f64 * 0.01) + price_offset,
                    size,
                });
            }
            
            // Generate 3 simulated asks with pseudo-random values
            for i in 0..3 {
                let price_offset = ((i as f64 * 0.041) % 0.1) - 0.05;
                let size = (i as f64 * 1.7) % 5.0 + 1.0;
                asks.push(OrderbookEntry {
                    price: base_ask + (i as f64 * 0.01) + price_offset,
                    size,
                });
            }
            
            // Sort bids descending and asks ascending
            bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
            asks.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
            
            // Create the orderbook
            let orderbook = Orderbook { bids, asks };
            
            // Call all callbacks with the simulated orderbook
            for callback in &subscription.callbacks {
                callback(&orderbook);
            }
        }
        
        Ok(())
    }
}