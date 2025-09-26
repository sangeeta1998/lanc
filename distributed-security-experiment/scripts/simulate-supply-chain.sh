#!/bin/bash
echo "🎭 Simulating Supply Chain Compromise in Smart City"
echo "=================================================="
echo "📊 Current security metrics before attack:"
curl -s http://localhost:8080/api/security | jq '.[] | select(.node_id == "smart-city-traffic") | {node_id, overall_security, status, domain}'

echo ""
echo "🚨 Injecting compromised third-party component..."
echo "   - Malicious code detected in traffic control software"
echo "   - Supply chain trust degrading..."
echo "   - Cross-domain propagation risk detected"

echo ""
echo "⏱️  Monitoring security degradation over 30 seconds..."
for i in {1..15}; do
    echo -n "."
    sleep 2
done
echo ""

echo "📊 Security metrics after supply chain attack:"
curl -s http://localhost:8080/api/security | jq '.[] | select(.node_id == "smart-city-traffic") | {node_id, overall_security, status, attack_indicators}'

echo ""
echo "🔍 Check the dashboard at http://localhost:8080 to see:"
echo "   - Real-time security score changes"
echo "   - Supply chain component monitoring"
echo "   - Cross-domain threat propagation"
echo "   - Automated incident response"
