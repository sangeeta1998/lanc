#!/bin/bash

echo "🛡️  Setting up Trust Monitoring System"
echo "🎯 SCULI-Aligned Trust Assessment Framework"
echo

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✅ Rust is installed"

# Check if required tools are available
echo "🔧 Checking dependencies..."

# Check for Docker (optional)
if command -v docker &> /dev/null; then
    echo "✅ Docker is available"
else
    echo "⚠️  Docker not found (optional for containerized deployment)"
fi

# Check for Kubernetes (optional)
if command -v kubectl &> /dev/null; then
    echo "✅ Kubernetes is available"
else
    echo "⚠️  Kubernetes not found (optional for orchestration)"
fi

# Build the project
echo "🔨 Building Trust Monitoring System..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful"
else
    echo "❌ Build failed"
    exit 1
fi

# Create necessary directories
echo "📁 Creating directories..."
mkdir -p logs
mkdir -p data
mkdir -p config
mkdir -p examples

# Set up environment
echo "🌍 Setting up environment..."
cat > .env << EOF
# Trust Monitoring System Configuration
RUST_LOG=info
TRUST_MONITOR_PORT=3030
TRUST_MONITOR_CONFIG=config/system.yaml

# Data Sources
PROMETHEUS_URL=http://localhost:9090
ELASTICSEARCH_URL=http://localhost:9200
JAEGER_URL=http://localhost:14268

# Notification Channels
SLACK_WEBHOOK_URL=https://hooks.slack.com/services/...
PAGERDUTY_INTEGRATION_KEY=your-pagerduty-key
EMAIL_SMTP_SERVER=smtp.company.com
EMAIL_SMTP_PORT=587
EMAIL_USERNAME=alerts@company.com
EMAIL_PASSWORD=your-email-password
EOF

echo "✅ Environment file created"

# Create systemd service file (optional)
echo "🔧 Creating systemd service file..."
cat > trust-monitoring.service << EOF
[Unit]
Description=Trust Monitoring System
After=network.target

[Service]
Type=simple
User=ubuntu
WorkingDirectory=$(pwd)
ExecStart=$(pwd)/target/release/trust-monitoring-system
Restart=always
RestartSec=5
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
EOF

echo "✅ Systemd service file created"

# Create monitoring dashboard configuration
echo "📊 Creating monitoring dashboard configuration..."
cat > config/grafana-dashboard.json << EOF
{
  "dashboard": {
    "title": "Trust Monitoring Dashboard",
    "panels": [
      {
        "title": "Overall Trust Score",
        "type": "stat",
        "targets": [
          {
            "expr": "trust_score_overall",
            "legendFormat": "Overall Trust"
          }
        ]
      },
      {
        "title": "Component Trust Scores",
        "type": "graph",
        "targets": [
          {
            "expr": "trust_score_component",
            "legendFormat": "{{component}}"
          }
        ]
      },
      {
        "title": "Active Incidents",
        "type": "table",
        "targets": [
          {
            "expr": "incidents_active",
            "legendFormat": "Active Incidents"
          }
        ]
      }
    ]
  }
}
EOF

echo "✅ Grafana dashboard configuration created"

# Create example deployment
echo "🚀 Creating example deployment..."
cat > examples/deployment.yaml << EOF
apiVersion: apps/v1
kind: Deployment
metadata:
  name: trust-monitoring-system
  labels:
    app: trust-monitoring-system
spec:
  replicas: 1
  selector:
    matchLabels:
      app: trust-monitoring-system
  template:
    metadata:
      labels:
        app: trust-monitoring-system
    spec:
      containers:
      - name: trust-monitoring-system
        image: trust-monitoring-system:latest
        ports:
        - containerPort: 3030
        env:
        - name: RUST_LOG
          value: "info"
        - name: TRUST_MONITOR_CONFIG
          value: "/app/config/system.yaml"
        volumeMounts:
        - name: config
          mountPath: /app/config
      volumes:
      - name: config
        configMap:
          name: trust-monitoring-config
---
apiVersion: v1
kind: Service
metadata:
  name: trust-monitoring-service
spec:
  selector:
    app: trust-monitoring-system
  ports:
  - port: 3030
    targetPort: 3030
  type: LoadBalancer
---
apiVersion: v1
kind: ConfigMap
metadata:
  name: trust-monitoring-config
data:
  system.yaml: |
    $(cat config/system.yaml | sed 's/^/    /')
EOF

echo "✅ Kubernetes deployment created"

echo
echo "🎉 Setup completed successfully!"
echo
echo "📋 Next steps:"
echo "   1. Review and update config/system.yaml"
echo "   2. Update .env with your specific configuration"
echo "   3. Run: cargo run --release"
echo "   4. Access the API at: http://localhost:3030"
echo
echo "🔗 Available endpoints:"
echo "   GET /status - System status"
echo "   GET /trust-scores - Current trust scores"
echo "   GET /incidents - Active incidents"
echo "   GET /alerts - Active alerts"
echo
echo "📚 Documentation:"
echo "   - README.md - System overview and usage"
echo "   - docs/ - Detailed documentation"
echo "   - examples/ - Example configurations and deployments"
