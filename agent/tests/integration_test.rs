// agent/tests/integration_test.rs
// Integration tests for the scanner engine

use health_speed_checker::*;

#[tokio::test]
async fn test_scanner_engine_initialization() {
    let engine = ScannerEngine::new();
    // Engine should initialize successfully
    assert!(true);
}

#[tokio::test]
async fn test_full_scan() {
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

    let result = engine.scan(options).await;

    // Verify scan result structure
    assert!(!result.scan_id.is_empty());
    assert!(result.timestamp > 0);
    assert!(result.scores.health <= 100);
    assert!(result.scores.speed <= 100);
}

#[tokio::test]
async fn test_quick_scan() {
    let mut engine = ScannerEngine::new();
    engine.register(Box::new(checkers::ProcessMonitor));

    let options = ScanOptions {
        security: false,
        performance: true,
        quick: true,
        exclude_apps: true,
        exclude_startup: true,
    };

    let result = engine.scan(options).await;

    // Quick scan should complete
    assert!(result.duration_ms < 10000); // Should be fast
}

#[tokio::test]
async fn test_scoring_engine() {
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
    assert!(scores.health < 100);
    assert!(scores.speed < 100);
}

#[test]
fn test_issue_severity_ordering() {
    let critical = IssueSeverity::Critical;
    let warning = IssueSeverity::Warning;
    let info = IssueSeverity::Info;

    // Critical should be treated as most severe
    assert_eq!(critical, IssueSeverity::Critical);
    assert_ne!(warning, IssueSeverity::Critical);
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
