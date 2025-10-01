#!/bin/bash

echo "🎬 SCULI Trust Monitoring Demo"
echo "🎯 Ultra-Large Scale Distributed System Trust Assessment"
echo

# Start the trust monitoring system
echo "🚀 Starting Trust Monitoring System..."
./target/release/trust-monitor &
MONITOR_PID=$!

echo "✅ Trust Monitor started with PID: $MONITOR_PID"
echo "🌐 API available at: http://localhost:3030"
echo

# Wait for system to start
echo "⏳ Waiting for system to initialize..."
sleep 3

# Check if system is running
if ! kill -0 $MONITOR_PID 2>/dev/null; then
    echo "❌ Trust Monitor failed to start"
    exit 1
fi

# Start web dashboard
echo "🌐 Starting web dashboard..."
./scripts/start-web.sh &
WEB_PID=$!

echo "✅ Web dashboard started with PID: $WEB_PID"
echo "🌐 Dashboard available at: http://localhost:8080"
echo

# Wait for web server to start
sleep 2

echo "🎉 Demo is ready!"
echo
echo "📋 Available interfaces:"
echo "   🌐 Web Dashboard: http://localhost:8080"
echo "   🔌 API Endpoints: http://localhost:3030"
echo
echo "📊 API Endpoints:"
echo "   GET /status - System status with SCULI objectives"
echo "   GET /trust-scores - Current trust scores"
echo "   GET /alerts - Active alerts"
echo "   POST /simulate-degradation - Simulate trust degradation"
echo "   POST /simulate-recovery - Simulate recovery"
echo
echo "🎮 Demo Controls:"
echo "   1. Open http://localhost:8080 in your browser"
echo "   2. Click 'Simulate Degradation' to see trust scores drop"
echo "   3. Click 'Simulate Recovery' to restore trust scores"
echo "   4. Watch the real-time dashboard update"
echo
echo "Press Ctrl+C to stop the demo"

# Keep running until interrupted
trap "echo; echo '🛑 Stopping SCULI Trust Monitoring Demo...'; kill $MONITOR_PID $WEB_PID 2>/dev/null; echo '✅ Demo stopped'; exit 0" INT

while true; do
    sleep 1
done
