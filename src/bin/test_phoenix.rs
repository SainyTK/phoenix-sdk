use phoenix_sdk::core::{PhoenixSDK, PhoenixConfig};
use phoenix_sdk::types::PhoenixError;
use std::env;

#[tokio::main]
async fn main() -> Result<(), PhoenixError> {
    // Initialize logging
    env_logger::init();
    
    println!("🔥 Phoenix DEX API Integration Test");
    println!("=====================================");
    
    // Get configuration from environment or use default
    let config = if env::var("PHOENIX_USE_DEVNET").is_ok() {
        println!("Using Devnet configuration");
        PhoenixConfig::devnet()
    } else {
        println!("Using Mainnet configuration");
        PhoenixConfig::mainnet()
    };
    
    // Create SDK instance
    let sdk = PhoenixSDK::new(config);
    
    // Test 1: Fetch Phoenix markets
    println!("\n📊 Test 1: Fetching Phoenix markets...");
    match sdk.fetch_phoenix_markets().await {
        Ok(markets) => {
            println!("✅ Successfully fetched {} markets:", markets.len());
            for market in &markets {
                println!("  - {} ({})", market.name, market.address);
                println!("    Base: {}", market.base_mint);
                println!("    Quote: {}", market.quote_mint);
                println!("    Status: {:?}", market.status);
                println!("    Fees: {}bps maker, {}bps taker", 
                         market.fees.maker_fee_bps, market.fees.taker_fee_bps);
                println!();
            }
        }
        Err(e) => {
            println!("❌ Error fetching markets: {}", e);
            return Err(e);
        }
    }
    
    // Test 2: Fetch specific market details
    println!("\n🔍 Test 2: Fetching market details...");
    let sol_usdc_market = "4DoNfFBfF7UokCC2FQzriy7yHK6DY6NVdYpuekQ5pRgg";
    match sdk.get_phoenix_market_details(sol_usdc_market).await {
        Ok(market) => {
            println!("✅ Market details for SOL/USDC:");
            println!("  Address: {}", market.address);
            println!("  Name: {}", market.name);
            println!("  Base Mint: {}", market.base_mint);
            println!("  Quote Mint: {}", market.quote_mint);
            println!("  Sequence Number: {}", market.sequence_number);
            println!("  Base Lot Size: {}", market.size_params.base_lot_size);
            println!("  Quote Lot Size: {}", market.size_params.quote_lot_size);
            println!("  Tick Size: {}", market.size_params.tick_size);
        }
        Err(e) => {
            println!("❌ Error fetching market details: {}", e);
        }
    }
    
    // Test 3: Fetch orderbook
    println!("\n📈 Test 3: Fetching orderbook...");
    match sdk.fetch_phoenix_orderbook(sol_usdc_market, Some(5)).await {
        Ok(orderbook) => {
            println!("✅ Orderbook for {} (depth: 5):", orderbook.market);
            println!("  Sequence Number: {}", orderbook.sequence_number);
            println!("  Slot: {}", orderbook.slot);
            println!("  Timestamp: {}", orderbook.timestamp);
            
            println!("\n  📊 Bids:");
            for (i, bid) in orderbook.bids.iter().enumerate() {
                println!("    {}: ${:.4} x {:.2} (ticks: {}, lots: {})", 
                         i + 1, bid.price, bid.size, bid.price_in_ticks, bid.size_in_base_lots);
            }
            
            println!("\n  📊 Asks:");
            for (i, ask) in orderbook.asks.iter().enumerate() {
                println!("    {}: ${:.4} x {:.2} (ticks: {}, lots: {})", 
                         i + 1, ask.price, ask.size, ask.price_in_ticks, ask.size_in_base_lots);
            }
        }
        Err(e) => {
            println!("❌ Error fetching orderbook: {}", e);
        }
    }
    
    // Test 4: Fetch multiple orderbooks
    println!("\n📊 Test 4: Fetching multiple orderbooks...");
    let markets = [
        "4DoNfFBfF7UokCC2FQzriy7yHK6DY6NVdYpuekQ5pRgg", // SOL/USDC
        "Ew9W18yHYdMySb5PFKryeGikqMNPXzSnaJ1pWVgWNKa6", // ETH/USDC
    ];
    
    match sdk.fetch_phoenix_orderbooks(&markets, Some(3)).await {
        Ok(orderbooks) => {
            println!("✅ Successfully fetched {} orderbooks:", orderbooks.len());
            for (address, orderbook) in &orderbooks {
                println!("  📈 {} ({})", orderbook.market, address);
                println!("    Best bid: ${:.4} x {:.2}", 
                         orderbook.bids.get(0).map(|b| b.price).unwrap_or(0.0),
                         orderbook.bids.get(0).map(|b| b.size).unwrap_or(0.0));
                println!("    Best ask: ${:.4} x {:.2}", 
                         orderbook.asks.get(0).map(|a| a.price).unwrap_or(0.0),
                         orderbook.asks.get(0).map(|a| a.size).unwrap_or(0.0));
            }
        }
        Err(e) => {
            println!("❌ Error fetching multiple orderbooks: {}", e);
        }
    }
    
    // Test 5: Test legacy format conversion
    println!("\n🔄 Test 5: Testing legacy format conversion...");
    match sdk.get_phoenix_orderbook_legacy(sol_usdc_market, Some(3)).await {
        Ok(legacy_orderbook) => {
            println!("✅ Legacy format orderbook:");
            println!("  Bids: {} levels", legacy_orderbook.bids.len());
            for (i, bid) in legacy_orderbook.bids.iter().enumerate() {
                println!("    {}: ${:.4} x {:.2}", i + 1, bid.price, bid.size);
            }
            println!("  Asks: {} levels", legacy_orderbook.asks.len());
            for (i, ask) in legacy_orderbook.asks.iter().enumerate() {
                println!("    {}: ${:.4} x {:.2}", i + 1, ask.price, ask.size);
            }
        }
        Err(e) => {
            println!("❌ Error with legacy format: {}", e);
        }
    }
    
    // Test 6: Test caching
    println!("\n💾 Test 6: Testing cached data...");
    if let Some(cached_orderbook) = sdk.get_cached_phoenix_orderbook(sol_usdc_market) {
        println!("✅ Found cached orderbook for SOL/USDC:");
        println!("  Bids: {} levels", cached_orderbook.bids.len());
        println!("  Asks: {} levels", cached_orderbook.asks.len());
        println!("  Sequence: {}", cached_orderbook.sequence_number);
    } else {
        println!("❌ No cached orderbook found");
    }
    
    if let Some(cached_market) = sdk.get_cached_phoenix_market(sol_usdc_market) {
        println!("✅ Found cached market for SOL/USDC:");
        println!("  Name: {}", cached_market.name);
        println!("  Status: {:?}", cached_market.status);
    } else {
        println!("❌ No cached market found");
    }
    
    println!("\n🎉 All tests completed!");
    Ok(())
}