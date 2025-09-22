# Trust-Aware Container Orchestration Demo

## Overview
This demonstration showcases a practical approach to monitoring and assessing trust in large distributed systems, specifically designed for the SCULI project's "securing-a-compromised-system" paradigm.

## Key Features
- **Runtime Trust Assessment**: Continuous monitoring of container behavior and integrity
- **Dynamic Trust Propagation**: Trust metrics flow through the distributed system
- **Cross-Architecture Support**: Leverages WebAssembly for portable trust assessment
- **Real-time Visualization**: Dashboard showing trust states across nodes
- **Compromise Detection**: Identifies and isolates compromised containers

## Architecture
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Edge Node 1   │    │   Edge Node 2   │    │   Cloud Node    │
│                 │    │                 │    │                 │
│ ┌─────────────┐ │    │ ┌─────────────┐ │    │ ┌─────────────┐ │
│ │ Trust Agent │ │◄──►│ │ Trust Agent │ │◄──►│ │ Trust Agent │ │
│ │ (WASM)      │ │    │ │ (WASM)      │ │    │ │ (WASM)      │ │
│ └─────────────┘ │    │ └─────────────┘ │    │ └─────────────┘ │
│                 │    │                 │    │                 │
│ ┌─────────────┐ │    │ ┌─────────────┐ │    │ ┌─────────────┐ │
│ │ Container A │ │    │ │ Container B │ │    │ │ Container C │ │
│ └─────────────┘ │    │ └─────────────┘ │    │ └─────────────┘ │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                    ┌─────────────────┐
                    │ Trust Dashboard │
                    │ (Real-time UI)  │
                    └─────────────────┘
```

## Trust Assessment Metrics
1. **Container Integrity**: Hash verification, signature validation
2. **Behavioral Analysis**: Resource usage patterns, network activity
3. **Communication Trust**: Inter-container communication patterns
4. **Node Health**: System resource monitoring, anomaly detection
5. **Supply Chain Trust**: Image provenance, vulnerability scanning

## Technologies Used
- **WebAssembly**: Portable trust assessment agents
- **Rust**: High-performance trust computation
- **Docker/Containerd**: Container orchestration
- **k3s**: Lightweight Kubernetes for edge deployment
- **WebSocket**: Real-time trust metric streaming
- **React**: Trust visualization dashboard

## Demo Scenarios
1. **Normal Operation**: Trust metrics flow normally
2. **Compromise Detection**: Malicious container identified and isolated
3. **Trust Degradation**: Gradual trust loss due to suspicious behavior
4. **Recovery**: Trust restoration after remediation
5. **Cross-Architecture**: Trust assessment on ARM64, RISC-V, x86_64

## Running the Demo
```bash
# Start the trust monitoring system
./scripts/start-demo.sh

# Access the dashboard
open http://localhost:3000

# Simulate different scenarios
./scripts/simulate-compromise.sh
./scripts/simulate-trust-degradation.sh
```

## Research Contributions
This demo addresses key SCULI challenges:
- **Compositional Security**: Trust assessment across heterogeneous components
- **Runtime Assurance**: Continuous monitoring without system redesign
- **Scalability**: WebAssembly enables lightweight, portable trust agents
- **Practical Implementation**: Real-world applicable to existing infrastructure
