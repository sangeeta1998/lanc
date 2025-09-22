use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};
use tracing::info;
use uuid::Uuid;
use warp::Filter;
use dashmap::DashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMetrics {
    pub node_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub threat_level: f64,
    pub trust_score: f64,
    pub behavioral_score: f64,
    pub network_score: f64,
    pub system_score: f64,
    pub overall_security: f64,
    pub status: SecurityStatus,
    pub attack_indicators: Vec<AttackIndicator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityStatus {
    Secure,
    Suspicious,
    Compromised,
    UnderAttack,
    Isolated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackIndicator {
    pub indicator_type: String,
    pub severity: f64,
    pub description: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentResponse {
    pub incident_id: String,
    pub node_id: String,
    pub response_type: ResponseType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub status: ResponseStatus,
    pub actions_taken: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseType {
    Isolation,
    Quarantine,
    Alert,
    Mitigation,
    Recovery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemTopology {
    pub nodes: Vec<NodeInfo>,
    pub connections: Vec<Connection>,
    pub security_policies: Vec<SecurityPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub node_id: String,
    pub node_type: NodeType,
    pub architecture: String,
    pub location: String,
    pub capabilities: Vec<String>,
    pub trust_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    SmartCity,
    IndustrialIoT,
    EdgeCloud,
    LegacySystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub from_node: String,
    pub to_node: String,
    pub connection_type: String,
    pub security_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    pub policy_id: String,
    pub name: String,
    pub rules: Vec<SecurityRule>,
    pub enforcement_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRule {
    pub rule_id: String,
    pub condition: String,
    pub action: String,
    pub priority: i32,
}

pub type SecurityStore = Arc<DashMap<String, SecurityMetrics>>;
pub type IncidentStore = Arc<DashMap<String, IncidentResponse>>;
pub type TopologyStore = Arc<RwLock<SystemTopology>>;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let security_store: SecurityStore = Arc::new(DashMap::new());
    let incident_store: IncidentStore = Arc::new(DashMap::new());
    let topology_store: TopologyStore = Arc::new(RwLock::new(SystemTopology {
        nodes: vec![],
        connections: vec![],
        security_policies: vec![],
    }));
    
    // Initialize demo data
    initialize_demo_data(&security_store, &topology_store).await;
    
    // Start security monitoring loop
    let security_store_clone = security_store.clone();
    let incident_store_clone = incident_store.clone();
    let topology_store_clone = topology_store.clone();
    tokio::spawn(async move {
        security_monitoring_loop(security_store_clone, incident_store_clone, topology_store_clone).await;
    });
    
    // Start threat prediction loop
    let security_store_clone = security_store.clone();
    tokio::spawn(async move {
        threat_prediction_loop(security_store_clone).await;
    });
    
    // Setup web server
    let routes = create_routes(security_store, incident_store, topology_store);
    
    info!("Starting Cyber-Trust Experiment server on http://localhost:8080");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    
    Ok(())
}

async fn initialize_demo_data(security_store: &SecurityStore, topology_store: &TopologyStore) {
    let mut topology = topology_store.write().await;
    
    // Initialize nodes
    let nodes = vec![
        ("smart-city-1", NodeType::SmartCity, "aarch64", "City Center", vec!["traffic-control", "sensors", "cameras"]),
        ("smart-city-2", NodeType::SmartCity, "aarch64", "Suburbs", vec!["environmental", "lighting", "parking"]),
        ("industrial-1", NodeType::IndustrialIoT, "x86_64", "Factory A", vec!["sensors", "actuators", "plcs"]),
        ("industrial-2", NodeType::IndustrialIoT, "x86_64", "Factory B", vec!["hmi", "scada", "sensors"]),
        ("edge-cloud-1", NodeType::EdgeCloud, "x86_64", "Data Center", vec!["compute", "storage", "services"]),
        ("edge-cloud-2", NodeType::EdgeCloud, "riscv64", "Edge Node", vec!["compute", "networking", "apis"]),
        ("legacy-1", NodeType::LegacySystem, "x86_64", "Legacy Site", vec!["legacy-apps", "databases"]),
    ];
    
    for (node_id, node_type, arch, location, capabilities) in nodes {
        let node = NodeInfo {
            node_id: node_id.to_string(),
            node_type,
            architecture: arch.to_string(),
            location: location.to_string(),
            capabilities: capabilities.iter().map(|s| s.to_string()).collect(),
            trust_level: 0.85,
        };
        
        topology.nodes.push(node);
        
        // Initialize security metrics
        security_store.insert(node_id.to_string(), SecurityMetrics {
            node_id: node_id.to_string(),
            timestamp: chrono::Utc::now(),
            threat_level: 0.1,
            trust_score: 0.9,
            behavioral_score: 0.85,
            network_score: 0.9,
            system_score: 0.88,
            overall_security: 0.88,
            status: SecurityStatus::Secure,
            attack_indicators: vec![],
        });
    }
    
    // Initialize connections
    topology.connections = vec![
        Connection { from_node: "smart-city-1".to_string(), to_node: "smart-city-2".to_string(), connection_type: "mesh".to_string(), security_level: 0.9 },
        Connection { from_node: "industrial-1".to_string(), to_node: "industrial-2".to_string(), connection_type: "vpn".to_string(), security_level: 0.85 },
        Connection { from_node: "edge-cloud-1".to_string(), to_node: "edge-cloud-2".to_string(), connection_type: "secure-tunnel".to_string(), security_level: 0.95 },
        Connection { from_node: "smart-city-1".to_string(), to_node: "edge-cloud-1".to_string(), connection_type: "api".to_string(), security_level: 0.8 },
        Connection { from_node: "industrial-1".to_string(), to_node: "edge-cloud-1".to_string(), connection_type: "mqtt".to_string(), security_level: 0.75 },
    ];
    
    // Initialize security policies
    topology.security_policies = vec![
        SecurityPolicy {
            policy_id: "policy-1".to_string(),
            name: "Zero Trust Network".to_string(),
            rules: vec![
                SecurityRule { rule_id: "rule-1".to_string(), condition: "trust_score < 0.7".to_string(), action: "block_communication".to_string(), priority: 1 },
                SecurityRule { rule_id: "rule-2".to_string(), condition: "threat_level > 0.8".to_string(), action: "isolate_node".to_string(), priority: 1 },
            ],
            enforcement_level: 0.9,
        },
        SecurityPolicy {
            policy_id: "policy-2".to_string(),
            name: "Behavioral Analysis".to_string(),
            rules: vec![
                SecurityRule { rule_id: "rule-3".to_string(), condition: "behavioral_score < 0.6".to_string(), action: "alert_security_team".to_string(), priority: 2 },
                SecurityRule { rule_id: "rule-4".to_string(), condition: "anomaly_detected".to_string(), action: "quarantine_node".to_string(), priority: 1 },
            ],
            enforcement_level: 0.8,
        },
    ];
}

async fn security_monitoring_loop(security_store: SecurityStore, incident_store: IncidentStore, _topology_store: TopologyStore) {
    let mut interval = interval(Duration::from_secs(3));
    
    loop {
        interval.tick().await;
        
        // Simulate security monitoring for each node
        for mut entry in security_store.iter_mut() {
            let node_id = entry.key().clone();
            let metrics = entry.value_mut();
            
            // Simulate different attack scenarios
            let attack_probability = match node_id.as_str() {
                "smart-city-1" => 0.1, // Supply chain compromise
                "industrial-1" => 0.05, // Zero-day exploit
                "edge-cloud-1" => 0.08, // DDoS attack
                "legacy-1" => 0.03, // Insider threat
                _ => 0.02, // Normal variation
            };
            
            if rand::random::<f64>() < attack_probability {
                match node_id.as_str() {
                    "smart-city-1" => {
                        // Supply chain compromise
                        metrics.threat_level = (metrics.threat_level + 0.2).min(1.0);
                        metrics.trust_score = (metrics.trust_score - 0.15).max(0.0);
                        metrics.attack_indicators.push(AttackIndicator {
                            indicator_type: "supply_chain".to_string(),
                            severity: 0.8,
                            description: "Suspicious software component detected".to_string(),
                            timestamp: chrono::Utc::now(),
                        });
                    },
                    "industrial-1" => {
                        // Zero-day exploit
                        metrics.threat_level = (metrics.threat_level + 0.3).min(1.0);
                        metrics.behavioral_score = (metrics.behavioral_score - 0.2).max(0.0);
                        metrics.attack_indicators.push(AttackIndicator {
                            indicator_type: "zero_day".to_string(),
                            severity: 0.9,
                            description: "Unknown attack pattern detected".to_string(),
                            timestamp: chrono::Utc::now(),
                        });
                    },
                    "edge-cloud-1" => {
                        // DDoS attack
                        metrics.threat_level = (metrics.threat_level + 0.25).min(1.0);
                        metrics.network_score = (metrics.network_score - 0.3).max(0.0);
                        metrics.attack_indicators.push(AttackIndicator {
                            indicator_type: "ddos".to_string(),
                            severity: 0.7,
                            description: "High volume of suspicious network traffic".to_string(),
                            timestamp: chrono::Utc::now(),
                        });
                    },
                    "legacy-1" => {
                        // Insider threat
                        metrics.threat_level = (metrics.threat_level + 0.15).min(1.0);
                        metrics.behavioral_score = (metrics.behavioral_score - 0.1).max(0.0);
                        metrics.attack_indicators.push(AttackIndicator {
                            indicator_type: "insider_threat".to_string(),
                            severity: 0.6,
                            description: "Unusual user behavior patterns detected".to_string(),
                            timestamp: chrono::Utc::now(),
                        });
                    },
                    _ => {
                        // General attack
                        metrics.threat_level = (metrics.threat_level + 0.1).min(1.0);
                        metrics.trust_score = (metrics.trust_score - 0.05).max(0.0);
                    }
                }
            } else {
                // Normal operation with slight variations
                let variation = rand::random::<f64>() * 0.04 - 0.02; // -0.02 to 0.02
                metrics.threat_level = (metrics.threat_level + variation).max(0.0).min(1.0);
                metrics.trust_score = (metrics.trust_score + variation).max(0.0).min(1.0);
            }
            
            // Recalculate overall security
            metrics.overall_security = (metrics.trust_score + metrics.behavioral_score + metrics.network_score + metrics.system_score) / 4.0;
            metrics.timestamp = chrono::Utc::now();
            
            // Update status based on security level
            metrics.status = match metrics.overall_security {
                s if s >= 0.8 => SecurityStatus::Secure,
                s if s >= 0.6 => SecurityStatus::Suspicious,
                s if s >= 0.4 => SecurityStatus::Compromised,
                s if s >= 0.2 => SecurityStatus::UnderAttack,
                _ => SecurityStatus::Isolated,
            };
            
            // Trigger incident response if needed
            if metrics.overall_security < 0.6 {
                let incident_id = Uuid::new_v4().to_string();
                let response_type = match metrics.overall_security {
                    s if s < 0.2 => ResponseType::Isolation,
                    s if s < 0.4 => ResponseType::Quarantine,
                    _ => ResponseType::Alert,
                };
                
                let incident = IncidentResponse {
                    incident_id: incident_id.clone(),
                    node_id: node_id.clone(),
                    response_type,
                    timestamp: chrono::Utc::now(),
                    status: ResponseStatus::InProgress,
                    actions_taken: vec!["Security assessment initiated".to_string()],
                };
                
                incident_store.insert(incident_id, incident);
            }
        }
        
        info!("Security monitoring completed for {} nodes", security_store.len());
    }
}

async fn threat_prediction_loop(security_store: SecurityStore) {
    let mut interval = interval(Duration::from_secs(10));
    
    loop {
        interval.tick().await;
        
        // Simulate ML-based threat prediction
        for mut entry in security_store.iter_mut() {
            let metrics = entry.value_mut();
            
            // Simple threat prediction based on current metrics
            let predicted_threat = (metrics.threat_level * 0.7 + metrics.behavioral_score * 0.3).min(1.0);
            
            if predicted_threat > 0.7 {
                metrics.attack_indicators.push(AttackIndicator {
                    indicator_type: "predicted_threat".to_string(),
                    severity: predicted_threat,
                    description: "ML model predicts potential security incident".to_string(),
                    timestamp: chrono::Utc::now(),
                });
            }
        }
        
        info!("Threat prediction completed for {} nodes", security_store.len());
    }
}

fn create_routes(
    security_store: SecurityStore,
    incident_store: IncidentStore,
    topology_store: TopologyStore,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let security_store = warp::any().map(move || security_store.clone());
    let incident_store = warp::any().map(move || incident_store.clone());
    let topology_store = warp::any().map(move || topology_store.clone());
    
    let security_metrics = warp::path("api")
        .and(warp::path("security"))
        .and(warp::get())
        .and(security_store)
        .and_then(get_security_metrics);
    
    let incidents = warp::path("api")
        .and(warp::path("incidents"))
        .and(warp::get())
        .and(incident_store)
        .and_then(get_incidents);
    
    let topology = warp::path("api")
        .and(warp::path("topology"))
        .and(warp::get())
        .and(topology_store)
        .and_then(get_topology);
    
    let dashboard = warp::path::end()
        .and(warp::get())
        .map(|| warp::reply::html(include_str!("../static/dashboard.html")));
    
    security_metrics.or(incidents).or(topology).or(dashboard)
}

async fn get_security_metrics(security_store: SecurityStore) -> Result<impl warp::Reply, warp::Rejection> {
    let metrics: Vec<SecurityMetrics> = security_store.iter().map(|entry| entry.value().clone()).collect();
    Ok(warp::reply::json(&metrics))
}

async fn get_incidents(incident_store: IncidentStore) -> Result<impl warp::Reply, warp::Rejection> {
    let incidents: Vec<IncidentResponse> = incident_store.iter().map(|entry| entry.value().clone()).collect();
    Ok(warp::reply::json(&incidents))
}

async fn get_topology(topology_store: TopologyStore) -> Result<impl warp::Reply, warp::Rejection> {
    let topology = topology_store.read().await;
    Ok(warp::reply::json(&*topology))
}
