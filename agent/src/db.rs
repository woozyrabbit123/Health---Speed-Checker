use rusqlite::{params, Connection, OpenFlags, OptionalExtension};
use serde::{Deserialize, Serialize};

const SCHEMA_SQL: &str = include_str!("../../db/schema.sql");

#[derive(Debug, Serialize, Deserialize)]
pub struct StoredScanSummary {
    pub scan_id: String,
    pub timestamp: u64,
    pub duration_ms: u64,
    pub health: u8,
    pub speed: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationSettings {
    pub automation_enabled: bool,
    pub run_schedule: String,
    pub auto_fix_enabled: bool,
}

impl Default for AutomationSettings {
    fn default() -> Self {
        Self {
            automation_enabled: false,
            run_schedule: "weekly".to_string(),
            auto_fix_enabled: false,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ChangelogEntry {
    pub timestamp: i64,
    pub action: String,
    pub path: String,
    pub size_bytes: i64,
    pub reason: String,
}

pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn open(path: &str) -> Result<Db, String> {
        let flags = OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_CREATE
            | OpenFlags::SQLITE_OPEN_NO_MUTEX; // connection used on a single thread

        let conn = Connection::open_with_flags(path, flags)
            .map_err(|e| format!("failed to open db: {}", e))?;

        // Apply schema
        conn.execute_batch(SCHEMA_SQL)
            .map_err(|e| format!("failed to apply schema: {}", e))?;

        Ok(Db { conn })
    }

    pub fn save_scan(&self, scan: &crate::ScanResult) -> Result<(), String> {
        let json = serde_json::to_string(scan)
            .map_err(|e| format!("failed to serialize scan: {}", e))?;

        self.conn
            .execute(
                "INSERT OR REPLACE INTO scans (
                    scan_id, timestamp, duration_ms, health_score, speed_score, health_delta, speed_delta, scan_data
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    scan.scan_id,
                    scan.timestamp as i64,
                    scan.duration_ms as i64,
                    scan.scores.health as i64,
                    scan.scores.speed as i64,
                    scan.scores.health_delta.map(|v| v as i64),
                    scan.scores.speed_delta.map(|v| v as i64),
                    json,
                ],
            )
            .map_err(|e| format!("failed to insert scan: {}", e))?;

        Ok(())
    }

    pub fn recent_scans(&self, limit: usize) -> Result<Vec<StoredScanSummary>, String> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT scan_id, timestamp, duration_ms, health_score, speed_score
                 FROM scans
                 ORDER BY timestamp DESC
                 LIMIT ?1",
            )
            .map_err(|e| format!("failed to prepare: {}", e))?;

        let rows = stmt
            .query_map([limit as i64], |row| {
                Ok(StoredScanSummary {
                    scan_id: row.get(0)?,
                    timestamp: row.get::<_, i64>(1)? as u64,
                    duration_ms: row.get::<_, i64>(2)? as u64,
                    health: row.get::<_, i64>(3)? as u8,
                    speed: row.get::<_, i64>(4)? as u8,
                })
            })
            .map_err(|e| format!("failed to query: {}", e))?;

        let mut out = Vec::new();
        for r in rows {
            out.push(r.map_err(|e| format!("row error: {}", e))?);
        }
        Ok(out)
    }

    pub fn get_automation_settings(&self) -> Result<AutomationSettings, String> {
        let settings = self
            .conn
            .query_row(
                "SELECT automation_enabled, run_schedule, auto_fix_enabled FROM settings WHERE id = 1",
                [],
                |row| {
                    let automation_enabled: i64 = row.get(0)?;
                    let run_schedule: String = row.get(1)?;
                    let auto_fix_enabled: i64 = row.get(2)?;
                    Ok(AutomationSettings {
                        automation_enabled: automation_enabled != 0,
                        run_schedule,
                        auto_fix_enabled: auto_fix_enabled != 0,
                    })
                },
            )
            .optional()
            .map_err(|e| format!("failed to load automation settings: {}", e))?;

        Ok(settings.unwrap_or_default())
    }

    pub fn set_automation_settings(
        &self,
        settings: &AutomationSettings,
    ) -> Result<(), String> {
        let run_schedule = settings.run_schedule.to_lowercase();

        match run_schedule.as_str() {
            "daily" | "weekly" | "monthly" => {}
            other => {
                return Err(format!("invalid run schedule: {}", other));
            }
        }

        self.conn
            .execute(
                "INSERT INTO settings (id, automation_enabled, run_schedule, auto_fix_enabled, updated_at)
                 VALUES (1, ?1, ?2, ?3, CURRENT_TIMESTAMP)
                 ON CONFLICT(id) DO UPDATE SET
                    automation_enabled = excluded.automation_enabled,
                    run_schedule = excluded.run_schedule,
                    auto_fix_enabled = excluded.auto_fix_enabled,
                    updated_at = CURRENT_TIMESTAMP",
                params![
                    if settings.automation_enabled { 1 } else { 0 },
                    run_schedule,
                    if settings.auto_fix_enabled { 1 } else { 0 },
                ],
            )
            .map_err(|e| format!("failed to persist automation settings: {}", e))?;

        Ok(())
    }

    pub fn last_scan_timestamp(&self) -> Result<Option<u64>, String> {
        let ts = self
            .conn
            .query_row(
                "SELECT MAX(timestamp) FROM scans",
                [],
                |row| {
                    let value: Option<i64> = row.get(0)?;
                    Ok(value)
                },
            )
            .optional()
            .map_err(|e| format!("failed to query last scan timestamp: {}", e))?;

        Ok(ts.flatten().map(|v| v as u64))
    }

    pub fn get_changelog_entries(&self) -> Result<Vec<ChangelogEntry>, String> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT timestamp, action, file_path, file_size_bytes, reason
                 FROM changelog
                 ORDER BY timestamp DESC
                 LIMIT 50",
            )
            .map_err(|e| format!("failed to prepare changelog query: {}", e))?;

        let rows = stmt
            .query_map([], |row| {
                let timestamp: i64 = row.get(0)?;
                let action: String = row.get(1)?;
                let path: String = row.get(2)?;
                let size_bytes: Option<i64> = row.get(3)?;
                let reason: String = row.get(4)?;

                Ok(ChangelogEntry {
                    timestamp,
                    action: action.to_uppercase(),
                    path,
                    size_bytes: size_bytes.unwrap_or(0),
                    reason,
                })
            })
            .map_err(|e| format!("failed to read changelog rows: {}", e))?;

        let mut entries = Vec::new();
        for entry in rows {
            entries.push(entry.map_err(|e| format!("changelog row error: {}", e))?);
        }

        Ok(entries)
    }
}
