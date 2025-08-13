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

# Build the contract library first
echo "🔨 Building contract library..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Library build failed!"
    exit 1
fi

# Build the contract using cargo-contract
echo "🔨 Building contract with cargo-contract..."
cargo contract build --release

if [ $? -eq 0 ]; then
    echo "✅ Contract built successfully!"
    
    # Check for build artifacts in the correct location
    if [ -d "target/ink" ]; then
        echo "📁 Build artifacts are in target/ink/"
        echo "📋 Build artifacts:"
        ls -la target/ink/
    elif [ -d "target/ink/lending_smart_contract" ]; then
        echo "📁 Build artifacts are in target/ink/lending_smart_contract/"
        echo "📋 Build artifacts:"
        ls -la target/ink/lending_smart_contract/
    else
        echo "📁 Build artifacts location:"
        find target/ -name "*.wasm" -o -name "*.json" -o -name "*.contract" 2>/dev/null | head -10
    fi
else
    echo "❌ Contract build failed!"
    echo "💡 Trying alternative build method..."
    
    # Fallback: try building just the library
    echo "🔨 Building library only..."
    cargo build --release --lib
    
    if [ $? -eq 0 ]; then
        echo "✅ Library built successfully!"
        echo "📁 Library artifacts are in target/release/"
        echo "📋 Library artifacts:"
        ls -la target/release/ | grep -E "(\.rlib|\.d|\.so|\.dylib|\.dll)"
    else
        echo "❌ Library build also failed!"
        exit 1
    fi
fi

echo "🎉 Build process completed!" 