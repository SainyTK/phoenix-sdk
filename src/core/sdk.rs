use crate::types::{MarketSymbol, Orderbook, OrderbookEntry, WebSocketOrderbookUpdate};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rand::Rng;

/// Configuration for Phoenix SDK
pub struct PhoenixConfig {
    pub api_base_url: String,
    pub ws_url: String,
    pub max_reconnect_attempts: u32,
    pub reconnect_delay_ms: u32,
}

impl Default for PhoenixConfig {
    fn default() -> Self {
        PhoenixConfig {
            api_base_url: "https://api.mainnet-beta.solana.com".to_string(),
            ws_url: "wss://api.mainnet-beta.solana.com".to_string(),
            max_reconnect_attempts: 5,
            reconnect_delay_ms: 1000,
        }
    }
}

/// Callback type for orderbook updates
pub type OrderbookUpdateCallback = Box<dyn Fn(&WebSocketOrderbookUpdate) + Send + 'static>;

/// Phoenix SDK core implementation
pub struct PhoenixSDK {
    config: PhoenixConfig,
    markets: Arc<Mutex<HashMap<String, MarketSymbol>>>,
    orderbooks: Arc<Mutex<HashMap<String, Orderbook>>>,
    subscriptions: Arc<Mutex<HashMap<String, Vec<Arc<OrderbookUpdateCallback>>>>>,
    connected: Arc<Mutex<bool>>,
}

impl PhoenixSDK {
    /// Create a new Phoenix SDK instance with custom configuration
    pub fn new(config: PhoenixConfig) -> Self {
        PhoenixSDK {
            config,
            markets: Arc::new(Mutex::new(HashMap::new())),
            orderbooks: Arc::new(Mutex::new(HashMap::new())),
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
            connected: Arc::new(Mutex::new(false)),
        }
    }

    /// Create a new Phoenix SDK instance with default configuration
    pub fn default() -> Self {
        PhoenixSDK::new(PhoenixConfig::default())
    }

    /// Initialize the SDK
    pub fn init(&self) -> Result<(), String> {
        // Initialize markets cache
        self.fetch_markets()?;
        
        // Set up connection - in a full implementation, this would
        // establish the websocket connection
        *self.connected.lock().unwrap() = true;
        
        Ok(())
    }

    /// Connect to the Phoenix WebSocket server
    pub fn connect(&self) -> Result<(), String> {
        // In a real implementation, this would establish a WebSocket connection
        // to Phoenix using a library like tungstenite
        println!("Connecting to Phoenix WebSocket at {}", self.config.ws_url);
        
        *self.connected.lock().unwrap() = true;
        Ok(())
    }

    /// Fetch available markets
    pub fn fetch_markets(&self) -> Result<Vec<MarketSymbol>, String> {
        // In a real implementation, this would call the Phoenix API
        // For now, return sample data
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
            MarketSymbol {
                id: 3,
                symbol: "ETH_USDC".to_string(),
                info: r#"{"pubkey": "FrYH387rNbqV97JQxbiy3oVcYhZ6nxF9Ex1YDzBNLEmh"}"#.to_string(),
            },
        ];
        
        Ok(markets)
    }

    /// Get market information by symbol
    pub fn get_market(&self, symbol: &str) -> Option<MarketSymbol> {
        let markets = self.markets.lock().unwrap();
        markets.get(symbol).cloned()
    }

    /// Fetch orderbook for a specific market
    pub fn fetch_orderbook(&self, symbol: &str) -> Result<Orderbook, String> {
        // In a real implementation, this would call the Phoenix API
        // For now, return sample data
        let orderbook = match symbol {
            "SOL/USDC" => {
                Orderbook {
                    bids: vec![
                        OrderbookEntry { price: 19.9240, size: 214.3670 },
                        OrderbookEntry { price: 19.9090, size: 53.5920 },
                        OrderbookEntry { price: 19.8860, size: 46.8330 },
                    ],
                    asks: vec![
                        OrderbookEntry { price: 19.9710, size: 37.4840 },
                        OrderbookEntry { price: 19.9860, size: 62.3450 },
                        OrderbookEntry { price: 20.0000, size: 105.7230 },
                    ],
                }
            },
            "BTC/USDC" => {
                Orderbook {
                    bids: vec![
                        OrderbookEntry { price: 28500.00, size: 0.5210 },
                        OrderbookEntry { price: 28450.00, size: 1.2340 },
                        OrderbookEntry { price: 28400.00, size: 0.8970 },
                    ],
                    asks: vec![
                        OrderbookEntry { price: 28550.00, size: 0.7650 },
                        OrderbookEntry { price: 28600.00, size: 1.5430 },
                        OrderbookEntry { price: 28650.00, size: 0.9870 },
                    ],
                }
            },
            "ETH/USDC" => {
                Orderbook {
                    bids: vec![
                        OrderbookEntry { price: 1820.50, size: 3.2140 },
                        OrderbookEntry { price: 1820.00, size: 5.7650 },
                        OrderbookEntry { price: 1819.50, size: 2.9870 },
                    ],
                    asks: vec![
                        OrderbookEntry { price: 1821.00, size: 2.6540 },
                        OrderbookEntry { price: 1821.50, size: 4.3210 },
                        OrderbookEntry { price: 1822.00, size: 3.7890 },
                    ],
                }
            },
            _ => {
                return Err(format!("Unknown market symbol: {}", symbol));
            }
        };
        
        // Update orderbooks cache
        let mut orderbooks = self.orderbooks.lock().unwrap();
        orderbooks.insert(symbol.to_string(), orderbook.clone());
        
        Ok(orderbook)
    }

    /// Fetch multiple orderbooks
    pub fn fetch_orderbooks(&self, symbols: &[&str]) -> Result<HashMap<String, Orderbook>, String> {
        let mut result = HashMap::new();
        
        for symbol in symbols {
            match self.fetch_orderbook(symbol) {
                Ok(orderbook) => {
                    result.insert(symbol.to_string(), orderbook);
                },
                Err(e) => {
                    return Err(format!("Error fetching orderbook for {}: {}", symbol, e));
                }
            }
        }
        
        Ok(result)
    }

    /// Subscribe to orderbook updates
    pub fn subscribe_orderbook(&self, symbol: &str, callback: OrderbookUpdateCallback) -> Result<String, String> {
        // Ensure we're connected
        if !*self.connected.lock().unwrap() {
            self.connect()?;
        }
        
        // Generate a subscription ID
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let subscription_id = format!("phoenix_sub_{}_{}", symbol, timestamp);
        
        // Add the callback to subscriptions
        let mut subscriptions = self.subscriptions.lock().unwrap();
        let callbacks = subscriptions.entry(symbol.to_string())
            .or_insert_with(Vec::new);
        callbacks.push(Arc::new(callback));
        
        // In a real implementation, we would send a subscription message to the server
        println!("Subscribed to Phoenix orderbook for {}", symbol);
        
        Ok(subscription_id)
    }

    /// Unsubscribe from orderbook updates
    pub fn unsubscribe_orderbook(&self, subscription_id: &str) -> Result<bool, String> {
        // Parse the subscription ID to extract the symbol
        let parts: Vec<&str> = subscription_id.split('_').collect();
        if parts.len() >= 3 {
            let symbol = parts[2];
            
            let mut subscriptions = self.subscriptions.lock().unwrap();
            if subscriptions.remove(symbol).is_some() {
                println!("Unsubscribed from Phoenix orderbook for {}", symbol);
                return Ok(true);
            }
        }
        
        Ok(false)
    }

    /// Process an orderbook update
    pub fn process_orderbook_update(&self, update: WebSocketOrderbookUpdate) {
        // Update the orderbook cache
        let mut orderbooks = self.orderbooks.lock().unwrap();
        orderbooks.insert(update.symbol.clone(), update.orderbook.clone());
        
        // Notify subscribers
        let subscriptions = self.subscriptions.lock().unwrap();
        if let Some(callbacks) = subscriptions.get(&update.symbol) {
            for callback in callbacks {
                callback(&update);
            }
        }
    }

    /// Simulate an orderbook update (for testing)
    pub fn simulate_update(&self, symbol: &str) -> Result<(), String> {
        // Ensure the market exists
        if self.get_market(symbol).is_none() {
            return Err(format!("Unknown market: {}", symbol));
        }
        
        // Create a simulated orderbook based on the current one
        let current_orderbook = match self.orderbooks.lock().unwrap().get(symbol) {
            Some(ob) => ob.clone(),
            None => match self.fetch_orderbook(symbol) {
                Ok(ob) => ob,
                Err(e) => return Err(e),
            },
        };
        
        // Slightly modify the orderbook
        let mut bids = current_orderbook.bids.clone();
        let mut asks = current_orderbook.asks.clone();
        
        // Adjust prices slightly to simulate market movement
        for bid in &mut bids {
            bid.price += (rand::random::<f64>() - 0.5) * 0.01;
            bid.size += (rand::random::<f64>() - 0.5) * 0.1;
            if bid.size < 0.1 {
                bid.size = 0.1;
            }
        }
        
        for ask in &mut asks {
            ask.price += (rand::random::<f64>() - 0.5) * 0.01;
            ask.size += (rand::random::<f64>() - 0.5) * 0.1;
            if ask.size < 0.1 {
                ask.size = 0.1;
            }
        }
        
        // Re-sort the orderbook
        bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
        asks.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
        
        // Create the updated orderbook
        let updated_orderbook = Orderbook { bids, asks };
        
        // Create the update message
        let update = WebSocketOrderbookUpdate {
            symbol: symbol.to_string(),
            orderbook: updated_orderbook,
        };
        
        // Process the update
        self.process_orderbook_update(update);
        
        Ok(())
    }

    /// Close all connections
    pub fn close(&self) -> Result<(), String> {
        // In a real implementation, this would close the WebSocket connection
        println!("Closing Phoenix WebSocket connection");
        
        // Clear subscriptions
        self.subscriptions.lock().unwrap().clear();
        *self.connected.lock().unwrap() = false;
        
        Ok(())
    }
} 