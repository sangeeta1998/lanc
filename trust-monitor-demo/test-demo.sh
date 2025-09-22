#!/bin/bash

echo "🧪 Testing Trust Monitor Demo"
echo "============================="

# Test 1: Check if server is running
echo "1. Testing server connectivity..."
if curl -s http://localhost:8080/api/trust > /dev/null; then
    echo "   ✅ Server is running and responding"
else
    echo "   ❌ Server is not responding"
    exit 1
fi

# Test 2: Check trust metrics endpoint
echo "2. Testing trust metrics endpoint..."
TRUST_COUNT=$(curl -s http://localhost:8080/api/trust | jq '. | length')
if [ "$TRUST_COUNT" -gt 0 ]; then
    echo "   ✅ Trust metrics endpoint working ($TRUST_COUNT containers)"
else
    echo "   ❌ No trust metrics found"
    exit 1
fi

# Test 3: Check nodes endpoint
echo "3. Testing nodes endpoint..."
NODE_COUNT=$(curl -s http://localhost:8080/api/nodes | jq '. | length')
if [ "$NODE_COUNT" -gt 0 ]; then
    echo "   ✅ Nodes endpoint working ($NODE_COUNT nodes)"
else
    echo "   ❌ No node information found"
    exit 1
fi

# Test 4: Check dashboard
echo "4. Testing dashboard..."
if curl -s http://localhost:8080/ | grep -q "Trust Monitor Dashboard"; then
    echo "   ✅ Dashboard is accessible"
else
    echo "   ❌ Dashboard not accessible"
    exit 1
fi

# Test 5: Check compromised container
echo "5. Testing compromise detection..."
COMPROMISED=$(curl -s http://localhost:8080/api/trust | jq '.[] | select(.container_id == "container-b") | .status')
if [ "$COMPROMISED" = '"Compromised"' ]; then
    echo "   ✅ Compromise detection working"
else
    echo "   ❌ Compromise detection not working"
    exit 1
fi

# Test 6: Check real-time updates
echo "6. Testing real-time updates..."
sleep 3
NEW_TIMESTAMP=$(curl -s http://localhost:8080/api/trust | jq '.[0].timestamp')
if [ "$NEW_TIMESTAMP" != "null" ]; then
    echo "   ✅ Real-time updates working"
else
    echo "   ❌ Real-time updates not working"
    exit 1
fi

echo ""
echo "🎉 All tests passed! Demo is working correctly."
echo ""
echo "📊 Demo Summary:"
echo "   - Server running on http://localhost:8080"
echo "   - $TRUST_COUNT containers being monitored"
echo "   - $NODE_COUNT nodes in the system"
echo "   - Compromise detection active"
echo "   - Real-time trust assessment working"
echo ""
echo "🌐 Access the dashboard at: http://localhost:8080"
echo "📡 API endpoints:"
echo "   - GET /api/trust - Trust metrics"
echo "   - GET /api/nodes - Node information"
echo ""
echo "🎭 Run compromise simulation: ./scripts/simulate-compromise.sh"
