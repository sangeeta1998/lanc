#!/bin/bash

echo "🚀 Setting up Cyber-Trust Experiment"
echo "===================================="

# Check prerequisites
echo "Checking prerequisites..."

if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Please install Rust first."
    echo "   Visit: https://rustup.rs/"
    exit 1
fi

if ! command -v docker &> /dev/null; then
    echo "❌ Docker not found. Please install Docker first."
    exit 1
fi

if ! command -v jq &> /dev/null; then
    echo "❌ jq not found. Installing jq..."
    sudo apt-get update && sudo apt-get install -y jq
fi

echo "✅ Prerequisites check passed"

# Build the cyber-trust experiment
echo "🔨 Building cyber-trust experiment..."
cd "$(dirname "$0")/.."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    exit 1
fi

echo "✅ Build successful"

# Create demo containers for different scenarios
echo "🐳 Creating demo containers..."

# Smart City containers
docker run -d --name smart-city-traffic --rm alpine:latest sleep 3600
docker run -d --name smart-city-sensors --rm alpine:latest sleep 3600

# Industrial IoT containers
docker run -d --name industrial-sensors --rm alpine:latest sleep 3600
docker run -d --name industrial-plcs --rm alpine:latest sleep 3600

# Edge Cloud containers
docker run -d --name edge-compute --rm alpine:latest sleep 3600
docker run -d --name edge-storage --rm alpine:latest sleep 3600

# Legacy system container
docker run -d --name legacy-system --rm alpine:latest sleep 3600

echo "✅ Demo containers created"

# Create attack simulation scripts
echo "🎭 Creating attack simulation scripts..."

cat > scripts/simulate-supply-chain.sh << 'EOF'
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
EOF

cat > scripts/simulate-zero-day.sh << 'EOF'
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
EOF

cat > scripts/simulate-ddos.sh << 'EOF'
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
EOF

cat > scripts/simulate-insider-threat.sh << 'EOF'
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
EOF

chmod +x scripts/simulate-*.sh

echo "✅ Attack simulation scripts created"

echo ""
echo "🎉 Setup complete!"
echo ""
echo "📋 Next steps:"
echo "   1. Start the experiment: ./scripts/start-experiment.sh"
echo "   2. Access the dashboard: http://localhost:8080"
echo "   3. Run attack scenarios: ./scripts/simulate-*.sh"
echo ""
echo "🎯 Available attack scenarios:"
echo "   - Supply Chain Compromise: ./scripts/simulate-supply-chain.sh"
echo "   - Zero-Day Exploit: ./scripts/simulate-zero-day.sh"
echo "   - DDoS Attack: ./scripts/simulate-ddos.sh"
echo "   - Insider Threat: ./scripts/simulate-insider-threat.sh"
