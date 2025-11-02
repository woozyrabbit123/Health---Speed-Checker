-- Health & Speed Checker - Database Schema
-- Stores scan history for trend analysis and delta calculations

CREATE TABLE IF NOT EXISTS scans (
    scan_id TEXT PRIMARY KEY,
    timestamp INTEGER NOT NULL,
    duration_ms INTEGER NOT NULL,
    health_score INTEGER NOT NULL,
    speed_score INTEGER NOT NULL,
    health_delta INTEGER,
    speed_delta INTEGER,
    scan_data TEXT NOT NULL,
    scan_type TEXT DEFAULT 'full'  -- 'quick' or 'full'
);

-- Index for efficient historical queries
CREATE INDEX IF NOT EXISTS idx_scans_timestamp ON scans(timestamp DESC);

-- Changelog table for "Provable Privacy" - tracks all file operations
CREATE TABLE IF NOT EXISTS changelog (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp INTEGER NOT NULL,
    action TEXT NOT NULL,  -- 'deleted', 'modified', 'created', 'scanned'
    file_path TEXT NOT NULL,
    reason TEXT NOT NULL,  -- Why this action was taken
    scan_id TEXT,
    file_size_bytes INTEGER,
    restored BOOLEAN DEFAULT 0,
    FOREIGN KEY (scan_id) REFERENCES scans(scan_id)
);

CREATE INDEX IF NOT EXISTS idx_changelog_timestamp ON changelog(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_changelog_scan_id ON changelog(scan_id);

-- Settings table for automation configuration
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Insert default settings
INSERT OR IGNORE INTO settings (key, value, updated_at)
VALUES
    ('auto_scan_enabled', 'false', strftime('%s', 'now')),
    ('auto_scan_frequency', 'weekly', strftime('%s', 'now')),
    ('auto_fix_enabled', 'false', strftime('%s', 'now')),
    ('telemetry_enabled', 'false', strftime('%s', 'now')),
    ('first_launch', 'true', strftime('%s', 'now'));
