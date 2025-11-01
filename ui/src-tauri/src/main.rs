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
                "pdf" => {
                    generate_pdf_export(result)
                        .map_err(|e| format!("Failed to export as PDF: {}", e))
                },
                _ => Err(format!("Export format '{}' is not supported. Please choose JSON, HTML, or CSV.", format)),
            }
        }
        _ => Err("Scan not found".to_string()),
    }
}

fn generate_csv_export(result: &ScanResult) -> Result<String, String> {
    let mut csv = String::new();

    // Header section
    csv.push_str("Health & Speed Checker - Scan Report\n");
    csv.push_str(&format!("Scan ID,{}\n", result.scan_id));
    csv.push_str(&format!("Timestamp,{}\n",
        chrono::DateTime::from_timestamp(result.timestamp as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Unknown".to_string())));
    csv.push_str(&format!("Duration (ms),{}\n", result.duration_ms));
    csv.push_str(&format!("Health Score,{}\n", result.scores.health));
    csv.push_str(&format!("Speed Score,{}\n", result.scores.speed));
    csv.push_str(&format!("Total Issues,{}\n", result.issues.len()));
    csv.push_str("\n");

    // Issues table
    csv.push_str("Issue ID,Severity,Title,Description,Impact Category,Fixable,Fix Label\n");

    for issue in &result.issues {
        let severity = format!("{:?}", issue.severity);
        let title = escape_csv_field(&issue.title);
        let description = escape_csv_field(&issue.description);
        let category = format!("{:?}", issue.impact_category);
        let fixable = if issue.fix.is_some() { "Yes" } else { "No" };
        let fix_label = issue.fix.as_ref()
            .map(|f| escape_csv_field(&f.label))
            .unwrap_or_else(|| "\"\"".to_string());

        csv.push_str(&format!("\"{}\",{},{},{},{},{},{}\n",
            issue.id, severity, title, description, category, fixable, fix_label));
    }

    Ok(csv)
}

fn escape_csv_field(field: &str) -> String {
    let escaped = field.replace("\"", "\"\"").replace("\n", " ").replace("\r", "");
    format!("\"{}\"", escaped)
}

fn generate_html_export(result: &ScanResult, options: &ExportOptions) -> Result<String, String> {
    let timestamp_str = chrono::DateTime::from_timestamp(result.timestamp as i64, 0)
        .map(|dt| dt.format("%B %d, %Y at %H:%M:%S").to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let duration_str = if result.duration_ms < 1000 {
        format!("{}ms", result.duration_ms)
    } else {
        format!("{:.1}s", result.duration_ms as f64 / 1000.0)
    };

    // Count issues by severity
    let critical_count = result.issues.iter().filter(|i| matches!(i.severity, health_speed_checker::IssueSeverity::Critical)).count();
    let warning_count = result.issues.iter().filter(|i| matches!(i.severity, health_speed_checker::IssueSeverity::Warning)).count();
    let info_count = result.issues.iter().filter(|i| matches!(i.severity, health_speed_checker::IssueSeverity::Info)).count();

    let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Health & Speed Report - {}</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
               background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
               padding: 40px 20px; min-height: 100vh; }}
        .container {{ max-width: 1200px; margin: 0 auto; background: white; border-radius: 16px; overflow: hidden; box-shadow: 0 20px 60px rgba(0,0,0,0.3); }}
        .header {{ background: linear-gradient(135deg, #3b82f6, #8b5cf6); color: white; padding: 40px; }}
        .header h1 {{ font-size: 32px; margin-bottom: 8px; }}
        .header .meta {{ opacity: 0.9; font-size: 14px; }}
        .header .meta span {{ margin-right: 20px; }}

        .summary {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px; padding: 30px; background: #f8fafc; }}
        .summary-card {{ background: white; padding: 24px; border-radius: 12px; box-shadow: 0 2px 8px rgba(0,0,0,0.08); }}
        .summary-card h3 {{ font-size: 14px; color: #64748b; text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 12px; }}
        .summary-card .value {{ font-size: 36px; font-weight: bold; margin-bottom: 4px; }}
        .summary-card .subtitle {{ font-size: 13px; color: #94a3b8; }}

        .stats {{ display: flex; gap: 12px; padding: 0 30px; }}
        .stat-badge {{ flex: 1; padding: 12px; border-radius: 8px; text-align: center; font-size: 14px; font-weight: 600; }}
        .stat-badge.critical {{ background: #fef2f2; color: #dc2626; border: 1px solid #fecaca; }}
        .stat-badge.warning {{ background: #fffbeb; color: #d97706; border: 1px solid #fde68a; }}
        .stat-badge.info {{ background: #eff6ff; color: #2563eb; border: 1px solid #bfdbfe; }}

        .content {{ padding: 30px; }}
        .section {{ margin-bottom: 30px; }}
        .section h2 {{ font-size: 24px; color: #1e293b; margin-bottom: 20px; padding-bottom: 10px; border-bottom: 2px solid #e2e8f0; }}

        .issue {{ border-left: 4px solid #cbd5e1; padding: 20px; margin-bottom: 16px; background: #f8fafc; border-radius: 0 8px 8px 0; transition: transform 0.2s; }}
        .issue:hover {{ transform: translateX(4px); box-shadow: 0 4px 12px rgba(0,0,0,0.1); }}
        .issue.critical {{ border-color: #dc2626; background: #fef2f2; }}
        .issue.warning {{ border-color: #ea580c; background: #fff7ed; }}
        .issue.info {{ border-color: #2563eb; background: #eff6ff; }}
        .issue h4 {{ font-size: 18px; color: #0f172a; margin-bottom: 8px; }}
        .issue p {{ color: #475569; line-height: 1.6; margin-bottom: 12px; }}
        .issue .meta {{ display: flex; gap: 16px; font-size: 13px; color: #64748b; }}
        .issue .badge {{ display: inline-flex; align-items: center; padding: 4px 10px; border-radius: 12px; font-size: 11px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.5px; }}
        .issue .badge.critical {{ background: #dc2626; color: white; }}
        .issue .badge.warning {{ background: #ea580c; color: white; }}
        .issue .badge.info {{ background: #2563eb; color: white; }}
        .issue .fix-available {{ color: #059669; font-weight: 600; }}

        .footer {{ text-align: center; padding: 30px; background: #f8fafc; color: #64748b; font-size: 13px; border-top: 1px solid #e2e8f0; }}

        @media print {{
            body {{ background: white; padding: 0; }}
            .container {{ box-shadow: none; }}
            .issue {{ page-break-inside: avoid; }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🏥 System Health & Speed Report</h1>
            <div class="meta">
                <span>📅 {}</span>
                <span>⏱️ Scan completed in {}</span>
                <span>🔍 {} issues detected</span>
            </div>
        </div>

        <div class="summary">
            <div class="summary-card">
                <h3>Health Score</h3>
                <div class="value" style="color: {};">{}</div>
                <div class="subtitle">out of 100</div>
            </div>
            <div class="summary-card">
                <h3>Speed Score</h3>
                <div class="value" style="color: {};">{}</div>
                <div class="subtitle">out of 100</div>
            </div>
            <div class="summary-card">
                <h3>Scan Duration</h3>
                <div class="value" style="color: #6366f1; font-size: 28px;">{}</div>
                <div class="subtitle">total time</div>
            </div>
        </div>

        <div class="stats">
            <div class="stat-badge critical">{} Critical</div>
            <div class="stat-badge warning">{} Warnings</div>
            <div class="stat-badge info">{} Info</div>
        </div>

        <div class="content">
            <div class="section">
                <h2>📋 Detected Issues</h2>
                {}
            </div>
        </div>

        <div class="footer">
            <p><strong>Generated with Health & Speed Checker</strong></p>
            <p style="margin-top: 8px;">Scan ID: {} | {}</p>
            {}
        </div>
    </div>
</body>
</html>"#,
        result.scan_id,
        timestamp_str,
        duration_str,
        result.issues.len(),
        get_score_color(result.scores.health),
        result.scores.health,
        get_score_color(result.scores.speed),
        result.scores.speed,
        duration_str,
        critical_count,
        warning_count,
        info_count,
        if result.issues.is_empty() {
            r#"<div style="text-align: center; padding: 40px; color: #22c55e;">
                <h3 style="font-size: 24px; margin-bottom: 8px;">✅ All Clear!</h3>
                <p style="color: #64748b;">No issues detected. Your system is running smoothly.</p>
            </div>"#.to_string()
        } else {
            result.issues.iter().map(|issue| {
                let severity_class = format!("{:?}", issue.severity).to_lowercase();
                let fix_info = if let Some(fix) = &issue.fix {
                    format!(r#"<span class="fix-available">✓ {}</span>"#, fix.label)
                } else {
                    String::new()
                };

                format!(r#"<div class="issue {}">
                    <h4>{}</h4>
                    <p>{}</p>
                    <div class="meta">
                        <span class="badge {}">{:?}</span>
                        <span>Impact: {:?}</span>
                        {}
                    </div>
                </div>"#,
                    severity_class,
                    issue.title,
                    issue.description,
                    severity_class,
                    issue.severity,
                    issue.impact_category,
                    fix_info
                )
            }).collect::<Vec<_>>().join("\n")
        },
        result.scan_id,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
        if options.include_history {
            "<p style=\"margin-top: 12px; font-style: italic;\">📊 Historical trend data: Coming soon</p>"
        } else { "" }
    );

    Ok(html)
}

fn generate_pdf_export(result: &ScanResult) -> Result<String, String> {
    use printpdf::*;
    use std::fs::File;
    use std::io::BufWriter;

    // Create PDF document
    let (doc, page1, layer1) = PdfDocument::new(
        "Health & Speed Checker Report",
        Mm(210.0),  // A4 width
        Mm(297.0),  // A4 height
        "Layer 1",
    );

    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Load fonts
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| format!("Failed to load font: {}", e))?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|e| format!("Failed to load bold font: {}", e))?;

    let mut y_position = 270.0; // Start from top of page

    // Title
    current_layer.use_text("Health & Speed Checker Report", 24.0, Mm(20.0), Mm(y_position), &font_bold);
    y_position -= 10.0;

    // Timestamp
    let timestamp_str = chrono::DateTime::from_timestamp(result.timestamp as i64, 0)
        .map(|dt| dt.format("%B %d, %Y at %H:%M:%S").to_string())
        .unwrap_or_else(|| "Unknown time".to_string());

    current_layer.use_text(&format!("Generated: {}", timestamp_str), 10.0, Mm(20.0), Mm(y_position), &font);
    y_position -= 15.0;

    // Scores Box
    current_layer.use_text("System Scores", 16.0, Mm(20.0), Mm(y_position), &font_bold);
    y_position -= 8.0;

    current_layer.use_text(&format!("Health Score: {}/100", result.scores.health), 12.0, Mm(25.0), Mm(y_position), &font);
    y_position -= 6.0;

    current_layer.use_text(&format!("Speed Score: {}/100", result.scores.speed), 12.0, Mm(25.0), Mm(y_position), &font);
    y_position -= 12.0;

    // Issues Summary
    let critical_count = result.issues.iter().filter(|i| matches!(i.severity, health_speed_checker::IssueSeverity::Critical)).count();
    let warning_count = result.issues.iter().filter(|i| matches!(i.severity, health_speed_checker::IssueSeverity::Warning)).count();
    let info_count = result.issues.iter().filter(|i| matches!(i.severity, health_speed_checker::IssueSeverity::Info)).count();

    current_layer.use_text("Issues Summary", 16.0, Mm(20.0), Mm(y_position), &font_bold);
    y_position -= 8.0;

    if result.issues.is_empty() {
        current_layer.use_text("No issues detected! Your system is healthy.", 12.0, Mm(25.0), Mm(y_position), &font);
        y_position -= 8.0;
    } else {
        current_layer.use_text(&format!("Critical: {}", critical_count), 11.0, Mm(25.0), Mm(y_position), &font);
        y_position -= 6.0;
        current_layer.use_text(&format!("Warnings: {}", warning_count), 11.0, Mm(25.0), Mm(y_position), &font);
        y_position -= 6.0;
        current_layer.use_text(&format!("Info: {}", info_count), 11.0, Mm(25.0), Mm(y_position), &font);
        y_position -= 10.0;

        // List Issues
        current_layer.use_text("Detected Issues", 16.0, Mm(20.0), Mm(y_position), &font_bold);
        y_position -= 8.0;

        for (i, issue) in result.issues.iter().enumerate() {
            // Check if we need a new page
            if y_position < 30.0 {
                let (page_id, layer_id) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
                let new_layer = doc.get_page(page_id).get_layer(layer_id);
                y_position = 270.0;

                // Continue on new page
                new_layer.use_text(&format!("{}. {}", i + 1, issue.title), 11.0, Mm(25.0), Mm(y_position), &font_bold);
            } else {
                current_layer.use_text(&format!("{}. {}", i + 1, issue.title), 11.0, Mm(25.0), Mm(y_position), &font_bold);
            }
            y_position -= 6.0;

            let severity_text = format!("Severity: {:?}", issue.severity);
            current_layer.use_text(&severity_text, 9.0, Mm(30.0), Mm(y_position), &font);
            y_position -= 5.0;

            // Wrap description text
            let desc_words = issue.description.split_whitespace().collect::<Vec<_>>();
            let mut current_line = String::new();
            let max_chars_per_line = 80;

            for word in desc_words {
                if current_line.len() + word.len() + 1 > max_chars_per_line {
                    current_layer.use_text(&current_line, 9.0, Mm(30.0), Mm(y_position), &font);
                    y_position -= 5.0;
                    current_line = word.to_string();
                } else {
                    if !current_line.is_empty() {
                        current_line.push(' ');
                    }
                    current_line.push_str(word);
                }
            }
            if !current_line.is_empty() {
                current_layer.use_text(&current_line, 9.0, Mm(30.0), Mm(y_position), &font);
                y_position -= 5.0;
            }

            if let Some(fix) = &issue.fix {
                let fix_text = format!("Fix: {}", fix.label);
                current_layer.use_text(&fix_text, 9.0, Mm(30.0), Mm(y_position), &font);
                y_position -= 5.0;
            }

            y_position -= 3.0; // Extra spacing between issues
        }
    }

    // Footer
    current_layer.use_text(
        &format!("Scan ID: {}", result.scan_id),
        8.0,
        Mm(20.0),
        Mm(15.0),
        &font,
    );

    // Save PDF to temporary file and return as base64
    let temp_path = std::env::temp_dir().join(format!("health-report-{}.pdf", result.scan_id));

    doc.save(&mut BufWriter::new(
        File::create(&temp_path)
            .map_err(|e| format!("Failed to create PDF file: {}", e))?
    )).map_err(|e| format!("Failed to save PDF: {}", e))?;

    // Read the PDF file and convert to base64
    let pdf_bytes = std::fs::read(&temp_path)
        .map_err(|e| format!("Failed to read PDF file: {}", e))?;

    let base64_pdf = base64::engine::general_purpose::STANDARD.encode(&pdf_bytes);

    // Clean up temporary file
    let _ = std::fs::remove_file(&temp_path);

    Ok(base64_pdf)
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
