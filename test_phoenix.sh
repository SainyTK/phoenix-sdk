#!/bin/bash

# Phoenix DEX API Integration Test Script
# This script provides easy commands to test the Phoenix integration

set -e

echo "🔥 Phoenix DEX API Integration Test Runner"
echo "=========================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_help() {
    echo "Usage: $0 [command]"
    echo ""
    echo "Commands:"
    echo "  test-mainnet    Test Phoenix API integration on Mainnet"
    echo "  test-devnet     Test Phoenix API integration on Devnet"
    echo "  build           Build the project"
    echo "  quick-test      Quick test with basic functionality"
    echo "  help            Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 test-mainnet"
    echo "  $0 test-devnet"
    echo "  $0 build"
    echo ""
}

build_project() {
    echo -e "${YELLOW}Building project...${NC}"
    cargo build --release
    echo -e "${GREEN}✅ Build completed successfully!${NC}"
}

test_mainnet() {
    echo -e "${YELLOW}🌐 Testing Phoenix API integration on Mainnet...${NC}"
    echo "This will test against real Solana Mainnet (read-only operations)"
    
    # Build first
    build_project
    
    # Run the test
    echo -e "${YELLOW}Running Phoenix API tests...${NC}"
    RUST_LOG=info cargo run --bin test_phoenix
    
    echo -e "${GREEN}✅ Mainnet tests completed!${NC}"
}

test_devnet() {
    echo -e "${YELLOW}🧪 Testing Phoenix API integration on Devnet...${NC}"
    echo "This will test against Solana Devnet (read-only operations)"
    
    # Build first
    build_project
    
    # Run the test with devnet flag
    echo -e "${YELLOW}Running Phoenix API tests on Devnet...${NC}"
    RUST_LOG=info PHOENIX_USE_DEVNET=1 cargo run --bin test_phoenix
    
    echo -e "${GREEN}✅ Devnet tests completed!${NC}"
}

quick_test() {
    echo -e "${YELLOW}⚡ Running quick test...${NC}"
    echo "This will build and run a basic test to verify everything is working"
    
    # Build first
    build_project
    
    # Run a quick test
    echo -e "${YELLOW}Running quick functionality test...${NC}"
    RUST_LOG=warn cargo run --bin test_phoenix
    
    echo -e "${GREEN}✅ Quick test completed!${NC}"
}

# Main command handling
case "${1:-help}" in
    "test-mainnet")
        test_mainnet
        ;;
    "test-devnet")
        test_devnet
        ;;
    "build")
        build_project
        ;;
    "quick-test")
        quick_test
        ;;
    "help"|"--help"|"-h")
        print_help
        ;;
    *)
        echo -e "${RED}❌ Unknown command: $1${NC}"
        echo ""
        print_help
        exit 1
        ;;
esac

echo ""
echo -e "${GREEN}🎉 Phoenix API integration test completed!${NC}"
echo ""
echo "📝 Notes:"
echo "- This implementation uses sample data for demonstration"
echo "- Real Phoenix DEX integration would require the actual Phoenix SDK"
echo "- The orderbook data is generated for testing purposes"
echo "- All operations are read-only and safe to run"
echo ""
echo "🔗 For more information:"
echo "- Phoenix DEX: https://phoenix.trade"
echo "- Solana: https://solana.com"
echo "- Phoenix SDK: https://github.com/Ellipsis-Labs/phoenix-v1"