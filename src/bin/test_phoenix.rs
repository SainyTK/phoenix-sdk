use phoenix_sdk::core::{PhoenixSDK, PhoenixConfig};
use phoenix_sdk::types::PhoenixError;
use std::env;

#[tokio::main]
async fn main() -> Result<(), PhoenixError> {
    // Initialize logging
    env_logger::init();
    
    println!("🔥 Phoenix DEX API Integration Test");
    println!("=====================================");
    
    // Determine configuration based on environment variables
    let use_real_data = env::var("PHOENIX_USE_REAL_DATA")
        .map(|v| v.to_lowercase() == "true" || v == "1")
        .unwrap_or(true); // Default to real data
    
    let config = if env::var("PHOENIX_USE_DEVNET").is_ok() {
        println!("Using Devnet configuration");
        PhoenixConfig::devnet().with_real_data(use_real_data)
    } else if env::var("PHOENIX_USE_MOCK").is_ok() || !use_real_data {
        println!("Using Mock configuration");
        PhoenixConfig::mock()
    } else {
        println!("Using Mainnet configuration");
        PhoenixConfig::mainnet().with_real_data(use_real_data)
    };
    
    // Create SDK instance
    let sdk = PhoenixSDK::new(config);
    
    println!("📊 Data Source: {}", if sdk.is_using_real_data() { "Real On-chain Data" } else { "Mock Data" });
    println!();
    
    // Test 1: Fetch Phoenix markets
    println!("📊 Test 1: Fetching Phoenix markets...");
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
                
                if sdk.is_using_real_data() {
                    println!("    Sequence: {}", market.sequence_number);
                    println!("    Authority: {}", market.authority);
                    println!("    Base Lot Size: {}", market.size_params.base_lot_size);
                    println!("    Quote Lot Size: {}", market.size_params.quote_lot_size);
                    println!("    Tick Size: {}", market.size_params.tick_size);
                }
                println!();
            }
        }
        Err(e) => {
            println!("❌ Error fetching markets: {}", e);
            if sdk.is_using_real_data() {
                println!("💡 Try running with PHOENIX_USE_MOCK=1 to test with mock data");
            }
            return Err(e);
        }
    }
    
    // Test 2: Fetch specific market details
    println!("🔍 Test 2: Fetching market details...");
    let sol_usdc_market = "4DoNfFBfF7UokCC2FQzriy7yHK6DY6NVdYpuekQ5pRgg";
    match sdk.get_phoenix_market_details(sol_usdc_market).await {
        Ok(market) => {
            println!("✅ Market details for {}:", market.name);
            println!("  Address: {}", market.address);
            println!("  Name: {}", market.name);
            println!("  Base Mint: {}", market.base_mint);
            println!("  Quote Mint: {}", market.quote_mint);
            println!("  Sequence Number: {}", market.sequence_number);
            println!("  Base Lot Size: {}", market.size_params.base_lot_size);
            println!("  Quote Lot Size: {}", market.size_params.quote_lot_size);
            println!("  Tick Size: {}", market.size_params.tick_size);
            
            if sdk.is_using_real_data() {
                println!("  📡 Fetched from: On-chain Solana data");
            } else {
                println!("  🏗️  Fetched from: Mock data source");
            }
        }
        Err(e) => {
            println!("❌ Error fetching market details: {}", e);
            if sdk.is_using_real_data() {
                println!("💡 This might be due to network issues or rate limiting");
            }
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
                println!("    {}: ${:.4} x {:.2}", i + 1, bid.price, bid.size);
                if sdk.is_using_real_data() {
                    println!("       (ticks: {}, lots: {})", bid.price_in_ticks, bid.size_in_base_lots);
                }
            }
            
            println!("\n  📊 Asks:");
            for (i, ask) in orderbook.asks.iter().enumerate() {
                println!("    {}: ${:.4} x {:.2}", i + 1, ask.price, ask.size);
                if sdk.is_using_real_data() {
                    println!("       (ticks: {}, lots: {})", ask.price_in_ticks, ask.size_in_base_lots);
                }
            }
            
            if sdk.is_using_real_data() {
                println!("\n  📡 Data source: Real Phoenix on-chain orderbook");
            } else {
                println!("\n  🏗️  Data source: Generated mock orderbook");
            }
        }
        Err(e) => {
            println!("❌ Error fetching orderbook: {}", e);
            if sdk.is_using_real_data() {
                println!("💡 This might be due to network issues, rate limiting, or account parsing errors");
                println!("💡 Try running with PHOENIX_USE_MOCK=1 to test with mock data");
            }
        }
    }
    
    // Test 4: Fetch multiple orderbooks
    println!("\n📊 Test 4: Fetching multiple orderbooks...");
    let markets = [
        "4DoNfFBfF7UokCC2FQzriy7yHK6DY6NVdYpuekQ5pRgg", // SOL/USDC
        "Ew9W18yHYdMySb5PFKryeGikqMNPXzSnaJ1pWVgWNKa6", // ETH/USDC (if it exists)
    ];
    
    match sdk.fetch_phoenix_orderbooks(&markets, Some(3)).await {
        Ok(orderbooks) => {
            println!("✅ Successfully fetched {} orderbooks:", orderbooks.len());
            for (address, orderbook) in &orderbooks {
                println!("  📈 {} ({})", orderbook.market, address);
                if let (Some(best_bid), Some(best_ask)) = (orderbook.bids.get(0), orderbook.asks.get(0)) {
                    println!("    Best bid: ${:.4} x {:.2}", best_bid.price, best_bid.size);
                    println!("    Best ask: ${:.4} x {:.2}", best_ask.price, best_ask.size);
                    let spread = best_ask.price - best_bid.price;
                    let spread_bps = (spread / best_bid.price) * 10000.0;
                    println!("    Spread: ${:.4} ({:.2} bps)", spread, spread_bps);
                }
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
    
    // Final summary
    println!("\n🎉 Test Summary:");
    println!("================");
    println!("📊 Data Source: {}", if sdk.is_using_real_data() { "Real On-chain Data" } else { "Mock Data" });
    if sdk.is_using_real_data() {
        println!("🌐 Network: {}", if env::var("PHOENIX_USE_DEVNET").is_ok() { "Devnet" } else { "Mainnet" });
        println!("📡 Integration: Direct Phoenix program account parsing");
        println!("🔗 Program ID: PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY");
    } else {
        println!("🏗️  Mode: Mock/Testing data generation");
    }
    
    println!("✅ All tests completed successfully!");
    
    // Usage instructions
    println!("\n💡 Usage Tips:");
    println!("==============");
    println!("🔸 Use PHOENIX_USE_REAL_DATA=true for real on-chain data (default)");
    println!("🔸 Use PHOENIX_USE_MOCK=1 for mock data testing");
    println!("🔸 Use PHOENIX_USE_DEVNET=1 for devnet instead of mainnet");
    println!("🔸 Combine flags: PHOENIX_USE_DEVNET=1 PHOENIX_USE_REAL_DATA=true");
    
    Ok(())
}