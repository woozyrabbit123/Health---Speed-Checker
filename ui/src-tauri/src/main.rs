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

    let engine = state.scanner_engine.lock().await;
    let result = engine.scan(options).await;

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
    let result = engine.fix_issue(&action_id, &params).await;

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

#[tauri::command]
async fn export_report(
    scan_id: String,
    format: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    tracing::info!("Exporting report: {} as {}", scan_id, format);

    let current_scan = state.current_scan.lock().await;

    match current_scan.as_ref() {
        Some(result) if result.scan_id == scan_id => {
            match format.as_str() {
                "json" => {
                    let json = serde_json::to_string_pretty(result)
                        .map_err(|e| format!("Failed to serialize: {}", e))?;
                    Ok(json)
                }
                "pdf" => Err("PDF export not yet implemented".to_string()),
                "html" => Err("HTML export not yet implemented".to_string()),
                _ => Err(format!("Unsupported format: {}", format)),
            }
        }
        _ => Err("Scan not found".to_string()),
    }
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
