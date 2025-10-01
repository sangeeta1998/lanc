#!/bin/bash

echo "🎬 Trust Monitoring System Demo"
echo "🎯 SCULI-Aligned Trust Assessment Framework"
echo

# Check if the system is built
if [ ! -f "target/debug/trust-monitoring-system" ]; then
    echo "🔨 Building the system..."
    cargo build
    if [ $? -ne 0 ]; then
        echo "❌ Build failed"
        exit 1
    fi
fi

echo "🚀 Starting Trust Monitoring System in background..."

# Start the system in the background
cargo run &
SYSTEM_PID=$!

echo "✅ System started with PID: $SYSTEM_PID"
echo "🌐 API available at: http://localhost:3030"
echo

# Wait for system to start
echo "⏳ Waiting for system to initialize..."
sleep 3

# Check if system is running
if ! kill -0 $SYSTEM_PID 2>/dev/null; then
    echo "❌ System failed to start"
    exit 1
fi

echo "✅ System is running!"
echo

# Demo the API endpoints
echo "📊 Demo: Checking system status..."
curl -s http://localhost:3030/status | jq '.' 2>/dev/null || curl -s http://localhost:3030/status
echo
echo

echo "📊 Demo: Checking trust scores..."
curl -s http://localhost:3030/trust-scores | jq '.' 2>/dev/null || curl -s http://localhost:3030/trust-scores
echo
echo

echo "📊 Demo: Checking alerts..."
curl -s http://localhost:3030/alerts | jq '.' 2>/dev/null || curl -s http://localhost:3030/alerts
echo
echo

echo "⚠️  Demo: Simulating trust degradation..."
curl -s -X POST http://localhost:3030/simulate-degradation | jq '.' 2>/dev/null || curl -s -X POST http://localhost:3030/simulate-degradation
echo
echo

echo "📊 Demo: Checking trust scores after degradation..."
curl -s http://localhost:3030/trust-scores | jq '.' 2>/dev/null || curl -s http://localhost:3030/trust-scores
echo
echo

echo "📊 Demo: Checking alerts after degradation..."
curl -s http://localhost:3030/alerts | jq '.' 2>/dev/null || curl -s http://localhost:3030/alerts
echo
echo

echo "🔧 Demo: Simulating recovery..."
curl -s -X POST http://localhost:3030/simulate-recovery | jq '.' 2>/dev/null || curl -s -X POST http://localhost:3030/simulate-recovery
echo
echo

echo "📊 Demo: Checking trust scores after recovery..."
curl -s http://localhost:3030/trust-scores | jq '.' 2>/dev/null || curl -s http://localhost:3030/trust-scores
echo
echo

echo "📊 Demo: Final system status..."
curl -s http://localhost:3030/status | jq '.' 2>/dev/null || curl -s http://localhost:3030/status
echo
echo

echo "🎉 Demo completed successfully!"
echo
echo "📋 Available endpoints:"
echo "   GET /status - System status"
echo "   GET /trust-scores - Current trust scores"
echo "   GET /incidents - Active incidents"
echo "   GET /alerts - Active alerts"
echo "   POST /simulate-degradation - Simulate trust degradation"
echo "   POST /simulate-recovery - Simulate recovery"
echo

# Keep system running
echo "🔄 System is running in the background..."
echo "Press Ctrl+C to stop the system"
echo

# Wait for user to stop
trap "echo; echo '🛑 Stopping Trust Monitoring System...'; kill $SYSTEM_PID 2>/dev/null; echo '✅ System stopped'; exit 0" INT

# Keep running until interrupted
while true; do
    sleep 1
done
