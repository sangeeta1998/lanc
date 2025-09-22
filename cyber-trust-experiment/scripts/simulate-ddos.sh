#!/bin/bash
echo "🎭 Simulating DDoS Attack"
echo "========================"
echo "📊 Current security metrics before attack:"
curl -s http://localhost:8080/api/security | jq '.[] | select(.node_id == "edge-cloud-1") | {node_id, overall_security, status}'

echo ""
echo "🚨 DDoS attack detected..."
echo "   - High volume of suspicious network traffic"
echo "   - Network performance degradation"
echo "   - Load balancing and traffic filtering activated"

echo ""
echo "⏱️  Monitoring security degradation over 30 seconds..."
for i in {1..15}; do
    echo -n "."
    sleep 2
done
echo ""

echo "📊 Security metrics after DDoS attack:"
curl -s http://localhost:8080/api/security | jq '.[] | select(.node_id == "edge-cloud-1") | {node_id, overall_security, status, attack_indicators}'

echo ""
echo "🔍 Check the dashboard at http://localhost:8080 to see:"
echo "   - Network security monitoring"
echo "   - Distributed defense mechanisms"
echo "   - System resilience under load"
echo "   - Automated traffic management"
