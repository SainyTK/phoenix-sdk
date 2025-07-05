use crate::types::{PhoenixMarket, PhoenixOrderbook, PhoenixOrderbookEntry, PhoenixError, MarketStatus, MarketFees, MarketSizeParams};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::collections::HashMap;
use std::str::FromStr;

/// Phoenix DEX API client configuration
#[derive(Clone, Debug)]
pub struct PhoenixApiConfig {
    pub solana_rpc_url: String,
    pub phoenix_program_id: String,
    pub commitment: CommitmentConfig,
    pub timeout_seconds: u64,
}

impl Default for PhoenixApiConfig {
    fn default() -> Self {
        Self {
            solana_rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
            phoenix_program_id: "PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY".to_string(),
            commitment: CommitmentConfig::confirmed(),
            timeout_seconds: 30,
        }
    }
}

impl PhoenixApiConfig {
    pub fn devnet() -> Self {
        Self {
            solana_rpc_url: "https://api.devnet.solana.com".to_string(),
            phoenix_program_id: "PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY".to_string(),
            commitment: CommitmentConfig::confirmed(),
            timeout_seconds: 30,
        }
    }
    
    pub fn mainnet() -> Self {
        Self {
            solana_rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
            phoenix_program_id: "PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY".to_string(),
            commitment: CommitmentConfig::confirmed(),
            timeout_seconds: 30,
        }
    }
}

/// Phoenix API client for interacting with Phoenix DEX
pub struct PhoenixApiClient {
    config: PhoenixApiConfig,
    rpc_client: RpcClient,
    http_client: reqwest::Client,
}

impl PhoenixApiClient {
    pub fn new(config: PhoenixApiConfig) -> Self {
        let rpc_client = RpcClient::new_with_commitment(
            config.solana_rpc_url.clone(),
            config.commitment,
        );
        
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");
        
        Self {
            config,
            rpc_client,
            http_client,
        }
    }
    
    /// Fetch all Phoenix markets
    pub async fn fetch_phoenix_markets(&self) -> Result<Vec<PhoenixMarket>, PhoenixError> {
        // Since we don't have the actual Phoenix SDK integrated yet, we'll use the Phoenix API
        // In a real implementation, this would use solana_client to fetch program accounts
        
        // For now, return some sample Phoenix markets based on known markets
        let sample_markets = vec![
            PhoenixMarket {
                address: "4DoNfFBfF7UokCC2FQzriy7yHK6DY6NVdYpuekQ5pRgg".to_string(),
                name: "SOL/USDC".to_string(),
                base_mint: "So11111111111111111111111111111111111111112".to_string(),
                quote_mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
                base_vault: "8CvwxZ9Db6XbLD46NZwwmVDZZRDy7eydFcAGkXKh9axa".to_string(),
                quote_vault: "2kVNVEgHicvfwiyhT2T51YiQGMPFWLMSp8qXc1hHzkpU".to_string(),
                authority: "11111111111111111111111111111111".to_string(),
                sequence_number: 1,
                status: MarketStatus::Active,
                fees: MarketFees {
                    maker_fee_bps: 0,
                    taker_fee_bps: 4,
                },
                size_params: MarketSizeParams {
                    base_lot_size: 1000000,
                    quote_lot_size: 1000,
                    tick_size: 1,
                },
            },
            PhoenixMarket {
                address: "Ew9W18yHYdMySb5PFKryeGikqMNPXzSnaJ1pWVgWNKa6".to_string(),
                name: "ETH/USDC".to_string(),
                base_mint: "7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs".to_string(),
                quote_mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
                base_vault: "7F4HMZs9fMoKVp1KBuEJzfYxF7iKN8bj5nA2VJ6VrZx9".to_string(),
                quote_vault: "8HGyAAB1yoM1ttS7pXjHMa3dukTFGQggnFFH3hJZgzQh".to_string(),
                authority: "11111111111111111111111111111111".to_string(),
                sequence_number: 1,
                status: MarketStatus::Active,
                fees: MarketFees {
                    maker_fee_bps: 0,
                    taker_fee_bps: 4,
                },
                size_params: MarketSizeParams {
                    base_lot_size: 1000000,
                    quote_lot_size: 1000,
                    tick_size: 1,
                },
            },
        ];
        
        Ok(sample_markets)
    }
    
    /// Fetch market details for a specific market
    pub async fn get_market_details(&self, market_address: &str) -> Result<PhoenixMarket, PhoenixError> {
        // Validate the market address
        let _pubkey = Pubkey::from_str(market_address)
            .map_err(|e| PhoenixError::Parse(format!("Invalid market address: {}", e)))?;
        
        // Fetch all markets and find the one with matching address
        let markets = self.fetch_phoenix_markets().await?;
        
        markets
            .into_iter()
            .find(|m| m.address == market_address)
            .ok_or_else(|| PhoenixError::MarketNotFound(market_address.to_string()))
    }
    
    /// Fetch orderbook for a specific market
    pub async fn fetch_phoenix_orderbook(&self, market_address: &str, depth: Option<u32>) -> Result<PhoenixOrderbook, PhoenixError> {
        // Validate the market address
        let _pubkey = Pubkey::from_str(market_address)
            .map_err(|e| PhoenixError::Parse(format!("Invalid market address: {}", e)))?;
        
        // Get market details first
        let market = self.get_market_details(market_address).await?;
        
        // For now, we'll use a Phoenix API endpoint to fetch orderbook data
        // In a real implementation, this would parse the on-chain orderbook data
        let orderbook = self.fetch_orderbook_from_api(&market.name, depth).await?;
        
        Ok(orderbook)
    }
    
    /// Fetch orderbook from Phoenix API (helper method)
    async fn fetch_orderbook_from_api(&self, market_name: &str, depth: Option<u32>) -> Result<PhoenixOrderbook, PhoenixError> {
        // Since Phoenix doesn't have a public REST API for orderbook data, we'll generate sample data
        // In a real implementation, this would call the actual Phoenix API or parse on-chain data
        
        let depth = depth.unwrap_or(20);
        
        // Generate sample orderbook data
        let (bids, asks) = self.generate_sample_orderbook_data(market_name, depth as usize)?;
        
        Ok(PhoenixOrderbook {
            market: market_name.to_string(),
            bids,
            asks,
            sequence_number: rand::random::<u64>(),
            slot: self.get_current_slot().await?,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }
    
    /// Generate sample orderbook data (placeholder implementation)
    fn generate_sample_orderbook_data(&self, market_name: &str, depth: usize) -> Result<(Vec<PhoenixOrderbookEntry>, Vec<PhoenixOrderbookEntry>), PhoenixError> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        let (base_price, tick_size) = match market_name {
            "SOL/USDC" => (20.0, 0.01),
            "ETH/USDC" => (1800.0, 0.1),
            "BTC/USDC" => (28000.0, 1.0),
            _ => (100.0, 0.01),
        };
        
        let mut bids = Vec::new();
        let mut asks = Vec::new();
        
        // Generate bids (descending price)
        for i in 0..depth {
            let price = base_price - (i as f64 * tick_size * rng.gen_range(1.0..3.0));
            let size = rng.gen_range(0.1..100.0);
            let price_in_ticks = (price / tick_size) as u64;
            let size_in_base_lots = (size * 1000000.0) as u64;
            
            bids.push(PhoenixOrderbookEntry {
                price,
                size,
                price_in_ticks,
                size_in_base_lots,
            });
        }
        
        // Generate asks (ascending price)
        for i in 0..depth {
            let price = base_price + (i as f64 * tick_size * rng.gen_range(1.0..3.0));
            let size = rng.gen_range(0.1..100.0);
            let price_in_ticks = (price / tick_size) as u64;
            let size_in_base_lots = (size * 1000000.0) as u64;
            
            asks.push(PhoenixOrderbookEntry {
                price,
                size,
                price_in_ticks,
                size_in_base_lots,
            });
        }
        
        // Sort bids (highest price first)
        bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
        
        // Sort asks (lowest price first)
        asks.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
        
        Ok((bids, asks))
    }
    
    /// Get current Solana slot
    async fn get_current_slot(&self) -> Result<u64, PhoenixError> {
        self.rpc_client
            .get_slot()
            .map_err(PhoenixError::SolanaClient)
    }
    
    /// Fetch multiple orderbooks in parallel
    pub async fn fetch_multiple_orderbooks(&self, market_addresses: &[&str], depth: Option<u32>) -> Result<HashMap<String, PhoenixOrderbook>, PhoenixError> {
        let mut results = HashMap::new();
        
        // For now, we'll fetch them sequentially
        // In a real implementation, we'd use futures::join_all for parallel fetching
        for &market_address in market_addresses {
            match self.fetch_phoenix_orderbook(market_address, depth).await {
                Ok(orderbook) => {
                    results.insert(market_address.to_string(), orderbook);
                }
                Err(e) => {
                    log::warn!("Failed to fetch orderbook for {}: {}", market_address, e);
                }
            }
        }
        
        Ok(results)
    }
    
    /// Check if a market exists
    pub async fn market_exists(&self, market_address: &str) -> bool {
        self.get_market_details(market_address).await.is_ok()
    }
}