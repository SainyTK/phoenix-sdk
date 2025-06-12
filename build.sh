#!/bin/bash
set -e

# Install wasm-pack if not already installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the WebAssembly package
echo "Building WebAssembly package..."
wasm-pack build --target web --out-dir dist

# Copy TypeScript definitions
echo "Copying TypeScript definitions..."
mkdir -p dist
cp typescript/index.d.ts dist/

echo "Build completed successfully!" 