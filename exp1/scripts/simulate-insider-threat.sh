#!/bin/bash
echo "🎭 Simulating Insider Threat"
echo "==========================="
echo "📊 Current security metrics before attack:"
curl -s http://localhost:8080/api/security | jq '.[] | select(.node_id == "legacy-1") | {node_id, overall_security, status}'

echo ""
echo "🚨 Insider threat detected..."
echo "   - Unusual user behavior patterns"
echo "   - Access to sensitive data outside normal hours"
echo "   - Behavioral analysis flagging suspicious activity"

echo ""
echo "⏱️  Monitoring security degradation over 30 seconds..."
for i in {1..15}; do
    echo -n "."
    sleep 2
done
echo ""

echo "📊 Security metrics after insider threat:"
curl -s http://localhost:8080/api/security | jq '.[] | select(.node_id == "legacy-1") | {node_id, overall_security, status, attack_indicators}'

echo ""
echo "🔍 Check the dashboard at http://localhost:8080 to see:"
echo "   - Behavioral analysis in action"
echo "   - User activity monitoring"
echo "   - Gradual trust degradation"
echo "   - Insider threat detection"
