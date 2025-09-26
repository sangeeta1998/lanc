#!/bin/bash

echo "🚀 Starting Distributed Security Experiment"
echo "==========================================="

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
    echo "   - Legacy Vulnerability: ./scripts/simulate-legacy-vulnerability.sh"
    echo "   - Cross-Domain Attack: ./scripts/simulate-cross-domain-attack.sh"
    echo "   - Critical Infrastructure: ./scripts/simulate-critical-infrastructure.sh"
    exit 0
fi

# Start the distributed security experiment
echo "🌐 Starting distributed security experiment server..."
echo "   Dashboard will be available at: http://localhost:8080"
echo "   API endpoints:"
echo "   - GET /api/security - Security metrics"
echo "   - GET /api/incidents - Active incidents"
echo "   - GET /api/topology - System topology"
echo ""
echo "🎭 Available attack scenarios:"
echo "   - Supply Chain: ./scripts/simulate-supply-chain.sh"
echo "   - Legacy Vulnerability: ./scripts/simulate-legacy-vulnerability.sh"
echo "   - Cross-Domain Attack: ./scripts/simulate-cross-domain-attack.sh"
echo "   - Critical Infrastructure: ./scripts/simulate-critical-infrastructure.sh"
echo ""
echo "🌐 This experiment demonstrates:"
echo "   - Converged digital infrastructure security"
echo "   - Cross-domain threat monitoring and response"
echo "   - Supply chain security in cyber-physical systems"
echo "   - Legacy system integration challenges"
echo "   - Zero-trust architecture implementation"
echo ""
echo "Press Ctrl+C to stop the experiment"

cd "$(dirname "$0")/.."
./target/release/distributed-security-experiment
