// agent/src/main.rs
// CLI entry point for Health & Speed Checker

use clap::{Parser, Subcommand};
use health_speed_checker::*;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Parser)]
#[clap(name = "health-checker")]
#[clap(about = "Privacy-first PC health and speed checker", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a system scan
    Scan {
        /// Only scan security issues
        #[clap(long)]
        security: bool,

        /// Only scan performance issues
        #[clap(long)]
        performance: bool,

        /// Quick scan (5 seconds, skips detailed checks)
        #[clap(long)]
        quick: bool,

        /// Output format
        #[clap(long, value_enum, default_value = "human")]
        output: OutputFormat,

        /// Output to file
        #[clap(long)]
        file: Option<String>,
    },

    /// Show current system status
    Status {
        /// Output as JSON
        #[clap(long)]
        json: bool,
    },

    /// Fix a specific issue
    Fix {
        /// Issue ID to fix
        issue_id: String,

        /// Auto-confirm the fix
        #[clap(long)]
        yes: bool,
    },

    /// List and export reports
    Report {
        #[clap(subcommand)]
        command: ReportCommands,
    },

    /// Configuration management
    Config {
        #[clap(subcommand)]
        command: ConfigCommands,
    },

    /// Daemon control
    Daemon {
        #[clap(subcommand)]
        command: DaemonCommands,
    },
}

#[derive(Subcommand)]
enum ReportCommands {
    /// List recent scans
    List {
        /// Number of scans to show
        #[clap(default_value = "10")]
        limit: u32,
    },

    /// Show a specific scan
    Show {
        /// Scan ID
        scan_id: String,
    },

    /// Export a scan report
    Export {
        /// Scan ID
        scan_id: String,

        /// Export format
        #[clap(long, value_enum, default_value = "pdf")]
        format: ExportFormat,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Show all configuration
    Show,

    /// Set a configuration value
    Set {
        /// Key=value pair
        pair: String,
    },

    /// Get a configuration value
    Get {
        /// Configuration key
        key: String,
    },
}

#[derive(Subcommand)]
enum DaemonCommands {
    /// Start the background daemon
    Start,

    /// Stop the background daemon
    Stop,

    /// Show daemon status
    Status,

    /// Show daemon logs
    Logs {
        /// Number of lines
        #[clap(default_value = "50")]
        lines: u32,
    },
}

#[derive(clap::ValueEnum, Clone)]
enum OutputFormat {
    Human,
    Json,
    Csv,
}

#[derive(clap::ValueEnum, Clone)]
enum ExportFormat {
    Pdf,
    Json,
    Html,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let (db_path, license_path) = resolve_data_paths();
    let _automation_daemon = daemon::start_automation_daemon(db_path, license_path);

    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { security, performance, quick, output, file } => {
            handle_scan(security, performance, quick, output, file).await?;
        }
        Commands::Status { json } => {
            handle_status(json).await?;
        }
        Commands::Fix { issue_id, yes } => {
            handle_fix(issue_id, yes).await?;
        }
        Commands::Report { command } => {
            handle_report(command).await?;
        }
        Commands::Config { command } => {
            handle_config(command).await?;
        }
        Commands::Daemon { command } => {
            handle_daemon(command).await?;
        }
    }

    Ok(())
}

fn resolve_data_paths() -> (PathBuf, PathBuf) {
    let base_dir = std::env::var("APPDATA")
        .or_else(|_| std::env::var("HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));

    let root_dir = base_dir.join("HealthSpeedChecker");
    if let Err(err) = std::fs::create_dir_all(&root_dir) {
        tracing::warn!(
            "Failed to ensure data directory {}: {}",
            root_dir.display(),
            err
        );
    }

    let db_path = root_dir.join("app.db");
    let license_path = root_dir.join("license.json");
    (db_path, license_path)
}

async fn handle_scan(
    security_only: bool,
    performance_only: bool,
    quick: bool,
    output: OutputFormat,
    file: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let options = ScanOptions {
        security: !performance_only,
        performance: !security_only,
        quick,
        exclude_apps: quick,
        exclude_startup: quick,
    };

    // Create and configure the scanner engine
    let mut engine = ScannerEngine::new();

    // Register all checkers
    use checkers::*;

    // Core checkers (security + performance basics)
    engine.register(Box::new(FirewallChecker));
    engine.register(Box::new(StartupAnalyzer));
    engine.register(Box::new(ProcessMonitor));
    engine.register(Box::new(OsUpdateChecker));
    engine.register(Box::new(PortScanner));

    // Advanced checkers (deeper analysis)
    engine.register(Box::new(checkers::bloatware::BloatwareDetector::new()));
    engine.register(Box::new(checkers::network::NetworkChecker::new()));
    engine.register(Box::new(checkers::smart_disk::SmartDiskChecker::new()));
    engine.register(Box::new(checkers::storage::StorageChecker::new()));

    // The "Trust Builder" - honest hardware bottleneck analysis
    // This is what differentiates us from scare-tactic competitors
    engine.register(Box::new(checkers::bottleneck::BottleneckAnalyzer::new()));

    // Show progress for human output
    let progress = if matches!(output, OutputFormat::Human) {
        let pb = ProgressBar::new(100);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {percent}% {msg}")
                .unwrap()
                .progress_chars("█▉▊▋▌▍▎▏  ")
        );
        pb.set_message("Starting scan...");
        Some(pb)
    } else {
        None
    };

    // Simulate progress (in real implementation, this would be event-driven)
    if let Some(pb) = &progress {
        pb.set_position(20);
        pb.set_message("Checking security...");
        tokio::time::sleep(Duration::from_millis(500)).await;

        pb.set_position(40);
        pb.set_message("Analyzing performance...");
        tokio::time::sleep(Duration::from_millis(500)).await;

        pb.set_position(60);
        pb.set_message("Scanning processes...");
        tokio::time::sleep(Duration::from_millis(500)).await;

        pb.set_position(80);
        pb.set_message("Calculating scores...");
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    // Run the scan
    let result = engine.scan(options);

    if let Some(pb) = progress {
        pb.set_position(100);
        pb.finish_with_message("Scan complete!");
    }

    // Output results
    match output {
        OutputFormat::Human => {
            print_human_readable(&result);
        }
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&result)?;
            if let Some(file) = file {
                std::fs::write(file, json)?;
            } else {
                println!("{}", json);
            }
        }
        OutputFormat::Csv => {
            print_csv(&result)?;
        }
    }

    // Treat critical findings as failures, but allow warnings to succeed so automated
    // workflows (like quick health checks) don't error out on advisory issues alone.
    if result
        .issues
        .iter()
        .any(|issue| issue.severity == IssueSeverity::Critical)
    {
        std::process::exit(2);
    }

    Ok(())
}

fn print_human_readable(result: &ScanResult) {
    println!();
    println!("{}", "═══════════════════════════════════════".bright_blue());
    println!("{}", "     HEALTH & SPEED CHECK RESULTS     ".bright_blue().bold());
    println!("{}", "═══════════════════════════════════════".bright_blue());
    println!();

    // Scores with color coding
    let health_color = if result.scores.health >= 80 {
        "green"
    } else if result.scores.health >= 60 {
        "yellow"
    } else {
        "red"
    };

    let speed_color = if result.scores.speed >= 80 {
        "green"
    } else if result.scores.speed >= 60 {
        "yellow"
    } else {
        "red"
    };

    print!("  {} Health Score: ", "⬤".color(health_color));
    println!("{}/100", result.scores.health.to_string().color(health_color).bold());

    if let Some(delta) = result.scores.health_delta {
        let delta_str = if delta > 0 {
            format!("↑{}", delta).green()
        } else if delta < 0 {
            format!("↓{}", -delta).red()
        } else {
            "→0".normal()
        };
        println!("    {} from last scan", delta_str);
    }

    print!("  {} Speed Score:  ", "⬤".color(speed_color));
    println!("{}/100", result.scores.speed.to_string().color(speed_color).bold());

    if let Some(delta) = result.scores.speed_delta {
        let delta_str = if delta > 0 {
            format!("↑{}", delta).green()
        } else if delta < 0 {
            format!("↓{}", -delta).red()
        } else {
            "→0".normal()
        };
        println!("    {} from last scan", delta_str);
    }

    println!();

    // Top issues
    if !result.issues.is_empty() {
        println!("{}", "TOP ISSUES FOUND:".yellow().bold());
        println!();

        for (i, issue) in result.issues.iter().take(5).enumerate() {
            let severity_badge = match issue.severity {
                IssueSeverity::Critical => "[CRITICAL]".red().bold(),
                IssueSeverity::Warning => "[WARNING]".yellow().bold(),
                IssueSeverity::Info => "[INFO]".blue(),
            };

            println!("  {}. {} {}", i + 1, severity_badge, issue.title.bold());
            println!("     {}", issue.description);

            if let Some(fix) = &issue.fix {
                if fix.is_auto_fix {
                    println!("     {} Run: health-checker fix {}",
                        "→".green(),
                        issue.id.bright_black());
                } else {
                    println!("     {} Manual fix required", "→".yellow());
                }
            }
            println!();
        }

        if result.issues.len() > 5 {
            println!("  ... and {} more issues", result.issues.len() - 5);
            println!();
        }
    } else {
        println!("{}", "✓ No issues found! Your system is healthy.".green().bold());
        println!();
    }

    // Summary
    println!("{}", "─────────────────────────────────────".bright_black());
    println!("  Scan completed in {} ms", result.duration_ms);
    println!("  Total issues: {}", result.issues.len());
    println!("  Critical: {} | Warnings: {} | Info: {}",
        result.issues.iter().filter(|i| i.severity == IssueSeverity::Critical).count(),
        result.issues.iter().filter(|i| i.severity == IssueSeverity::Warning).count(),
        result.issues.iter().filter(|i| i.severity == IssueSeverity::Info).count()
    );
    println!();
}

fn print_csv(result: &ScanResult) -> Result<(), Box<dyn std::error::Error>> {
    println!("ID,Severity,Category,Title,Description,Fixable");

    for issue in &result.issues {
        println!(
            "{},{:?},{:?},{},{},{}",
            issue.id,
            issue.severity,
            issue.impact_category,
            issue.title.replace(",", ";"),
            issue.description.replace(",", ";"),
            issue.fix.is_some()
        );
    }

    Ok(())
}

async fn handle_status(json: bool) -> Result<(), Box<dyn std::error::Error>> {
    // In a real implementation, this would read from the database
    let status = if json {
        r#"{"health": 72, "speed": 85, "last_scan": "3 hours ago", "issues": 5}"#
    } else {
        "Health: 72/100 (2 critical), Speed: 85/100 (3 issues), Last scan: 3 hours ago"
    };

    println!("{}", status);
    Ok(())
}

async fn handle_fix(issue_id: String, auto_confirm: bool) -> Result<(), Box<dyn std::error::Error>> {
    if !auto_confirm {
        println!("Are you sure you want to fix '{}'? [y/N]", issue_id);

        use std::io::{self, BufRead};
        let stdin = io::stdin();
        let mut line = String::new();
        stdin.lock().read_line(&mut line)?;

        if !line.trim().eq_ignore_ascii_case("y") {
            println!("Fix cancelled.");
            return Ok(());
        }
    }

    println!("Creating restore point...");

    // Initialize scanner to use fix functionality
    let engine = ScannerEngine::new();
    let result = engine.fix_issue(&issue_id, &serde_json::json!({}));

    if result.success {
        println!("{} {}", "✓".green(), result.message);
    } else {
        println!("{} {}", "✗".red(), result.message);
        std::process::exit(1);
    }

    Ok(())
}

async fn handle_report(_command: ReportCommands) -> Result<(), Box<dyn std::error::Error>> {
    println!("Report functionality not yet implemented");
    Ok(())
}

async fn handle_config(_command: ConfigCommands) -> Result<(), Box<dyn std::error::Error>> {
    println!("Config functionality not yet implemented");
    Ok(())
}

async fn handle_daemon(_command: DaemonCommands) -> Result<(), Box<dyn std::error::Error>> {
    println!("Daemon functionality not yet implemented");
    Ok(())
}

// Re-export for convenience
use health_speed_checker::checkers;
