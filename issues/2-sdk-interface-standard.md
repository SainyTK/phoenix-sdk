# ISSUE 2: Rust SDK Interface Standard

We want to ensure that the entry point of the SDK follows the defined interface.

This is how the entry point of the SDK should look like:
```rust
use phoenix_sdk::{PhoenixSDK, PhoenixConfig};

#[tokio::main]
async fn main() {
    let config = PhoenixConfig::new(
        api_base_url: String::from("https://api.mainnet-beta.solana.com"),
        ws_url: String::from("wss://api.mainnet-beta.solana.com"),
    );
    // Create a new Phoenix market client
    let market = PhoenixSDK::new(config);
    
    // Fetch market symbols
    let symbols = await market.fetch_market_symbols().unwrap();
    println!("Available markets: {:?}", symbols); // { "SOL_USDC": { "id": 1, "symbol": "SOL_USDC", "info": "..." }, "ETH_USDC": { "id": 2, "symbol": "ETH_USDC", "info": "..." } }
    
    // Fetch orderbooks
    let orderbooks = market.fetch_order_books(
        &[String::from("SOL_USDC")], 
        Some(10)
    ).unwrap();
    println!("Orderbook: {:?}", orderbooks); // { "SOL_USDC": { "bids": [ { "price": 100, "amount": 100 }, { "price": 99, "amount": 200 } ], "asks": [ { "price": 101, "amount": 100 }, { "price": 102, "amount": 200 } ] } }

    let r_id = market.subscribe_orderbook("SOL_USDC", |orderbook| {
        println!("Orderbook: {:?}", orderbook);
        // { "SOL_USDC": { "bids": [ { "price": 100, "amount": 100 }, { "price": 99, "amount": 200 } ], "asks": [ { "price": 101, "amount": 100 }, { "price": 102, "amount": 200 } ] } }
    });
    println!("Subscription ID: {:?}", r_id); // "sub_1234567890"

    // Unsubscribe from orderbook
    await market.unsubscribe_orderbook(r_id);
}
```

## Tasks
- [ ] Update the entry point of the SDK to follow the defined interface.
- [ ] Update the types of input parameters and return values to match the defined interface.
- [ ] Update the examples to match the defined interface.
- [ ] Update the tests to match the defined interface. 