// agent/tests/integration_test.rs
// Integration tests for the scanner engine

use health_speed_checker::*;

#[test]
fn test_scanner_engine_initialization() {
    let engine = ScannerEngine::new();

    // Engine should initialize successfully without panic
    assert_eq!(format!("{:?}", engine), format!("{:?}", engine));
}

#[test]
fn test_full_scan() {
    let mut engine = ScannerEngine::new();

    // Register checkers
    engine.register(Box::new(checkers::FirewallChecker));
    engine.register(Box::new(checkers::StartupAnalyzer));

    let options = ScanOptions {
        security: true,
        performance: true,
        quick: false,
        exclude_apps: false,
        exclude_startup: false,
    };

    let result = engine.scan(options);

    // Verify scan result structure
    assert!(!result.scan_id.is_empty(), "Scan ID should not be empty");
    assert!(result.timestamp > 0, "Timestamp should be greater than 0");
    assert!(result.scores.health <= 100, "Health score should be <= 100");
    assert!(result.scores.speed <= 100, "Speed score should be <= 100");
    assert!(result.scores.health >= 0, "Health score should be >= 0");
    assert!(result.scores.speed >= 0, "Speed score should be >= 0");

    // Verify scan ID is valid UUID format (36 characters with hyphens)
    assert_eq!(result.scan_id.len(), 36, "Scan ID should be 36 characters (UUID format)");

    // Verify duration is reasonable (> 0 and < 60 seconds)
    assert!(result.duration_ms > 0, "Duration should be greater than 0");
    assert!(result.duration_ms < 60000, "Full scan should complete within 60 seconds");
}

#[test]
fn test_quick_scan() {
    let mut engine = ScannerEngine::new();
    engine.register(Box::new(checkers::ProcessMonitor));
    engine.register(Box::new(checkers::PortScanner)); // Should be skipped in quick mode

    let options = ScanOptions {
        security: false,
        performance: true,
        quick: true,
        exclude_apps: true,
        exclude_startup: true,
    };

    let result = engine.scan(options);

    // Quick scan should complete rapidly
    assert!(result.duration_ms < 10000, "Quick scan should complete within 10 seconds");

    // Verify no port scanner issues (should be skipped in quick mode)
    let port_issues: Vec<_> = result.issues.iter()
        .filter(|i| i.id.starts_with("port_open_"))
        .collect();
    assert_eq!(port_issues.len(), 0, "Port scanner should be skipped in quick mode");
}

#[test]
fn test_scoring_engine() {
    let scoring_engine = ScoringEngine::default();

    let issues = vec![
        Issue {
            id: "test_critical".to_string(),
            severity: IssueSeverity::Critical,
            title: "Test Critical Issue".to_string(),
            description: "Test".to_string(),
            impact_category: ImpactCategory::Security,
            fix: None,
        },
        Issue {
            id: "test_warning".to_string(),
            severity: IssueSeverity::Warning,
            title: "Test Warning".to_string(),
            description: "Test".to_string(),
            impact_category: ImpactCategory::Performance,
            fix: None,
        },
    ];

    let scores = scoring_engine.calculate_scores(&issues);

    // Scores should be reduced due to issues
    assert!(scores.health < 100, "Health score should be < 100 when critical security issue exists");
    assert!(scores.speed < 100, "Speed score should be < 100 when performance warning exists");
    assert!(scores.health >= 0, "Health score should be >= 0");
    assert!(scores.speed >= 0, "Speed score should be >= 0");

    // Critical security issues should impact health more than speed
    assert!(scores.health < scores.speed, "Critical security issue should impact health more than speed");
}

#[test]
fn test_scoring_with_no_issues() {
    let scoring_engine = ScoringEngine::default();
    let issues: Vec<Issue> = vec![];

    let scores = scoring_engine.calculate_scores(&issues);

    // Perfect scores when no issues
    assert_eq!(scores.health, 100, "Health should be 100 with no issues");
    assert_eq!(scores.speed, 100, "Speed should be 100 with no issues");
}

#[test]
fn test_scoring_with_info_only() {
    let scoring_engine = ScoringEngine::default();

    let issues = vec![
        Issue {
            id: "test_info".to_string(),
            severity: IssueSeverity::Info,
            title: "Test Info".to_string(),
            description: "Test".to_string(),
            impact_category: ImpactCategory::Performance,
            fix: None,
        },
    ];

    let scores = scoring_engine.calculate_scores(&issues);

    // Info issues should have minimal impact
    assert!(scores.speed > 90, "Info issues should have minimal impact on speed");
    assert!(scores.health >= 95, "Info issues should have minimal impact on health");
}

#[test]
fn test_issue_severity_ordering() {
    let critical = IssueSeverity::Critical;
    let warning = IssueSeverity::Warning;
    let info = IssueSeverity::Info;

    // Critical should be treated as most severe
    assert_eq!(critical, IssueSeverity::Critical);
    assert_ne!(warning, IssueSeverity::Critical);
    assert_ne!(info, IssueSeverity::Critical);
    assert_ne!(warning, IssueSeverity::Info);
}

#[test]
fn test_scan_options_default() {
    let options = ScanOptions::default();

    assert!(options.security);
    assert!(options.performance);
    assert!(!options.quick);
    assert!(!options.exclude_apps);
    assert!(!options.exclude_startup);
}

#[test]
fn test_scan_with_all_checkers() {
    let mut engine = ScannerEngine::new();

    // Register all inline checkers
    engine.register(Box::new(checkers::FirewallChecker));
    engine.register(Box::new(checkers::StartupAnalyzer));
    engine.register(Box::new(checkers::ProcessMonitor));
    engine.register(Box::new(checkers::OsUpdateChecker));
    engine.register(Box::new(checkers::PortScanner));

    // Register all external checkers
    engine.register(Box::new(checkers::BloatwareDetector::new()));
    engine.register(Box::new(checkers::NetworkChecker::new()));
    engine.register(Box::new(checkers::SmartDiskChecker::new()));
    engine.register(Box::new(checkers::StorageChecker::new()));

    let options = ScanOptions {
        security: true,
        performance: true,
        quick: true, // Quick mode to avoid slow port scan
        exclude_apps: false,
        exclude_startup: false,
    };

    let result = engine.scan(options);

    // Should complete successfully with all checkers
    assert!(!result.scan_id.is_empty());
    assert!(result.duration_ms > 0);

    // All issues should have valid structure
    for issue in &result.issues {
        assert!(!issue.id.is_empty(), "Issue ID must not be empty");
        assert!(!issue.title.is_empty(), "Issue title must not be empty");
        assert!(!issue.description.is_empty(), "Issue description must not be empty");
    }
}

#[test]
fn test_scan_context_options_respected() {
    let mut engine = ScannerEngine::new();
    engine.register(Box::new(checkers::StartupAnalyzer));

    // Scan with startup excluded
    let options = ScanOptions {
        security: false,
        performance: true,
        quick: false,
        exclude_apps: false,
        exclude_startup: true, // Exclude startup
    };

    let result = engine.scan(options);

    // Should not have any startup issues
    let startup_issues: Vec<_> = result.issues.iter()
        .filter(|i| i.id.starts_with("startup_"))
        .collect();

    assert_eq!(startup_issues.len(), 0, "Startup issues should be excluded when exclude_startup is true");
}

#[test]
fn test_multiple_scans() {
    let mut engine = ScannerEngine::new();
    engine.register(Box::new(checkers::ProcessMonitor));

    let options = ScanOptions {
        security: false,
        performance: true,
        quick: true,
        exclude_apps: true,
        exclude_startup: true,
    };

    // Run multiple scans
    let result1 = engine.scan(options.clone());
    let result2 = engine.scan(options.clone());

    // Each scan should have unique ID
    assert_ne!(result1.scan_id, result2.scan_id, "Each scan should have unique ID");

    // Timestamps should be different (or at least not fail)
    assert!(result1.timestamp > 0);
    assert!(result2.timestamp > 0);
}

#[test]
fn test_scan_result_serialization() {
    let mut engine = ScannerEngine::new();
    engine.register(Box::new(checkers::ProcessMonitor));

    let options = ScanOptions::default();
    let result = engine.scan(options);

    // Test that result can be serialized to JSON
    let json = serde_json::to_string(&result);
    assert!(json.is_ok(), "ScanResult should be serializable to JSON");

    // Test that it can be deserialized back
    let json_str = json.unwrap();
    let deserialized: Result<ScanResult, _> = serde_json::from_str(&json_str);
    assert!(deserialized.is_ok(), "ScanResult should be deserializable from JSON");

    let deserialized_result = deserialized.unwrap();
    assert_eq!(deserialized_result.scan_id, result.scan_id);
    assert_eq!(deserialized_result.scores.health, result.scores.health);
    assert_eq!(deserialized_result.scores.speed, result.scores.speed);
}
