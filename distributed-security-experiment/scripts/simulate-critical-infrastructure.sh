#!/bin/bash
echo "🎭 Simulating Critical Infrastructure Attack"
echo "==========================================="
echo "📊 Current security metrics before attack:"
curl -s http://localhost:8080/api/security | jq '.[] | select(.node_id == "energy-grid") | {node_id, overall_security, status, domain}'

echo ""
echo "🚨 Critical infrastructure attack detected..."
echo "   - Coordinated attack on energy grid"
echo "   - High-severity threat to critical systems"
echo "   - Cross-domain impact on dependent systems"

echo ""
echo "⏱️  Monitoring critical infrastructure response over 30 seconds..."
for i in {1..15}; do
    echo -n "."
    sleep 2
done
echo ""

echo "📊 Security metrics after critical infrastructure attack:"
curl -s http://localhost:8080/api/security | jq '.[] | select(.node_id == "energy-grid") | {node_id, overall_security, status, attack_indicators}'

echo ""
echo "🔍 Check the dashboard at http://localhost:8080 to see:"
echo "   - Critical infrastructure protection"
echo "   - High-severity threat response"
echo "   - Cross-domain impact assessment"
echo "   - Coordinated incident response"
