use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use tracing::{debug, error, info, warn};

use crate::db::{AutomationSettings, Db};
use crate::license::{LicenseManager, ProFeature};
use crate::{checkers, ScanOptions, ScannerEngine};

const SLEEP_INTERVAL: Duration = Duration::from_secs(3600);

fn build_scanner_engine() -> ScannerEngine {
    let mut engine = ScannerEngine::new();

    use checkers::*;

    engine.register(Box::new(FirewallChecker));
    engine.register(Box::new(StartupAnalyzer));
    engine.register(Box::new(ProcessMonitor));
    engine.register(Box::new(OsUpdateChecker));
    engine.register(Box::new(PortScanner));
    engine.register(Box::new(bloatware::BloatwareDetector::new()));
    engine.register(Box::new(network::NetworkChecker::new()));
    engine.register(Box::new(smart_disk::SmartDiskChecker::new()));
    engine.register(Box::new(storage::StorageChecker::new()));
    engine.register(Box::new(bottleneck::BottleneckAnalyzer::new()));

    engine
}

fn required_interval_seconds(schedule: &str) -> u64 {
    match schedule {
        "daily" => 86_400,
        "weekly" => 7 * 86_400,
        "monthly" => 30 * 86_400,
        _ => 7 * 86_400,
    }
}

fn should_run_scan(
    settings: &AutomationSettings,
    db: &Db,
) -> Result<bool, String> {
    if !settings.automation_enabled {
        return Ok(false);
    }

    let last_scan = db.last_scan_timestamp()?;
    let now = chrono::Utc::now().timestamp() as u64;
    let interval = required_interval_seconds(&settings.run_schedule);

    match last_scan {
        Some(ts) => Ok(now >= ts + interval),
        None => Ok(true),
    }
}

fn run_automation_iteration(
    db_path: &PathBuf,
    license_path: &PathBuf,
) -> Result<(), String> {
    let db = Db::open(&db_path.to_string_lossy())?;
    let settings = db.get_automation_settings()?;

    if !settings.automation_enabled {
        debug!("Automation disabled; skipping scheduler iteration");
        return Ok(());
    }

    let license_manager = LicenseManager::new(license_path.clone());
    let license = license_manager
        .load()
        .map_err(|e| format!("failed to load license: {}", e))?;

    if !license.has_pro_feature(ProFeature::Automation) {
        debug!("Automation feature not available for current license; skipping");
        return Ok(());
    }

    if !should_run_scan(&settings, &db)? {
        debug!("No scheduled scan required at this time");
        return Ok(());
    }

    info!(
        "Automation scheduler starting {} scan (auto-fix: {})",
        settings.run_schedule, settings.auto_fix_enabled
    );

    let engine = build_scanner_engine();

    let options = ScanOptions::default();
    let result = engine.scan_with_license(options, &license);

    if settings.auto_fix_enabled {
        for issue in &result.issues {
            if let Some(fix) = &issue.fix {
                if fix.is_auto_fix {
                    let fix_result = engine.fix_issue(&fix.action_id, &fix.params);
                    if fix_result.success {
                        info!("Auto-fix succeeded for {}", issue.id);
                    } else {
                        warn!(
                            "Auto-fix failed for {}: {}",
                            issue.id, fix_result.message
                        );
                    }
                }
            }
        }
    }

    db.save_scan(&result)?;
    info!(
        "Automation scan completed: health={}, speed={}, issues={}",
        result.scores.health,
        result.scores.speed,
        result.issues.len()
    );

    Ok(())
}

pub fn start_automation_daemon(
    db_path: PathBuf,
    license_path: PathBuf,
) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        if let Err(err) = run_automation_iteration(&db_path, &license_path) {
            error!("Automation scheduler error: {}", err);
        }
        thread::sleep(SLEEP_INTERVAL);
    })
}
