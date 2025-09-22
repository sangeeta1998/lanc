#!/bin/bash

echo "🎭 Simulating Container Compromise Scenario"
echo "==========================================="

# This script simulates a compromised container scenario
# In a real implementation, this would trigger actual security events

echo "📊 Current trust metrics before compromise:"
curl -s http://localhost:8080/api/trust | jq '.[] | select(.container_id == "container-b") | {container_id, overall_trust, status}'

echo ""
echo "🚨 Simulating compromise of container-b..."
echo "   - Injecting suspicious network activity"
echo "   - Simulating resource abuse"
echo "   - Triggering behavioral anomaly detection"

# In a real system, this would:
# 1. Modify container behavior
# 2. Update trust metrics
# 3. Trigger isolation mechanisms
# 4. Send alerts

echo ""
echo "⏱️  Monitoring trust degradation over 30 seconds..."
for i in {1..15}; do
    echo -n "."
    sleep 2
done
echo ""

echo "📊 Trust metrics after compromise simulation:"
curl -s http://localhost:8080/api/trust | jq '.[] | select(.container_id == "container-b") | {container_id, overall_trust, status}'

echo ""
echo "🔍 Check the dashboard at http://localhost:8080 to see:"
echo "   - Real-time trust score changes"
echo "   - Status transitions (Trusted → Suspicious → Compromised)"
echo "   - Node trust level updates"
echo "   - System-wide impact visualization"
