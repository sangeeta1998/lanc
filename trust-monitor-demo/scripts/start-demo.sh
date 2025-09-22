#!/bin/bash

echo "🚀 Starting Trust Monitor Demo for SCULI Interview"
echo "=================================================="

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Please install Rust first."
    echo "   Visit: https://rustup.rs/"
    exit 1
fi

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "❌ Docker not found. Please install Docker first."
    exit 1
fi

echo "✅ Prerequisites check passed"

# Build the trust monitor
echo "🔨 Building trust monitor..."
cd "$(dirname "$0")/.."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    exit 1
fi

echo "✅ Build successful"

# Start some demo containers
echo "🐳 Starting demo containers..."
docker run -d --name demo-container-a --rm alpine:latest sleep 3600
docker run -d --name demo-container-b --rm alpine:latest sleep 3600
docker run -d --name demo-container-c --rm alpine:latest sleep 3600

echo "✅ Demo containers started"

# Start the trust monitor
echo "🛡️ Starting trust monitor server..."
echo "   Dashboard will be available at: http://localhost:8080"
echo "   API endpoints:"
echo "   - GET /api/trust - Trust metrics"
echo "   - GET /api/nodes - Node information"
echo ""
echo "Press Ctrl+C to stop the demo"

./target/release/trust-monitor
