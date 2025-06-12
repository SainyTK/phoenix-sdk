use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MarketSymbol {
    pub id: u64,
    pub symbol: String,
    pub info: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrderbookEntry {
    pub price: f64,
    pub size: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Orderbook {
    pub bids: Vec<OrderbookEntry>,
    pub asks: Vec<OrderbookEntry>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrderbooksResponse {
    #[serde(flatten)]
    pub orderbooks: std::collections::HashMap<String, Orderbook>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubscriptionResponse {
    pub subscription_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnsubscribeResponse {
    pub success: bool,
}

// WebSocket message types
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebSocketSubscribeMessage {
    pub op: String,
    pub symbols: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebSocketUnsubscribeMessage {
    pub op: String,
    pub subscription_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebSocketOrderbookUpdate {
    pub symbol: String,
    pub orderbook: Orderbook,
} 