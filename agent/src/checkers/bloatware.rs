// Bloatware Detection Module
// Detects unnecessary startup programs and resource-heavy background apps

use crate::{Checker, CheckCategory, Issue, IssueSeverity, ScanContext, ImpactCategory, FixAction};
use std::collections::HashMap;

pub struct BloatwareDetector;

impl BloatwareDetector {
    pub fn new() -> Self {
        Self
    }

    /// Known bloatware patterns (common unnecessary startup programs)
    fn bloatware_patterns() -> HashMap<&'static str, (&'static str, IssueSeverity)> {
        let mut patterns = HashMap::new();

        // Common bloatware
        patterns.insert("spotify", ("Spotify auto-start", IssueSeverity::Low));
        patterns.insert("discord", ("Discord auto-start", IssueSeverity::Low));
        patterns.insert("skype", ("Skype auto-start", IssueSeverity::Low));
        patterns.insert("steam", ("Steam auto-start", IssueSeverity::Low));
        patterns.insert("epicgameslauncher", ("Epic Games Launcher", IssueSeverity::Low));
        patterns.insert("origin", ("Origin auto-start", IssueSeverity::Low));
        patterns.insert("uplay", ("Uplay auto-start", IssueSeverity::Low));

        // Resource-heavy apps
        patterns.insert("onedrive", ("OneDrive sync", IssueSeverity::Medium));
        patterns.insert("dropbox", ("Dropbox sync", IssueSeverity::Medium));
        patterns.insert("googledrive", ("Google Drive sync", IssueSeverity::Medium));
        patterns.insert("adobecreativecloud", ("Adobe Creative Cloud", IssueSeverity::Medium));

        // Known resource hogs
        patterns.insert("teamviewer", ("TeamViewer", IssueSeverity::Medium));
        patterns.insert("logmein", ("LogMeIn", IssueSeverity::Medium));
        patterns.insert("anydesk", ("AnyDesk", IssueSeverity::Medium));

        // Unnecessary manufacturer software
        patterns.insert("hpwuschd", ("HP Update Scheduler", IssueSeverity::Low));
        patterns.insert("ccapp", ("Norton/Symantec", IssueSeverity::Medium));
        patterns.insert("avgui", ("AVG Antivirus", IssueSeverity::Medium));
        patterns.insert("mcafee", ("McAfee", IssueSeverity::High));
        patterns.insert("norton", ("Norton Antivirus", IssueSeverity::Medium));

        // Additional common bloatware
        patterns.insert("slack", ("Slack auto-start", IssueSeverity::Low));
        patterns.insert("zoom", ("Zoom auto-start", IssueSeverity::Low));
        patterns.insert("teams", ("Microsoft Teams", IssueSeverity::Medium));
        patterns.insert("onenote", ("OneNote auto-start", IssueSeverity::Low));
        patterns.insert("itunes", ("iTunes Helper", IssueSeverity::Medium));
        patterns.insert("icloud", ("iCloud sync", IssueSeverity::Medium));
        patterns.insert("acrobat", ("Adobe Acrobat Updater", IssueSeverity::Low));
        patterns.insert("java", ("Java Update Scheduler", IssueSeverity::Low));
        patterns.insert("realtek", ("Realtek Audio Manager", IssueSeverity::Low));
        patterns.insert("nvidia", ("NVIDIA Ge Force Experience", IssueSeverity::Medium));

        patterns
    }

    #[cfg(target_os = "windows")]
    fn scan_windows_startup(&self) -> Vec<Issue> {
        use std::process::Command;

        let mut issues = Vec::new();
        let patterns = Self::bloatware_patterns();

        // Query startup programs via registry
        let output = Command::new("reg")
            .args(&["query", "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run"])
            .output();

        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);

            for line in stdout.lines() {
                let line_lower = line.to_lowercase();

                for (pattern, (name, severity)) in &patterns {
                    if line_lower.contains(pattern) {
                        issues.push(Issue {
                            id: format!("bloatware_{}", pattern),
                            severity: *severity,
                            title: format!("Unnecessary startup program: {}", name),
                            description: format!(
                                "{} is set to run at startup. This may slow down your boot time and consume system resources. Consider disabling it if you don't need it running constantly.",
                                name
                            ),
                            impact_category: ImpactCategory::Performance,
                            fix: None,
                        });
                        break;
                    }
                }
            }
        }

        // Check task scheduler for bloatware
        let schtasks_output = Command::new("schtasks")
            .args(&["/query", "/fo", "LIST", "/v"])
            .output();

        if let Ok(output) = schtasks_output {
            let stdout = String::from_utf8_lossy(&output.stdout);

            for (pattern, (name, severity)) in &patterns {
                if stdout.to_lowercase().contains(pattern) {
                    let id = format!("bloatware_task_{}", pattern);

                    // Don't add duplicate
                    if !issues.iter().any(|i| i.id == id) {
                        issues.push(Issue {
                            id,
                            severity: *severity,
                            title: format!("Scheduled bloatware task: {}", name),
                            description: format!(
                                "{} has scheduled tasks that run automatically. This may impact system performance.",
                                name
                            ),
                            impact_category: ImpactCategory::Performance,
                            fix: None,
                        });
                    }
                }
            }
        }

        issues
    }

    #[cfg(target_os = "macos")]
    fn scan_macos_startup(&self) -> Vec<Issue> {
        use std::process::Command;

        let mut issues = Vec::new();
        let patterns = Self::bloatware_patterns();

        // Check Launch Agents
        let output = Command::new("launchctl")
            .args(&["list"])
            .output();

        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);

            for line in stdout.lines() {
                let line_lower = line.to_lowercase();

                for (pattern, (name, severity)) in &patterns {
                    if line_lower.contains(pattern) {
                        issues.push(Issue {
                            id: format!("bloatware_{}", pattern),
                            severity: *severity,
                            title: format!("Unnecessary launch agent: {}", name),
                            description: format!(
                                "{} is configured to launch automatically. Consider disabling it to improve system performance.",
                                name
                            ),
                            impact_category: ImpactCategory::Performance,
                            fix: None,
                        });
                        break;
                    }
                }
            }
        }

        issues
    }

    #[cfg(target_os = "linux")]
    fn scan_linux_startup(&self) -> Vec<Issue> {
        use std::fs;

        let mut issues = Vec::new();
        let patterns = Self::bloatware_patterns();

        // Check systemd services
        if let Ok(entries) = fs::read_dir("/etc/systemd/system") {
            for entry in entries.flatten() {
                let path = entry.path();
                let filename = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_lowercase();

                for (pattern, (name, severity)) in &patterns {
                    if filename.contains(pattern) {
                        issues.push(Issue {
                            id: format!("bloatware_{}", pattern),
                            severity: *severity,
                            title: format!("Unnecessary systemd service: {}", name),
                            description: format!(
                                "{} service is enabled at startup. Consider disabling if not needed.",
                                name
                            ),
                            impact_category: ImpactCategory::Performance,
                            fix: None,
                        });
                        break;
                    }
                }
            }
        }

        // Check .config/autostart
        if let Some(home) = std::env::var_os("HOME") {
            let autostart_path = std::path::Path::new(&home).join(".config/autostart");
            if let Ok(entries) = fs::read_dir(autostart_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let filename = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_lowercase();

                    for (pattern, (name, severity)) in &patterns {
                        if filename.contains(pattern) {
                            let id = format!("bloatware_autostart_{}", pattern);
                            if !issues.iter().any(|i| i.id == id) {
                                issues.push(Issue {
                                    id,
                                    severity: *severity,
                                    title: format!("Autostart application: {}", name),
                                    description: format!(
                                        "{} is configured to start automatically. Remove from autostart to improve boot time.",
                                        name
                                    ),
                                    impact_category: ImpactCategory::Performance,
                                    fix: None,
                                });
                            }
                            break;
                        }
                    }
                }
            }
        }

        issues
    }
}

impl Checker for BloatwareDetector {
    fn name(&self) -> &'static str {
        "Bloatware Detector"
    }

    fn category(&self) -> CheckCategory {
        CheckCategory::Performance
    }

    fn run(&self, _context: &ScanContext) -> Vec<Issue> {
        #[cfg(target_os = "windows")]
        return self.scan_windows_startup();

        #[cfg(target_os = "macos")]
        return self.scan_macos_startup();

        #[cfg(target_os = "linux")]
        return self.scan_linux_startup();

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        Vec::new()
    }

    fn fix(&self, issue_id: &str, _params: &serde_json::Value) -> Result<crate::FixResult, String> {
        #[cfg(target_os = "windows")]
        {
            // Extract pattern from issue_id (format: "bloatware_pattern")
            if let Some(pattern) = issue_id.strip_prefix("bloatware_") {
                // SECURITY: Validate pattern against whitelist to prevent command injection
                let valid_patterns = Self::bloatware_patterns();
                if !valid_patterns.contains_key(pattern) {
                    return Err(format!("Invalid bloatware pattern: {}", pattern));
                }

                // SECURITY: Additional sanitization - only allow alphanumeric and safe chars
                if !pattern.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
                    return Err("Pattern contains invalid characters".to_string());
                }

                use std::process::Command;

                // Attempt to disable via registry
                let output = Command::new("reg")
                    .args(&[
                        "delete",
                        "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                        "/v",
                        pattern,
                        "/f"
                    ])
                    .output();

                match output {
                    Ok(out) if out.status.success() => {
                        return Ok(crate::FixResult {
                            success: true,
                            message: format!("Disabled {} from startup", pattern),
                            rollback_available: false,
                            restore_point_id: None,
                        });
                    }
                    _ => {
                        return Err(format!(
                            "Failed to disable {}. You may need to disable it manually in Task Manager > Startup tab.",
                            pattern
                        ));
                    }
                }
            }
        }

        Err("Manual fix required. Disable this program from your system's startup settings.".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bloatware_patterns() {
        let patterns = BloatwareDetector::bloatware_patterns();
        assert!(patterns.contains_key("spotify"));
        assert!(patterns.contains_key("mcafee"));
    }

    #[test]
    fn test_checker_name() {
        let detector = BloatwareDetector::new();
        assert_eq!(detector.name(), "Bloatware Detector");
    }

    #[test]
    fn test_checker_category() {
        let detector = BloatwareDetector::new();
        assert_eq!(detector.category(), CheckCategory::Performance);
    }
}
