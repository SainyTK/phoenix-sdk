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

// Phoenix DEX specific types
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MarketStatus {
    Active,
    Inactive,
    Paused,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MarketFees {
    pub maker_fee_bps: u16,
    pub taker_fee_bps: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MarketSizeParams {
    pub base_lot_size: u64,
    pub quote_lot_size: u64,
    pub tick_size: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PhoenixMarket {
    pub address: String,
    pub name: String,
    pub base_mint: String,
    pub quote_mint: String,
    pub base_vault: String,
    pub quote_vault: String,
    pub authority: String,
    pub sequence_number: u64,
    pub status: MarketStatus,
    pub fees: MarketFees,
    pub size_params: MarketSizeParams,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum OrderType {
    Limit,
    Market,
    ImmediateOrCancel,
    FillOrKill,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TimeInForce {
    GoodTillCancel,
    ImmediateOrCancel,
    FillOrKill,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PhoenixOrder {
    pub order_id: u128,
    pub side: Side,
    pub price_in_ticks: u64,
    pub size_in_base_lots: u64,
    pub trader: String,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SeatStatus {
    NotApproved,
    Approved,
    Retired,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PhoenixSeat {
    pub address: String,
    pub trader: String,
    pub status: SeatStatus,
    pub base_lots_locked: u64,
    pub base_lots_free: u64,
    pub quote_lots_locked: u64,
    pub quote_lots_free: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PhoenixOrderbook {
    pub market: String,
    pub bids: Vec<PhoenixOrderbookEntry>,
    pub asks: Vec<PhoenixOrderbookEntry>,
    pub sequence_number: u64,
    pub slot: u64,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PhoenixOrderbookEntry {
    pub price: f64,
    pub size: f64,
    pub price_in_ticks: u64,
    pub size_in_base_lots: u64,
}

// Error types
#[derive(Debug, thiserror::Error)]
pub enum PhoenixError {
    #[error("Solana client error: {0}")]
    SolanaClient(#[from] solana_client::client_error::ClientError),
    
    #[error("Anchor error: {0}")]
    Anchor(#[from] anchor_client::ClientError),
    
    #[error("Market not found: {0}")]
    MarketNotFound(String),
    
    #[error("Insufficient funds")]
    InsufficientFunds,
    
    #[error("Order not found: {0}")]
    OrderNotFound(u128),
    
    #[error("Seat not approved")]
    SeatNotApproved,
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Generic error: {0}")]
    Generic(String),
} 