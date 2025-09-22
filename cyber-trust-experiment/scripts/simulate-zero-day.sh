#!/bin/bash
echo "🎭 Simulating Zero-Day Exploit"
echo "============================="
echo "📊 Current security metrics before attack:"
curl -s http://localhost:8080/api/security | jq '.[] | select(.node_id == "industrial-1") | {node_id, overall_security, status}'

echo ""
echo "🚨 Zero-day exploit detected..."
echo "   - Unknown attack pattern identified"
echo "   - ML model triggered anomaly detection"
echo "   - Behavioral analysis showing suspicious activity"

echo ""
echo "⏱️  Monitoring security degradation over 30 seconds..."
for i in {1..15}; do
    echo -n "."
    sleep 2
done
echo ""

echo "📊 Security metrics after zero-day attack:"
curl -s http://localhost:8080/api/security | jq '.[] | select(.node_id == "industrial-1") | {node_id, overall_security, status, attack_indicators}'

echo ""
echo "🔍 Check the dashboard at http://localhost:8080 to see:"
echo "   - ML-based threat detection in action"
echo "   - Real-time behavioral analysis"
echo "   - Automated response to unknown threats"
echo "   - System resilience under attack"
