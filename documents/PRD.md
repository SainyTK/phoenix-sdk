# Product Requirements Document (PRD)
## @arbit-x/drift-sdk

### Overview
The drift-sdk is a JavaScript/TypeScript SDK built with Rust and WebAssembly (WASM) that provides access to market data and real-time orderbook information for trading applications. The SDK will be published to npm and used by JavaScript developers to integrate market data functionality into their applications.

### Technical Architecture
- **Core Language**: Rust (for performance and safety)
- **Target Platform**: JavaScript/TypeScript (via WebAssembly)
- **Build Tool**: wasm-bindgen for Rust-to-JS bindings
- **Package Manager**: npm
- **Real-time Communication**: WebSocket for live data streaming

### Functional Requirements

#### FR1: SDK Initialization
- Developers can instantiate the SDK with `new DriftSDK()`
- The constructor should initialize necessary connections and configurations
- Should be lightweight and non-blocking during instantiation

#### FR2: Market Symbol Fetching
- Method: `fetchMarketSymbols()`
- Returns: Array of market objects with structure `{ id: number, symbol: string, info: string }`
- Should fetch available trading pairs from the backend
- Must be asynchronous and return a Promise

#### FR3: Orderbook Data Fetching
- Method: `fetchOrderBooks(symbols: string[], limit?: number)`
- Parameters:
  - `symbols`: Array of market symbols (e.g., ["BTC_USDC"])
  - `limit`: Optional parameter to limit number of orderbook entries (default behavior if not specified)
- Returns: Object with symbol keys containing bid/ask arrays
- Each bid/ask entry: `{ price: number, size: number }`
- Must be asynchronous and return a Promise

#### FR4: Real-time Orderbook Subscription
- Method: `subscribeOrderBooks(symbols: string[], callback: Function)`
- Parameters:
  - `symbols`: Array of market symbols to subscribe to
  - `callback`: Function called when orderbook updates are received
- Returns: Subscription ID for later unsubscription
- Uses WebSocket for real-time data streaming
- Callback receives the same data structure as fetchOrderBooks

#### FR5: Orderbook Unsubscription
- Method: `unsubscribeOrderBooks(subscriptionId: string)`
- Parameter: `subscriptionId` returned from subscribeOrderBooks
- Cleanly terminates the WebSocket subscription
- Must be asynchronous and return a Promise

### Non-Functional Requirements

#### NFR1: Performance
- Minimal bundle size for web applications
- Efficient memory usage in WebAssembly
- Low latency for real-time data updates

#### NFR2: Code Quality
- Follow Rust naming conventions (snake_case) in Rust code
- Use `js_name` attribute in wasm_bindgen for camelCase JavaScript API
- Comprehensive error handling and meaningful error messages
- Type safety through TypeScript definitions

#### NFR3: Developer Experience
- Clear and comprehensive documentation
- Easy installation via npm
- TypeScript support out of the box
- Development environment setup instructions

#### NFR4: Reliability
- Graceful WebSocket reconnection handling
- Proper resource cleanup on unsubscription
- Robust error handling for network failures

### Data Models

#### Market Symbol Object
```typescript
interface MarketSymbol {
  id: number;
  symbol: string;  // Format: "BASE_QUOTE" (e.g., "BTC_USDC")
  info: string;    // JSON string with additional metadata
}
```

#### Orderbook Object
```typescript
interface OrderbookEntry {
  price: number;
  size: number;
}

interface Orderbook {
  bids: OrderbookEntry[];
  asks: OrderbookEntry[];
}

interface OrderbooksResponse {
  [symbol: string]: Orderbook;
}
```

### Success Criteria
1. Package successfully published to npm as @arbit-x/drift-sdk
2. All API methods work as specified in the examples
3. WebSocket subscriptions maintain stable connections
4. TypeScript definitions are automatically generated and accurate
5. Documentation is complete and examples work out of the box
6. Development environment can be set up by following README instructions

### Out of Scope
- Trading/order placement functionality
- Authentication mechanisms
- Historical data retrieval
- Advanced charting capabilities
- Portfolio management features
