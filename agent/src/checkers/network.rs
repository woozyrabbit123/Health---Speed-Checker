// Network Speed & Connectivity Checker
// Tests internet speed, latency, and connection stability

use crate::{Checker, CheckCategory, Issue, IssueSeverity, ImpactCategory, ScanContext, FixAction};
use std::io::Read;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};

pub struct NetworkChecker;

impl NetworkChecker {
    pub fn new() -> Self {
        Self
    }

    /// Test ping to multiple servers
    fn test_latency(&self) -> (u128, bool) {
        let test_hosts = [
            ("1.1.1.1:80", "Cloudflare"),
            ("8.8.8.8:80", "Google DNS"),
            ("208.67.222.222:80", "OpenDNS"),
        ];

        let mut total_latency = 0u128;
        let mut successful_pings = 0;

        for (host, _name) in &test_hosts {
            let start = Instant::now();

            if let Ok(addr) = host.to_socket_addrs() {
                if let Some(socket_addr) = addr.into_iter().next() {
                    if TcpStream::connect_timeout(&socket_addr, Duration::from_secs(2)).is_ok() {
                        let latency = start.elapsed().as_millis();
                        total_latency += latency;
                        successful_pings += 1;
                    }
                }
            }
        }

        if successful_pings > 0 {
            (total_latency / successful_pings as u128, true)
        } else {
            (999, false)
        }
    }

    /// Download speed test using ureq HTTP client
    /// Downloads a small file and measures transfer speed
    fn test_download_speed(&self) -> Option<f64> {
        // Test URL: 10MB file from Cloudflare speed test
        let test_url = "https://speed.cloudflare.com/__down?bytes=10000000";

        let start = Instant::now();

        match ureq::get(test_url)
            .timeout(Duration::from_secs(10))
            .call()
        {
            Ok(response) => {
                let mut bytes_downloaded = 0usize;
                let mut buffer = vec![0u8; 8192]; // 8KB buffer
                let mut reader = response.into_reader();

                loop {
                    match reader.read(&mut buffer) {
                        Ok(0) => break,
                        Ok(n) => bytes_downloaded += n,
                        Err(_) => break,
                    }
                }

                let elapsed = start.elapsed().as_secs_f64();

                if elapsed > 0.0 && bytes_downloaded > 0 {
                    // Convert to Mbps: (bytes * 8) / (seconds * 1_000_000)
                    let mbps = (bytes_downloaded as f64 * 8.0) / (elapsed * 1_000_000.0);
                    Some(mbps)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    /// Test DNS resolution speed
    fn test_dns_resolution(&self) -> (u128, bool) {
        let test_domains = [
            "google.com",
            "cloudflare.com",
            "amazon.com",
        ];

        let mut total_time = 0u128;
        let mut successful = 0;

        for domain in &test_domains {
            let start = Instant::now();
            if format!("{}:80", domain).to_socket_addrs().is_ok() {
                total_time += start.elapsed().as_millis();
                successful += 1;
            }
        }

        if successful > 0 {
            (total_time / successful as u128, true)
        } else {
            (999, false)
        }
    }

    /// Check if behind a proxy or VPN
    fn detect_proxy(&self) -> bool {
        // Check common proxy environment variables
        std::env::var("HTTP_PROXY").is_ok() ||
        std::env::var("HTTPS_PROXY").is_ok() ||
        std::env::var("http_proxy").is_ok() ||
        std::env::var("https_proxy").is_ok()
    }

    /// Get the name of the active network adapter (Windows)
    #[cfg(target_os = "windows")]
    fn get_active_network_adapter(&self) -> Option<String> {
        use std::process::Command;

        // Use ipconfig to find the active adapter with a default gateway
        let output = Command::new("ipconfig")
            .arg("/all")
            .output()
            .ok()?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        let mut current_adapter: Option<String> = None;
        let mut has_gateway = false;

        for line in stdout.lines() {
            let trimmed = line.trim();

            // New adapter section starts
            if !trimmed.starts_with(' ') && trimmed.contains("adapter") {
                // Save previous adapter if it had a gateway
                if let Some(adapter) = current_adapter.take() {
                    if has_gateway {
                        return Some(adapter);
                    }
                }

                // Extract adapter name from line like "Ethernet adapter Ethernet:"
                if let Some(name) = trimmed.split("adapter").nth(1) {
                    let adapter_name = name.trim_end_matches(':').trim().to_string();
                    current_adapter = Some(adapter_name);
                    has_gateway = false;
                }
            }

            // Check if this adapter has a default gateway (active connection)
            if trimmed.contains("Default Gateway") && !trimmed.ends_with(':') {
                has_gateway = true;
            }
        }

        // Check last adapter
        if let Some(adapter) = current_adapter {
            if has_gateway {
                return Some(adapter);
            }
        }

        None
    }

    /// Get active adapter for non-Windows platforms
    #[cfg(not(target_os = "windows"))]
    fn get_active_network_adapter(&self) -> Option<String> {
        // On Linux/macOS, use the interface with default route
        use std::process::Command;

        let output = Command::new("ip")
            .args(&["route", "show", "default"])
            .output()
            .ok()?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Parse line like: "default via 192.168.1.1 dev eth0"
        for part in stdout.split_whitespace() {
            if let Some(pos) = stdout.find("dev") {
                if let Some(interface) = stdout[pos..].split_whitespace().nth(1) {
                    return Some(interface.to_string());
                }
            }
        }

        None
    }
}

impl Checker for NetworkChecker {
    fn name(&self) -> &'static str {
        "Network & Speed Checker"
    }

    fn category(&self) -> CheckCategory {
        CheckCategory::Performance
    }

    fn run(&self, _context: &ScanContext) -> Vec<Issue> {
        let mut issues = Vec::new();

        // Test latency
        let (avg_latency, latency_success) = self.test_latency();

        if !latency_success {
            issues.push(Issue {
                id: "network_no_connection".to_string(),
                severity: IssueSeverity::Critical,
                title: "No Internet Connection".to_string(),
                description: "Unable to reach external servers. Check your network connection.".to_string(),
                impact_category: ImpactCategory::Performance,
                fix: None,
            });
        } else if avg_latency > 150 {
            issues.push(Issue {
                id: "network_high_latency".to_string(),
                severity: if avg_latency > 300 { IssueSeverity::Critical } else { IssueSeverity::Warning },
                title: format!("High Network Latency ({}ms)", avg_latency),
                description: format!(
                    "Your network latency is {}ms. Good latency is under 50ms. This may cause lag in online activities.",
                    avg_latency
                ),
                impact_category: ImpactCategory::Performance,
                fix: None,
            });
        }

        // Test DNS resolution
        let (dns_time, dns_success) = self.test_dns_resolution();

        if !dns_success {
            issues.push(Issue {
                id: "network_dns_failure".to_string(),
                severity: IssueSeverity::Critical,
                title: "DNS Resolution Failure".to_string(),
                description: "Unable to resolve domain names. Your DNS server may be unavailable.".to_string(),
                impact_category: ImpactCategory::Performance,
                fix: Some(FixAction {
                    action_id: "fix_dns".to_string(),
                    label: if cfg!(target_os = "windows") {
                        "Change DNS to Cloudflare (1.1.1.1)".to_string()
                    } else {
                        "Show DNS Fix Instructions".to_string()
                    },
                    is_auto_fix: cfg!(target_os = "windows"),  // Auto-fix on Windows only
                    params: serde_json::json!({}),
                }),
            });
        } else if dns_time > 100 {
            issues.push(Issue {
                id: "network_slow_dns".to_string(),
                severity: IssueSeverity::Info,
                title: format!("Slow DNS Resolution ({}ms)", dns_time),
                description: format!(
                    "DNS lookups are taking {}ms. Consider switching to faster DNS servers like Cloudflare (1.1.1.1) or Google (8.8.8.8).",
                    dns_time
                ),
                impact_category: ImpactCategory::Performance,
                fix: Some(FixAction {
                    action_id: "fix_dns".to_string(),
                    label: if cfg!(target_os = "windows") {
                        "Change DNS to Cloudflare (1.1.1.1)".to_string()
                    } else {
                        "Show DNS Fix Instructions".to_string()
                    },
                    is_auto_fix: cfg!(target_os = "windows"),  // Auto-fix on Windows only
                    params: serde_json::json!({}),
                }),
            });
        }

        // Download speed test (now enabled with ureq)
        if let Some(speed_mbps) = self.test_download_speed() {
            if speed_mbps < 5.0 {
                issues.push(Issue {
                    id: "network_slow_speed".to_string(),
                    severity: if speed_mbps < 1.0 { IssueSeverity::Critical } else { IssueSeverity::Warning },
                    title: format!("Slow Download Speed ({:.1} Mbps)", speed_mbps),
                    description: format!(
                        "Your download speed is {:.1} Mbps. This is quite slow for modern usage. Contact your ISP if this persists.",
                        speed_mbps
                    ),
                    impact_category: ImpactCategory::Performance,
                    fix: None,
                });
            }
        }

        // Check for proxy/VPN
        if self.detect_proxy() {
            issues.push(Issue {
                id: "network_proxy_detected".to_string(),
                severity: IssueSeverity::Info,
                title: "Proxy/VPN Detected".to_string(),
                description: "A proxy or VPN is configured. This may slow down your connection.".to_string(),
                impact_category: ImpactCategory::Performance,
                fix: None,
            });
        }

        issues
    }

    fn fix(&self, issue_id: &str, _params: &serde_json::Value) -> Result<crate::FixResult, String> {
        match issue_id {
            "network_dns_failure" | "network_slow_dns" => {
                #[cfg(target_os = "windows")]
                {
                    use std::process::Command;

                    // Find the active network adapter
                    let adapter_name = self.get_active_network_adapter()
                        .ok_or_else(|| "Could not detect active network adapter".to_string())?;

                    use std::time::Duration;
                    use crate::util::command::run_with_timeout;

                    // Set DNS to Cloudflare (1.1.1.1) using netsh with timeout
                    let output = run_with_timeout({
                        let mut c = Command::new("netsh");
                        c.args([
                            "interface",
                            "ip",
                            "set",
                            "dns",
                            &format!("name=\"{}\"", adapter_name),
                            "static",
                            "1.1.1.1",
                            "primary",
                        ]);
                        c
                    }, Duration::from_secs(5))
                    .map_err(|e| format!("Failed to set DNS: {}. You may need administrator privileges.", e))?;

                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        return Err(format!("Failed to set DNS: {}. Try running as administrator.", stderr));
                    }

                    // Add secondary DNS (1.0.0.1)
                    let _ = run_with_timeout({
                        let mut c = Command::new("netsh");
                        c.args([
                            "interface",
                            "ip",
                            "add",
                            "dns",
                            &format!("name=\"{}\"", adapter_name),
                            "1.0.0.1",
                            "index=2",
                        ]);
                        c
                    }, Duration::from_secs(5));

                    Ok(crate::FixResult {
                        success: true,
                        message: format!(
                            "DNS changed to Cloudflare (1.1.1.1) on adapter '{}'. \
                            You may need to restart your browser for changes to take effect.",
                            adapter_name
                        ),
                        rollback_available: true,
                        restore_point_id: Some(adapter_name.clone()),
                    })
                }

                #[cfg(not(target_os = "windows"))]
                {
                    // For Linux/macOS, provide manual instructions
                    Err(
                        "DNS auto-fix is only available on Windows. To manually fix:\n\
                        Linux: Edit /etc/resolv.conf and add 'nameserver 1.1.1.1'\n\
                        macOS: System Preferences > Network > Advanced > DNS > Add 1.1.1.1".to_string()
                    )
                }
            }
            _ => Err("This issue cannot be fixed automatically.".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checker_name() {
        let checker = NetworkChecker::new();
        assert_eq!(checker.name(), "Network & Speed Checker");
    }

    #[test]
    fn test_proxy_detection() {
        let checker = NetworkChecker::new();
        // This will pass even if no proxy is set
        let _ = checker.detect_proxy();
    }
}
