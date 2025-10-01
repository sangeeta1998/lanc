#!/bin/bash

echo "🚀 Setting up SCULI Trust Monitoring Demo"
echo "🎯 Ultra-Large Scale Distributed System Trust Assessment"
echo

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✅ Rust is installed"

# Check for required tools
echo "🔧 Checking dependencies..."

# Check for Docker (optional)
if command -v docker &> /dev/null; then
    echo "✅ Docker is available"
else
    echo "⚠️  Docker not found (optional for containerized deployment)"
fi

# Check for Node.js (for web server)
if command -v node &> /dev/null; then
    echo "✅ Node.js is available"
else
    echo "⚠️  Node.js not found (optional for web dashboard)"
fi

# Build the project
echo "🔨 Building SCULI Trust Monitoring System..."
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
mkdir -p examples
mkdir -p config

# Create example WebAssembly module
echo "🔧 Creating example WebAssembly module..."
cat > examples/wasm-service.wat << 'EOF'
(module
  (func $add (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.add)
  (export "add" (func $add))
  
  (func $trust_score (result f64)
    f64.const 0.85)
  (export "trust_score" (func $trust_score))
)
EOF

# Convert WAT to WASM
if command -v wat2wasm &> /dev/null; then
    wat2wasm examples/wasm-service.wat -o examples/wasm-service.wasm
    echo "✅ WebAssembly module created"
else
    echo "⚠️  wat2wasm not found, skipping WASM module creation"
fi

# Create systemd service file (optional)
echo "🔧 Creating systemd service file..."
cat > sculi-trust-monitor.service << EOF
[Unit]
Description=SCULI Trust Monitoring System
After=network.target

[Service]
Type=simple
User=ubuntu
WorkingDirectory=$(pwd)
ExecStart=$(pwd)/target/release/trust-monitor
Restart=always
RestartSec=5
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
EOF

echo "✅ Systemd service file created"

# Create web server script
echo "🌐 Creating web server script..."
cat > scripts/start-web.sh << 'EOF'
#!/bin/bash
cd web
if command -v python3 &> /dev/null; then
    python3 -m http.server 8080
elif command -v python &> /dev/null; then
    python -m SimpleHTTPServer 8080
elif command -v node &> /dev/null; then
    npx http-server -p 8080
else
    echo "No web server found. Please install Python or Node.js"
    exit 1
fi
EOF

chmod +x scripts/start-web.sh

echo "✅ Web server script created"

# Create demo script
echo "🎬 Creating demo script..."
cat > scripts/run-demo.sh << 'EOF'
#!/bin/bash

echo "🎬 SCULI Trust Monitoring Demo"
echo "🎯 Ultra-Large Scale Distributed System Trust Assessment"
echo

# Start the trust monitoring system
echo "🚀 Starting Trust Monitoring System..."
./target/release/trust-monitor &
MONITOR_PID=$!

echo "✅ Trust Monitor started with PID: $MONITOR_PID"
echo "🌐 API available at: http://localhost:3030"
echo

# Wait for system to start
echo "⏳ Waiting for system to initialize..."
sleep 3

# Check if system is running
if ! kill -0 $MONITOR_PID 2>/dev/null; then
    echo "❌ Trust Monitor failed to start"
    exit 1
fi

# Start web dashboard
echo "🌐 Starting web dashboard..."
./scripts/start-web.sh &
WEB_PID=$!

echo "✅ Web dashboard started with PID: $WEB_PID"
echo "🌐 Dashboard available at: http://localhost:8080"
echo

# Wait for web server to start
sleep 2

echo "🎉 Demo is ready!"
echo
echo "📋 Available interfaces:"
echo "   🌐 Web Dashboard: http://localhost:8080"
echo "   🔌 API Endpoints: http://localhost:3030"
echo
echo "📊 API Endpoints:"
echo "   GET /status - System status with SCULI objectives"
echo "   GET /trust-scores - Current trust scores"
echo "   GET /alerts - Active alerts"
echo "   POST /simulate-degradation - Simulate trust degradation"
echo "   POST /simulate-recovery - Simulate recovery"
echo
echo "🎮 Demo Controls:"
echo "   1. Open http://localhost:8080 in your browser"
echo "   2. Click 'Simulate Degradation' to see trust scores drop"
echo "   3. Click 'Simulate Recovery' to restore trust scores"
echo "   4. Watch the real-time dashboard update"
echo
echo "Press Ctrl+C to stop the demo"

# Keep running until interrupted
trap "echo; echo '🛑 Stopping SCULI Trust Monitoring Demo...'; kill $MONITOR_PID $WEB_PID 2>/dev/null; echo '✅ Demo stopped'; exit 0" INT

while true; do
    sleep 1
done
EOF

chmod +x scripts/run-demo.sh

echo "✅ Demo script created"

echo
echo "🎉 Setup completed successfully!"
echo
echo "📋 Next steps:"
echo "   1. Run the demo: ./scripts/run-demo.sh"
echo "   2. Open http://localhost:8080 in your browser"
echo "   3. Use the demo controls to simulate trust scenarios"
echo
echo "🔗 Quick start:"
echo "   ./scripts/run-demo.sh"
echo
echo "📚 Documentation:"
echo "   - README.md - System overview and SCULI alignment"
echo "   - web/index.html - Interactive dashboard"
echo "   - examples/ - WebAssembly modules and configurations"

