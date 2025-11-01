// ui/src-tauri/src/main.rs
// Tauri application entry point and command handlers

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use health_speed_checker::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::State;

mod tray;

// ============================================================================
// APPLICATION STATE
// ============================================================================

pub struct AppState {
    scanner_engine: Arc<Mutex<ScannerEngine>>,
    current_scan: Arc<Mutex<Option<ScanResult>>>,
}

impl AppState {
    fn new() -> Self {
        let mut engine = ScannerEngine::new();

        // Register all checkers
        engine.register(Box::new(checkers::FirewallChecker));
        engine.register(Box::new(checkers::StartupAnalyzer));
        engine.register(Box::new(checkers::ProcessMonitor));
        engine.register(Box::new(checkers::OsUpdateChecker));
        engine.register(Box::new(checkers::PortScanner));

        // Register new checkers
        engine.register(Box::new(checkers::BloatwareDetector::new()));
        engine.register(Box::new(checkers::NetworkChecker::new()));
        engine.register(Box::new(checkers::SmartDiskChecker::new()));
        engine.register(Box::new(checkers::StorageChecker::new()));

        Self {
            scanner_engine: Arc::new(Mutex::new(engine)),
            current_scan: Arc::new(Mutex::new(None)),
        }
    }
}

// ============================================================================
// TAURI COMMANDS (UI → Rust Bridge)
// ============================================================================

#[tauri::command]
async fn scan_start(
    options: ScanOptions,
    state: State<'_, AppState>,
) -> Result<String, String> {
    tracing::info!("Starting scan with options: {:?}", options);

    let mut engine = state.scanner_engine.lock().await;
    let result = engine.scan(options);

    let scan_id = result.scan_id.clone();

    // Store the result
    let mut current_scan = state.current_scan.lock().await;
    *current_scan = Some(result);

    tracing::info!("Scan completed: {}", scan_id);
    Ok(scan_id)
}

#[tauri::command]
async fn get_scan_result(
    scan_id: String,
    state: State<'_, AppState>,
) -> Result<ScanResult, String> {
    tracing::info!("Retrieving scan result: {}", scan_id);

    let current_scan = state.current_scan.lock().await;

    match current_scan.as_ref() {
        Some(result) if result.scan_id == scan_id => Ok(result.clone()),
        Some(_) => Err("Scan ID mismatch".to_string()),
        None => Err("No scan results available".to_string()),
    }
}

#[tauri::command]
async fn fix_action(
    action_id: String,
    params: serde_json::Value,
    state: State<'_, AppState>,
) -> Result<FixResult, String> {
    tracing::info!("Executing fix action: {}", action_id);

    let engine = state.scanner_engine.lock().await;
    let result = engine.fix_issue(&action_id, &params);

    tracing::info!("Fix result: success={}", result.success);
    Ok(result)
}

#[tauri::command]
async fn get_system_info() -> Result<SystemInfo, String> {
    tracing::info!("Retrieving system information");

    Ok(SystemInfo {
        os_name: std::env::consts::OS.to_string(),
        os_version: get_os_version(),
        hostname: hostname::get()
            .ok()
            .and_then(|h| h.into_string().ok())
            .unwrap_or_else(|| "Unknown".to_string()),
    })
}

#[tauri::command]
async fn get_scan_history() -> Result<Vec<ScanHistoryItem>, String> {
    tracing::info!("Retrieving scan history");

    // TODO: Implement database query
    Ok(vec![])
}

#[derive(Debug, Deserialize)]
struct ExportOptions {
    #[serde(rename = "includeCharts")]
    include_charts: bool,
    #[serde(rename = "includeHistory")]
    include_history: bool,
}

#[tauri::command]
async fn export_report(
    scan_id: String,
    format: String,
    options: ExportOptions,
    state: State<'_, AppState>,
) -> Result<String, String> {
    tracing::info!("Exporting report: {} as {} (charts: {}, history: {})",
        scan_id, format, options.include_charts, options.include_history);

    let current_scan = state.current_scan.lock().await;

    match current_scan.as_ref() {
        Some(result) if result.scan_id == scan_id => {
            match format.as_str() {
                "json" => {
                    let json = serde_json::to_string_pretty(result)
                        .map_err(|e| format!("Failed to export as JSON: {}. The scan data may be corrupted.", e))?;
                    Ok(json)
                }
                "csv" => {
                    generate_csv_export(result)
                        .map_err(|e| format!("Failed to export as CSV: {}", e))
                }
                "html" => {
                    generate_html_export(result, &options)
                        .map_err(|e| format!("Failed to export as HTML: {}", e))
                }
                "pdf" => Err("PDF export is not yet available. Try exporting as HTML or JSON instead.".to_string()),
                _ => Err(format!("Export format '{}' is not supported. Please choose JSON, HTML, or CSV.", format)),
            }
        }
        _ => Err("Scan not found".to_string()),
    }
}

fn generate_csv_export(result: &ScanResult) -> Result<String, String> {
    let mut csv = String::from("Severity,Title,Description,Category\n");

    for issue in &result.issues {
        let severity = format!("{:?}", issue.severity);
        let title = issue.title.replace(",", ";");
        let description = issue.description.replace(",", ";").replace("\n", " ");
        let category = format!("{:?}", issue.impact_category);

        csv.push_str(&format!("{},{},{},{}\n", severity, title, description, category));
    }

    Ok(csv)
}

fn generate_html_export(result: &ScanResult, options: &ExportOptions) -> Result<String, String> {
    let html = format!(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Health & Speed Report - {}</title>
    <style>
        body {{ font-family: Arial, sans-serif; max-width: 1200px; margin: 0 auto; padding: 20px; background: #f5f5f5; }}
        .header {{ background: linear-gradient(135deg, #3b82f6, #8b5cf6); color: white; padding: 30px; border-radius: 10px; margin-bottom: 20px; }}
        .scores {{ display: flex; gap: 20px; margin-bottom: 30px; }}
        .score-card {{ flex: 1; background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
        .score-value {{ font-size: 48px; font-weight: bold; }}
        .issues {{ background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
        .issue {{ border-left: 4px solid #ccc; padding: 15px; margin-bottom: 15px; background: #f9f9f9; }}
        .issue.critical {{ border-color: #ef4444; }}
        .issue.warning {{ border-color: #f59e0b; }}
        .issue.info {{ border-color: #3b82f6; }}
        .footer {{ margin-top: 30px; text-align: center; color: #666; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>Health & Speed Report</h1>
        <p>Scan ID: {}</p>
        <p>Generated: {}</p>
    </div>

    <div class="scores">
        <div class="score-card">
            <h3>Health Score</h3>
            <div class="score-value" style="color: {};">{}</div>
            <p>out of 100</p>
        </div>
        <div class="score-card">
            <h3>Speed Score</h3>
            <div class="score-value" style="color: {};">{}</div>
            <p>out of 100</p>
        </div>
    </div>

    <div class="issues">
        <h2>Issues Found ({})</h2>
        {}
    </div>

    <div class="footer">
        <p>Generated with Health & Speed Checker</p>
        {}
    </div>
</body>
</html>"#,
        result.scan_id,
        result.scan_id,
        chrono::DateTime::from_timestamp(result.timestamp as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Unknown".to_string()),
        get_score_color(result.scores.health),
        result.scores.health,
        get_score_color(result.scores.speed),
        result.scores.speed,
        result.issues.len(),
        result.issues.iter().map(|issue| format!(
            r#"<div class="issue {}">
                <h4>{}</h4>
                <p>{}</p>
                <small>Category: {:?} | Severity: {:?}</small>
            </div>"#,
            format!("{:?}", issue.severity).to_lowercase(),
            issue.title,
            issue.description,
            issue.impact_category,
            issue.severity
        )).collect::<Vec<_>>().join("\n"),
        if options.include_history {
            "<p><em>Historical data would be included here</em></p>"
        } else { "" }
    );

    Ok(html)
}

fn get_score_color(score: u8) -> &'static str {
    if score >= 80 { "#22c55e" } // green
    else if score >= 60 { "#f59e0b" } // yellow
    else { "#ef4444" } // red
}

// ============================================================================
// HELPER TYPES
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct SystemInfo {
    os_name: String,
    os_version: String,
    hostname: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ScanHistoryItem {
    scan_id: String,
    timestamp: u64,
    health_score: u8,
    speed_score: u8,
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

fn get_os_version() -> String {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("cmd").args(&["/c", "ver"]).output() {
            return String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("sw_vers").arg("-productVersion").output() {
            return String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("uname").arg("-r").output() {
            return String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
    }

    "Unknown".to_string()
}

// ============================================================================
// MAIN APPLICATION
// ============================================================================

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    tracing::info!("Starting Health & Speed Checker...");

    // Build the Tauri application
    tauri::Builder::default()
        .manage(AppState::new())
        .system_tray(tray::create_tray())
        .on_system_tray_event(tray::handle_tray_event)
        .invoke_handler(tauri::generate_handler![
            scan_start,
            get_scan_result,
            fix_action,
            get_system_info,
            get_scan_history,
            export_report,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    tracing::info!("Application shutdown");
}
