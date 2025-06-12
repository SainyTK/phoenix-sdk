// Type definitions for @arbit-x/phoenix-sdk
// Project: https://github.com/SainyTK/phoenix-sdk
// Definitions by: Arbit-X

export interface MarketSymbol {
  id: number;
  symbol: string;
  info: string;
}

export interface OrderbookEntry {
  price: number;
  size: number;
}

export interface Orderbook {
  bids: OrderbookEntry[];
  asks: OrderbookEntry[];
}

export interface OrderbooksResponse {
  [symbol: string]: Orderbook;
}

export interface SubscriptionResponse {
  subscription_id: string;
}

export interface UnsubscribeResponse {
  success: boolean;
}

export class PhoenixSDK {
  constructor();
  
  /**
   * Fetches available market symbols
   * @returns Promise resolving to an array of market symbols
   */
  fetchMarketSymbols(): Promise<MarketSymbol[]>;
  
  /**
   * Fetches orderbook data for the given symbols
   * @param symbols Array of market symbols (e.g. ["BTC_USDC"])
   * @param limit Optional parameter to limit the number of orderbook entries
   * @returns Promise resolving to an object with symbol keys containing orderbook data
   */
  fetchOrderBooks(symbols: string[], limit?: number): Promise<OrderbooksResponse>;
  
  /**
   * Subscribes to real-time orderbook updates for the given symbols
   * @param symbols Array of market symbols to subscribe to
   * @param callback Function to be called when orderbook updates are received
   * @returns Subscription ID for later unsubscription
   */
  subscribeOrderBooks(symbols: string[], callback: (orderbook: Orderbook) => void): string;
  
  /**
   * Unsubscribes from orderbook updates
   * @param subscriptionId The subscription ID returned from subscribeOrderBooks
   * @returns Promise resolving to an object with success status
   */
  unsubscribeOrderBooks(subscriptionId: string): Promise<UnsubscribeResponse>;
} 