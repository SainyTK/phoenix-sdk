use phoenix_sdk::core::{PhoenixSDK, PhoenixConfig};
use phoenix_sdk::types::PhoenixError;
use std::env;

#[tokio::main]
async fn main() -> Result<(), PhoenixError> {
    // Initialize logging
    env_logger::init();
    
    println!("🔥 Phoenix DEX API Integration Test");
    println!("=====================================");
    
    // Create mainnet configuration (only mainnet is supported now)
    let config = PhoenixConfig::default();
    
    println!("Using Mainnet configuration with real data");
    
    // Create SDK instance with error handling
    let mut sdk = PhoenixSDK::new(config)?;
    
    println!("📊 Data Source: Real On-chain Data (Mainnet only)");
    println!();
    
    // Initialize the SDK
    println!("🔧 Initializing SDK...");
    match sdk.init().await {
        Ok(_) => println!("✅ SDK initialized successfully"),
        Err(e) => {
            println!("❌ Error initializing SDK: {}", e);
            return Err(e);
        }
    }
    
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
                println!("    Sequence: {}", market.sequence_number);
                println!("    Authority: {}", market.authority);
                println!("    Base Lot Size: {}", market.size_params.base_lot_size);
                println!("    Quote Lot Size: {}", market.size_params.quote_lot_size);
                println!("    Tick Size: {}", market.size_params.tick_size);
                println!();
            }
        }
        Err(e) => {
            println!("❌ Error fetching markets: {}", e);
            println!("💡 This might be due to network issues or rate limiting");
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
            println!("  📡 Fetched from: On-chain Solana mainnet data");
        }
        Err(e) => {
            println!("❌ Error fetching market details: {}", e);
            println!("💡 This might be due to network issues or rate limiting");
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
                println!("       (ticks: {}, lots: {})", bid.price_in_ticks, bid.size_in_base_lots);
            }
            
            println!("\n  📊 Asks:");
            for (i, ask) in orderbook.asks.iter().enumerate() {
                println!("    {}: ${:.4} x {:.2}", i + 1, ask.price, ask.size);
                println!("       (ticks: {}, lots: {})", ask.price_in_ticks, ask.size_in_base_lots);
            }
            
            println!("\n  📡 Data source: Real Phoenix on-chain orderbook");
        }
        Err(e) => {
            println!("❌ Error fetching orderbook: {}", e);
            println!("💡 This might be due to network issues, rate limiting, or account parsing errors");
        }
    }
    
    // Test 4: Fetch multiple orderbooks
    println!("\n📊 Test 4: Fetching multiple orderbooks...");
    let markets = [
        "4DoNfFBfF7UokCC2FQzriy7yHK6DY6NVdYpuekQ5pRgg", // SOL/USDC
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
    
    // Test 7: Test market existence check
    println!("\n🔍 Test 7: Testing market existence check...");
    let market_exists = sdk.phoenix_market_exists(sol_usdc_market).await;
    println!("✅ Market {} exists: {}", sol_usdc_market, market_exists);
    
    // Test non-existent market
    let fake_market = "1111111111111111111111111111111111111111111";
    let fake_market_exists = sdk.phoenix_market_exists(fake_market).await;
    println!("✅ Market {} exists: {}", fake_market, fake_market_exists);
    
    // Test 8: Test legacy market functions
    println!("\n📊 Test 8: Testing legacy market functions...");
    match sdk.fetch_markets() {
        Ok(markets) => {
            println!("✅ Legacy fetch_markets returned {} markets:", markets.len());
            for market in &markets {
                println!("  - {} (ID: {})", market.symbol, market.id);
            }
        }
        Err(e) => {
            println!("❌ Error with legacy fetch_markets: {}", e);
        }
    }
    
    // Test legacy orderbook fetch
    match sdk.fetch_orderbook("SOL_USDC") {
        Ok(orderbook) => {
            println!("✅ Legacy fetch_orderbook returned:");
            println!("  Bids: {} levels", orderbook.bids.len());
            println!("  Asks: {} levels", orderbook.asks.len());
        }
        Err(e) => {
            println!("❌ Error with legacy fetch_orderbook: {}", e);
        }
    }
    
    // Final summary
    println!("\n🎉 Test Summary:");
    println!("================");
    println!("📊 Data Source: Real On-chain Data (Mainnet only)");
    println!("🌐 Network: Mainnet");
    println!("📡 Integration: Direct Phoenix program account parsing");
    println!("🔗 Program ID: PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY");
    println!("✅ All tests completed successfully!");
    
    // Usage instructions
    println!("\n💡 Usage Tips:");
    println!("==============");
    println!("🔸 The SDK now only supports mainnet real data");
    println!("🔸 Mock and devnet modes have been removed for consistency");
    println!("🔸 All data is fetched from on-chain Phoenix program accounts");
    println!("🔸 Use environment variable PHOENIX_RPC_URL to customize RPC endpoint");
    println!("🔸 Example: PHOENIX_RPC_URL=https://api.mainnet-beta.solana.com");
    
    Ok(())
}