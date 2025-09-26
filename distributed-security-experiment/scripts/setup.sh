#!/bin/bash

echo "🚀 Setting up Distributed Security Experiment"
echo "============================================="

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

# Build the distributed security experiment
echo "🔨 Building distributed security experiment..."
cd "$(dirname "$0")/.."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    exit 1
fi

echo "✅ Build successful"

# Create demo containers for different domains
echo "🐳 Creating demo containers for converged infrastructure..."

# Smart City containers
docker run -d --name smart-city-traffic --rm alpine:latest sleep 3600
docker run -d --name smart-city-energy --rm alpine:latest sleep 3600

# Industrial IoT containers
docker run -d --name industrial-sensors --rm alpine:latest sleep 3600
docker run -d --name industrial-hmi --rm alpine:latest sleep 3600

# Transportation containers
docker run -d --name transport-autonomous --rm alpine:latest sleep 3600
docker run -d --name transport-logistics --rm alpine:latest sleep 3600

# Energy Grid containers
docker run -d --name energy-grid --rm alpine:latest sleep 3600

# Healthcare containers
docker run -d --name healthcare-iot --rm alpine:latest sleep 3600

echo "✅ Demo containers created"

# Create attack simulation scripts
echo "🎭 Creating attack simulation scripts..."

cat > scripts/simulate-supply-chain.sh << 'EOF'
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
EOF

cat > scripts/simulate-legacy-vulnerability.sh << 'EOF'
#!/bin/bash
echo "🎭 Simulating Legacy System Vulnerability"
echo "========================================"
echo "📊 Current security metrics before attack:"
curl -s http://localhost:8080/api/security | jq '.[] | select(.node_id == "industrial-sensors") | {node_id, overall_security, status, hardware_type}'

echo ""
echo "🚨 Legacy system vulnerability exploited..."
echo "   - Outdated protocol security bypassed"
echo "   - Legacy system trust degrading..."
echo "   - Cross-domain attack propagation detected"

echo ""
echo "⏱️  Monitoring security degradation over 30 seconds..."
for i in {1..15}; do
    echo -n "."
    sleep 2
done
echo ""

echo "📊 Security metrics after legacy vulnerability attack:"
curl -s http://localhost:8080/api/security | jq '.[] | select(.node_id == "industrial-sensors") | {node_id, overall_security, status, attack_indicators}'

echo ""
echo "🔍 Check the dashboard at http://localhost:8080 to see:"
echo "   - Legacy system security monitoring"
echo "   - Cross-domain attack propagation"
echo "   - Heterogeneous hardware security"
echo "   - Automated response coordination"
EOF

cat > scripts/simulate-cross-domain-attack.sh << 'EOF'
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
EOF

cat > scripts/simulate-critical-infrastructure.sh << 'EOF'
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
echo "   - Legacy System Vulnerability: ./scripts/simulate-legacy-vulnerability.sh"
echo "   - Cross-Domain Attack: ./scripts/simulate-cross-domain-attack.sh"
echo "   - Critical Infrastructure Attack: ./scripts/simulate-critical-infrastructure.sh"
echo ""
echo "🌐 This experiment demonstrates:"
echo "   - Converged digital infrastructure security"
echo "   - Cross-domain threat monitoring and response"
echo "   - Supply chain security in cyber-physical systems"
echo "   - Legacy system integration challenges"
echo "   - Zero-trust architecture implementation"
