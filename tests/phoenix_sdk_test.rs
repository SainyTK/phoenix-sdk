#[cfg(test)]
mod tests {
    use phoenix_sdk::core::{PhoenixSDK, PhoenixConfig};

    #[test]
    fn test_fetch_market_symbols() {
        let sdk = PhoenixSDK::new(PhoenixConfig::default());
        
        // Test fetching market symbols
        let symbols_result = sdk.fetch_markets();
        assert!(symbols_result.is_ok());
        
        let symbols = symbols_result.unwrap();
        assert!(!symbols.is_empty());
        assert_eq!(symbols.len(), 3);
        assert_eq!(symbols[0].symbol, "SOL_USDC");
        assert_eq!(symbols[1].symbol, "BTC_USDC");
    }

    // #[test]
    // fn test_fetch_order_books_single_symbol() {
    //     let market = PhoenixMarket::default();
        
    //     // Test fetching orderbook with a single symbol
    //     let symbols = vec!["SOL_USDC".to_string()];
    //     let orderbooks_result = market.fetch_order_books(&symbols, None);
    //     assert!(orderbooks_result.is_ok());
        
    //     let orderbooks = orderbooks_result.unwrap();
    //     assert_eq!(orderbooks.len(), 1);
        
    //     // Check SOL_USDC orderbook
    //     assert!(orderbooks.contains_key("SOL_USDC"));
    //     let sol_usdc = &orderbooks["SOL_USDC"];
    //     assert!(!sol_usdc.bids.is_empty());
    //     assert!(!sol_usdc.asks.is_empty());
    //     assert_eq!(sol_usdc.bids.len(), 3);
    //     assert_eq!(sol_usdc.asks.len(), 3);
        
    //     // Check specific values
    //     assert_eq!(sol_usdc.bids[0].price, 19.9240);
    //     assert_eq!(sol_usdc.bids[0].size, 214.3670);
    //     assert_eq!(sol_usdc.asks[0].price, 19.9710);
    //     assert_eq!(sol_usdc.asks[0].size, 37.4840);
    // }

    // #[test]
    // fn test_fetch_order_books_multiple_symbols() {
    //     let market = PhoenixMarket::default();
        
    //     // Test fetching orderbooks with multiple symbols
    //     let symbols = vec!["SOL_USDC".to_string(), "BTC_USDC".to_string()];
    //     let orderbooks_result = market.fetch_order_books(&symbols, Some(2));
    //     assert!(orderbooks_result.is_ok());
        
    //     let orderbooks = orderbooks_result.unwrap();
    //     assert_eq!(orderbooks.len(), 2);
        
    //     // Check SOL_USDC orderbook
    //     assert!(orderbooks.contains_key("SOL_USDC"));
    //     let sol_usdc = &orderbooks["SOL_USDC"];
    //     assert!(!sol_usdc.bids.is_empty());
    //     assert!(!sol_usdc.asks.is_empty());
    //     assert_eq!(sol_usdc.bids.len(), 2); // Limited to 2 entries
    //     assert_eq!(sol_usdc.asks.len(), 2); // Limited to 2 entries
        
    //     // Check BTC_USDC orderbook
    //     assert!(orderbooks.contains_key("BTC_USDC"));
    //     let btc_usdc = &orderbooks["BTC_USDC"];
    //     assert!(!btc_usdc.bids.is_empty());
    //     assert!(!btc_usdc.asks.is_empty());
    //     assert_eq!(btc_usdc.bids.len(), 2); // Limited to 2 entries
    //     assert_eq!(btc_usdc.asks.len(), 2); // Limited to 2 entries
    // }

    // #[test]
    // fn test_fetch_order_books_with_limit() {
    //     let market = PhoenixMarket::default();
        
    //     // Test with limit parameter
    //     let symbols = vec!["SOL_USDC".to_string()];
        
    //     // First get without limit to see total size
    //     let orderbooks_full = market.fetch_order_books(&symbols, None).unwrap();
    //     let full_orderbook = &orderbooks_full["SOL_USDC"];
    //     let full_bids_count = full_orderbook.bids.len();
    //     let full_asks_count = full_orderbook.asks.len();
        
    //     // Now with limit = 1
    //     let orderbooks_limited = market.fetch_order_books(&symbols, Some(1)).unwrap();
    //     let limited_orderbook = &orderbooks_limited["SOL_USDC"];
        
    //     // Verify limiting worked
    //     assert_eq!(limited_orderbook.bids.len(), 1);
    //     assert_eq!(limited_orderbook.asks.len(), 1);
    //     assert!(full_bids_count > limited_orderbook.bids.len());
    //     assert!(full_asks_count > limited_orderbook.asks.len());
        
    //     // Verify we got the top bid and ask
    //     assert_eq!(limited_orderbook.bids[0].price, full_orderbook.bids[0].price);
    //     assert_eq!(limited_orderbook.asks[0].price, full_orderbook.asks[0].price);
    // }

    // #[test]
    // fn test_orderbook_subscription() {
    //     // Create orderbook manager
    //     let mut orderbook_manager = OrderbookManager::default();
        
    //     // Create a flag to track callback execution
    //     let callback_executed = Arc::new(Mutex::new(false));
    //     let callback_executed_clone = callback_executed.clone();
        
    //     // Create callback that sets the flag
    //     let callback = Box::new(move |orderbook: &Orderbook| {
    //         assert!(!orderbook.bids.is_empty());
    //         assert!(!orderbook.asks.is_empty());
    //         *callback_executed_clone.lock().unwrap() = true;
    //     });
        
    //     // Subscribe to a market
    //     let subscription_result = orderbook_manager.subscribe("SOL_USDC", callback);
    //     assert!(subscription_result.is_ok());
        
    //     // Verify subscription ID format
    //     let subscription_id = subscription_result.unwrap();
    //     assert!(subscription_id.starts_with("sub_SOL_USDC_"));
        
    //     // Simulate an update
    //     let update_result = orderbook_manager.simulate_update("SOL_USDC");
    //     assert!(update_result.is_ok());
        
    //     // Check that the callback was executed
    //     assert!(*callback_executed.lock().unwrap());
    // }

    // #[test]
    // fn test_unsubscribe() {
    //     // Create orderbook manager
    //     let mut orderbook_manager = OrderbookManager::default();
        
    //     // Subscribe to a market
    //     let callback = Box::new(|_: &Orderbook| {});
    //     let subscription_result = orderbook_manager.subscribe("SOL_USDC", callback);
    //     assert!(subscription_result.is_ok());
        
    //     let subscription_id = subscription_result.unwrap();
        
    //     // Unsubscribe
    //     let unsubscribe_result = orderbook_manager.unsubscribe(&subscription_id);
    //     assert!(unsubscribe_result.is_ok());
    //     assert!(unsubscribe_result.unwrap());
        
    //     // Unsubscribe again (should return false but not error)
    //     let unsubscribe_result = orderbook_manager.unsubscribe(&subscription_id);
    //     assert!(unsubscribe_result.is_ok());
    //     assert!(!unsubscribe_result.unwrap());
    // }

    // #[test]
    // fn test_connect_and_close() {
    //     let mut orderbook_manager = OrderbookManager::default();
        
    //     // Test connect
    //     let connect_result = orderbook_manager.connect();
    //     assert!(connect_result.is_ok());
        
    //     // Test close
    //     let close_result = orderbook_manager.close();
    //     assert!(close_result.is_ok());
    // }
} 