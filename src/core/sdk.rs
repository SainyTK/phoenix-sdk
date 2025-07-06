use crate::types::{MarketSymbol, Orderbook, OrderbookEntry, WebSocketOrderbookUpdate, PhoenixMarket, PhoenixOrderbook, PhoenixError};
use crate::core::phoenix_api::{PhoenixApiClient, PhoenixApiConfig};
use crate::core::market::PhoenixMarket as MarketManager;
use crate::core::orderbook::OrderbookManager;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rand::Rng;

/// Configuration for Phoenix SDK
pub struct PhoenixConfig {
    pub api_base_url: String,
    pub ws_url: String,
    pub max_reconnect_attempts: u32,
    pub reconnect_delay_ms: u32,
    pub phoenix_api_config: PhoenixApiConfig,
}

impl Default for PhoenixConfig {
    fn default() -> Self {
        PhoenixConfig {
            api_base_url: "https://api.mainnet-beta.solana.com".to_string(),
            ws_url: "wss://api.mainnet-beta.solana.com".to_string(),
            max_reconnect_attempts: 5,
            reconnect_delay_ms: 1000,
            phoenix_api_config: PhoenixApiConfig::default(),
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
    phoenix_api_client: PhoenixApiClient,
    phoenix_markets: Arc<Mutex<HashMap<String, PhoenixMarket>>>,
    phoenix_orderbooks: Arc<Mutex<HashMap<String, PhoenixOrderbook>>>,
    market_manager: MarketManager,
    orderbook_manager: OrderbookManager,
}

impl PhoenixSDK {
    /// Create a new Phoenix SDK instance with custom configuration
    pub fn new(config: PhoenixConfig) -> Result<Self, PhoenixError> {
        let phoenix_api_client = PhoenixApiClient::new(config.phoenix_api_config.clone())?;
        let market_manager = MarketManager::new(&config.api_base_url)?;
        let orderbook_manager = OrderbookManager::new(&config.ws_url)?;
        
        Ok(PhoenixSDK {
            config,
            markets: Arc::new(Mutex::new(HashMap::new())),
            orderbooks: Arc::new(Mutex::new(HashMap::new())),
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
            connected: Arc::new(Mutex::new(false)),
            phoenix_api_client,
            phoenix_markets: Arc::new(Mutex::new(HashMap::new())),
            phoenix_orderbooks: Arc::new(Mutex::new(HashMap::new())),
            market_manager,
            orderbook_manager,
        })
    }

    /// Create a new Phoenix SDK instance with default configuration
    pub fn default() -> Result<Self, PhoenixError> {
        PhoenixSDK::new(PhoenixConfig::default())
    }

    /// Initialize the SDK
    pub async fn init(&mut self) -> Result<(), PhoenixError> {
        // Initialize markets cache
        self.fetch_phoenix_markets().await?;
        
        // Set up connection - in a full implementation, this would
        // establish the websocket connection
        *self.connected.lock().unwrap() = true;
        
        println!("🔥 Phoenix SDK initialized with mainnet data");
        Ok(())
    }

    /// Connect to the Phoenix WebSocket server
    pub fn connect(&mut self) -> Result<(), String> {
        // Connect using the orderbook manager
        self.orderbook_manager.connect()?;
        
        println!("Connected to Phoenix WebSocket at {}", self.config.ws_url);
        *self.connected.lock().unwrap() = true;
        Ok(())
    }

    /// Fetch available markets using the market manager
    pub fn fetch_markets(&self) -> Result<Vec<MarketSymbol>, String> {
        self.market_manager.fetch_market_symbols()
    }

    /// Get market information by symbol
    pub fn get_market(&self, symbol: &str) -> Option<MarketSymbol> {
        let markets = self.markets.lock().unwrap();
        markets.get(symbol).cloned()
    }

    /// Fetch orderbook for a specific market using the market manager
    pub fn fetch_orderbook(&self, symbol: &str) -> Result<Orderbook, String> {
        let symbols = vec![symbol.to_string()];
        let orderbooks = self.market_manager.fetch_order_books(&symbols, None)?;
        
        match orderbooks.get(symbol) {
            Some(orderbook) => {
                // Update orderbooks cache
                let mut cache = self.orderbooks.lock().unwrap();
                cache.insert(symbol.to_string(), orderbook.clone());
                Ok(orderbook.clone())
            }
            None => Err(format!("No orderbook found for symbol: {}", symbol))
        }
    }

    /// Fetch multiple orderbooks using the market manager
    pub fn fetch_orderbooks(&self, symbols: &[&str]) -> Result<HashMap<String, Orderbook>, String> {
        let symbol_strings: Vec<String> = symbols.iter().map(|s| s.to_string()).collect();
        let orderbooks = self.market_manager.fetch_order_books(&symbol_strings, None)?;
        
        // Update orderbooks cache
        let mut cache = self.orderbooks.lock().unwrap();
        for (symbol, orderbook) in &orderbooks {
            cache.insert(symbol.clone(), orderbook.clone());
        }
        
        Ok(orderbooks)
    }

    /// Subscribe to orderbook updates using the orderbook manager
    pub fn subscribe_orderbook(&mut self, symbol: &str, callback: OrderbookUpdateCallback) -> Result<String, String> {
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
        
        // Subscribe using the orderbook manager
        let orderbook_callback = Box::new(move |orderbook: &Orderbook| {
            // Convert to WebSocketOrderbookUpdate and process
            let update = WebSocketOrderbookUpdate {
                symbol: symbol.to_string(),
                orderbook: orderbook.clone(),
            };
            // Note: In a full implementation, we'd need to handle this callback properly
        });
        
        self.orderbook_manager.subscribe(symbol, orderbook_callback)?;
        
        println!("Subscribed to Phoenix orderbook for {}", symbol);
        Ok(subscription_id)
    }

    /// Unsubscribe from orderbook updates
    pub fn unsubscribe_orderbook(&mut self, subscription_id: &str) -> Result<bool, String> {
        // Parse the subscription ID to extract the symbol
        let parts: Vec<&str> = subscription_id.split('_').collect();
        if parts.len() >= 3 {
            let symbol = parts[2];
            
            let mut subscriptions = self.subscriptions.lock().unwrap();
            if subscriptions.remove(symbol).is_some() {
                self.orderbook_manager.unsubscribe(subscription_id)?;
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

    /// Simulate an orderbook update using the orderbook manager
    pub fn simulate_update(&self, symbol: &str) -> Result<(), String> {
        self.orderbook_manager.simulate_update(symbol)
    }

    /// Close all connections
    pub fn close(&mut self) -> Result<(), String> {
        // Close orderbook manager connection
        self.orderbook_manager.close()?;
        
        // Clear subscriptions
        self.subscriptions.lock().unwrap().clear();
        *self.connected.lock().unwrap() = false;
        
        println!("Phoenix SDK connections closed");
        Ok(())
    }

    // Phoenix DEX specific methods using phoenix_api_client

    /// Fetch Phoenix DEX markets
    pub async fn fetch_phoenix_markets(&self) -> Result<Vec<PhoenixMarket>, PhoenixError> {
        let markets = self.phoenix_api_client.fetch_phoenix_markets().await?;
        
        // Update the cache
        let mut phoenix_markets = self.phoenix_markets.lock().unwrap();
        phoenix_markets.clear();
        for market in &markets {
            phoenix_markets.insert(market.address.clone(), market.clone());
        }
        
        Ok(markets)
    }

    /// Get Phoenix market details
    pub async fn get_phoenix_market_details(&self, market_address: &str) -> Result<PhoenixMarket, PhoenixError> {
        self.phoenix_api_client.get_market_details(market_address).await
    }

    /// Fetch Phoenix orderbook
    pub async fn fetch_phoenix_orderbook(&self, market_address: &str, depth: Option<u32>) -> Result<PhoenixOrderbook, PhoenixError> {
        let orderbook = self.phoenix_api_client.fetch_phoenix_orderbook(market_address, depth).await?;
        
        // Update the cache
        let mut phoenix_orderbooks = self.phoenix_orderbooks.lock().unwrap();
        phoenix_orderbooks.insert(market_address.to_string(), orderbook.clone());
        
        Ok(orderbook)
    }

    /// Fetch multiple Phoenix orderbooks
    pub async fn fetch_phoenix_orderbooks(&self, market_addresses: &[&str], depth: Option<u32>) -> Result<HashMap<String, PhoenixOrderbook>, PhoenixError> {
        let orderbooks = self.phoenix_api_client.fetch_multiple_orderbooks(market_addresses, depth).await?;
        
        // Update the cache
        let mut phoenix_orderbooks = self.phoenix_orderbooks.lock().unwrap();
        for (address, orderbook) in &orderbooks {
            phoenix_orderbooks.insert(address.clone(), orderbook.clone());
        }
        
        Ok(orderbooks)
    }

    /// Get cached Phoenix orderbook
    pub fn get_cached_phoenix_orderbook(&self, market_address: &str) -> Option<PhoenixOrderbook> {
        let phoenix_orderbooks = self.phoenix_orderbooks.lock().unwrap();
        phoenix_orderbooks.get(market_address).cloned()
    }

    /// Get cached Phoenix market
    pub fn get_cached_phoenix_market(&self, market_address: &str) -> Option<PhoenixMarket> {
        let phoenix_markets = self.phoenix_markets.lock().unwrap();
        phoenix_markets.get(market_address).cloned()
    }

    /// Check if Phoenix market exists
    pub async fn phoenix_market_exists(&self, market_address: &str) -> bool {
        self.phoenix_api_client.market_exists(market_address).await
    }

    /// Convert Phoenix orderbook to legacy format
    pub fn phoenix_to_legacy_orderbook(&self, phoenix_orderbook: &PhoenixOrderbook) -> Orderbook {
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
        
        Orderbook { bids, asks }
    }

    /// Get Phoenix orderbook in legacy format
    pub async fn get_phoenix_orderbook_legacy(&self, market_address: &str, depth: Option<u32>) -> Result<Orderbook, PhoenixError> {
        let phoenix_orderbook = self.fetch_phoenix_orderbook(market_address, depth).await?;
        Ok(self.phoenix_to_legacy_orderbook(&phoenix_orderbook))
    }
} 