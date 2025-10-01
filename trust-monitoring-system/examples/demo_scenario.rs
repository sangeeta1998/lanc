use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use chrono::Utc;

// This is a demo scenario showing how the trust monitoring system works
// in a real distributed system environment

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎬 Trust Monitoring System Demo Scenario");
    println!("🎯 SCULI-Aligned Trust Assessment Framework");
    println!();
    
    // Simulate a distributed system with multiple components
    let components = vec![
        "user-service",
        "payment-service", 
        "inventory-service",
        "notification-service",
        "database-primary",
        "database-replica",
        "cache-redis",
        "message-queue",
        "load-balancer",
        "api-gateway"
    ];
    
    println!("🏗️  Simulating distributed system with {} components", components.len());
    
    // Simulate normal operation
    println!("📊 Phase 1: Normal Operation");
    simulate_normal_operation(&components).await;
    
    // Simulate performance degradation
    println!("⚠️  Phase 2: Performance Degradation");
    simulate_performance_degradation(&components).await;
    
    // Simulate security incident
    println!("🚨 Phase 3: Security Incident");
    simulate_security_incident(&components).await;
    
    // Simulate recovery
    println!("🔧 Phase 4: Recovery Process");
    simulate_recovery_process(&components).await;
    
    println!();
    println!("✅ Demo scenario completed!");
    println!("📊 Check the API endpoints to see the trust monitoring in action:");
    println!("   http://localhost:3030/status");
    println!("   http://localhost:3030/trust-scores");
    println!("   http://localhost:3030/incidents");
    println!("   http://localhost:3030/alerts");
    
    Ok(())
}

async fn simulate_normal_operation(components: &[&str]) {
    println!("   🔄 Components operating normally...");
    
    for component in components {
        println!("   ✅ {} - Trust Score: 0.85", component);
        sleep(Duration::from_millis(100)).await;
    }
    
    println!("   📈 System overall trust: 0.85");
    println!("   🟢 System health: Healthy");
    sleep(Duration::from_secs(2)).await;
}

async fn simulate_performance_degradation(components: &[&str]) {
    println!("   ⚠️  Detecting performance issues...");
    
    // Simulate database performance issues
    println!("   🔻 database-primary - Trust Score: 0.45 (Performance degradation)");
    println!("   🔻 database-replica - Trust Score: 0.50 (High latency)");
    
    // Simulate cache issues
    println!("   🔻 cache-redis - Trust Score: 0.30 (Memory pressure)");
    
    // Other components remain healthy
    for component in components {
        if !["database-primary", "database-replica", "cache-redis"].contains(component) {
            println!("   ✅ {} - Trust Score: 0.85", component);
        }
    }
    
    println!("   📉 System overall trust: 0.65");
    println!("   🟡 System health: Warning");
    println!("   🚨 Triggered actions: Enhanced monitoring, Resource scaling");
    sleep(Duration::from_secs(3)).await;
}

async fn simulate_security_incident(components: &[&str]) {
    println!("   🚨 Security incident detected!");
    
    // Simulate compromised service
    println!("   🔴 payment-service - Trust Score: 0.15 (Security breach detected)");
    println!("   🔴 api-gateway - Trust Score: 0.25 (Suspicious traffic patterns)");
    
    // Simulate cascading effects
    println!("   🔻 user-service - Trust Score: 0.40 (Dependency on compromised service)");
    println!("   🔻 notification-service - Trust Score: 0.35 (Communication with compromised service)");
    
    // Other components
    for component in components {
        if !["payment-service", "api-gateway", "user-service", "notification-service"].contains(component) {
            println!("   ✅ {} - Trust Score: 0.85", component);
        }
    }
    
    println!("   📉 System overall trust: 0.45");
    println!("   🔴 System health: Critical");
    println!("   🚨 Triggered actions:");
    println!("      - Isolated payment-service");
    println!("      - Updated firewall rules");
    println!("      - Escalated to security team");
    println!("      - Enabled enhanced monitoring");
    sleep(Duration::from_secs(4)).await;
}

async fn simulate_recovery_process(components: &[&str]) {
    println!("   🔧 Starting recovery process...");
    
    // Simulate gradual recovery
    println!("   🔄 payment-service - Trust Score: 0.25 (Recovery in progress)");
    println!("   🔄 api-gateway - Trust Score: 0.45 (Security patches applied)");
    
    sleep(Duration::from_secs(2)).await;
    
    println!("   📈 payment-service - Trust Score: 0.60 (Security review completed)");
    println!("   📈 api-gateway - Trust Score: 0.70 (Traffic patterns normalized)");
    println!("   📈 user-service - Trust Score: 0.75 (Dependency restored)");
    println!("   📈 notification-service - Trust Score: 0.80 (Communication restored)");
    
    // All components back to normal
    for component in components {
        if !["payment-service", "api-gateway", "user-service", "notification-service"].contains(component) {
            println!("   ✅ {} - Trust Score: 0.85", component);
        }
    }
    
    println!("   📊 System overall trust: 0.78");
    println!("   🟢 System health: Healthy");
    println!("   ✅ Recovery completed successfully");
    sleep(Duration::from_secs(2)).await;
}

// Example of how to integrate with the trust monitoring system
pub struct TrustMonitoringDemo {
    pub component_trust_scores: HashMap<String, f64>,
    pub system_health: String,
    pub active_incidents: Vec<String>,
}

impl TrustMonitoringDemo {
    pub fn new() -> Self {
        Self {
            component_trust_scores: HashMap::new(),
            system_health: "Unknown".to_string(),
            active_incidents: Vec::new(),
        }
    }
    
    pub async fn update_trust_score(&mut self, component_id: &str, trust_score: f64) {
        self.component_trust_scores.insert(component_id.to_string(), trust_score);
        
        // Calculate system health based on trust scores
        let avg_trust = self.component_trust_scores.values().sum::<f64>() / 
                       self.component_trust_scores.len() as f64;
        
        self.system_health = if avg_trust > 0.8 {
            "Healthy".to_string()
        } else if avg_trust > 0.5 {
            "Warning".to_string()
        } else {
            "Critical".to_string()
        };
        
        // Check for incidents
        if trust_score < 0.2 {
            let incident = format!("Critical trust score for {}: {:.2}", component_id, trust_score);
            if !self.active_incidents.contains(&incident) {
                self.active_incidents.push(incident);
            }
        }
    }
    
    pub fn get_system_status(&self) -> SystemStatus {
        let overall_trust = if self.component_trust_scores.is_empty() {
            0.0
        } else {
            self.component_trust_scores.values().sum::<f64>() / 
            self.component_trust_scores.len() as f64
        };
        
        SystemStatus {
            overall_trust,
            component_count: self.component_trust_scores.len(),
            active_incidents: self.active_incidents.len(),
            active_alerts: self.active_incidents.len(),
            system_health: self.system_health.clone(),
            last_updated: Utc::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SystemStatus {
    pub overall_trust: f64,
    pub component_count: usize,
    pub active_incidents: usize,
    pub active_alerts: usize,
    pub system_health: String,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}
