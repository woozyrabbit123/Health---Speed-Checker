// Honest Bottleneck Analyzer
// The "Trust Builder" that tells users the REAL cause of slowness
// Unlike competitors' scare tactics, this provides genuine advice

use crate::{Checker, CheckCategory, Issue, IssueSeverity, ImpactCategory, ScanContext, FixAction};
use serde_json::json;
use sysinfo::{Disks, System};

pub struct BottleneckAnalyzer;

impl BottleneckAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// Analyze if HDD is the primary bottleneck
    fn analyze_disk_bottleneck(&self, _sys: &System) -> Option<Issue> {
        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
            let name = disk.name().to_string_lossy();

            // Check if it's a mechanical HDD (common indicators)
            // Real detection would use SMART data, but this is a good heuristic
            let total_gb = disk.total_space() / 1_073_741_824;

            // HDDs typically >500GB, SSDs usually smaller or NVMe
            // This is simplified - production would check disk type via WMI/ioctl
            if total_gb > 500 && !name.contains("SSD") && !name.contains("NVMe") {
                return Some(Issue {
                    id: "bottleneck_mechanical_hdd".to_string(),
                    severity: IssueSeverity::Warning,
                    title: "Mechanical Hard Drive Detected - This is Your #1 Slowdown".to_string(),
                    description: format!(
                        "Your system is using a mechanical hard drive ({}). This is the most common \
                        cause of slow PC performance. Software optimizations can only improve speed by \
                        5-10% when your storage is the bottleneck.\n\n\
                        HONEST RECOMMENDATION: Upgrading to an SSD (Solid State Drive) will make your \
                        PC feel 5-10x faster. This is a hardware upgrade, not something software can fix.\n\n\
                        Expected improvement from SSD upgrade:\n\
                        - Boot time: 60s -> 10s\n\
                        - App launch: 10s -> 1s\n\
                        - File operations: 10x faster\n\n\
                        Cost: $50-150 for 500GB SSD\n\
                        Difficulty: Moderate (or pay tech $50-100 to install)",
                        name
                    ),
                    impact_category: ImpactCategory::Performance,
                    fix: Some(FixAction {
                        action_id: "show_ssd_guide".to_string(),
                        label: "Show SSD Upgrade Guide".to_string(),
                        is_auto_fix: false,
                        params: json!({}),
                    }),
                });
            }
        }
        None
    }

    /// Analyze if RAM is the bottleneck
    fn analyze_ram_bottleneck(&self, sys: &System) -> Option<Issue> {
        let total_ram_gb = sys.total_memory() / 1_073_741_824;
        let used_ram_gb = sys.used_memory() / 1_073_741_824;
        let usage_percent = (used_ram_gb as f64 / total_ram_gb as f64) * 100.0;

        // Low RAM systems (<8GB) are a real bottleneck
        if total_ram_gb < 8 {
            return Some(Issue {
                id: "bottleneck_low_ram".to_string(),
                severity: IssueSeverity::Warning,
                title: format!("Low RAM Detected - {}GB is Below Modern Requirements", total_ram_gb),
                description: format!(
                    "Your system has {}GB of RAM. Modern applications (browsers, video calls, etc.) \
                    require 8GB minimum for smooth operation. You're currently using {}GB ({:.0}%).\n\n\
                    HONEST ASSESSMENT:\n\
                    - Closing background apps will help temporarily\n\
                    - But you'll constantly hit this limit\n\
                    - Adding more RAM is the permanent solution\n\n\
                    Expected improvement from RAM upgrade to 16GB:\n\
                    - Can run more apps simultaneously\n\
                    - Less disk swapping (which is very slow)\n\
                    - Better multitasking\n\n\
                    Cost: $30-80 for 8GB RAM upgrade\n\
                    Difficulty: Easy (just plug it in, or pay tech $20-40)",
                    total_ram_gb, used_ram_gb, usage_percent
                ),
                impact_category: ImpactCategory::Performance,
                fix: Some(FixAction {
                    action_id: "show_ram_guide".to_string(),
                    label: "Show RAM Upgrade Guide".to_string(),
                    is_auto_fix: false,
                    params: json!({}),
                }),
            });
        }

        // High RAM usage (>90%) even with enough RAM
        if total_ram_gb >= 8 && usage_percent > 90.0 {
            return Some(Issue {
                id: "bottleneck_ram_exhaustion".to_string(),
                severity: IssueSeverity::Warning,
                title: format!("RAM Exhaustion - Using {:.0}% of {}GB", usage_percent, total_ram_gb),
                description: format!(
                    "You have enough RAM ({}GB), but you're using {:.0}% of it. This causes disk swapping, \
                    which makes your PC feel sluggish.\n\n\
                    HONEST SOLUTIONS (in order of impact):\n\
                    1. Close unused browser tabs (Chrome/Edge use 100-500MB per tab)\n\
                    2. Quit apps you're not actively using\n\
                    3. Check 'Startup' tab in Task Manager - disable unnecessary apps\n\
                    4. If this is constant, you may need more RAM\n\n\
                    What WON'T help:\n\
                    - 'RAM optimizers' (they just force disk swapping, making it worse)\n\
                    - Registry cleaners (negligible impact)\n\
                    - Defragmentation (you need to close apps, not reorganize files)",
                    total_ram_gb, usage_percent
                ),
                impact_category: ImpactCategory::Performance,
                fix: Some(FixAction {
                    action_id: "analyze_ram_hogs".to_string(),
                    label: "Show RAM-Heavy Apps".to_string(),
                    is_auto_fix: false,
                    params: json!({}),
                }),
            });
        }

        None
    }

    /// Analyze if CPU is the bottleneck
    fn analyze_cpu_bottleneck(&self, sys: &System) -> Option<Issue> {
        let cpu_count = sys.cpus().len();
        let cpu_name = sys.global_cpu_info().brand();

        // Detect old/low-end CPUs (simplified heuristic)
        let is_old_cpu = cpu_name.contains("Celeron")
            || cpu_name.contains("Pentium")
            || cpu_name.contains("Atom")
            || (cpu_count < 4);

        if is_old_cpu {
            return Some(Issue {
                id: "bottleneck_weak_cpu".to_string(),
                severity: IssueSeverity::Info,
                title: format!("Entry-Level CPU Detected - {} ({} cores)", cpu_name, cpu_count),
                description: format!(
                    "Your CPU ({}) is entry-level. This limits performance in CPU-heavy tasks like:\n\
                    - Video editing\n\
                    - Gaming\n\
                    - Video calls with background blur\n\
                    - Compiling code\n\n\
                    HONEST ASSESSMENT:\n\
                    For basic tasks (web browsing, documents, email), your CPU is fine. Software tweaks \
                    won't make a noticeable difference.\n\n\
                    If you do heavy tasks:\n\
                    - CPU upgrade = Entire PC replacement (CPU is not easily upgraded)\n\
                    - Cost: $400-1200 for new PC\n\
                    - Or: Adjust expectations - close heavy apps, lower video quality\n\n\
                    What WILL help a little:\n\
                    - Close background apps during heavy tasks\n\
                    - Disable startup programs\n\
                    - Keep only 1-2 browser windows open",
                    cpu_name
                ),
                impact_category: ImpactCategory::Performance,
                fix: None,  // Can't fix CPU with software
            });
        }

        None
    }

    /// The "Truth Bomb" - tell users when software can't fix hardware
    fn generate_honest_summary(&self, sys: &System) -> Option<Issue> {
        let total_ram_gb = sys.total_memory() / 1_073_741_824;
        let cpu_count = sys.cpus().len();

        // If system is reasonably modern, software optimizations WILL help
        if total_ram_gb >= 8 && cpu_count >= 4 {
            return Some(Issue {
                id: "bottleneck_software_optimizable".to_string(),
                severity: IssueSeverity::Info,
                title: "Good News: Your Hardware is Capable - Software Optimizations Will Help".to_string(),
                description: format!(
                    "Your system specs:\n\
                    - RAM: {}GB (Good)\n\
                    - CPU: {} cores (Good)\n\
                    - Disk: See other issues\n\n\
                    HONEST ASSESSMENT:\n\
                    Your hardware is capable of running smoothly. The slowness is likely from:\n\
                    - Bloatware/startup programs (we can fix)\n\
                    - Background processes (we can fix)\n\
                    - Outdated drivers (we can detect)\n\
                    - Disk fragmentation (we can fix)\n\n\
                    Software optimizations will make a REAL difference on your system. Let's fix the \
                    issues we found above.\n\n\
                    Unlike other 'PC cleaners' that promise miracles, we're telling you the truth: \
                    your hardware is good, so cleaning up software will genuinely help.",
                    total_ram_gb, cpu_count
                ),
                impact_category: ImpactCategory::Performance,
                fix: None,
            });
        }

        None
    }
}

impl Checker for BottleneckAnalyzer {
    fn name(&self) -> &'static str {
        "bottleneck_analyzer"
    }

    fn category(&self) -> CheckCategory {
        CheckCategory::Performance
    }

    fn run(&self, _context: &ScanContext) -> Vec<Issue> {
        let mut issues = Vec::new();
        let mut sys = System::new_all();
        sys.refresh_all();

        // Analyze hardware bottlenecks in order of impact
        if let Some(issue) = self.analyze_disk_bottleneck(&sys) {
            issues.push(issue);
        }

        if let Some(issue) = self.analyze_ram_bottleneck(&sys) {
            issues.push(issue);
        }

        if let Some(issue) = self.analyze_cpu_bottleneck(&sys) {
            issues.push(issue);
        }

        // Add honest summary
        if let Some(issue) = self.generate_honest_summary(&sys) {
            issues.push(issue);
        }

        // If no bottlenecks found, that's good news
        if issues.is_empty() {
            issues.push(Issue {
                id: "bottleneck_none".to_string(),
                severity: IssueSeverity::Info,
                title: "No Major Hardware Bottlenecks Detected".to_string(),
                description: "Your system has decent hardware. Any slowness is likely from software \
                    issues (bloatware, startup programs, etc.), which we can fix. Check the other \
                    issues in this scan.".to_string(),
                impact_category: ImpactCategory::Performance,
                fix: None,
            });
        }

        issues
    }

    fn fix(&self, issue_id: &str, _params: &serde_json::Value) -> Result<crate::FixResult, String> {
        match issue_id {
            "show_ssd_guide" => {
                Ok(crate::FixResult {
                    success: true,
                    message: "SSD Upgrade Guide:\n\n\
                        1. Check your PC model - find compatible SSD size (2.5\" SATA or M.2 NVMe)\n\
                        2. Buy SSD ($50-150 on Amazon/Newegg)\n\
                        3. Clone your current drive (free tools: Macrium Reflect, Clonezilla)\n\
                        4. Install SSD (YouTube has guides for your specific model)\n\
                        5. Boot from SSD - enjoy 10x faster PC!\n\n\
                        Or pay a local tech shop $50-100 to do it all for you.".to_string(),
                    rollback_available: false,
                    restore_point_id: None,
                })
            }
            "show_ram_guide" => {
                Ok(crate::FixResult {
                    success: true,
                    message: "RAM Upgrade Guide:\n\n\
                        1. Check your PC model - find RAM type (DDR3/DDR4/DDR5)\n\
                        2. Buy compatible RAM ($30-80 on Amazon)\n\
                        3. Shut down PC, open case\n\
                        4. Push RAM stick into empty slot until it clicks\n\
                        5. Close case, boot - done!\n\n\
                        This is the easiest PC upgrade. YouTube has 5-minute tutorials.".to_string(),
                    rollback_available: false,
                    restore_point_id: None,
                })
            }
            "analyze_ram_hogs" => {
                // This would show top RAM-using processes
                // Already handled by ProcessMonitor, so just guide user
                Ok(crate::FixResult {
                    success: true,
                    message: "Check the 'Process Monitor' issues above to see which apps are using the most RAM.\n\n\
                        Common RAM hogs:\n\
                        - Chrome/Edge: Close unused tabs, or use Firefox (uses 30-50% less RAM)\n\
                        - Discord: Uses 400-800MB even minimized\n\
                        - Spotify: Uses 300-600MB\n\
                        - OneDrive/Dropbox: Pause syncing when doing heavy work".to_string(),
                    rollback_available: false,
                    restore_point_id: None,
                })
            }
            _ => Err(format!("Unknown bottleneck fix: {}", issue_id))
        }
    }
}
