# Phoenix SDK

A dual-purpose SDK for accessing Phoenix market data and orderbook information:
- Pure Rust implementation for Rust applications
- WebAssembly (WASM) bindings for JavaScript/TypeScript applications

## Architecture

This SDK is designed with a clean separation between core functionality and WebAssembly bindings:

```
src/
├── core/           # Pure Rust implementation 
│   ├── market.rs   # Market data functionality
│   └── orderbook.rs # Orderbook management
├── types/          # Shared data types
├── wasm/           # WebAssembly bindings (compiled only for wasm32 target)
│   └── bindings.rs # JavaScript interop layer
└── lib.rs          # Entry point
```

### Benefits of this Architecture

1. **Clean separation of concerns**:
   - Core business logic is implemented in pure Rust
   - WASM binding code is isolated in a separate module

2. **Multiple usage patterns**:
   - Use directly from Rust applications
   - Use via WebAssembly in JavaScript/TypeScript applications

3. **Conditional compilation**:
   - WASM-specific code only compiles when targeting wasm32
   - No WASM dependencies in pure Rust builds

4. **Better testability**:
   - Core functionality can be tested with standard Rust testing tools
   - No need for wasm-bindgen-test for core logic

## Usage

### In Rust Projects

```rust
use phoenix_sdk::{PhoenixMarket, OrderbookManager};

fn main() {
    // Create a new Phoenix market client
    let market = PhoenixMarket::default();
    
    // Fetch market symbols
    let symbols = market.fetch_market_symbols().unwrap();
    println!("Available markets: {:?}", symbols);
    
    // Fetch orderbooks
    let orderbooks = market.fetch_order_books(
        &[String::from("SOL_USDC")], 
        Some(10)
    ).unwrap();
    println!("Orderbook: {:?}", orderbooks);
}
```

### In JavaScript/TypeScript Projects

```javascript
import { PhoenixSDK } from 'phoenix-sdk';

// Create a new SDK instance
const sdk = new PhoenixSDK();

// Fetch market symbols
const markets = await sdk.fetchMarketSymbols();
console.log('Available markets:', markets);

// Fetch orderbook data
const orderbooks = await sdk.fetchOrderBooks(['SOL_USDC'], 10);
console.log('Orderbook:', orderbooks);

// Subscribe to orderbook updates
const subscriptionId = sdk.subscribeOrderBooks(['SOL_USDC'], (orderbook) => {
  console.log('Orderbook update:', orderbook);
});

// Later, unsubscribe
sdk.unsubscribeOrderBooks(subscriptionId);
```

## Building

### For Rust

```
cargo build --release
```

### For WebAssembly

```
wasm-pack build --target web
```

## 🚀 Features

- 📈 **Market Data Access**: Fetch available trading pairs and market information
- 📊 **Orderbook Data**: Get current bid/ask spreads with customizable depth
- ⚡ **Real-time Streaming**: WebSocket-based live orderbook updates
- 🦀 **Rust Performance**: Built with Rust and compiled to WebAssembly for speed
- 📦 **TypeScript Ready**: Full TypeScript support with auto-generated type definitions
- 🪶 **Lightweight**: Minimal bundle size impact for web applications

## 📦 Installation

```bash
npm install @arbit-x/phoenix-sdk
```

Or with yarn:

```bash
yarn add @arbit-x/phoenix-sdk
```

## 🔧 Usage

### 1. Initialize the SDK

```javascript
import { PhoenixSDK } from '@arbit-x/phoenix-sdk';

const phoenixSDK = new PhoenixSDK();
```

### 2. Fetch Market Symbols

```javascript
const marketSymbols = await phoenixSDK.fetchMarketSymbols();
console.log(marketSymbols);
/*
Output example:
[
  { id: 1, symbol: "BTC_USDC", info: "{}" },
  { id: 2, symbol: "ETH_USDC", info: "{}" }
]
*/
```

### 3. Fetch Orderbooks

```javascript
const orderbooks = await phoenixSDK.fetchOrderBooks(["BTC_USDC"], 10); // limit = 10
console.log(orderbooks);
/*
Output example:
{
  "BTC_USDC": {
    bids: [{price: 100, size: 1}, {price: 99, size: 2}],
    asks: [{price: 101, size: 1}, {price: 102, size: 2}]
  }
}
*/
```

### 4. Subscribe to Real-time Orderbook Updates

```javascript
const subscriptionId = await phoenixSDK.subscribeOrderBooks(["BTC_USDC"], (orderbooks) => {
  console.log(orderbooks);
  /*
  Output example:
  {
    "BTC_USDC": {
      bids: [{price: 100, size: 1}, {price: 99, size: 2}],
      asks: [{price: 101, size: 1}, {price: 102, size: 2}]
    }
  }
  */
});
```

### 5. Unsubscribe from Updates

```javascript
await phoenixSDK.unsubscribeOrderBooks(subscriptionId);
```

## 📚 API Reference

### `PhoenixSDK`

#### Constructor
- `new PhoenixSDK()`: Creates a new instance of the SDK

#### Methods

##### `fetchMarketSymbols(): Promise<MarketSymbol[]>`
Fetches all available market symbols.

**Returns**: Array of market symbol objects

##### `fetchOrderBooks(symbols: string[], limit?: number): Promise<OrderbooksResponse>`
Fetches current orderbook data for specified symbols.

**Parameters**:
- `symbols`: Array of market symbols (e.g., `["BTC_USDC", "ETH_USDC"]`)
- `limit`: Optional. Maximum number of bid/ask entries to return

**Returns**: Object with symbol keys containing orderbook data

##### `subscribeOrderBooks(symbols: string[], callback: (data: OrderbooksResponse) => void): Promise<string>`
Subscribes to real-time orderbook updates via WebSocket.

**Parameters**:
- `symbols`: Array of market symbols to subscribe to
- `callback`: Function called when orderbook data is updated

**Returns**: Subscription ID for later unsubscription

##### `unsubscribeOrderBooks(subscriptionId: string): Promise<void>`
Unsubscribes from real-time orderbook updates.

**Parameters**:
- `subscriptionId`: The subscription ID returned from `subscribeOrderBooks`

## 🛠️ Development

### Prerequisites

- [Node.js](https://nodejs.org/) (v16 or higher)
- [Rust](https://rustup.rs/) (latest stable)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Setup

1. Clone the repository:
```bash
git clone https://github.com/SainyTK/phoenix-sdk.git
cd phoenix-sdk
```

2. Build (WASM):
```bash
wasm-pack build --target bundler --out-dit pkg
```

3. Build the project:
```bash
npm run build
```

4. Run tests:
```bash
npm test
```

### Development Commands

- `npm run build`: Build the Rust code to WebAssembly and generate TypeScript bindings
- `npm run dev`: Start development mode with file watching
- `npm test`: Run the test suite
- `npm run lint`: Lint the codebase
- `npm run docs`: Generate documentation

### Project Structure

```
phoenix-sdk/
├── src/                 # Rust source code
│   ├── lib.rs          # Main library entry point
│   ├── sdk.rs          # SDK implementation
│   └── types.rs        # Type definitions
├── pkg/                # Generated WebAssembly output
├── tests/              # Test files
├── examples/           # Usage examples
├── Cargo.toml         # Rust dependencies
├── package.json       # Node.js dependencies
└── webpack.config.js  # Build configuration
```

## 🤝 Contributing

We welcome contributions! Please follow these steps:

1. **Fork the repository** on GitHub
2. **Create a feature branch**: `git checkout -b feature/your-feature-name`
3. **Make your changes** and ensure they follow our coding standards:
   - Rust code should follow `snake_case` conventions
   - Use `#[wasm_bindgen(js_name = "camelCase")]` for external APIs
   - Add tests for new functionality
   - Update documentation as needed
4. **Run tests**: `npm test`
5. **Commit your changes**: `git commit -m "feat: add your feature description"`
6. **Push to your fork**: `git push origin feature/your-feature-name`
7. **Create a Pull Request** on GitHub

### Code Style

- **Rust**: Follow standard Rust formatting with `cargo fmt`
- **JavaScript/TypeScript**: Use ESLint and Prettier configurations
- **Commit Messages**: Follow [Conventional Commits](https://conventionalcommits.org/)

### Reporting Issues

If you find a bug or have a feature request, please [open an issue](https://github.com/SainyTK/phoenix-sdk/issues) on GitHub.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [GitHub Repository](https://github.com/SainyTK/phoenix-sdk)
- [npm Package](https://www.npmjs.com/package/@arbit-x/phoenix-sdk)
- [Documentation](https://github.com/SainyTK/phoenix-sdk#readme)

## 🆘 Support

If you need help or have questions:

- 📖 Check the documentation above
- 🐛 [Report bugs](https://github.com/SainyTK/phoenix-sdk/issues)
- 💬 [Start a discussion](https://github.com/SainyTK/phoenix-sdk/discussions)

---

Made with ❤️ using Rust and WebAssembly
