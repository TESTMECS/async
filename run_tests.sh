#!/bin/bash

echo "🦀 Running std_async test suite..."
echo "=================================="

echo ""
echo "📝 Running unit tests..."
cargo test --lib

echo ""
echo "🔗 Running integration tests..."
cargo test --test integration_tests

echo ""
echo "⚡ Running benchmark tests..."
cargo test --test benchmark_tests

echo ""
echo "✅ All tests completed!"
