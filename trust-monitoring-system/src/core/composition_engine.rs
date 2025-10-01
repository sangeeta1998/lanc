use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Trust composition engine for system-of-systems trust assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustGraph {
    pub nodes: HashMap<String, TrustNode>,
    pub edges: HashMap<String, TrustEdge>,
    pub dependencies: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustNode {
    pub id: String,
    pub trust_score: f64,
    pub component_type: ComponentType,
    pub security_posture: SecurityPosture,
    pub last_updated: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustEdge {
    pub from: String,
    pub to: String,
    pub relationship_type: RelationshipType,
    pub trust_weight: f64,
    pub data_flow_volume: f64,
    pub criticality: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentType {
    Microservice,
    Database,
    API,
    LoadBalancer,
    MessageQueue,
    Cache,
    ExternalService,
    LegacySystem,
    EdgeDevice,
    Container,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPosture {
    pub vulnerability_score: f64,
    pub patch_status: f64,
    pub compliance_score: f64,
    pub encryption_status: f64,
    pub access_control_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    DataFlow,
    Dependency,
    Communication,
    Control,
    Monitoring,
    Backup,
    LoadBalancing,
}

/// Trust propagation algorithms for compositional analysis
pub struct CompositionEngine {
    trust_graph: Arc<RwLock<TrustGraph>>,
    propagation_models: Arc<RwLock<HashMap<String, Box<dyn TrustPropagationModel + Send + Sync>>>>,
    composition_rules: Arc<RwLock<Vec<CompositionRule>>>,
}

pub trait TrustPropagationModel {
    fn propagate_trust(&self, graph: &TrustGraph, source: &str) -> HashMap<String, f64>;
    fn get_model_name(&self) -> String;
}

/// Weighted average propagation model
pub struct WeightedAverageModel {
    pub name: String,
}

impl TrustPropagationModel for WeightedAverageModel {
    fn propagate_trust(&self, graph: &TrustGraph, source: &str) -> HashMap<String, f64> {
        let mut propagated_scores = HashMap::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        queue.push_back((source.to_string(), 1.0));
        visited.insert(source.to_string());
        
        while let Some((current_id, current_trust)) = queue.pop_front() {
            propagated_scores.insert(current_id.clone(), current_trust);
            
            // Find all outgoing edges from current node
            for (edge_id, edge) in &graph.edges {
                if edge.from == current_id {
                    let target_id = &edge.to;
                    if !visited.contains(target_id) {
                        let propagated_trust = current_trust * edge.trust_weight;
                        queue.push_back((target_id.clone(), propagated_trust));
                        visited.insert(target_id.clone());
                    }
                }
            }
        }
        
        propagated_scores
    }
    
    fn get_model_name(&self) -> String {
        self.name.clone()
    }
}

/// Minimum trust propagation model (pessimistic)
pub struct MinimumTrustModel {
    pub name: String,
}

impl TrustPropagationModel for MinimumTrustModel {
    fn propagate_trust(&self, graph: &TrustGraph, source: &str) -> HashMap<String, f64> {
        let mut propagated_scores = HashMap::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        queue.push_back((source.to_string(), 1.0));
        visited.insert(source.to_string());
        
        while let Some((current_id, current_trust)) = queue.pop_front() {
            propagated_scores.insert(current_id.clone(), current_trust);
            
            // Find all outgoing edges from current node
            for (edge_id, edge) in &graph.edges {
                if edge.from == current_id {
                    let target_id = &edge.to;
                    let propagated_trust = current_trust.min(edge.trust_weight);
                    
                    if !visited.contains(target_id) || 
                       propagated_scores.get(target_id).map_or(true, |&existing| propagated_trust < existing) {
                        queue.push_back((target_id.clone(), propagated_trust));
                        visited.insert(target_id.clone());
                        propagated_scores.insert(target_id.clone(), propagated_trust);
                    }
                }
            }
        }
        
        propagated_scores
    }
    
    fn get_model_name(&self) -> String {
        self.name.clone()
    }
}

/// Bayesian network propagation model
pub struct BayesianPropagationModel {
    pub name: String,
    pub conditional_probabilities: HashMap<String, f64>,
}

impl TrustPropagationModel for BayesianPropagationModel {
    fn propagate_trust(&self, graph: &TrustGraph, source: &str) -> HashMap<String, f64> {
        let mut propagated_scores = HashMap::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        queue.push_back((source.to_string(), 1.0));
        visited.insert(source.to_string());
        
        while let Some((current_id, current_trust)) = queue.pop_front() {
            propagated_scores.insert(current_id.clone(), current_trust);
            
            // Find all outgoing edges from current node
            for (edge_id, edge) in &graph.edges {
                if edge.from == current_id {
                    let target_id = &edge.to;
                    if !visited.contains(target_id) {
                        // Use conditional probability for trust propagation
                        let conditional_prob = self.conditional_probabilities
                            .get(&format!("{}->{}", current_id, target_id))
                            .unwrap_or(&0.5);
                        
                        let propagated_trust = current_trust * edge.trust_weight * conditional_prob;
                        queue.push_back((target_id.clone(), propagated_trust));
                        visited.insert(target_id.clone());
                    }
                }
            }
        }
        
        propagated_scores
    }
    
    fn get_model_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionRule {
    pub rule_id: String,
    pub rule_type: CompositionRuleType,
    pub conditions: Vec<CompositionCondition>,
    pub actions: Vec<CompositionAction>,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompositionRuleType {
    TrustThreshold,
    DependencyFailure,
    SecurityViolation,
    PerformanceDegradation,
    ComplianceViolation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionCondition {
    pub component_type: Option<ComponentType>,
    pub trust_threshold: Option<f64>,
    pub security_condition: Option<SecurityCondition>,
    pub performance_condition: Option<PerformanceCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCondition {
    pub vulnerability_threshold: f64,
    pub patch_status_required: bool,
    pub compliance_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceCondition {
    pub response_time_threshold: f64,
    pub error_rate_threshold: f64,
    pub availability_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionAction {
    pub action_type: CompositionActionType,
    pub target_components: Vec<String>,
    pub parameters: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompositionActionType {
    IsolateComponent,
    ReduceTrustWeight,
    TriggerAlert,
    UpdateSecurityPolicy,
    ScaleResources,
    FailoverToBackup,
}

impl CompositionEngine {
    pub fn new() -> Self {
        Self {
            trust_graph: Arc::new(RwLock::new(TrustGraph {
                nodes: HashMap::new(),
                edges: HashMap::new(),
                dependencies: HashMap::new(),
            })),
            propagation_models: Arc::new(RwLock::new(HashMap::new())),
            composition_rules: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Add a component to the trust graph
    pub async fn add_component(&self, node: TrustNode) {
        let mut graph = self.trust_graph.write().await;
        graph.nodes.insert(node.id.clone(), node);
    }

    /// Add a relationship between components
    pub async fn add_relationship(&self, edge: TrustEdge) {
        let mut graph = self.trust_graph.write().await;
        let edge_id = format!("{}->{}", edge.from, edge.to);
        graph.edges.insert(edge_id, edge);
        
        // Update dependencies
        graph.dependencies.entry(edge.from.clone())
            .or_insert_with(Vec::new)
            .push(edge.to.clone());
    }

    /// Calculate compositional trust score for the entire system
    pub async fn calculate_system_trust(&self, root_components: &[String]) -> SystemTrustScore {
        let graph = self.trust_graph.read().await;
        let models = self.propagation_models.read().await;
        
        let mut system_scores = HashMap::new();
        
        // Use different propagation models for comprehensive analysis
        for (model_name, model) in models.iter() {
            for root in root_components {
                let propagated = model.propagate_trust(&graph, root);
                for (component_id, score) in propagated {
                    system_scores.entry(component_id)
                        .or_insert_with(Vec::new)
                        .push(score);
                }
            }
        }
        
        // Calculate weighted average of all propagation results
        let mut final_scores = HashMap::new();
        for (component_id, scores) in system_scores {
            let average_score = scores.iter().sum::<f64>() / scores.len() as f64;
            final_scores.insert(component_id, average_score);
        }
        
        // Calculate overall system trust
        let overall_trust = if final_scores.is_empty() {
            0.0
        } else {
            final_scores.values().sum::<f64>() / final_scores.len() as f64
        };
        
        SystemTrustScore {
            overall_trust,
            component_scores: final_scores,
            critical_paths: self.identify_critical_paths(&graph, root_components).await,
            weak_links: self.identify_weak_links(&graph, &final_scores).await,
            timestamp: Utc::now(),
        }
    }

    /// Identify critical paths in the system
    async fn identify_critical_paths(&self, graph: &TrustGraph, root_components: &[String]) -> Vec<CriticalPath> {
        let mut critical_paths = Vec::new();
        
        for root in root_components {
            let mut visited = HashSet::new();
            let mut path = Vec::new();
            self.dfs_critical_paths(graph, root, &mut visited, &mut path, &mut critical_paths);
        }
        
        critical_paths
    }

    fn dfs_critical_paths(
        &self,
        graph: &TrustGraph,
        current: &str,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
        critical_paths: &mut Vec<CriticalPath>,
    ) {
        if visited.contains(current) {
            // Found a cycle - this is a critical path
            if let Some(cycle_start) = path.iter().position(|x| x == current) {
                let cycle = path[cycle_start..].to_vec();
                critical_paths.push(CriticalPath {
                    path: cycle,
                    criticality: 1.0,
                    description: "Circular dependency detected".to_string(),
                });
            }
            return;
        }
        
        visited.insert(current.to_string());
        path.push(current.to_string());
        
        // Find all outgoing edges
        for edge in graph.edges.values() {
            if edge.from == current {
                self.dfs_critical_paths(graph, &edge.to, visited, path, critical_paths);
            }
        }
        
        path.pop();
        visited.remove(current);
    }

    /// Identify weak links in the system
    async fn identify_weak_links(&self, graph: &TrustGraph, scores: &HashMap<String, f64>) -> Vec<WeakLink> {
        let mut weak_links = Vec::new();
        
        for (component_id, score) in scores {
            if *score < 0.3 {
                weak_links.push(WeakLink {
                    component_id: component_id.clone(),
                    trust_score: *score,
                    impact_assessment: self.assess_impact(graph, component_id).await,
                    mitigation_suggestions: self.get_mitigation_suggestions(component_id, *score),
                });
            }
        }
        
        weak_links
    }

    async fn assess_impact(&self, graph: &TrustGraph, component_id: &str) -> ImpactAssessment {
        let mut affected_components = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(component_id.to_string());
        
        while let Some(current) = queue.pop_front() {
            if affected_components.contains(&current) {
                continue;
            }
            affected_components.insert(current.clone());
            
            // Find all components that depend on this one
            for edge in graph.edges.values() {
                if edge.from == current {
                    queue.push_back(edge.to.clone());
                }
            }
        }
        
        ImpactAssessment {
            affected_components: affected_components.into_iter().collect(),
            severity: if affected_components.len() > 10 { 1.0 } else { 0.5 },
            business_impact: "High".to_string(),
        }
    }

    fn get_mitigation_suggestions(&self, component_id: &str, score: f64) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        if score < 0.1 {
            suggestions.push("Immediate isolation required".to_string());
            suggestions.push("Emergency security review".to_string());
        } else if score < 0.3 {
            suggestions.push("Enhanced monitoring required".to_string());
            suggestions.push("Security patch deployment".to_string());
        } else if score < 0.5 {
            suggestions.push("Regular security assessment".to_string());
            suggestions.push("Performance optimization".to_string());
        }
        
        suggestions
    }

    /// Add a trust propagation model
    pub async fn add_propagation_model(&self, name: String, model: Box<dyn TrustPropagationModel + Send + Sync>) {
        let mut models = self.propagation_models.write().await;
        models.insert(name, model);
    }

    /// Add a composition rule
    pub async fn add_composition_rule(&self, rule: CompositionRule) {
        let mut rules = self.composition_rules.write().await;
        rules.push(rule);
        rules.sort_by_key(|r| r.priority);
    }

    /// Evaluate composition rules and trigger actions
    pub async fn evaluate_composition_rules(&self) -> Vec<CompositionAction> {
        let graph = self.trust_graph.read().await;
        let rules = self.composition_rules.read().await;
        let mut triggered_actions = Vec::new();
        
        for rule in rules.iter() {
            if self.evaluate_rule_conditions(&graph, rule).await {
                triggered_actions.extend(rule.actions.clone());
            }
        }
        
        triggered_actions
    }

    async fn evaluate_rule_conditions(&self, graph: &TrustGraph, rule: &CompositionRule) -> bool {
        for condition in &rule.conditions {
            if let Some(trust_threshold) = condition.trust_threshold {
                for node in graph.nodes.values() {
                    if node.trust_score < trust_threshold {
                        return true;
                    }
                }
            }
            
            if let Some(security_condition) = &condition.security_condition {
                for node in graph.nodes.values() {
                    if node.security_posture.vulnerability_score > security_condition.vulnerability_threshold {
                        return true;
                    }
                }
            }
        }
        
        false
    }

    /// Get trust propagation analysis
    pub async fn get_propagation_analysis(&self, source: &str) -> PropagationAnalysis {
        let graph = self.trust_graph.read().await;
        let models = self.propagation_models.read().await;
        
        let mut analysis_results = HashMap::new();
        
        for (model_name, model) in models.iter() {
            let propagated = model.propagate_trust(&graph, source);
            analysis_results.insert(model_name.clone(), propagated);
        }
        
        PropagationAnalysis {
            source_component: source.to_string(),
            propagation_results: analysis_results,
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemTrustScore {
    pub overall_trust: f64,
    pub component_scores: HashMap<String, f64>,
    pub critical_paths: Vec<CriticalPath>,
    pub weak_links: Vec<WeakLink>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriticalPath {
    pub path: Vec<String>,
    pub criticality: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeakLink {
    pub component_id: String,
    pub trust_score: f64,
    pub impact_assessment: ImpactAssessment,
    pub mitigation_suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAssessment {
    pub affected_components: Vec<String>,
    pub severity: f64,
    pub business_impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropagationAnalysis {
    pub source_component: String,
    pub propagation_results: HashMap<String, HashMap<String, f64>>,
    pub timestamp: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_composition_engine() {
        let engine = CompositionEngine::new();
        
        // Add test components
        let node1 = TrustNode {
            id: "service1".to_string(),
            trust_score: 0.8,
            component_type: ComponentType::Microservice,
            security_posture: SecurityPosture {
                vulnerability_score: 0.1,
                patch_status: 0.9,
                compliance_score: 0.8,
                encryption_status: 0.9,
                access_control_score: 0.8,
            },
            last_updated: Utc::now(),
            metadata: HashMap::new(),
        };
        
        engine.add_component(node1).await;
        
        // Add test relationship
        let edge = TrustEdge {
            from: "service1".to_string(),
            to: "database1".to_string(),
            relationship_type: RelationshipType::DataFlow,
            trust_weight: 0.9,
            data_flow_volume: 1000.0,
            criticality: 0.8,
        };
        
        // Add weighted average model
        let weighted_model = WeightedAverageModel {
            name: "weighted_average".to_string(),
        };
        engine.add_propagation_model("weighted".to_string(), Box::new(weighted_model)).await;
        
        // Test system trust calculation
        let system_trust = engine.calculate_system_trust(&["service1".to_string()]).await;
        assert!(system_trust.overall_trust >= 0.0 && system_trust.overall_trust <= 1.0);
    }
}
