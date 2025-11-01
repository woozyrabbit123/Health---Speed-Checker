// agent/tests/checker_tests.rs
// Unit tests for individual checkers

use health_speed_checker::*;

// ===== INLINE CHECKERS (from mod.rs) =====

#[test]
fn test_firewall_checker_metadata() {
    let checker = checkers::FirewallChecker;
    assert_eq!(checker.name(), "firewall_checker");
    assert!(matches!(checker.category(), CheckCategory::Security));
}

#[test]
fn test_firewall_checker_run() {
    let checker = checkers::FirewallChecker;
    let context = ScanContext {
        options: ScanOptions::default(),
    };

    let issues = checker.run(&context);

    // Firewall checker should always complete without panic
    // May return 0 issues (firewall on) or 1+ issues (firewall off/misconfigured)
    for issue in &issues {
        // Verify all issues have proper structure
        assert!(!issue.id.is_empty(), "Issue ID must not be empty");
        assert!(!issue.title.is_empty(), "Issue title must not be empty");
        assert!(!issue.description.is_empty(), "Issue description must not be empty");
        assert!(issue.title.to_lowercase().contains("firewall"),
            "Firewall issue should mention 'firewall' in title");
    }
}

#[test]
fn test_startup_analyzer_metadata() {
    let checker = checkers::StartupAnalyzer;
    assert_eq!(checker.name(), "startup_analyzer");
    assert!(matches!(checker.category(), CheckCategory::Performance));
}

#[test]
fn test_startup_analyzer_run() {
    let checker = checkers::StartupAnalyzer;
    let context = ScanContext {
        options: ScanOptions::default(),
    };

    let issues = checker.run(&context);

    // Verify all issues have proper structure
    for issue in &issues {
        assert!(!issue.id.is_empty());
        assert!(!issue.title.is_empty());
        assert!(!issue.description.is_empty());
        assert!(matches!(issue.severity, IssueSeverity::Critical | IssueSeverity::Warning | IssueSeverity::Info));
        assert!(matches!(issue.impact_category, ImpactCategory::Performance | ImpactCategory::Both));
    }
}

#[test]
fn test_startup_analyzer_skip_when_excluded() {
    let checker = checkers::StartupAnalyzer;
    let context = ScanContext {
        options: ScanOptions {
            exclude_startup: true,
            ..Default::default()
        },
    };

    let issues = checker.run(&context);

    // Should return empty when startup is excluded
    assert_eq!(issues.len(), 0, "StartupAnalyzer should skip when exclude_startup is true");
}

#[test]
fn test_process_monitor_metadata() {
    let checker = checkers::ProcessMonitor;
    assert_eq!(checker.name(), "process_monitor");
    assert!(matches!(checker.category(), CheckCategory::Performance));
}

#[test]
fn test_process_monitor_run() {
    let checker = checkers::ProcessMonitor;
    let context = ScanContext {
        options: ScanOptions::default(),
    };

    let issues = checker.run(&context);

    // Process monitor should always complete
    for issue in &issues {
        assert!(!issue.id.is_empty());
        assert!(!issue.title.is_empty());
        assert!(matches!(issue.impact_category, ImpactCategory::Performance | ImpactCategory::Both));
    }
}

#[test]
fn test_os_update_checker_metadata() {
    let checker = checkers::OsUpdateChecker;
    assert_eq!(checker.name(), "os_update_checker");
    assert!(matches!(checker.category(), CheckCategory::Security));
}

#[test]
fn test_os_update_checker_run() {
    let checker = checkers::OsUpdateChecker;
    let context = ScanContext {
        options: ScanOptions::default(),
    };

    let issues = checker.run(&context);

    // OS update checker should complete without panic
    for issue in &issues {
        assert!(!issue.id.is_empty());
        assert!(!issue.title.is_empty());
        assert!(matches!(issue.impact_category, ImpactCategory::Security | ImpactCategory::Both));

        // OS update issues should be at least Warning severity
        assert!(matches!(issue.severity, IssueSeverity::Critical | IssueSeverity::Warning));
    }
}

#[test]
fn test_port_scanner_metadata() {
    let checker = checkers::PortScanner;
    assert_eq!(checker.name(), "port_scanner");
    assert!(matches!(checker.category(), CheckCategory::Security));
}

#[test]
fn test_port_scanner_skip_quick_mode() {
    let checker = checkers::PortScanner;

    // Test with quick mode (should skip port scan)
    let context_quick = ScanContext {
        options: ScanOptions {
            quick: true,
            ..Default::default()
        },
    };

    let issues_quick = checker.run(&context_quick);
    assert_eq!(issues_quick.len(), 0, "PortScanner should skip in quick mode");
}

#[test]
fn test_port_scanner_full_mode() {
    let checker = checkers::PortScanner;

    // Test with full mode
    let context_full = ScanContext {
        options: ScanOptions {
            quick: false,
            ..Default::default()
        },
    };

    let issues_full = checker.run(&context_full);

    // May find open ports or not, but should complete without panic
    for issue in &issues_full {
        assert!(!issue.id.is_empty());
        assert!(issue.id.starts_with("port_open_"));
        assert!(matches!(issue.severity, IssueSeverity::Warning | IssueSeverity::Info));
        assert!(matches!(issue.impact_category, ImpactCategory::Security));
    }
}

// ===== EXTERNAL CHECKERS (separate files) =====

#[test]
fn test_bloatware_detector_metadata() {
    let checker = checkers::BloatwareDetector::new();
    assert_eq!(checker.name(), "Bloatware Detector");
    assert!(matches!(checker.category(), CheckCategory::Performance));
}

#[test]
fn test_bloatware_detector_run() {
    let checker = checkers::BloatwareDetector::new();
    let context = ScanContext {
        options: ScanOptions::default(),
    };

    let issues = checker.run(&context);

    // Bloatware detector should complete without panic
    for issue in &issues {
        assert!(!issue.id.is_empty());
        assert!(issue.id.starts_with("bloatware_"), "Bloatware issue IDs should start with 'bloatware_'");
        assert!(!issue.title.is_empty());
        assert!(!issue.description.is_empty());
        assert!(matches!(issue.severity, IssueSeverity::Info | IssueSeverity::Warning));
        assert!(matches!(issue.impact_category, ImpactCategory::Performance | ImpactCategory::Privacy));

        // All bloatware issues should have a fix action
        assert!(issue.fix.is_some(), "Bloatware issues should have fix actions");

        if let Some(fix) = &issue.fix {
            assert!(!fix.action_id.is_empty());
            assert!(!fix.label.is_empty());
            // Bloatware fixes should be auto-fixable
            assert!(fix.is_auto_fix, "Bloatware fixes should be automatic");
        }
    }
}

#[test]
fn test_network_checker_metadata() {
    let checker = checkers::NetworkChecker::new();
    assert_eq!(checker.name(), "Network & Speed Checker");
    assert!(matches!(checker.category(), CheckCategory::Performance));
}

#[test]
fn test_network_checker_run() {
    let checker = checkers::NetworkChecker::new();
    let context = ScanContext {
        options: ScanOptions::default(),
    };

    let issues = checker.run(&context);

    // Network checker should complete without panic
    for issue in &issues {
        assert!(!issue.id.is_empty());
        assert!(issue.id.starts_with("network_"), "Network issue IDs should start with 'network_'");
        assert!(!issue.title.is_empty());
        assert!(!issue.description.is_empty());
        assert!(matches!(issue.impact_category, ImpactCategory::Performance));

        // DNS issues should have fix actions
        if issue.id.contains("dns") {
            assert!(issue.fix.is_some(), "DNS issues should have fix actions");

            if let Some(fix) = &issue.fix {
                // DNS fixes should be manual (not auto-fix)
                assert!(!fix.is_auto_fix, "DNS fixes should be manual");
                assert_eq!(fix.label, "Show DNS Fix Instructions");
            }
        }
    }
}

#[test]
fn test_smart_disk_checker_metadata() {
    let checker = checkers::SmartDiskChecker::new();
    assert_eq!(checker.name(), "S.M.A.R.T. Disk Health Checker");
    assert!(matches!(checker.category(), CheckCategory::Performance));
}

#[test]
fn test_smart_disk_checker_run() {
    let checker = checkers::SmartDiskChecker::new();
    let context = ScanContext {
        options: ScanOptions::default(),
    };

    let issues = checker.run(&context);

    // S.M.A.R.T. checker should complete without panic
    for issue in &issues {
        assert!(!issue.id.is_empty());
        assert!(issue.id.starts_with("smart_"), "S.M.A.R.T. issue IDs should start with 'smart_'");
        assert!(!issue.title.is_empty());
        assert!(!issue.description.is_empty());

        // S.M.A.R.T. issues are typically critical or warnings
        assert!(matches!(issue.severity, IssueSeverity::Critical | IssueSeverity::Warning));
        assert!(matches!(issue.impact_category, ImpactCategory::Performance | ImpactCategory::Both));
    }
}

#[test]
fn test_storage_checker_metadata() {
    let checker = checkers::StorageChecker::new();
    assert_eq!(checker.name(), "Storage Health Checker");
    assert!(matches!(checker.category(), CheckCategory::Performance));
}

#[test]
fn test_storage_checker_run() {
    let checker = checkers::StorageChecker::new();
    let context = ScanContext {
        options: ScanOptions::default(),
    };

    let issues = checker.run(&context);

    // Storage checker should complete without panic
    for issue in &issues {
        assert!(!issue.id.is_empty());
        assert!(issue.id.starts_with("storage_"), "Storage issue IDs should start with 'storage_'");
        assert!(!issue.title.is_empty());
        assert!(!issue.description.is_empty());
        assert!(matches!(issue.impact_category, ImpactCategory::Performance | ImpactCategory::Both));

        // Disk space issues should be critical
        if issue.id.contains("critical_space") {
            assert!(matches!(issue.severity, IssueSeverity::Critical));
        }

        // Fragmentation issues should have fix actions
        if issue.id.contains("fragmentation") {
            assert!(issue.fix.is_some(), "Fragmentation issues should have fix actions");
        }
    }
}

// ===== HELPER TYPES =====

#[test]
fn test_fix_result_constructors() {
    let success = FixResult::success("Test success");
    assert!(success.success);
    assert_eq!(success.message, "Test success");
    assert!(!success.rollback_available);
    assert!(success.restore_point_id.is_none());

    let failure = FixResult::failure("Test failure");
    assert!(!failure.success);
    assert_eq!(failure.message, "Test failure");
    assert!(!failure.rollback_available);
    assert!(failure.restore_point_id.is_none());
}

#[test]
fn test_fix_result_with_rollback() {
    let result = FixResult {
        success: true,
        message: "Fixed with rollback".to_string(),
        rollback_available: true,
        restore_point_id: Some("restore_123".to_string()),
    };

    assert!(result.success);
    assert!(result.rollback_available);
    assert_eq!(result.restore_point_id, Some("restore_123".to_string()));
}

#[test]
fn test_issue_severity_levels() {
    // Verify all severity levels exist and can be created
    let critical = IssueSeverity::Critical;
    let warning = IssueSeverity::Warning;
    let info = IssueSeverity::Info;

    assert_eq!(format!("{:?}", critical), "Critical");
    assert_eq!(format!("{:?}", warning), "Warning");
    assert_eq!(format!("{:?}", info), "Info");
}

#[test]
fn test_impact_categories() {
    // Verify all impact categories exist
    let security = ImpactCategory::Security;
    let performance = ImpactCategory::Performance;
    let privacy = ImpactCategory::Privacy;
    let both = ImpactCategory::Both;

    assert_eq!(format!("{:?}", security), "Security");
    assert_eq!(format!("{:?}", performance), "Performance");
    assert_eq!(format!("{:?}", privacy), "Privacy");
    assert_eq!(format!("{:?}", both), "Both");
}

#[test]
fn test_scan_options_default() {
    let options = ScanOptions::default();

    assert!(options.security, "Default should enable security checks");
    assert!(options.performance, "Default should enable performance checks");
    assert!(!options.quick, "Default should be full scan");
    assert!(!options.exclude_apps, "Default should include app checks");
    assert!(!options.exclude_startup, "Default should include startup checks");
}

#[test]
fn test_scan_context_creation() {
    let options = ScanOptions {
        security: true,
        performance: false,
        quick: true,
        exclude_apps: true,
        exclude_startup: false,
    };

    let context = ScanContext { options: options.clone() };

    assert_eq!(context.options.security, true);
    assert_eq!(context.options.performance, false);
    assert_eq!(context.options.quick, true);
    assert_eq!(context.options.exclude_apps, true);
    assert_eq!(context.options.exclude_startup, false);
}

#[test]
fn test_issue_structure() {
    let issue = Issue {
        id: "test_issue_1".to_string(),
        severity: IssueSeverity::Warning,
        title: "Test Issue".to_string(),
        description: "This is a test issue".to_string(),
        impact_category: ImpactCategory::Performance,
        fix: Some(FixAction {
            action_id: "fix_test".to_string(),
            label: "Fix Test Issue".to_string(),
            is_auto_fix: true,
            params: serde_json::json!({"param": "value"}),
        }),
    };

    assert_eq!(issue.id, "test_issue_1");
    assert!(matches!(issue.severity, IssueSeverity::Warning));
    assert_eq!(issue.title, "Test Issue");
    assert_eq!(issue.description, "This is a test issue");
    assert!(matches!(issue.impact_category, ImpactCategory::Performance));
    assert!(issue.fix.is_some());

    let fix = issue.fix.unwrap();
    assert_eq!(fix.action_id, "fix_test");
    assert_eq!(fix.label, "Fix Test Issue");
    assert!(fix.is_auto_fix);
}
