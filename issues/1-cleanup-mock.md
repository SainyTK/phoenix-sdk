# ISSUE 1: Cleanup Mock Implementation

## Background
The current SDK implementation contains mock code and a mock mode, which are no longer necessary. To streamline the codebase and ensure maintainability, we will remove all mock-related logic and focus solely on mainnet (real data) support.

## Objective
- Remove all mock and devnet code/modes from the SDK.
- Consolidate real data logic for clarity and maintainability.
- Ensure all modules fetch and format data consistently for SDK consumers.

## Scope & Tasks

### 1. `core/phoenix_api.rs`
- [ ] Remove support for devnet and mock mode; only mainnet should remain.
- [ ] Eliminate the `real_data` option and any related logic.
- [ ] Remove all mock functions and hardcoded/mock data.
- [ ] Move any remaining hardcoded data to appropriate files/modules for better organization.
- [ ] Refactor to ensure only real data is used throughout.

### 2. `core/phoenix_real.rs`
- [ ] Merge all functions and logic from this file into `core/phoenix_api.rs`.
- [ ] Remove this file after merging and updating references.

### 3. `core/market.rs`
- [ ] Refactor all functions to fetch market data exclusively via `core/phoenix_api.rs`.
- [ ] Ensure all raw data from `core/phoenix_api.rs` is formatted for compatibility with `sdk.rs`.

### 4. `core/orderbook.rs`
- [ ] Refactor all functions to fetch orderbook data exclusively via `core/phoenix_api.rs`.
- [ ] Ensure all raw data from `core/phoenix_api.rs` is formatted for compatibility with `sdk.rs`.

### 5. `core/sdk.rs`
- [ ] Remove support for devnet and mock mode; only mainnet should remain.
- [ ] Update market-related functions to use `core/market.rs` for data fetching.
- [ ] Update orderbook-related functions to use `core/orderbook.rs` for data fetching.

## Acceptance Criteria
- No references to mock or devnet modes remain in the codebase.
- All data is fetched from mainnet sources via `core/phoenix_api.rs`.
- Code is organized, with no redundant or hardcoded mock data.
- All modules (`market.rs`, `orderbook.rs`, `sdk.rs`) interact with `phoenix_api.rs` as the single source of truth for data.
- All tests pass and documentation is updated to reflect these changes.

## Rationale
Removing mock and devnet logic will:
- Simplify the codebase, making it easier to maintain and extend.
- Reduce confusion for developers and users of the SDK.
- Ensure consistency and reliability by always using real, mainnet data. 