// Storage & Drive Health Checker
// Comprehensive storage analysis and health monitoring

use crate::{Checker, CheckCategory, Issue, IssueSeverity, ScanContext, ImpactCategory};
use std::process::Command;

pub struct StorageChecker;

impl StorageChecker {
    pub fn new() -> Self {
        Self
    }

    /// Get all storage drives and their info
    #[cfg(target_os = "windows")]
    fn get_drive_info(&self) -> Vec<DriveInfo> {
        use std::process::Command;
        use std::time::Duration;
        use crate::util::command::run_with_timeout;

        let mut drives = Vec::new();

        let output = run_with_timeout({
            let mut c = Command::new("wmic");
            c.args([
                "logicaldisk",
                "get",
                "Caption,DriveType,FileSystem,FreeSpace,Size,VolumeName",
                "/format:csv",
            ]);
            c
        }, Duration::from_secs(5));

        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);

            for line in stdout.lines().skip(2) {
                if line.trim().is_empty() {
                    continue;
                }

                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 6 {
                    if let (Some(caption), Some(free), Some(size)) = (
                        parts.get(1),
                        parts.get(3),
                        parts.get(5)
                    ) {
                        if let (Ok(free_bytes), Ok(total_bytes)) = (
                            free.trim().parse::<u64>(),
                            size.trim().parse::<u64>()
                        ) {
                            if total_bytes > 0 {
                                drives.push(DriveInfo {
                                    name: caption.trim().to_string(),
                                    total_bytes,
                                    free_bytes,
                                    drive_type: self.parse_drive_type(parts.get(2)),
                                    file_system: parts.get(4).map(|s| s.trim().to_string()),
                                });
                            }
                        }
                    }
                }
            }
        }

        drives
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    fn get_drive_info(&self) -> Vec<DriveInfo> {
        use std::process::Command;
        use std::time::Duration;
        use crate::util::command::run_with_timeout;

        let mut drives = Vec::new();

        let output = run_with_timeout({
            let mut c = Command::new("df");
            c.args(["-B1"]); // Output in bytes
            c
        }, Duration::from_secs(5));

        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);

            for line in stdout.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 6 {
                    if let (Ok(total), Ok(_used), Ok(free)) = (
                        parts[1].parse::<u64>(),
                        parts[2].parse::<u64>(),
                        parts[3].parse::<u64>()
                    ) {
                        drives.push(DriveInfo {
                            name: parts[5].to_string(),
                            total_bytes: total,
                            free_bytes: free,
                            drive_type: DriveType::Fixed,
                            file_system: Some(parts[0].to_string()),
                        });
                    }
                }
            }
        }

        drives
    }

    fn parse_drive_type(&self, type_str: Option<&&str>) -> DriveType {
        match type_str.map(|s| s.trim()) {
            Some("2") => DriveType::Removable,
            Some("3") => DriveType::Fixed,
            Some("5") => DriveType::CDRom,
            _ => DriveType::Unknown,
        }
    }

    fn check_fragmentation(&self, drive: &str) -> Option<u32> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            use std::time::Duration;
            use crate::util::command::run_with_timeout;

            // Query defrag status (requires admin, may fail)
            let output = run_with_timeout({
                let mut c = Command::new("defrag");
                c.args([drive, "/A", "/V"]);
                c
            }, Duration::from_secs(10));

            if let Ok(output) = output {
                let stdout = String::from_utf8_lossy(&output.stdout);

                // Parse fragmentation percentage
                for line in stdout.lines() {
                    if line.contains("fragmented") {
                        // Try to extract percentage
                        let words: Vec<&str> = line.split_whitespace().collect();
                        for word in words.iter() {
                            if word.ends_with('%') {
                                if let Ok(percent) = word.trim_end_matches('%').parse::<u32>() {
                                    return Some(percent);
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }
}

#[derive(Debug)]
struct DriveInfo {
    name: String,
    total_bytes: u64,
    free_bytes: u64,
    drive_type: DriveType,
    file_system: Option<String>,
}

#[derive(Debug, PartialEq)]
enum DriveType {
    Fixed,
    Removable,
    CDRom,
    Unknown,
}

impl Checker for StorageChecker {
    fn name(&self) -> &'static str {
        "Storage & Drive Health"
    }

    fn category(&self) -> CheckCategory {
        CheckCategory::Performance
    }

    fn run(&self, _context: &ScanContext) -> Vec<Issue> {
        let mut issues = Vec::new();
        let drives = self.get_drive_info();

        for drive in drives {
            // Skip removable drives and CD-ROMs
            if drive.drive_type == DriveType::Removable || drive.drive_type == DriveType::CDRom {
                continue;
            }

            let percent_free = (drive.free_bytes * 100) / drive.total_bytes;
            let percent_used = 100 - percent_free;

            // Low disk space warnings
            if percent_free < 10 {
                issues.push(Issue {
                    id: format!("storage_low_space_{}", drive.name.replace(':', "_").replace('/', "_")),
                    severity: if percent_free < 5 {
                        IssueSeverity::Critical
                    } else {
                        IssueSeverity::Critical
                    },
                    title: format!("Critically Low Disk Space: {}", drive.name),
                    description: format!(
                        "{} has only {:.1} GB free ({:.0}% full). System performance and stability will suffer. Free up space immediately.",
                        drive.name,
                        drive.free_bytes as f64 / 1_073_741_824.0,
                        percent_used
                    ),
                    impact_category: ImpactCategory::Performance,
                    fix: None,
                });
            } else if percent_free < 20 {
                issues.push(Issue {
                    id: format!("storage_low_space_{}", drive.name.replace(':', "_").replace('/', "_")),
                    severity: IssueSeverity::Warning,
                    title: format!("Low Disk Space: {}", drive.name),
                    description: format!(
                        "{} has {:.1} GB free ({:.0}% full). Consider freeing up space soon.",
                        drive.name,
                        drive.free_bytes as f64 / 1_073_741_824.0,
                        percent_used
                    ),
                    impact_category: ImpactCategory::Performance,
                    fix: None,
                });
            }

            // Check for fragmentation (Windows only)
            #[cfg(target_os = "windows")]
            if let Some(frag_percent) = self.check_fragmentation(&drive.name) {
                if frag_percent > 15 {
                    issues.push(Issue {
                        id: format!("storage_fragmentation_{}", drive.name.replace(':', "_")),
                        severity: if frag_percent > 30 {
                            IssueSeverity::Critical
                        } else {
                            IssueSeverity::Warning
                        },
                        title: format!("High Disk Fragmentation: {}", drive.name),
                        description: format!(
                            "{} is {}% fragmented. This slows down file access. Run defragmentation.",
                            drive.name, frag_percent
                        ),
                        impact_category: ImpactCategory::Performance,
                        fix: None,
                    });
                }
            }

            // Warn about FAT32 on large drives (inefficient)
            if let Some(ref fs) = drive.file_system {
                if fs.to_lowercase().contains("fat32") && drive.total_bytes > 32_000_000_000 {
                    issues.push(Issue {
                        id: format!("storage_fat32_{}", drive.name.replace(':', "_").replace('/', "_")),
                        severity: IssueSeverity::Info,
                        title: format!("Inefficient File System: {}", drive.name),
                        description: format!(
                            "{} uses FAT32, which is inefficient for large drives. Consider converting to NTFS or exFAT.",
                            drive.name
                        ),
                        impact_category: ImpactCategory::Performance,
                        fix: None,
                    });
                }
            }
        }

        // Check temp directory size
        #[cfg(target_os = "windows")]
        {
            if let Ok(temp_dir) = std::env::var("TEMP") {
                if let Ok(_metadata) = std::fs::metadata(&temp_dir) {
                    // Simplified check - in production, would recursively calculate size
                    issues.push(Issue {
                        id: "storage_temp_cleanup".to_string(),
                        severity: IssueSeverity::Info,
                        title: "Temporary Files May Need Cleanup".to_string(),
                        description: "Temporary files can accumulate over time. Run Disk Cleanup to free space.".to_string(),
                        impact_category: ImpactCategory::Performance,
                        fix: None,
                    });
                }
            }
        }

        issues
    }

    fn fix(&self, issue_id: &str, _params: &serde_json::Value) -> Result<crate::FixResult, String> {
        #[cfg(target_os = "windows")]
        {
            if issue_id.starts_with("storage_low_space_") || issue_id == "storage_temp_cleanup" {
                use std::process::Command;

                // Launch Disk Cleanup
                let result = Command::new("cleanmgr")
                    .args(&["/d", "C:"])
                    .spawn();

                if result.is_ok() {
                    return Ok(crate::FixResult {
                        success: true,
                        message: "Launched Windows Disk Cleanup. Follow the prompts to free up space.".to_string(),
                        rollback_available: false,
                        restore_point_id: None,
                    });
                }
            }

            if issue_id.starts_with("storage_fragmentation_") {
                // Extract drive letter from issue_id
                if let Some(drive) = issue_id.strip_prefix("storage_fragmentation_") {
                    // SECURITY: Validate drive letter to prevent command injection
                    // Only allow single uppercase letters A-Z
                    if drive.len() != 1 || !drive.chars().all(|c| c.is_ascii_uppercase()) {
                        return Err(format!("Invalid drive letter: {}", drive));
                    }

                    let drive_letter = format!("{}:", drive);

                    let result = Command::new("defrag")
                        .args(&[&drive_letter, "/O"])
                        .spawn();

                    if result.is_ok() {
                        return Ok(crate::FixResult {
                            success: true,
                            message: format!("Started defragmentation of {}. This may take a while.", drive_letter),
                            rollback_available: false,
                            restore_point_id: None,
                        });
                    }
                }
            }
        }

        Err("Storage issues require manual intervention. Free up disk space or run system optimization tools.".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checker_name() {
        let checker = StorageChecker::new();
        assert_eq!(checker.name(), "Storage & Drive Health");
    }

    #[test]
    fn test_drive_type_parsing() {
        let checker = StorageChecker::new();
        assert_eq!(checker.parse_drive_type(Some(&"3")), DriveType::Fixed);
        assert_eq!(checker.parse_drive_type(Some(&"2")), DriveType::Removable);
    }
}
