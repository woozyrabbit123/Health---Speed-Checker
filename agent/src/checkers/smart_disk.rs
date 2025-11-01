// S.M.A.R.T. Disk Health Checker
// Monitors hard drive health and predicts failures

use crate::{Checker, CheckCategory, Issue, IssueSeverity, ImpactCategory, ScanContext};
use std::process::Command;

pub struct SmartDiskChecker;

impl SmartDiskChecker {
    pub fn new() -> Self {
        Self
    }

    #[cfg(target_os = "windows")]
    fn check_windows_disks(&self) -> Vec<Issue> {
        let mut issues = Vec::new();

        // Use WMIC to query disk health
        let output = Command::new("wmic")
            .args(&["diskdrive", "get", "status,model,size", "/format:csv"])
            .output();

        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);

            for line in stdout.lines().skip(1) {
                if line.contains("Pred Fail") || line.contains("Error") {
                    issues.push(Issue {
                        id: "disk_smart_failure".to_string(),
                        severity: IssueSeverity::Critical,
                        title: "Hard Drive Failure Predicted".to_string(),
                        description: "S.M.A.R.T. indicates imminent drive failure. BACK UP YOUR DATA IMMEDIATELY and replace this drive.".to_string(),
                        impact_category: ImpactCategory::Performance,
                        fix: None,
                    });
                } else if line.contains("Degraded") {
                    issues.push(Issue {
                        id: "disk_smart_degraded".to_string(),
                        severity: IssueSeverity::Warning,
                        title: "Hard Drive Health Degraded".to_string(),
                        description: "The drive is showing signs of degradation. Monitor closely and plan for replacement.".to_string(),
                        impact_category: ImpactCategory::Performance,
                        fix: None,
                    });
                }
            }
        }

        // Check for low disk space
        let space_output = Command::new("wmic")
            .args(&["logicaldisk", "get", "size,freespace,caption", "/format:csv"])
            .output();

        if let Ok(output) = space_output {
            let stdout = String::from_utf8_lossy(&output.stdout);

            for line in stdout.lines().skip(1) {
                if line.is_empty() {
                    continue;
                }

                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 4 {
                    if let (Ok(free), Ok(total)) = (
                        parts[2].trim().parse::<u64>(),
                        parts[3].trim().parse::<u64>()
                    ) {
                        if total > 0 {
                            let percent_free = (free * 100) / total;
                            let drive = parts[1].trim();

                            if percent_free < 10 {
                                issues.push(Issue {
                                    id: format!("disk_low_space_{}", drive),
                                    severity: if percent_free < 5 {
                                        IssueSeverity::Critical
                                    } else {
                                        IssueSeverity::Warning
                                    },
                                    title: format!("Low Disk Space on {}", drive),
                                    description: format!(
                                        "Drive {} has only {}% free space. Free up disk space or your system may become unstable.",
                                        drive, percent_free
                                    ),
                                    impact_category: ImpactCategory::Performance,
                                    fix: None,
                                });
                            }
                        }
                    }
                }
            }
        }

        issues
    }

    #[cfg(target_os = "macos")]
    fn check_macos_disks(&self) -> Vec<Issue> {
        let mut issues = Vec::new();

        // Check S.M.A.R.T. status
        let output = Command::new("diskutil")
            .args(&["info", "disk0"])
            .output();

        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);

            if stdout.contains("S.M.A.R.T. Status: Failing") {
                issues.push(Issue {
                    id: "disk_smart_failure".to_string(),
                    severity: IssueSeverity::Critical,
                    title: "Hard Drive Failure Predicted".to_string(),
                    description: "S.M.A.R.T. indicates imminent drive failure. BACK UP YOUR DATA IMMEDIATELY.".to_string(),
                    impact_category: ImpactCategory::Performance,
                    fix: None,
                });
            }
        }

        // Check disk space
        let df_output = Command::new("df")
            .args(&["-h"])
            .output();

        if let Ok(output) = df_output {
            let stdout = String::from_utf8_lossy(&output.stdout);

            for line in stdout.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 5 {
                    let use_percent = parts[4].trim_end_matches('%');
                    if let Ok(percent) = use_percent.parse::<u8>() {
                        if percent > 90 {
                            let mount = parts[parts.len() - 1];
                            issues.push(Issue {
                                id: format!("disk_low_space_{}", mount.replace('/', "_")),
                                severity: if percent > 95 {
                                    IssueSeverity::Critical
                                } else {
                                    IssueSeverity::Warning
                                },
                                title: format!("Low Disk Space on {}", mount),
                                description: format!(
                                    "{} is {}% full. Free up disk space soon.",
                                    mount, percent
                                ),
                                impact_category: ImpactCategory::Performance,
                                fix: None,
                            });
                        }
                    }
                }
            }
        }

        issues
    }

    #[cfg(target_os = "linux")]
    fn check_linux_disks(&self) -> Vec<Issue> {
        let mut issues = Vec::new();

        // Check S.M.A.R.T. status using smartctl (if available)
        let smart_output = Command::new("smartctl")
            .args(&["-H", "/dev/sda"])
            .output();

        if let Ok(output) = smart_output {
            let stdout = String::from_utf8_lossy(&output.stdout);

            if stdout.contains("FAILING_NOW") || stdout.contains("PASSED: NO") {
                issues.push(Issue {
                    id: "disk_smart_failure".to_string(),
                    severity: IssueSeverity::Critical,
                    title: "Hard Drive Failure Detected".to_string(),
                    description: "S.M.A.R.T. test failed. Back up data immediately and replace drive.".to_string(),
                    impact_category: ImpactCategory::Performance,
                    fix: None,
                });
            }
        }

        // Check disk space
        let df_output = Command::new("df")
            .args(&["-h"])
            .output();

        if let Ok(output) = df_output {
            let stdout = String::from_utf8_lossy(&output.stdout);

            for line in stdout.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 5 {
                    let use_percent = parts[4].trim_end_matches('%');
                    if let Ok(percent) = use_percent.parse::<u8>() {
                        if percent > 90 {
                            let mount = parts[parts.len() - 1];
                            issues.push(Issue {
                                id: format!("disk_low_space_{}", mount.replace('/', "_")),
                                severity: if percent > 95 {
                                    IssueSeverity::Critical
                                } else {
                                    IssueSeverity::Warning
                                },
                                title: format!("Low Disk Space on {}", mount),
                                description: format!(
                                    "{} is {}% full. Consider cleaning up or expanding storage.",
                                    mount, percent
                                ),
                                impact_category: ImpactCategory::Performance,
                                fix: None,
                            });
                        }
                    }
                }
            }
        }

        issues
    }
}

impl Checker for SmartDiskChecker {
    fn name(&self) -> &'static str {
        "S.M.A.R.T. Disk Health"
    }

    fn category(&self) -> CheckCategory {
        CheckCategory::Performance
    }

    fn run(&self, _context: &ScanContext) -> Vec<Issue> {
        #[cfg(target_os = "windows")]
        return self.check_windows_disks();

        #[cfg(target_os = "macos")]
        return self.check_macos_disks();

        #[cfg(target_os = "linux")]
        return self.check_linux_disks();

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        Vec::new()
    }

    fn fix(&self, issue_id: &str, _params: &serde_json::Value) -> Result<crate::FixResult, String> {
        if issue_id.starts_with("disk_low_space_") {
            #[cfg(target_os = "windows")]
            {
                // Run Windows Disk Cleanup
                use std::process::Command;

                let output = Command::new("cleanmgr")
                    .args(&["/sagerun:1"])
                    .spawn();

                if output.is_ok() {
                    return Ok(crate::FixResult {
                        success: true,
                        message: "Launched Disk Cleanup utility. Follow prompts to free space.".to_string(),
                        rollback_available: false,
                        restore_point_id: None,
                    });
                }
            }

            return Err("Manual cleanup required. Delete unnecessary files, empty recycle bin, or uninstall unused programs.".to_string());
        }

        Err("Drive health issues cannot be automatically fixed. Replace failing drives immediately.".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checker_name() {
        let checker = SmartDiskChecker::new();
        assert_eq!(checker.name(), "S.M.A.R.T. Disk Health");
    }

    #[test]
    fn test_checker_category() {
        let checker = SmartDiskChecker::new();
        assert_eq!(checker.category(), CheckCategory::Performance);
    }
}
