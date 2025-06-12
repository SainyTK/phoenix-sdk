#![cfg(target_arch = "wasm32")]

use crate::core::{PhoenixMarket, OrderbookManager};
use crate::types::{Orderbook, UnsubscribeResponse};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::rc::Rc;

// Set up panic hook for better error messages
fn setup_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// WASM bindings for the Phoenix SDK
#[wasm_bindgen]
pub struct PhoenixSDK {
    market: PhoenixMarket,
    ws_manager: Rc<RefCell<OrderbookManager>>,
    callback_references: Rc<RefCell<HashMap<String, js_sys::Function>>>,
}

#[wasm_bindgen]
impl PhoenixSDK {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        setup_panic_hook();
        
        // Default URLs - in a real implementation, these would be configurable
        let api_base_url = "https://api.mainnet-beta.solana.com";
        let ws_url = "wss://api.mainnet-beta.solana.com";
        
        PhoenixSDK {
            market: PhoenixMarket::new(api_base_url),
            ws_manager: Rc::new(RefCell::new(OrderbookManager::new(ws_url))),
            callback_references: Rc::new(RefCell::new(HashMap::new())),
        }
    }
    
    #[wasm_bindgen(js_name = fetchMarketSymbols)]
    pub async fn fetch_market_symbols(&self) -> Result<JsValue, JsValue> {
        // Call the core Rust implementation
        match self.market.fetch_market_symbols() {
            Ok(symbols) => Ok(serde_wasm_bindgen::to_value(&symbols)?),
            Err(e) => Err(JsValue::from_str(&e)),
        }
    }
    
    #[wasm_bindgen(js_name = fetchOrderBooks)]
    pub async fn fetch_order_books(&self, symbols: Vec<String>, limit: Option<u32>) -> Result<JsValue, JsValue> {
        // Call the core Rust implementation
        match self.market.fetch_order_books(&symbols, limit) {
            Ok(orderbooks) => Ok(serde_wasm_bindgen::to_value(&orderbooks)?),
            Err(e) => Err(JsValue::from_str(&e)),
        }
    }
    
    #[wasm_bindgen(js_name = subscribeOrderBooks)]
    pub fn subscribe_order_books(&self, symbols: Vec<String>, callback: &js_sys::Function) -> Result<String, JsValue> {
        if symbols.is_empty() {
            return Err(JsValue::from_str("At least one symbol must be provided"));
        }
        
        let symbol = &symbols[0]; // Just using the first symbol for demo
        
        // Store the JS callback for later use
        let callback_clone = callback.clone();
        self.callback_references.borrow_mut().insert(symbol.clone(), callback_clone);
        
        // Create a Rust callback that will invoke the JS callback
        let js_callback = self.callback_references.borrow().get(symbol).unwrap().clone();
        let callback_fn = Box::new(move |orderbook: &Orderbook| {
            let orderbook_js = match serde_wasm_bindgen::to_value(orderbook) {
                Ok(val) => val,
                Err(_) => JsValue::NULL,
            };
            let _ = js_callback.call1(&JsValue::NULL, &orderbook_js);
        });
        
        // Subscribe using the core Rust implementation
        match self.ws_manager.borrow_mut().subscribe(symbol, callback_fn) {
            Ok(subscription_id) => Ok(subscription_id),
            Err(e) => Err(JsValue::from_str(&e)),
        }
    }
    
    #[wasm_bindgen(js_name = unsubscribeOrderBooks)]
    pub fn unsubscribe_order_books(&self, subscription_id: String) -> Result<JsValue, JsValue> {
        // Unsubscribe using the core Rust implementation
        match self.ws_manager.borrow_mut().unsubscribe(&subscription_id) {
            Ok(success) => {
                let response = UnsubscribeResponse { success };
                Ok(serde_wasm_bindgen::to_value(&response)?)
            },
            Err(e) => Err(JsValue::from_str(&e)),
        }
    }
    
    // For demo/testing - simulate an orderbook update
    #[wasm_bindgen(js_name = simulateUpdate)]
    pub fn simulate_update(&self, symbol: String) -> Result<(), JsValue> {
        match self.ws_manager.borrow().simulate_update(&symbol) {
            Ok(_) => Ok(()),
            Err(e) => Err(JsValue::from_str(&e)),
        }
    }
}

// Helper to convert between JavaScript and Rust types
use std::collections::HashMap; 