#!/bin/bash
echo "🎭 Simulating Supply Chain Compromise"
echo "===================================="
echo "📊 Current security metrics before attack:"
curl -s http://localhost:8080/api/security | jq '.[] | select(.node_id == "smart-city-1") | {node_id, overall_security, status}'

echo ""
echo "🚨 Injecting compromised software component..."
echo "   - Malicious code detected in traffic control system"
echo "   - Trust score degrading..."
echo "   - Behavioral anomalies detected"

echo ""
echo "⏱️  Monitoring security degradation over 30 seconds..."
for i in {1..15}; do
    echo -n "."
    sleep 2
done
echo ""

echo "📊 Security metrics after supply chain attack:"
curl -s http://localhost:8080/api/security | jq '.[] | select(.node_id == "smart-city-1") | {node_id, overall_security, status, attack_indicators}'

echo ""
echo "🔍 Check the dashboard at http://localhost:8080 to see:"
echo "   - Real-time security score changes"
echo "   - Attack indicators and threat detection"
echo "   - Automated incident response"
echo "   - System-wide impact visualization"
