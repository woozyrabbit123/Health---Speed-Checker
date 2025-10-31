// agent/src/checkers/mod.rs
// Checker implementations for Health & Speed Checker

pub mod firewall;
pub mod startup;
pub mod process;
pub mod os_update;
pub mod ports;

pub use firewall::FirewallChecker;
pub use startup::StartupAnalyzer;
pub use process::ProcessMonitor;
pub use os_update::OsUpdateChecker;
pub use ports::PortScanner;

// =============================================================================
// FIREWALL CHECKER
// =============================================================================

pub mod firewall {
    use crate::*;
    use async_trait::async_trait;

    pub struct FirewallChecker;

    #[async_trait]
    impl Checker for FirewallChecker {
        fn name(&self) -> &'static str {
            "firewall_checker"
        }

        fn category(&self) -> CheckCategory {
            CheckCategory::Security
        }

        async fn run(&self, _context: &ScanContext) -> Vec<Issue> {
            let mut issues = Vec::new();

            #[cfg(target_os = "windows")]
            {
                if let Ok(is_enabled) = check_windows_firewall().await {
                    if !is_enabled {
                        issues.push(Issue {
                            id: "firewall_disabled".to_string(),
                            severity: IssueSeverity::Critical,
                            title: "Windows Firewall is OFF".to_string(),
                            description: "Your firewall protects against network attacks. Having it disabled leaves your computer vulnerable.".to_string(),
                            impact_category: ImpactCategory::Security,
                            fix: Some(FixAction {
                                action_id: "enable_firewall".to_string(),
                                label: "Enable Firewall".to_string(),
                                is_auto_fix: true,
                                params: serde_json::json!({}),
                            }),
                        });
                    }
                }
            }

            issues
        }

        async fn fix(&self, issue_id: &str, _params: &serde_json::Value) -> Result<FixResult, String> {
            if issue_id == "enable_firewall" {
                #[cfg(target_os = "windows")]
                {
                    enable_windows_firewall().await?;
                    return Ok(FixResult::success("Windows Firewall enabled successfully"));
                }

                #[cfg(not(target_os = "windows"))]
                return Err("Firewall fix only implemented for Windows".to_string());
            }

            Err(format!("Unknown fix action: {}", issue_id))
        }
    }

    #[cfg(target_os = "windows")]
    async fn check_windows_firewall() -> Result<bool, String> {
        use std::process::Command;

        let output = Command::new("netsh")
            .args(&["advfirewall", "show", "currentprofile", "state"])
            .output()
            .map_err(|e| format!("Failed to check firewall: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.contains("ON"))
    }

    #[cfg(target_os = "windows")]
    async fn enable_windows_firewall() -> Result<(), String> {
        use std::process::Command;

        Command::new("netsh")
            .args(&["advfirewall", "set", "currentprofile", "state", "on"])
            .output()
            .map_err(|e| format!("Failed to enable firewall: {}", e))?;

        Ok(())
    }
}

// =============================================================================
// STARTUP ANALYZER
// =============================================================================

pub mod startup {
    use crate::*;
    use async_trait::async_trait;

    pub struct StartupAnalyzer;

    #[async_trait]
    impl Checker for StartupAnalyzer {
        fn name(&self) -> &'static str {
            "startup_analyzer"
        }

        fn category(&self) -> CheckCategory {
            CheckCategory::Performance
        }

        async fn run(&self, context: &ScanContext) -> Vec<Issue> {
            let mut issues = Vec::new();

            if context.options.exclude_startup {
                return issues;
            }

            let startup_items = get_startup_items().await.unwrap_or_default();

            if startup_items.len() > 15 {
                issues.push(Issue {
                    id: "excessive_startup_items".to_string(),
                    severity: IssueSeverity::Warning,
                    title: format!("{} apps slow your boot", startup_items.len()),
                    description: format!(
                        "You have {} programs starting with Windows. Each adds 0.5-2 seconds to boot time. Consider disabling unnecessary ones.",
                        startup_items.len()
                    ),
                    impact_category: ImpactCategory::Performance,
                    fix: Some(FixAction {
                        action_id: "optimize_startup".to_string(),
                        label: "Optimize Startup".to_string(),
                        is_auto_fix: false,
                        params: serde_json::json!({
                            "count": startup_items.len(),
                            "items": startup_items.iter()
                                .take(10)
                                .map(|item| &item.name)
                                .collect::<Vec<_>>()
                        }),
                    }),
                });
            }

            // Check for specific problematic startup items
            for item in &startup_items {
                if is_known_bloatware(&item.name) {
                    issues.push(Issue {
                        id: format!("bloatware_startup_{}", item.name.to_lowercase().replace(" ", "_")),
                        severity: IssueSeverity::Info,
                        title: format!("{} is known bloatware", item.name),
                        description: "This program is known to slow down your computer without providing much value.".to_string(),
                        impact_category: ImpactCategory::Performance,
                        fix: Some(FixAction {
                            action_id: format!("disable_startup_{}", item.name),
                            label: "Disable".to_string(),
                            is_auto_fix: true,
                            params: serde_json::json!({ "name": item.name }),
                        }),
                    });
                }
            }

            issues
        }
    }

    async fn get_startup_items() -> Result<Vec<StartupItem>, String> {
        let mut items = Vec::new();

        #[cfg(target_os = "windows")]
        {
            use std::process::Command;

            // Check registry startup items
            let output = Command::new("wmic")
                .args(&["startup", "get", "name,command", "/format:csv"])
                .output()
                .map_err(|e| format!("Failed to get startup items: {}", e))?;

            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines().skip(2) {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 3 {
                    items.push(StartupItem {
                        name: parts[1].to_string(),
                        path: parts[2].to_string(),
                        estimated_delay_ms: 1000, // Default estimate
                        can_disable: true,
                    });
                }
            }
        }

        Ok(items)
    }

    fn is_known_bloatware(name: &str) -> bool {
        let bloatware_patterns = [
            "mcafee",
            "norton",
            "wildtangent",
            "candy crush",
            "spotify web helper",
            "skype",
        ];

        let name_lower = name.to_lowercase();
        bloatware_patterns.iter().any(|pattern| name_lower.contains(pattern))
    }
}

// =============================================================================
// PROCESS MONITOR
// =============================================================================

pub mod process {
    use crate::*;
    use async_trait::async_trait;

    pub struct ProcessMonitor;

    #[async_trait]
    impl Checker for ProcessMonitor {
        fn name(&self) -> &'static str {
            "process_monitor"
        }

        fn category(&self) -> CheckCategory {
            CheckCategory::Performance
        }

        async fn run(&self, _context: &ScanContext) -> Vec<Issue> {
            let mut issues = Vec::new();

            if let Ok(top_processes) = get_top_cpu_processes(5).await {
                for process in top_processes {
                    if process.cpu_percent > 50.0 && !is_system_process(&process.name) {
                        issues.push(Issue {
                            id: format!("high_cpu_{}", sanitize_id(&process.name)),
                            severity: IssueSeverity::Warning,
                            title: format!("{} using {:.1}% CPU", process.name, process.cpu_percent),
                            description: format!(
                                "This application is consuming significant CPU resources, which may slow down your computer."
                            ),
                            impact_category: ImpactCategory::Performance,
                            fix: Some(FixAction {
                                action_id: "kill_process".to_string(),
                                label: "Stop Process".to_string(),
                                is_auto_fix: false,
                                params: serde_json::json!({
                                    "pid": process.pid,
                                    "name": process.name
                                }),
                            }),
                        });
                    }
                }

                // Check for memory hogs
                for process in &top_processes {
                    if process.memory_mb > 2048.0 && !is_system_process(&process.name) {
                        issues.push(Issue {
                            id: format!("high_memory_{}", sanitize_id(&process.name)),
                            severity: IssueSeverity::Info,
                            title: format!("{} using {:.1} GB RAM", process.name, process.memory_mb / 1024.0),
                            description: "This application is using a lot of memory.".to_string(),
                            impact_category: ImpactCategory::Performance,
                            fix: Some(FixAction {
                                action_id: "restart_process".to_string(),
                                label: "Restart App".to_string(),
                                is_auto_fix: false,
                                params: serde_json::json!({
                                    "pid": process.pid,
                                    "name": process.name
                                }),
                            }),
                        });
                    }
                }
            }

            issues
        }
    }

    async fn get_top_cpu_processes(limit: usize) -> Result<Vec<ProcessInfo>, String> {
        let mut processes = Vec::new();

        #[cfg(target_os = "windows")]
        {
            use std::process::Command;

            let output = Command::new("wmic")
                .args(&[
                    "process",
                    "get",
                    "ProcessId,Name,WorkingSetSize,PageFileUsage",
                    "/format:csv"
                ])
                .output()
                .map_err(|e| format!("Failed to get processes: {}", e))?;

            let stdout = String::from_utf8_lossy(&output.stdout);

            // Parse CSV output (skip header lines)
            for line in stdout.lines().skip(2).take(limit) {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 5 {
                    if let Ok(pid) = parts[3].parse::<u32>() {
                        if let Ok(memory_bytes) = parts[4].parse::<u64>() {
                            processes.push(ProcessInfo {
                                pid,
                                name: parts[1].to_string(),
                                cpu_percent: 0.0, // TODO: Get actual CPU usage
                                memory_mb: (memory_bytes / 1024 / 1024) as f32,
                            });
                        }
                    }
                }
            }
        }

        // Sort by memory usage for now (since we don't have CPU data yet)
        processes.sort_by(|a, b| b.memory_mb.partial_cmp(&a.memory_mb).unwrap());
        processes.truncate(limit);

        Ok(processes)
    }

    fn is_system_process(name: &str) -> bool {
        let system_processes = [
            "system",
            "registry",
            "smss.exe",
            "csrss.exe",
            "wininit.exe",
            "services.exe",
            "lsass.exe",
            "svchost.exe",
            "kernel_task",
            "systemd",
        ];

        let name_lower = name.to_lowercase();
        system_processes.iter().any(|&p| name_lower.contains(p))
    }

    fn sanitize_id(name: &str) -> String {
        name.to_lowercase()
            .replace(" ", "_")
            .replace(".", "_")
            .replace("(", "")
            .replace(")", "")
    }
}

// =============================================================================
// OS UPDATE CHECKER
// =============================================================================

pub mod os_update {
    use crate::*;
    use async_trait::async_trait;

    pub struct OsUpdateChecker;

    #[async_trait]
    impl Checker for OsUpdateChecker {
        fn name(&self) -> &'static str {
            "os_update_checker"
        }

        fn category(&self) -> CheckCategory {
            CheckCategory::Security
        }

        async fn run(&self, _context: &ScanContext) -> Vec<Issue> {
            let mut issues = Vec::new();

            #[cfg(target_os = "windows")]
            {
                if let Ok(update_status) = check_windows_updates().await {
                    if update_status.pending_updates > 0 {
                        let severity = if update_status.pending_updates > 5 {
                            IssueSeverity::Critical
                        } else {
                            IssueSeverity::Warning
                        };

                        issues.push(Issue {
                            id: "windows_update_pending".to_string(),
                            severity,
                            title: format!("{} Windows updates available", update_status.pending_updates),
                            description: "Keeping Windows updated is critical for security. Updates often include patches for vulnerabilities.".to_string(),
                            impact_category: ImpactCategory::Security,
                            fix: Some(FixAction {
                                action_id: "install_windows_updates".to_string(),
                                label: "Install Updates".to_string(),
                                is_auto_fix: false, // Requires user consent
                                params: serde_json::json!({
                                    "count": update_status.pending_updates
                                }),
                            }),
                        });
                    }
                }
            }

            issues
        }
    }

    #[cfg(target_os = "windows")]
    async fn check_windows_updates() -> Result<OsUpdateStatus, String> {
        use std::process::Command;

        // This is a simplified check - real implementation would use Windows Update API
        let output = Command::new("wmic")
            .args(&["qfe", "list", "brief", "/format:csv"])
            .output()
            .map_err(|e| format!("Failed to check updates: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let update_count = stdout.lines().count().saturating_sub(2); // Subtract header lines

        Ok(OsUpdateStatus {
            is_current: update_count == 0,
            current_build: get_windows_build().await.unwrap_or_else(|_| "Unknown".to_string()),
            latest_build: None,
            pending_updates: if update_count == 0 { 0 } else { 3 }, // Simplified
        })
    }

    #[cfg(target_os = "windows")]
    async fn get_windows_build() -> Result<String, String> {
        use std::process::Command;

        let output = Command::new("cmd")
            .args(&["/c", "ver"])
            .output()
            .map_err(|e| format!("Failed to get Windows version: {}", e))?;

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

// =============================================================================
// PORT SCANNER
// =============================================================================

pub mod ports {
    use crate::*;
    use async_trait::async_trait;
    use std::collections::HashSet;

    pub struct PortScanner;

    #[async_trait]
    impl Checker for PortScanner {
        fn name(&self) -> &'static str {
            "port_scanner"
        }

        fn category(&self) -> CheckCategory {
            CheckCategory::Security
        }

        async fn run(&self, context: &ScanContext) -> Vec<Issue> {
            let mut issues = Vec::new();

            if context.options.quick {
                // Skip port scan in quick mode
                return issues;
            }

            if let Ok(open_ports) = scan_open_ports().await {
                for port_info in open_ports {
                    if is_risky_port(&port_info) && !is_whitelisted_port(&port_info) {
                        issues.push(Issue {
                            id: format!("port_open_{}", port_info.port),
                            severity: match port_info.port {
                                3389 | 22 | 23 => IssueSeverity::Critical, // RDP, SSH, Telnet
                                445 | 139 => IssueSeverity::Warning,        // SMB
                                _ => IssueSeverity::Info,
                            },
                            title: format!(
                                "Port {} ({}) is open",
                                port_info.port,
                                port_info.service.as_ref().unwrap_or(&"Unknown".to_string())
                            ),
                            description: get_port_description(&port_info),
                            impact_category: ImpactCategory::Security,
                            fix: Some(FixAction {
                                action_id: format!("close_port_{}", port_info.port),
                                label: "Close Port".to_string(),
                                is_auto_fix: false,
                                params: serde_json::json!({
                                    "port": port_info.port,
                                    "service": port_info.service
                                }),
                            }),
                        });
                    }
                }
            }

            issues
        }
    }

    async fn scan_open_ports() -> Result<Vec<PortInfo>, String> {
        let mut ports = Vec::new();

        #[cfg(target_os = "windows")]
        {
            use std::process::Command;

            let output = Command::new("netstat")
                .args(&["-an"])
                .output()
                .map_err(|e| format!("Failed to scan ports: {}", e))?;

            let stdout = String::from_utf8_lossy(&output.stdout);

            let mut seen_ports = HashSet::new();

            for line in stdout.lines() {
                if line.contains("LISTENING") {
                    // Parse lines like "  TCP    0.0.0.0:3389           0.0.0.0:0              LISTENING"
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Some(addr) = parts.get(1) {
                            if let Some(port_str) = addr.split(':').last() {
                                if let Ok(port) = port_str.parse::<u16>() {
                                    if !seen_ports.contains(&port) && port < 10000 {
                                        seen_ports.insert(port);
                                        ports.push(PortInfo {
                                            port,
                                            protocol: "TCP".to_string(),
                                            service: get_service_name(port),
                                            process: None,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(ports)
    }

    fn get_service_name(port: u16) -> Option<String> {
        match port {
            22 => Some("SSH".to_string()),
            23 => Some("Telnet".to_string()),
            80 => Some("HTTP".to_string()),
            443 => Some("HTTPS".to_string()),
            445 => Some("SMB".to_string()),
            3389 => Some("RDP".to_string()),
            3306 => Some("MySQL".to_string()),
            5432 => Some("PostgreSQL".to_string()),
            8080 => Some("HTTP-Alt".to_string()),
            _ => None,
        }
    }

    fn is_risky_port(port_info: &PortInfo) -> bool {
        matches!(port_info.port, 22 | 23 | 139 | 445 | 3389 | 5900)
    }

    fn is_whitelisted_port(port_info: &PortInfo) -> bool {
        // Whitelist common development ports
        matches!(port_info.port, 3000 | 5000 | 8000 | 8080 | 5432 | 3306 | 6379)
    }

    fn get_port_description(port_info: &PortInfo) -> String {
        match port_info.port {
            3389 => "Remote Desktop (RDP) is exposed. This allows remote access to your computer. Close this unless you specifically need remote access.".to_string(),
            445 | 139 => "SMB file sharing is exposed. This can allow network access to your files.".to_string(),
            22 => "SSH is open. This allows remote command-line access to your computer.".to_string(),
            23 => "Telnet is open. This is an insecure protocol and should be disabled.".to_string(),
            _ => format!("Port {} is open to network connections.", port_info.port),
        }
    }
}
