// agent/src/lib.rs
// Core library for Health & Speed Checker

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// CORE DATA TYPES (Frozen v1 API)
// ============================================================================

/// Configuration options for a system scan.
///
/// Controls which categories of checks are performed and scan depth.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanOptions {
    /// Enable security-focused checks (firewall, ports, OS updates)
    pub security: bool,
    /// Enable performance-focused checks (processes, disk, network)
    pub performance: bool,
    /// Run quick scan (skips slow checkers like port scanning)
    pub quick: bool,
    /// Skip application-level checks
    pub exclude_apps: bool,
    /// Skip startup program analysis
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

/// Complete result of a system health & speed scan.
///
/// Contains scores, detected issues, and metadata about the scan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    /// Unique identifier for this scan (UUID v4)
    pub scan_id: String,
    /// Unix timestamp (seconds since epoch)
    pub timestamp: u64,
    /// How long the scan took to complete (milliseconds)
    pub duration_ms: u64,
    /// Calculated health and speed scores (0-100)
    pub scores: SystemScores,
    /// All issues detected during the scan
    pub issues: Vec<Issue>,
    /// Additional scan metadata
    pub details: ScanDetails,
}

/// Health and speed scores with optional deltas from previous scan.
///
/// Scores range from 0-100, where 100 is perfect health/speed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemScores {
    /// Overall system health score (0-100)
    pub health: u8,
    /// Overall system speed score (0-100)
    pub speed: u8,
    /// Change in health since last scan (-100 to +100)
    pub health_delta: Option<i8>,
    /// Change in speed since last scan (-100 to +100)
    pub speed_delta: Option<i8>,
}

/// A detected system issue with optional fix action.
///
/// # Schema (FROZEN v1 - do not modify!)
/// This schema is frozen and shared between Rust backend and TypeScript frontend.
/// Any changes will break the API contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    /// Unique identifier for this issue type (e.g., "firewall_disabled")
    pub id: String,
    /// How urgent this issue is
    pub severity: IssueSeverity,
    /// Short user-facing title (e.g., "Windows Firewall is OFF")
    pub title: String,
    /// Detailed explanation of the issue and its impact
    pub description: String,
    /// Whether this affects security, performance, privacy, or both
    pub impact_category: ImpactCategory,
    /// Optional action that can fix this issue
    pub fix: Option<FixAction>,
}

/// Severity level of a detected issue.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IssueSeverity {
    /// Urgent issue requiring immediate attention
    Critical,
    /// Important issue that should be addressed soon
    Warning,
    /// Informational item or minor optimization
    Info,
}

/// Category of impact an issue has on the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactCategory {
    /// Affects system security (firewall, updates, ports)
    Security,
    /// Affects system performance (CPU, disk, memory)
    Performance,
    /// Affects user privacy (tracking, telemetry)
    Privacy,
    /// Affects both security and performance
    Both,
}

/// An action that can be taken to fix an issue.
///
/// Can be automatic (one-click) or manual (show instructions).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixAction {
    /// Unique identifier for this fix (e.g., "enable_firewall")
    pub action_id: String,
    /// User-facing button label (e.g., "Enable Firewall")
    pub label: String,
    /// Whether this fix can run automatically without user interaction
    pub is_auto_fix: bool,
    /// Additional parameters needed for the fix (JSON)
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

/// Category of system check being performed.
#[derive(Debug, Clone, Copy)]
pub enum CheckCategory {
    /// Security-related checks (firewall, ports, updates)
    Security,
    /// Performance-related checks (CPU, disk, memory)
    Performance,
    /// Privacy-related checks (tracking, telemetry)
    Privacy,
    /// Firmware and BIOS checks
    Firmware,
    /// Threat detection (malware, suspicious processes)
    Threat,
    /// Compliance checks (regulations, standards)
    Compliance,
}

/// Context passed to checkers during a scan.
///
/// Contains scan options and will include progress reporting in the future.
pub struct ScanContext {
    /// Options for this scan
    pub options: ScanOptions,
    // TODO: Add progress reporting when needed
}

/// Core trait for all system health checkers.
///
/// # Implementation Requirements
/// - Must be `Send + Sync` for thread safety
/// - Must be synchronous (no async/await) for rusqlite compatibility
/// - Must return quickly (< 5 seconds) for good UX
///
/// # Example
/// ```ignore
/// struct MyChecker;
///
/// impl Checker for MyChecker {
///     fn name(&self) -> &'static str { "my_checker" }
///     fn category(&self) -> CheckCategory { CheckCategory::Security }
///     fn run(&self, context: &ScanContext) -> Vec<Issue> {
///         // Perform checks and return issues
///         vec![]
///     }
/// }
/// ```
pub trait Checker: Send + Sync {
    /// Unique identifier for this checker (lowercase snake_case).
    fn name(&self) -> &'static str;

    /// Category this checker belongs to.
    fn category(&self) -> CheckCategory;

    /// Run the checker and return detected issues.
    ///
    /// This method must be synchronous and should complete within a few seconds.
    /// Use `context.options` to check if this checker should be skipped.
    fn run(&self, context: &ScanContext) -> Vec<Issue>;

    /// Attempt to fix an issue detected by this checker.
    ///
    /// # Arguments
    /// * `issue_id` - The ID of the issue to fix (must match an Issue.id)
    /// * `params` - Additional parameters for the fix (JSON)
    ///
    /// # Returns
    /// * `Ok(FixResult)` - Fix succeeded or failed with details
    /// * `Err(String)` - Fix not implemented or invalid issue_id
    ///
    /// Default implementation returns "not implemented" error.
    fn fix(&self, issue_id: &str, params: &serde_json::Value) -> Result<FixResult, String> {
        Err(format!("Fix not implemented for {}", issue_id))
    }
}

// ============================================================================
// SCANNER ENGINE
// ============================================================================

/// Main orchestrator that runs all registered checkers and calculates scores.
///
/// # Thread Safety
/// The engine is designed to be wrapped in `Arc<Mutex<>>` for concurrent access
/// from Tauri commands.
///
/// # Example
/// ```ignore
/// let mut engine = ScannerEngine::new();
/// engine.register(Box::new(FirewallChecker));
/// engine.register(Box::new(DiskChecker));
///
/// let result = engine.scan(ScanOptions::default());
/// println!("Health: {}, Speed: {}", result.scores.health, result.scores.speed);
/// ```
pub struct ScannerEngine {
    checkers: Vec<Box<dyn Checker>>,
    scoring_engine: ScoringEngine,
}

impl ScannerEngine {
    /// Create a new scanner engine with no checkers registered.
    ///
    /// You must call `register()` to add checkers before scanning.
    pub fn new() -> Self {
        Self {
            checkers: Vec::new(),
            scoring_engine: ScoringEngine::default(),
        }
    }

    /// Register a checker to be run during scans.
    ///
    /// Checkers are run in the order they are registered.
    pub fn register(&mut self, checker: Box<dyn Checker>) {
        self.checkers.push(checker);
    }

    /// Run a full system scan with the specified options.
    ///
    /// # Process
    /// 1. Runs all registered checkers based on scan options
    /// 2. Collects all detected issues
    /// 3. Calculates health and speed scores
    /// 4. Returns complete ScanResult
    ///
    /// # Performance
    /// Full scan typically takes 8-28 seconds. Quick mode: 2-5 seconds.
    ///
    /// # Thread Safety
    /// This method is synchronous and thread-safe (&self, not &mut self).
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

    /// Attempt to fix an issue by delegating to the appropriate checker.
    ///
    /// # Arguments
    /// * `action_id` - The FixAction.action_id from an Issue
    /// * `params` - Additional parameters for the fix (FixAction.params)
    ///
    /// # Returns
    /// A FixResult indicating success or failure. Always returns a result,
    /// never panics.
    ///
    /// # Example
    /// ```ignore
    /// let params = serde_json::json!({});
    /// let result = engine.fix_issue("enable_firewall", &params);
    /// if result.success {
    ///     println!("Fixed: {}", result.message);
    /// }
    /// ```
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
