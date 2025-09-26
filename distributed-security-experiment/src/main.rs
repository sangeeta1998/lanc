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
    pub domain: SecurityDomain,
    pub hardware_type: HardwareType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub threat_level: f64,
    pub trust_score: f64,
    pub behavioral_score: f64,
    pub network_score: f64,
    pub supply_chain_score: f64,
    pub overall_security: f64,
    pub status: SecurityStatus,
    pub attack_indicators: Vec<AttackIndicator>,
    pub cross_domain_connections: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SecurityDomain {
    SmartCity,
    IndustrialIoT,
    Transportation,
    EnergyGrid,
    Healthcare,
    Finance,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum HardwareType {
    LegacyX86,
    ModernARM,
    EdgeRISC,
    WebAssembly,
    Containerized,
    Serverless,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SecurityStatus {
    Secure,
    Suspicious,
    Compromised,
    UnderAttack,
    Isolated,
    CrossDomainThreat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackIndicator {
    pub indicator_type: String,
    pub severity: f64,
    pub description: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub source_domain: Option<String>,
    pub propagation_risk: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentResponse {
    pub incident_id: String,
    pub node_id: String,
    pub affected_domains: Vec<String>,
    pub response_type: ResponseType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub status: ResponseStatus,
    pub actions_taken: Vec<String>,
    pub cross_domain_coordination: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseType {
    Isolation,
    Quarantine,
    Alert,
    Mitigation,
    Recovery,
    CrossDomainContainment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    CrossDomainPending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemTopology {
    pub nodes: Vec<NodeInfo>,
    pub cross_domain_connections: Vec<CrossDomainConnection>,
    pub security_policies: Vec<SecurityPolicy>,
    pub supply_chain_components: Vec<SupplyChainComponent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub node_id: String,
    pub domain: SecurityDomain,
    pub hardware_type: HardwareType,
    pub location: String,
    pub capabilities: Vec<String>,
    pub trust_level: f64,
    pub is_legacy: bool,
    pub third_party_components: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossDomainConnection {
    pub from_node: String,
    pub to_node: String,
    pub from_domain: SecurityDomain,
    pub to_domain: SecurityDomain,
    pub connection_type: String,
    pub security_level: f64,
    pub trust_relationship: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    pub policy_id: String,
    pub name: String,
    pub domain: SecurityDomain,
    pub rules: Vec<SecurityRule>,
    pub enforcement_level: f64,
    pub cross_domain_applicable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRule {
    pub rule_id: String,
    pub condition: String,
    pub action: String,
    pub priority: i32,
    pub cross_domain: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyChainComponent {
    pub component_id: String,
    pub vendor: String,
    pub version: String,
    pub trust_level: f64,
    pub vulnerabilities: Vec<String>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
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
        cross_domain_connections: vec![],
        security_policies: vec![],
        supply_chain_components: vec![],
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
    
    // Start cross-domain threat detection
    let security_store_clone = security_store.clone();
    tokio::spawn(async move {
        cross_domain_threat_detection(security_store_clone).await;
    });
    
    // Setup web server
    let routes = create_routes(security_store, incident_store, topology_store);
    
    info!("Starting Distributed Security Experiment server on http://localhost:8080");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    
    Ok(())
}

async fn initialize_demo_data(security_store: &SecurityStore, topology_store: &TopologyStore) {
    let mut topology = topology_store.write().await;
    
    // Initialize nodes across different domains
    let nodes = vec![
        ("smart-city-traffic", SecurityDomain::SmartCity, HardwareType::ModernARM, "City Center", vec!["traffic-control", "sensors", "cameras"], false, vec!["third-party-camera-software", "traffic-algorithm-vendor"]),
        ("smart-city-energy", SecurityDomain::SmartCity, HardwareType::EdgeRISC, "Energy District", vec!["energy-management", "grid-control"], false, vec!["energy-vendor-software", "grid-monitoring-tool"]),
        ("industrial-sensors", SecurityDomain::IndustrialIoT, HardwareType::LegacyX86, "Factory A", vec!["sensors", "actuators", "plcs"], true, vec!["legacy-plc-software", "scada-vendor-tool"]),
        ("industrial-hmi", SecurityDomain::IndustrialIoT, HardwareType::ModernARM, "Factory B", vec!["hmi", "scada", "sensors"], false, vec!["hmi-vendor-software", "scada-system"]),
        ("transport-autonomous", SecurityDomain::Transportation, HardwareType::WebAssembly, "Highway", vec!["autonomous-driving", "traffic-coordination"], false, vec!["ai-driving-software", "navigation-vendor"]),
        ("transport-logistics", SecurityDomain::Transportation, HardwareType::Containerized, "Port", vec!["logistics", "fleet-management"], false, vec!["logistics-software", "fleet-tracking-vendor"]),
        ("energy-grid", SecurityDomain::EnergyGrid, HardwareType::LegacyX86, "Grid Control", vec!["grid-management", "power-distribution"], true, vec!["legacy-grid-software", "power-monitoring-tool"]),
        ("healthcare-iot", SecurityDomain::Healthcare, HardwareType::ModernARM, "Hospital", vec!["patient-monitoring", "medical-devices"], false, vec!["medical-device-software", "patient-data-vendor"]),
    ];
    
    for (node_id, domain, hardware_type, location, capabilities, is_legacy, third_party) in nodes {
        let node = NodeInfo {
            node_id: node_id.to_string(),
            domain,
            hardware_type,
            location: location.to_string(),
            capabilities: capabilities.iter().map(|s| s.to_string()).collect(),
            trust_level: 0.85,
            is_legacy,
            third_party_components: third_party.iter().map(|s| s.to_string()).collect(),
        };
        
        topology.nodes.push(node);
        
        // Initialize security metrics
        security_store.insert(node_id.to_string(), SecurityMetrics {
            node_id: node_id.to_string(),
            domain,
            hardware_type,
            timestamp: chrono::Utc::now(),
            threat_level: 0.1,
            trust_score: 0.9,
            behavioral_score: 0.85,
            network_score: 0.9,
            supply_chain_score: 0.88,
            overall_security: 0.88,
            status: SecurityStatus::Secure,
            attack_indicators: vec![],
            cross_domain_connections: vec![],
        });
    }
    
    // Initialize cross-domain connections
    topology.cross_domain_connections = vec![
        CrossDomainConnection { 
            from_node: "smart-city-traffic".to_string(), 
            to_node: "transport-autonomous".to_string(), 
            from_domain: SecurityDomain::SmartCity, 
            to_domain: SecurityDomain::Transportation, 
            connection_type: "api".to_string(), 
            security_level: 0.9, 
            trust_relationship: 0.85 
        },
        CrossDomainConnection { 
            from_node: "industrial-sensors".to_string(), 
            to_node: "energy-grid".to_string(), 
            from_domain: SecurityDomain::IndustrialIoT, 
            to_domain: SecurityDomain::EnergyGrid, 
            connection_type: "legacy-protocol".to_string(), 
            security_level: 0.7, 
            trust_relationship: 0.8 
        },
        CrossDomainConnection { 
            from_node: "smart-city-energy".to_string(), 
            to_node: "energy-grid".to_string(), 
            from_domain: SecurityDomain::SmartCity, 
            to_domain: SecurityDomain::EnergyGrid, 
            connection_type: "secure-tunnel".to_string(), 
            security_level: 0.95, 
            trust_relationship: 0.9 
        },
    ];
    
    // Initialize supply chain components
    topology.supply_chain_components = vec![
        SupplyChainComponent {
            component_id: "third-party-camera-software".to_string(),
            vendor: "CameraVendor Inc".to_string(),
            version: "2.1.3".to_string(),
            trust_level: 0.8,
            vulnerabilities: vec!["CVE-2023-1234".to_string()],
            last_updated: chrono::Utc::now(),
        },
        SupplyChainComponent {
            component_id: "legacy-plc-software".to_string(),
            vendor: "LegacySystems Corp".to_string(),
            version: "1.0.5".to_string(),
            trust_level: 0.6,
            vulnerabilities: vec!["CVE-2023-5678".to_string(), "CVE-2023-9012".to_string()],
            last_updated: chrono::Utc::now(),
        },
    ];
    
    // Initialize security policies
    topology.security_policies = vec![
        SecurityPolicy {
            policy_id: "policy-1".to_string(),
            name: "Zero Trust Architecture".to_string(),
            domain: SecurityDomain::SmartCity,
            rules: vec![
                SecurityRule { rule_id: "rule-1".to_string(), condition: "trust_score < 0.7".to_string(), action: "block_communication".to_string(), priority: 1, cross_domain: false },
                SecurityRule { rule_id: "rule-2".to_string(), condition: "threat_level > 0.8".to_string(), action: "isolate_node".to_string(), priority: 1, cross_domain: true },
            ],
            enforcement_level: 0.9,
            cross_domain_applicable: true,
        },
        SecurityPolicy {
            policy_id: "policy-2".to_string(),
            name: "Supply Chain Security".to_string(),
            domain: SecurityDomain::IndustrialIoT,
            rules: vec![
                SecurityRule { rule_id: "rule-3".to_string(), condition: "supply_chain_score < 0.6".to_string(), action: "quarantine_component".to_string(), priority: 1, cross_domain: false },
                SecurityRule { rule_id: "rule-4".to_string(), condition: "vulnerability_detected".to_string(), action: "alert_security_team".to_string(), priority: 2, cross_domain: true },
            ],
            enforcement_level: 0.8,
            cross_domain_applicable: true,
        },
    ];
}

async fn security_monitoring_loop(security_store: SecurityStore, incident_store: IncidentStore, _topology_store: TopologyStore) {
    let mut interval = interval(Duration::from_secs(4));
    
    loop {
        interval.tick().await;
        
        // Simulate security monitoring for each node
        for mut entry in security_store.iter_mut() {
            let node_id = entry.key().clone();
            let metrics = entry.value_mut();
            
            // Simulate different attack scenarios based on domain and hardware type
            let attack_probability = match (metrics.domain, metrics.hardware_type) {
                (SecurityDomain::SmartCity, HardwareType::ModernARM) => 0.12, // Supply chain attacks
                (SecurityDomain::IndustrialIoT, HardwareType::LegacyX86) => 0.08, // Legacy system vulnerabilities
                (SecurityDomain::Transportation, HardwareType::WebAssembly) => 0.06, // Cross-domain attacks
                (SecurityDomain::EnergyGrid, HardwareType::LegacyX86) => 0.10, // Critical infrastructure attacks
                (SecurityDomain::Healthcare, HardwareType::ModernARM) => 0.04, // Medical device attacks
                _ => 0.03, // General attacks
            };
            
            if rand::random::<f64>() < attack_probability {
                match metrics.domain {
                    SecurityDomain::SmartCity => {
                        // Supply chain compromise
                        metrics.threat_level = (metrics.threat_level + 0.25).min(1.0);
                        metrics.supply_chain_score = (metrics.supply_chain_score - 0.2).max(0.0);
                        metrics.attack_indicators.push(AttackIndicator {
                            indicator_type: "supply_chain_compromise".to_string(),
                            severity: 0.8,
                            description: "Compromised third-party component detected".to_string(),
                            timestamp: chrono::Utc::now(),
                            source_domain: Some("SmartCity".to_string()),
                            propagation_risk: 0.7,
                        });
                    },
                    SecurityDomain::IndustrialIoT => {
                        // Legacy system vulnerability
                        metrics.threat_level = (metrics.threat_level + 0.3).min(1.0);
                        metrics.trust_score = (metrics.trust_score - 0.2).max(0.0);
                        metrics.attack_indicators.push(AttackIndicator {
                            indicator_type: "legacy_vulnerability".to_string(),
                            severity: 0.9,
                            description: "Exploited legacy system vulnerability".to_string(),
                            timestamp: chrono::Utc::now(),
                            source_domain: Some("IndustrialIoT".to_string()),
                            propagation_risk: 0.8,
                        });
                    },
                    SecurityDomain::Transportation => {
                        // Cross-domain attack
                        metrics.threat_level = (metrics.threat_level + 0.2).min(1.0);
                        metrics.behavioral_score = (metrics.behavioral_score - 0.15).max(0.0);
                        metrics.attack_indicators.push(AttackIndicator {
                            indicator_type: "cross_domain_attack".to_string(),
                            severity: 0.7,
                            description: "Attack propagated from another domain".to_string(),
                            timestamp: chrono::Utc::now(),
                            source_domain: Some("SmartCity".to_string()),
                            propagation_risk: 0.9,
                        });
                    },
                    SecurityDomain::EnergyGrid => {
                        // Critical infrastructure attack
                        metrics.threat_level = (metrics.threat_level + 0.35).min(1.0);
                        metrics.network_score = (metrics.network_score - 0.25).max(0.0);
                        metrics.attack_indicators.push(AttackIndicator {
                            indicator_type: "critical_infrastructure_attack".to_string(),
                            severity: 0.95,
                            description: "Coordinated attack on critical infrastructure".to_string(),
                            timestamp: chrono::Utc::now(),
                            source_domain: None,
                            propagation_risk: 0.95,
                        });
                    },
                    _ => {
                        // General attack
                        metrics.threat_level = (metrics.threat_level + 0.15).min(1.0);
                        metrics.trust_score = (metrics.trust_score - 0.1).max(0.0);
                    }
                }
            } else {
                // Normal operation with slight variations
                let variation = rand::random::<f64>() * 0.04 - 0.02; // -0.02 to 0.02
                metrics.threat_level = (metrics.threat_level + variation).max(0.0).min(1.0);
                metrics.trust_score = (metrics.trust_score + variation).max(0.0).min(1.0);
            }
            
            // Recalculate overall security
            metrics.overall_security = (metrics.trust_score + metrics.behavioral_score + metrics.network_score + metrics.supply_chain_score) / 4.0;
            metrics.timestamp = chrono::Utc::now();
            
            // Update status based on security level
            metrics.status = match metrics.overall_security {
                s if s >= 0.8 => SecurityStatus::Secure,
                s if s >= 0.6 => SecurityStatus::Suspicious,
                s if s >= 0.4 => SecurityStatus::Compromised,
                s if s >= 0.2 => SecurityStatus::UnderAttack,
                _ => SecurityStatus::Isolated,
            };
            
            // Check for cross-domain threats
            if metrics.attack_indicators.iter().any(|i| i.propagation_risk > 0.8) {
                metrics.status = SecurityStatus::CrossDomainThreat;
            }
            
            // Trigger incident response if needed
            if metrics.overall_security < 0.6 {
                let incident_id = Uuid::new_v4().to_string();
                let response_type = match metrics.overall_security {
                    s if s < 0.2 => ResponseType::Isolation,
                    s if s < 0.4 => ResponseType::Quarantine,
                    _ => ResponseType::Alert,
                };
                
                let cross_domain = metrics.status == SecurityStatus::CrossDomainThreat;
                
                let incident = IncidentResponse {
                    incident_id: incident_id.clone(),
                    node_id: node_id.clone(),
                    affected_domains: vec![format!("{:?}", metrics.domain)],
                    response_type,
                    timestamp: chrono::Utc::now(),
                    status: if cross_domain { ResponseStatus::CrossDomainPending } else { ResponseStatus::InProgress },
                    actions_taken: vec!["Security assessment initiated".to_string()],
                    cross_domain_coordination: cross_domain,
                };
                
                incident_store.insert(incident_id, incident);
            }
        }
        
        info!("Security monitoring completed for {} nodes", security_store.len());
    }
}

async fn cross_domain_threat_detection(security_store: SecurityStore) {
    let mut interval = interval(Duration::from_secs(8));
    
    loop {
        interval.tick().await;
        
        // Simulate cross-domain threat detection
        for mut entry in security_store.iter_mut() {
            let metrics = entry.value_mut();
            
            // Check for cross-domain propagation
            if metrics.attack_indicators.iter().any(|i| i.propagation_risk > 0.7) {
                metrics.attack_indicators.push(AttackIndicator {
                    indicator_type: "cross_domain_propagation".to_string(),
                    severity: 0.8,
                    description: "Threat detected propagating across domains".to_string(),
                    timestamp: chrono::Utc::now(),
                    source_domain: Some(format!("{:?}", metrics.domain)),
                    propagation_risk: 0.9,
                });
            }
        }
        
        info!("Cross-domain threat detection completed for {} nodes", security_store.len());
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
