#!/bin/bash

echo "🎬 Trust Monitoring System Demo"
echo "🎯 SCULI-Aligned Trust Assessment Framework"
echo

# Check if the system is built
if [ ! -f "target/release/trust-monitoring-system" ]; then
    echo "❌ System not built. Running setup first..."
    ./scripts/setup.sh
fi

echo "🚀 Starting Trust Monitoring System..."

# Start the system in the background
./target/release/trust-monitoring-system &
SYSTEM_PID=$!

echo "✅ System started with PID: $SYSTEM_PID"
echo "🌐 API available at: http://localhost:3030"
echo

# Wait for system to start
echo "⏳ Waiting for system to initialize..."
sleep 5

# Check if system is running
if ! kill -0 $SYSTEM_PID 2>/dev/null; then
    echo "❌ System failed to start"
    exit 1
fi

echo "✅ System is running!"
echo

# Run demo scenario
echo "🎬 Running demo scenario..."
cargo run --example demo_scenario

echo
echo "📊 Demo completed! Check the following endpoints:"
echo "   http://localhost:3030/status - System status"
echo "   http://localhost:3030/trust-scores - Current trust scores"
echo "   http://localhost:3030/incidents - Active incidents"
echo "   http://localhost:3030/alerts - Active alerts"
echo

# Keep system running
echo "🔄 System is running in the background..."
echo "Press Ctrl+C to stop the system"
echo

# Wait for user to stop
trap "echo; echo '🛑 Stopping Trust Monitoring System...'; kill $SYSTEM_PID; echo '✅ System stopped'; exit 0" INT

# Keep running until interrupted
while true; do
    sleep 1
done
