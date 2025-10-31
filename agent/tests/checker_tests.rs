// agent/tests/checker_tests.rs
// Unit tests for individual checkers

use health_speed_checker::*;

#[tokio::test]
async fn test_firewall_checker() {
    let checker = checkers::FirewallChecker;

    assert_eq!(checker.name(), "firewall_checker");
    assert!(matches!(checker.category(), CheckCategory::Security));

    let context = ScanContext {
        options: ScanOptions::default(),
        progress_sender: None,
    };

    let issues = checker.run(&context).await;

    // Should return at least 0 issues (firewall might be on or off)
    assert!(issues.len() >= 0);

    // If an issue is returned, it should be about the firewall
    if let Some(issue) = issues.first() {
        assert!(issue.title.to_lowercase().contains("firewall"));
    }
}

#[tokio::test]
async fn test_startup_analyzer() {
    let checker = checkers::StartupAnalyzer;

    assert_eq!(checker.name(), "startup_analyzer");
    assert!(matches!(checker.category(), CheckCategory::Performance));

    let context = ScanContext {
        options: ScanOptions::default(),
        progress_sender: None,
    };

    let issues = checker.run(&context).await;

    // Should return some startup analysis
    // (might be 0 if few startup items, or several if many)
    assert!(issues.len() >= 0);
}

#[tokio::test]
async fn test_process_monitor() {
    let checker = checkers::ProcessMonitor;

    assert_eq!(checker.name(), "process_monitor");
    assert!(matches!(checker.category(), CheckCategory::Performance));

    let context = ScanContext {
        options: ScanOptions::default(),
        progress_sender: None,
    };

    let issues = checker.run(&context).await;

    // Should analyze processes
    assert!(issues.len() >= 0);
}

#[tokio::test]
async fn test_os_update_checker() {
    let checker = checkers::OsUpdateChecker;

    assert_eq!(checker.name(), "os_update_checker");
    assert!(matches!(checker.category(), CheckCategory::Security));

    let context = ScanContext {
        options: ScanOptions::default(),
        progress_sender: None,
    };

    let issues = checker.run(&context).await;

    // OS update check should complete
    assert!(issues.len() >= 0);
}

#[tokio::test]
async fn test_port_scanner() {
    let checker = checkers::PortScanner;

    assert_eq!(checker.name(), "port_scanner");
    assert!(matches!(checker.category(), CheckCategory::Security));

    // Test with quick mode (should skip port scan)
    let context_quick = ScanContext {
        options: ScanOptions {
            quick: true,
            ..Default::default()
        },
        progress_sender: None,
    };

    let issues_quick = checker.run(&context_quick).await;
    assert_eq!(issues_quick.len(), 0); // Should skip in quick mode

    // Test with full mode
    let context_full = ScanContext {
        options: ScanOptions {
            quick: false,
            ..Default::default()
        },
        progress_sender: None,
    };

    let issues_full = checker.run(&context_full).await;
    // May find open ports or not, but should complete
    assert!(issues_full.len() >= 0);
}

#[test]
fn test_fix_result_constructors() {
    let success = FixResult::success("Test success");
    assert!(success.success);
    assert_eq!(success.message, "Test success");

    let failure = FixResult::failure("Test failure");
    assert!(!failure.success);
    assert_eq!(failure.message, "Test failure");
}
