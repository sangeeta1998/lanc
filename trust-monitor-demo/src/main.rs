use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};
use tracing::info;
use warp::Filter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustMetrics {
    pub container_id: String,
    pub node_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub integrity_score: f64,
    pub behavioral_score: f64,
    pub communication_score: f64,
    pub overall_trust: f64,
    pub status: ContainerStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContainerStatus {
    Trusted,
    Suspicious,
    Compromised,
    Isolated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub node_id: String,
    pub architecture: String,
    pub containers: Vec<String>,
    pub trust_level: f64,
}

pub type TrustStore = Arc<RwLock<HashMap<String, TrustMetrics>>>;
pub type NodeStore = Arc<RwLock<HashMap<String, NodeInfo>>>;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let trust_store: TrustStore = Arc::new(RwLock::new(HashMap::new()));
    let node_store: NodeStore = Arc::new(RwLock::new(HashMap::new()));
    
    // Initialize demo data
    initialize_demo_data(&trust_store, &node_store).await;
    
    // Start trust assessment loop
    let trust_store_clone = trust_store.clone();
    let node_store_clone = node_store.clone();
    tokio::spawn(async move {
        trust_assessment_loop(trust_store_clone, node_store_clone).await;
    });
    
    // Setup web server
    let routes = create_routes(trust_store, node_store);
    
    info!("Starting trust monitor server on http://localhost:8080");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    
    Ok(())
}

async fn initialize_demo_data(trust_store: &TrustStore, node_store: &NodeStore) {
    let mut trust_data = trust_store.write().await;
    let mut node_data = node_store.write().await;
    
    // Initialize nodes
    let nodes = vec![
        ("edge-node-1", "aarch64", vec!["container-a", "container-b"]),
        ("edge-node-2", "riscv64", vec!["container-c", "container-d"]),
        ("cloud-node-1", "x86_64", vec!["container-e", "container-f"]),
    ];
    
    for (node_id, arch, containers) in nodes {
        node_data.insert(node_id.to_string(), NodeInfo {
            node_id: node_id.to_string(),
            architecture: arch.to_string(),
            containers: containers.iter().map(|s| s.to_string()).collect(),
            trust_level: 0.85,
        });
        
        // Initialize container trust metrics
        for container_id in containers {
            trust_data.insert(container_id.to_string(), TrustMetrics {
                container_id: container_id.to_string(),
                node_id: node_id.to_string(),
                timestamp: chrono::Utc::now(),
                integrity_score: 0.9,
                behavioral_score: 0.8,
                communication_score: 0.85,
                overall_trust: 0.85,
                status: ContainerStatus::Trusted,
            });
        }
    }
}

async fn trust_assessment_loop(trust_store: TrustStore, node_store: NodeStore) {
    let mut interval = interval(Duration::from_secs(2));
    
    loop {
        interval.tick().await;
        
        let mut trust_data = trust_store.write().await;
        let mut node_data = node_store.write().await;
        
        // Simulate trust assessment for each container
        for (container_id, metrics) in trust_data.iter_mut() {
            // Simulate some containers becoming suspicious
            if container_id == "container-b" {
                metrics.behavioral_score = (metrics.behavioral_score - 0.1).max(0.0);
                metrics.communication_score = (metrics.communication_score - 0.05).max(0.0);
            }
            
            // Recalculate overall trust
            metrics.overall_trust = (metrics.integrity_score + metrics.behavioral_score + metrics.communication_score) / 3.0;
            metrics.timestamp = chrono::Utc::now();
            
            // Update status based on trust level
            metrics.status = match metrics.overall_trust {
                t if t >= 0.8 => ContainerStatus::Trusted,
                t if t >= 0.6 => ContainerStatus::Suspicious,
                t if t >= 0.3 => ContainerStatus::Compromised,
                _ => ContainerStatus::Isolated,
            };
        }
        
        // Update node trust levels after all container updates
        for (node_id, node) in node_data.iter_mut() {
            let container_trusts: Vec<f64> = trust_data
                .values()
                .filter(|m| m.node_id == *node_id)
                .map(|m| m.overall_trust)
                .collect();
            
            if !container_trusts.is_empty() {
                node.trust_level = container_trusts.iter().sum::<f64>() / container_trusts.len() as f64;
            }
        }
        
        info!("Trust assessment completed for {} containers", trust_data.len());
    }
}

fn create_routes(trust_store: TrustStore, node_store: NodeStore) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let trust_store = warp::any().map(move || trust_store.clone());
    let node_store = warp::any().map(move || node_store.clone());
    
    let trust_metrics = warp::path("api")
        .and(warp::path("trust"))
        .and(warp::get())
        .and(trust_store)
        .and_then(get_trust_metrics);
    
    let node_info = warp::path("api")
        .and(warp::path("nodes"))
        .and(warp::get())
        .and(node_store)
        .and_then(get_node_info);
    
    let dashboard = warp::path::end()
        .and(warp::get())
        .map(|| warp::reply::html(include_str!("../static/dashboard.html")));
    
    trust_metrics.or(node_info).or(dashboard)
}

async fn get_trust_metrics(trust_store: TrustStore) -> Result<impl warp::Reply, warp::Rejection> {
    let data = trust_store.read().await;
    let metrics: Vec<TrustMetrics> = data.values().cloned().collect();
    Ok(warp::reply::json(&metrics))
}

async fn get_node_info(node_store: NodeStore) -> Result<impl warp::Reply, warp::Rejection> {
    let data = node_store.read().await;
    let nodes: Vec<NodeInfo> = data.values().cloned().collect();
    Ok(warp::reply::json(&nodes))
}
