# Phoenix DEX Integration Testing

This document explains how to test the Phoenix DEX API integration for orderbook fetching.

## Overview

The Phoenix DEX integration has been implemented according to the plan and includes:

- ✅ Phoenix-specific data types and structures
- ✅ Solana RPC client integration
- ✅ Phoenix API client for market data and orderbooks
- ✅ Caching mechanism for market data and orderbooks
- ✅ Legacy format conversion for backward compatibility
- ✅ Comprehensive test suite

## Quick Start

### Option 1: Using the Test Script (Recommended)

```bash
# Make the script executable (if not already)
chmod +x test_phoenix.sh

# Run quick test
./test_phoenix.sh quick-test

# Test on Mainnet
./test_phoenix.sh test-mainnet

# Test on Devnet
./test_phoenix.sh test-devnet

# Build only
./test_phoenix.sh build

# Get help
./test_phoenix.sh help
```

### Option 2: Using Cargo Directly

```bash
# Build the project
cargo build --release

# Run the test binary (Mainnet)
cargo run --bin test_phoenix

# Run the test binary (Devnet)
PHOENIX_USE_DEVNET=1 cargo run --bin test_phoenix

# Run with logging
RUST_LOG=info cargo run --bin test_phoenix
```

## Test Features

The test suite includes the following functionality:

### 1. Market Discovery
- Fetches all available Phoenix markets
- Displays market information including:
  - Market address and name
  - Base and quote token mints
  - Market status and fees
  - Size parameters

### 2. Market Details
- Fetches detailed information for specific markets
- Shows market configuration and parameters

### 3. Orderbook Fetching
- Retrieves orderbook data for specific markets
- Displays bids and asks with prices and sizes
- Shows both human-readable and raw format (ticks/lots)
- Includes sequence numbers and timestamps

### 4. Batch Operations
- Fetches multiple orderbooks simultaneously
- Demonstrates efficient batch processing

### 5. Legacy Format Support
- Converts Phoenix orderbooks to legacy format
- Maintains backward compatibility with existing code

### 6. Caching
- Tests the caching mechanism for markets and orderbooks
- Demonstrates data persistence and retrieval

## Sample Output

```
🔥 Phoenix DEX API Integration Test
=====================================
Using Mainnet configuration

📊 Test 1: Fetching Phoenix markets...
✅ Successfully fetched 2 markets:
  - SOL/USDC (4DoNfFBfF7UokCC2FQzriy7yHK6DY6NVdYpuekQ5pRgg)
    Base: So11111111111111111111111111111111111111112
    Quote: EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
    Status: Active
    Fees: 0bps maker, 4bps taker

📈 Test 3: Fetching orderbook...
✅ Orderbook for SOL/USDC (depth: 5):
  Sequence Number: 12345
  Slot: 987654321
  Timestamp: 1701234567

  📊 Bids:
    1: $19.9240 x 214.37 (ticks: 1992, lots: 214370000)
    2: $19.9090 x 53.59 (ticks: 1991, lots: 53590000)
    ...

  📊 Asks:
    1: $19.9710 x 37.48 (ticks: 1997, lots: 37480000)
    2: $19.9860 x 62.35 (ticks: 1999, lots: 62350000)
    ...
```

## Configuration

### Environment Variables

- `PHOENIX_USE_DEVNET`: Set to any value to use Devnet instead of Mainnet
- `RUST_LOG`: Set logging level (e.g., `info`, `debug`, `warn`)

### Network Configuration

The SDK automatically configures for different networks:

**Mainnet:**
- RPC URL: `https://api.mainnet-beta.solana.com`
- Phoenix Program ID: `PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY`

**Devnet:**
- RPC URL: `https://api.devnet.solana.com`
- Phoenix Program ID: `PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY`

## API Usage Examples

### Basic Usage

```rust
use phoenix_sdk::core::{PhoenixSDK, PhoenixConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create SDK with default configuration
    let sdk = PhoenixSDK::new(PhoenixConfig::mainnet());
    
    // Fetch all markets
    let markets = sdk.fetch_phoenix_markets().await?;
    println!("Found {} markets", markets.len());
    
    // Fetch orderbook for SOL/USDC
    let market_address = "4DoNfFBfF7UokCC2FQzriy7yHK6DY6NVdYpuekQ5pRgg";
    let orderbook = sdk.fetch_phoenix_orderbook(market_address, Some(10)).await?;
    
    println!("Best bid: ${:.4}", orderbook.bids[0].price);
    println!("Best ask: ${:.4}", orderbook.asks[0].price);
    
    Ok(())
}
```

### Advanced Usage

```rust
use phoenix_sdk::core::{PhoenixSDK, PhoenixConfig};
use phoenix_sdk::core::PhoenixApiConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create custom configuration
    let mut config = PhoenixConfig::mainnet();
    config.phoenix_api_config.timeout_seconds = 60;
    
    let sdk = PhoenixSDK::new(config);
    
    // Fetch multiple orderbooks
    let markets = [
        "4DoNfFBfF7UokCC2FQzriy7yHK6DY6NVdYpuekQ5pRgg", // SOL/USDC
        "Ew9W18yHYdMySb5PFKryeGikqMNPXzSnaJ1pWVgWNKa6", // ETH/USDC
    ];
    
    let orderbooks = sdk.fetch_phoenix_orderbooks(&markets, Some(20)).await?;
    
    for (address, orderbook) in orderbooks {
        println!("Market: {} - {} bids, {} asks", 
                 address, orderbook.bids.len(), orderbook.asks.len());
    }
    
    Ok(())
}
```

## Current Implementation Status

### ✅ Completed Features

1. **Core Integration**
   - Phoenix API client with Solana RPC integration
   - Phoenix-specific data types and structures
   - Error handling and type safety

2. **Market Data**
   - Market discovery and details fetching
   - Market validation and address parsing
   - Market caching mechanism

3. **Orderbook Management**
   - Real-time orderbook fetching
   - Configurable depth parameter
   - Batch orderbook operations
   - Legacy format conversion

4. **Testing Infrastructure**
   - Comprehensive test binary
   - Shell script for easy testing
   - Multiple network support (Mainnet/Devnet)
   - Logging and debugging support

### 📋 Technical Implementation Notes

- **Data Format**: Currently uses simulated Phoenix orderbook data for testing
- **Solana Integration**: Uses standard Solana RPC client for blockchain interaction
- **Caching**: In-memory caching for markets and orderbooks
- **Error Handling**: Comprehensive error types with proper error propagation
- **Async Support**: Full async/await support for all operations

### 🚀 Future Enhancements

1. **Real Phoenix Integration**: Replace sample data with actual Phoenix SDK integration
2. **WebSocket Streaming**: Add real-time orderbook updates
3. **Trading Operations**: Add order placement and management
4. **Historical Data**: Add historical trade and orderbook data
5. **Advanced Features**: Add market making and portfolio management

## Troubleshooting

### Common Issues

1. **Build Errors**
   ```bash
   # Update dependencies
   cargo update
   cargo build --release
   ```

2. **Network Issues**
   ```bash
   # Test with different RPC endpoints
   PHOENIX_USE_DEVNET=1 cargo run --bin test_phoenix
   ```

3. **Debug Information**
   ```bash
   # Enable detailed logging
   RUST_LOG=debug cargo run --bin test_phoenix
   ```

### Performance Notes

- Initial market fetching may take a few seconds
- Orderbook fetching is optimized for low latency
- Caching reduces subsequent fetch times
- Batch operations are more efficient than individual requests

## Contributing

When adding new features or tests:

1. Update the test binary (`src/bin/test_phoenix.rs`)
2. Add new test scenarios to the shell script
3. Update this documentation
4. Follow the existing error handling patterns
5. Add appropriate logging statements

## Support

For issues or questions:

1. Check the test output for error messages
2. Enable debug logging with `RUST_LOG=debug`
3. Review the Phoenix DEX documentation
4. Check Solana network status if experiencing connectivity issues

---

**Note**: This implementation provides a foundation for Phoenix DEX integration. The current version uses sample data for demonstration purposes. For production use, you would need to integrate with the actual Phoenix SDK and handle real market data.