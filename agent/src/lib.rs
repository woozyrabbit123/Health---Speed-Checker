// agent/src/lib.rs
// Core library for Health & Speed Checker

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// CORE DATA TYPES (Frozen v1 API)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanOptions {
    pub security: bool,
    pub performance: bool,
    pub quick: bool,
    pub exclude_apps: bool,
    pub exclude_startup: bool,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            security: true,
            performance: true,
            quick: false,
            exclude_apps: false,
            exclude_startup: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub scan_id: String,
    pub timestamp: u64,
    pub duration_ms: u64,
    pub scores: SystemScores,
    pub issues: Vec<Issue>,
    pub details: ScanDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemScores {
    pub health: u8,
    pub speed: u8,
    pub health_delta: Option<i8>,
    pub speed_delta: Option<i8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: String,
    pub severity: IssueSeverity,
    pub title: String,
    pub description: String,
    pub impact_category: ImpactCategory,
    pub fix: Option<FixAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IssueSeverity {
    Critical,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactCategory {
    Security,
    Performance,
    Privacy,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixAction {
    pub action_id: String,
    pub label: String,
    pub is_auto_fix: bool,
    pub params: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanDetails {
    pub security: SecurityDetails,
    pub performance: PerformanceDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityDetails {
    pub os_update_status: OsUpdateStatus,
    pub firewall_status: FirewallStatus,
    pub open_ports: Vec<PortInfo>,
    pub vulnerable_apps: Vec<VulnerableApp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceDetails {
    pub system_metrics: SystemMetrics,
    pub top_processes: Vec<ProcessInfo>,
    pub startup_items: Vec<StartupItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsUpdateStatus {
    pub is_current: bool,
    pub current_build: String,
    pub latest_build: Option<String>,
    pub pending_updates: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallStatus {
    pub is_active: bool,
    pub provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortInfo {
    pub port: u16,
    pub protocol: String,
    pub service: Option<String>,
    pub process: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerableApp {
    pub name: String,
    pub version: String,
    pub cve_id: String,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub memory_used_gb: f32,
    pub memory_total_gb: f32,
    pub disk_used_gb: f32,
    pub disk_total_gb: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f32,
    pub memory_mb: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupItem {
    pub name: String,
    pub path: String,
    pub estimated_delay_ms: u32,
    pub can_disable: bool,
}

// ============================================================================
// PROGRESS EVENTS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ProgressEvent {
    Started { scan_id: String },
    TaskChanged { message: String },
    ProgressUpdate { percent: u8 },
    IssueFound(Issue),
    Complete { scan_id: String, duration_ms: u64 },
    Error { message: String },
}

// ============================================================================
// FIX SYSTEM
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixResult {
    pub success: bool,
    pub message: String,
    pub rollback_available: bool,
    pub restore_point_id: Option<String>,
}

impl FixResult {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: message.into(),
            rollback_available: false,
            restore_point_id: None,
        }
    }

    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            rollback_available: false,
            restore_point_id: None,
        }
    }
}

// ============================================================================
// CHECKER TRAIT (Plugin System)
// ============================================================================

#[derive(Debug, Clone, Copy)]
pub enum CheckCategory {
    Security,
    Performance,
    Privacy,
    Firmware,
    Threat,
    Compliance,
}

pub struct ScanContext {
    pub options: ScanOptions,
    // TODO: Add progress reporting when needed
}

pub trait Checker: Send + Sync {
    fn name(&self) -> &'static str;
    fn category(&self) -> CheckCategory;

    fn run(&self, context: &ScanContext) -> Vec<Issue>;

    fn fix(&self, issue_id: &str, params: &serde_json::Value) -> Result<FixResult, String> {
        Err(format!("Fix not implemented for {}", issue_id))
    }
}

// ============================================================================
// SCANNER ENGINE
// ============================================================================

pub struct ScannerEngine {
    checkers: Vec<Box<dyn Checker>>,
    scoring_engine: ScoringEngine,
}

impl ScannerEngine {
    pub fn new() -> Self {
        Self {
            checkers: Vec::new(),
            scoring_engine: ScoringEngine::default(),
        }
    }

    pub fn register(&mut self, checker: Box<dyn Checker>) {
        self.checkers.push(checker);
    }

    pub fn scan(&self, options: ScanOptions) -> ScanResult {
        let scan_id = uuid::Uuid::new_v4().to_string();
        let start_time = std::time::Instant::now();
        let timestamp = chrono::Utc::now().timestamp() as u64;

        let context = ScanContext {
            options: options.clone(),
        };

        let mut all_issues = Vec::new();

        // Run all checkers based on options
        for checker in &self.checkers {
            let should_run = match checker.category() {
                CheckCategory::Security => options.security,
                CheckCategory::Performance => options.performance,
                _ => true,
            };

            if should_run {
                let issues = checker.run(&context);
                all_issues.extend(issues);
            }
        }

        // Sort issues by priority
        all_issues.sort_by_key(|issue| {
            let severity_score = match issue.severity {
                IssueSeverity::Critical => 0,
                IssueSeverity::Warning => 1,
                IssueSeverity::Info => 2,
            };
            severity_score
        });

        // Calculate scores
        let scores = self.scoring_engine.calculate_scores(&all_issues);

        // Build details (simplified for now)
        let details = ScanDetails {
            security: SecurityDetails {
                os_update_status: OsUpdateStatus {
                    is_current: true,
                    current_build: "Unknown".to_string(),
                    latest_build: None,
                    pending_updates: 0,
                },
                firewall_status: FirewallStatus {
                    is_active: true,
                    provider: "Unknown".to_string(),
                },
                open_ports: vec![],
                vulnerable_apps: vec![],
            },
            performance: PerformanceDetails {
                system_metrics: SystemMetrics {
                    cpu_usage: 0.0,
                    memory_used_gb: 0.0,
                    memory_total_gb: 16.0,
                    disk_used_gb: 0.0,
                    disk_total_gb: 256.0,
                },
                top_processes: vec![],
                startup_items: vec![],
            },
        };

        ScanResult {
            scan_id,
            timestamp,
            duration_ms: start_time.elapsed().as_millis() as u64,
            scores,
            issues: all_issues,
            details,
        }
    }

    pub fn fix_issue(&self, action_id: &str, params: &serde_json::Value) -> FixResult {
        // Find the checker that can handle this fix
        for checker in &self.checkers {
            if let Ok(result) = checker.fix(action_id, params) {
                return result;
            }
        }

        FixResult::failure(format!("No handler found for action: {}", action_id))
    }
}

// ============================================================================
// SCORING ENGINE
// ============================================================================

pub struct ScoringEngine {
    weights: HashMap<String, f32>,
}

impl Default for ScoringEngine {
    fn default() -> Self {
        let mut weights = HashMap::new();
        weights.insert("windows_update_pending".to_string(), 1.5);
        weights.insert("firewall_disabled".to_string(), 2.0);
        weights.insert("rdp_port_open".to_string(), 2.0);
        weights.insert("excessive_startup_items".to_string(), 0.8);

        Self { weights }
    }
}

impl ScoringEngine {
    pub fn calculate_scores(&self, issues: &[Issue]) -> SystemScores {
        let mut health_score = 100.0;
        let mut speed_score = 100.0;

        for issue in issues {
            let weight = self.weights.get(&issue.id).unwrap_or(&1.0);

            match issue.impact_category {
                ImpactCategory::Security => {
                    health_score -= match issue.severity {
                        IssueSeverity::Critical => 20.0 * weight,
                        IssueSeverity::Warning => 10.0 * weight,
                        IssueSeverity::Info => 2.0 * weight,
                    };
                }
                ImpactCategory::Performance => {
                    speed_score -= match issue.severity {
                        IssueSeverity::Critical => 25.0 * weight,
                        IssueSeverity::Warning => 12.0 * weight,
                        IssueSeverity::Info => 3.0 * weight,
                    };
                }
                ImpactCategory::Both => {
                    health_score -= 15.0 * weight;
                    speed_score -= 15.0 * weight;
                }
                _ => {}
            }
        }

        SystemScores {
            health: health_score.max(0.0).min(100.0) as u8,
            speed: speed_score.max(0.0).min(100.0) as u8,
            health_delta: None, // TODO: Calculate from previous scan
            speed_delta: None,
        }
    }
}

// Re-export commonly used dependencies
pub use serde_json;
pub use uuid;

// Export checker modules
pub mod checkers;
