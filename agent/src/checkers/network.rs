// Network Speed & Connectivity Checker
// Tests internet speed, latency, and connection stability

use crate::{Checker, CheckCategory, Issue, IssueSeverity, ImpactCategory, ScanContext, FixAction};
use std::time::{Duration, Instant};
use std::net::{TcpStream, ToSocketAddrs};

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
            if let Ok(start) = Instant::now().elapsed().as_millis().try_into() {
                if TcpStream::connect_timeout(
                    &host.to_socket_addrs().ok()?.next()?,
                    Duration::from_secs(2)
                ).is_ok() {
                    let latency = Instant::now().duration_since(
                        Instant::now() - Duration::from_millis(start)
                    ).as_millis();
                    total_latency += latency;
                    successful_pings += 1;
                }
            }
        }

        if successful_pings > 0 {
            (total_latency / successful_pings as u128, true)
        } else {
            (999, false)
        }
    }

    /// Simple download speed test (approximate)
    fn test_download_speed(&self) -> Option<f64> {
        use std::io::Read;

        // Test download from a reliable source
        let test_url = "http://speedtest.ftp.otenet.gr/files/test1Mb.db";

        let start = Instant::now();

        // Simple HTTP GET (in production, use reqwest crate)
        if let Ok(mut stream) = TcpStream::connect("speedtest.ftp.otenet.gr:80") {
            use std::io::Write;
            let _ = stream.write_all(b"GET /files/test1Mb.db HTTP/1.0\r\nHost: speedtest.ftp.otenet.gr\r\n\r\n");

            let mut buffer = vec![0u8; 1024 * 1024]; // 1MB buffer
            let mut total_bytes = 0;

            while let Ok(bytes_read) = stream.read(&mut buffer) {
                if bytes_read == 0 {
                    break;
                }
                total_bytes += bytes_read;

                // Stop after 2 seconds or 5MB
                if start.elapsed().as_secs() > 2 || total_bytes > 5_000_000 {
                    break;
                }
            }

            let duration = start.elapsed().as_secs_f64();
            if duration > 0.0 {
                let mbps = (total_bytes as f64 * 8.0) / (duration * 1_000_000.0);
                return Some(mbps);
            }
        }

        None
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
                    label: "Switch to Cloudflare DNS".to_string(),
                    is_auto_fix: true,
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
                    label: "Switch to Cloudflare DNS".to_string(),
                    is_auto_fix: true,
                    params: serde_json::json!({}),
                }),
            });
        }

        // Test download speed (optional, may take time)
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

                    // Set Cloudflare DNS (1.1.1.1, 1.0.0.1)
                    let output = Command::new("netsh")
                        .args(&[
                            "interface", "ip", "set", "dns",
                            "name=\"Ethernet\"",
                            "static", "1.1.1.1", "primary"
                        ])
                        .output();

                    if output.is_ok() {
                        // Set secondary DNS
                        let _ = Command::new("netsh")
                            .args(&[
                                "interface", "ip", "add", "dns",
                                "name=\"Ethernet\"",
                                "1.0.0.1", "index=2"
                            ])
                            .output();

                        return Ok(crate::FixResult {
                            success: true,
                            message: "Switched DNS to Cloudflare (1.1.1.1). Restart your browser.".to_string(),
                            rollback_available: false,
                            restore_point_id: None,
                        });
                    }
                }

                Err("Cannot auto-fix DNS. Manually change DNS to 1.1.1.1 (Cloudflare) or 8.8.8.8 (Google) in network settings.".to_string())
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
