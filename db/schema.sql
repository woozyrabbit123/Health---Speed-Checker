-- db/schema.sql
-- Database schema for Health & Speed Checker

-- Enable foreign keys
PRAGMA foreign_keys = ON;

-- ============================================================================
-- SCAN HISTORY TABLE
-- ============================================================================

CREATE TABLE IF NOT EXISTS scans (
    scan_id TEXT PRIMARY KEY,
    timestamp INTEGER NOT NULL,
    duration_ms INTEGER NOT NULL,
    health_score INTEGER NOT NULL CHECK (health_score >= 0 AND health_score <= 100),
    speed_score INTEGER NOT NULL CHECK (speed_score >= 0 AND speed_score <= 100),
    health_delta INTEGER,
    speed_delta INTEGER,
    scan_data TEXT NOT NULL, -- JSON serialized full scan result
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Index for faster queries
CREATE INDEX IF NOT EXISTS idx_scans_timestamp ON scans(timestamp DESC);

-- ============================================================================
-- CVE DATABASE CACHE
-- ============================================================================

CREATE TABLE IF NOT EXISTS cve_data (
    cve_id TEXT PRIMARY KEY,
    severity TEXT NOT NULL CHECK (severity IN ('Critical', 'High', 'Medium', 'Low', 'Info')),
    description TEXT NOT NULL,
    affected_software TEXT NOT NULL, -- JSON array of software patterns
    published_date INTEGER,
    last_modified INTEGER,
    cvss_score REAL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Index for faster lookups
CREATE INDEX IF NOT EXISTS idx_cve_severity ON cve_data(severity);
CREATE INDEX IF NOT EXISTS idx_cve_published ON cve_data(published_date DESC);

-- ============================================================================
-- USER CONFIGURATION
-- ============================================================================

CREATE TABLE IF NOT EXISTS user_config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    description TEXT,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Default configuration values
INSERT OR IGNORE INTO user_config (key, value, description) VALUES
    ('telemetry_enabled', 'false', 'Send anonymous usage statistics'),
    ('auto_scan_enabled', 'false', 'Automatically scan on schedule'),
    ('scan_schedule', 'daily', 'Scan frequency: daily, weekly, monthly'),
    ('scan_time', '02:00', 'Time to run scheduled scans'),
    ('quick_scan_default', 'false', 'Use quick scan by default'),
    ('max_scan_history', '20', 'Maximum number of scans to keep'),
    ('ignore_localhost_ports', 'true', 'Ignore development ports on localhost'),
    ('create_restore_points', 'true', 'Create system restore points before fixes'),
    ('theme', 'auto', 'UI theme: light, dark, auto'),
    ('notification_enabled', 'true', 'Show desktop notifications'),
    ('notification_severity', 'critical', 'Minimum severity for notifications'),
    ('update_check_enabled', 'true', 'Check for app updates'),
    ('update_check_interval', '7', 'Days between update checks'),
    ('language', 'en', 'User interface language'),
    ('log_level', 'info', 'Logging level: debug, info, warn, error');

-- ============================================================================
-- FIX HISTORY (AUDIT TRAIL)
-- ============================================================================

CREATE TABLE IF NOT EXISTS fix_history (
    fix_id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp INTEGER NOT NULL,
    scan_id TEXT,
    action_id TEXT NOT NULL,
    issue_id TEXT NOT NULL,
    parameters TEXT, -- JSON parameters
    success BOOLEAN NOT NULL,
    error_message TEXT,
    restore_point_id TEXT,
    rollback_available BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (scan_id) REFERENCES scans(scan_id) ON DELETE SET NULL
);

-- Index for audit queries
CREATE INDEX IF NOT EXISTS idx_fix_timestamp ON fix_history(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_fix_scan ON fix_history(scan_id);

-- ============================================================================
-- IGNORED ISSUES
-- ============================================================================

CREATE TABLE IF NOT EXISTS ignored_issues (
    issue_id TEXT PRIMARY KEY,
    ignore_until INTEGER, -- Unix timestamp, NULL for permanent
    reason TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- SCHEDULED SCANS
-- ============================================================================

CREATE TABLE IF NOT EXISTS scheduled_scans (
    schedule_id INTEGER PRIMARY KEY AUTOINCREMENT,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    frequency TEXT NOT NULL CHECK (frequency IN ('once', 'daily', 'weekly', 'monthly')),
    next_run INTEGER NOT NULL, -- Unix timestamp
    last_run INTEGER,
    scan_options TEXT NOT NULL, -- JSON ScanOptions
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- BASELINE SCANS (FOR COMPARISON)
-- ============================================================================

CREATE TABLE IF NOT EXISTS baseline_scans (
    baseline_id INTEGER PRIMARY KEY AUTOINCREMENT,
    scan_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    is_active BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (scan_id) REFERENCES scans(scan_id) ON DELETE CASCADE
);

-- Only one baseline can be active
CREATE UNIQUE INDEX IF NOT EXISTS idx_active_baseline
    ON baseline_scans(is_active)
    WHERE is_active = 1;

-- ============================================================================
-- SYSTEM INFO CACHE
-- ============================================================================

CREATE TABLE IF NOT EXISTS system_info (
    info_key TEXT PRIMARY KEY,
    info_value TEXT NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Cache system information that doesn't change often
INSERT OR IGNORE INTO system_info (info_key, info_value) VALUES
    ('os_name', ''),
    ('os_version', ''),
    ('cpu_model', ''),
    ('total_memory_gb', ''),
    ('total_disk_gb', '');

-- ============================================================================
-- WHITELIST (PORTS, PROCESSES, ETC.)
-- ============================================================================

CREATE TABLE IF NOT EXISTS whitelist (
    whitelist_id INTEGER PRIMARY KEY AUTOINCREMENT,
    type TEXT NOT NULL CHECK (type IN ('port', 'process', 'startup', 'file')),
    value TEXT NOT NULL,
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Default whitelist entries
INSERT OR IGNORE INTO whitelist (type, value, description) VALUES
    ('port', '3000', 'Node.js development server'),
    ('port', '5000', 'Flask development server'),
    ('port', '8000', 'Django development server'),
    ('port', '8080', 'Alternative HTTP server'),
    ('port', '5432', 'PostgreSQL database'),
    ('port', '3306', 'MySQL database'),
    ('port', '6379', 'Redis cache'),
    ('port', '27017', 'MongoDB database'),
    ('process', 'docker', 'Docker containers'),
    ('process', 'node', 'Node.js runtime'),
    ('process', 'python', 'Python runtime'),
    ('process', 'code', 'Visual Studio Code');

-- ============================================================================
-- STATISTICS TABLE
-- ============================================================================

CREATE TABLE IF NOT EXISTS statistics (
    stat_key TEXT PRIMARY KEY,
    stat_value INTEGER NOT NULL DEFAULT 0,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Initialize statistics
INSERT OR IGNORE INTO statistics (stat_key, stat_value) VALUES
    ('total_scans', 0),
    ('total_fixes', 0),
    ('total_issues_found', 0),
    ('total_issues_fixed', 0),
    ('average_health_score', 0),
    ('average_speed_score', 0);

-- ============================================================================
-- TRIGGERS
-- ============================================================================

-- Update statistics after scan
CREATE TRIGGER IF NOT EXISTS update_scan_stats
AFTER INSERT ON scans
BEGIN
    UPDATE statistics SET
        stat_value = stat_value + 1,
        updated_at = CURRENT_TIMESTAMP
    WHERE stat_key = 'total_scans';

    UPDATE statistics SET
        stat_value = (SELECT AVG(health_score) FROM scans),
        updated_at = CURRENT_TIMESTAMP
    WHERE stat_key = 'average_health_score';

    UPDATE statistics SET
        stat_value = (SELECT AVG(speed_score) FROM scans),
        updated_at = CURRENT_TIMESTAMP
    WHERE stat_key = 'average_speed_score';
END;

-- Update statistics after fix
CREATE TRIGGER IF NOT EXISTS update_fix_stats
AFTER INSERT ON fix_history
BEGIN
    UPDATE statistics SET
        stat_value = stat_value + 1,
        updated_at = CURRENT_TIMESTAMP
    WHERE stat_key = 'total_fixes';

    UPDATE statistics SET
        stat_value = stat_value + 1,
        updated_at = CURRENT_TIMESTAMP
    WHERE stat_key = 'total_issues_fixed' AND NEW.success = 1;
END;

-- Auto-delete old scans beyond limit
CREATE TRIGGER IF NOT EXISTS cleanup_old_scans
AFTER INSERT ON scans
BEGIN
    DELETE FROM scans
    WHERE scan_id IN (
        SELECT scan_id FROM scans
        ORDER BY timestamp DESC
        LIMIT -1 OFFSET (
            SELECT value FROM user_config WHERE key = 'max_scan_history'
        )
    );
END;

-- ============================================================================
-- VIEWS
-- ============================================================================

-- Recent scans with issue counts
CREATE VIEW IF NOT EXISTS recent_scans_view AS
SELECT
    s.scan_id,
    s.timestamp,
    s.health_score,
    s.speed_score,
    s.health_delta,
    s.speed_delta,
    s.duration_ms,
    json_extract(s.scan_data, '$.issues') as issues
FROM scans s
ORDER BY s.timestamp DESC
LIMIT 10;

-- Fix success rate
CREATE VIEW IF NOT EXISTS fix_success_rate AS
SELECT
    action_id,
    COUNT(*) as total_attempts,
    SUM(CASE WHEN success = 1 THEN 1 ELSE 0 END) as successful,
    ROUND(100.0 * SUM(CASE WHEN success = 1 THEN 1 ELSE 0 END) / COUNT(*), 2) as success_rate
FROM fix_history
GROUP BY action_id;
