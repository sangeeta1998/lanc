#!/bin/bash

echo "🚀 Starting Cyber-Trust Experiment"
echo "=================================="

# Check if the experiment is already running
if curl -s http://localhost:8080/api/security > /dev/null 2>&1; then
    echo "⚠️  Experiment is already running on http://localhost:8080"
    echo "   Dashboard: http://localhost:8080"
    echo "   API endpoints:"
    echo "   - GET /api/security - Security metrics"
    echo "   - GET /api/incidents - Active incidents"
    echo "   - GET /api/topology - System topology"
    echo ""
    echo "🎭 Available attack scenarios:"
    echo "   - Supply Chain: ./scripts/simulate-supply-chain.sh"
    echo "   - Zero-Day: ./scripts/simulate-zero-day.sh"
    echo "   - DDoS: ./scripts/simulate-ddos.sh"
    echo "   - Insider Threat: ./scripts/simulate-insider-threat.sh"
    exit 0
fi

# Start the cyber-trust experiment
echo "🛡️ Starting cyber-trust experiment server..."
echo "   Dashboard will be available at: http://localhost:8080"
echo "   API endpoints:"
echo "   - GET /api/security - Security metrics"
echo "   - GET /api/incidents - Active incidents"
echo "   - GET /api/topology - System topology"
echo ""
echo "🎭 Available attack scenarios:"
echo "   - Supply Chain: ./scripts/simulate-supply-chain.sh"
echo "   - Zero-Day: ./scripts/simulate-zero-day.sh"
echo "   - DDoS: ./scripts/simulate-ddos.sh"
echo "   - Insider Threat: ./scripts/simulate-insider-threat.sh"
echo ""
echo "Press Ctrl+C to stop the experiment"

cd "$(dirname "$0")/.."
./target/release/cyber-trust-experiment
