#!/bin/bash

# Build script for Lending Smart Contract

echo "🚀 Building Lending Smart Contract..."

# Check if cargo-contract is installed
if ! command -v cargo-contract &> /dev/null; then
    echo "❌ cargo-contract not found. Installing..."
    cargo install cargo-contract --locked --git https://github.com/paritytech/cargo-contract
fi

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean

# Build the contract
echo "🔨 Building contract..."
cargo contract build

if [ $? -eq 0 ]; then
    echo "✅ Contract built successfully!"
    echo "📁 Build artifacts are in target/ink/"
    
    # List build artifacts
    echo "📋 Build artifacts:"
    ls -la target/ink/
else
    echo "❌ Build failed!"
    exit 1
fi

echo "🎉 Build process completed!" 