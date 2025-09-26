#!/bin/bash
echo "🎭 Simulating Cross-Domain Attack Propagation"
echo "============================================="
echo "📊 Current security metrics before attack:"
curl -s http://localhost:8080/api/security | jq '.[] | select(.node_id == "transport-autonomous") | {node_id, overall_security, status, domain}'

echo ""
echo "🚨 Cross-domain attack detected..."
echo "   - Attack propagating from Smart City to Transportation"
echo "   - Cross-domain trust relationships compromised"
echo "   - Coordinated response required across domains"

echo ""
echo "⏱️  Monitoring cross-domain propagation over 30 seconds..."
for i in {1..15}; do
    echo -n "."
    sleep 2
done
echo ""

echo "📊 Security metrics after cross-domain attack:"
curl -s http://localhost:8080/api/security | jq '.[] | select(.node_id == "transport-autonomous") | {node_id, overall_security, status, attack_indicators}'

echo ""
echo "🔍 Check the dashboard at http://localhost:8080 to see:"
echo "   - Cross-domain attack propagation"
echo "   - Multi-domain security coordination"
echo "   - Converged infrastructure monitoring"
echo "   - Automated cross-domain response"
