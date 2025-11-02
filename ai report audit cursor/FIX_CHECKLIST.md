# Fix Checklist - Health & Speed Checker

Use this checklist to systematically fix all identified issues.

---

## Phase 1: Make It Compile (CRITICAL)

### 1.1 Resolve Checker Module Duplication
- [ ] **Issue:** `agent/src/checkers/mod.rs` declares modules that don't exist
- [ ] **Action:** Choose ONE approach:
  - [ ] Option A: Delete lines 4-8 from `mod.rs` (keep inline implementations)
  - [ ] Option B: Delete inline implementations (keep modules), create missing files
- [ ] **Recommendation:** Option A - faster, less refactoring
- [ ] **File:** `agent/src/checkers/mod.rs`
- [ ] **Time:** 15 minutes
- [ ] **Priority:** P0 - Blocks all builds

### 1.2 Fix Async/Sync Mismatch
- [ ] **Issue:** `scan()` is sync but called with `.await`
- [ ] **Files:** `agent/src/lib.rs:257`, `agent/src/main.rs:248`
- [ ] **Action:** Choose ONE:
  - [ ] Make `scan()` async: `pub async fn scan(...)` 
  - [ ] Remove `.await` calls: `let result = engine.scan(options);`
- [ ] **Recommendation:** Make async for better Tauri integration
- [ ] **Also Fix:** `fix_issue()` method same issue
- [ ] **Time:** 30 minutes

### 1.3 Add Missing Dependencies
- [ ] **Issue:** `hostname` crate referenced but not in Cargo.toml
- [ ] **File:** `agent/Cargo.toml`
- [ ] **Action:** Add `hostname = "0.3"` to dependencies
- [ ] **Also Add:** `async-trait = "0.1"` (if going async route)
- [ ] **Time:** 5 minutes

### 1.4 Remove Duplicate Registration
- [ ] **Issue:** Checkers registered twice in Tauri code
- [ ] **File:** `ui/src-tauri/src/main.rs:30-41`
- [ ] **Action:** Keep only one set of registrations (inline or new checkers)
- [ ] **Time:** 10 minutes

### 1.5 Verify Build
- [ ] **Action:** Run `cd agent && cargo check`
- [ ] **Expected:** No compilation errors
- [ ] **If Errors:** Document any new errors not covered above
- [ ] **Time:** 5 minutes

**Phase 1 Total Time:** ~1 hour

---

## Phase 2: Database Integration (HIGH PRIORITY)

### 2.1 Create Database Module
- [ ] **File:** Create `agent/src/database.rs`
- [ ] **Action:** Initialize SQLite connection
- [ ] **Code Template:**
  ```rust
  use rusqlite::{Connection, Result as SqlResult};
  
  pub struct Database {
      conn: Connection,
  }
  
  impl Database {
      pub fn new(path: &Path) -> SqlResult<Self> {
          let conn = Connection::open(path)?;
          Self::init_schema(&conn)?;
          Ok(Self { conn })
      }
      
      fn init_schema(conn: &Connection) -> SqlResult<()> {
          let schema = include_str!("../../db/schema.sql");
          conn.execute_batch(schema)?;
          Ok(())
      }
  }
  ```
- [ ] **Time:** 2 hours

### 2.2 Implement Scan Storage
- [ ] **File:** `agent/src/database.rs`
- [ ] **Action:** Add method to store `ScanResult`
  ```rust
  pub fn save_scan(&self, result: &ScanResult) -> SqlResult<()> {
      let json = serde_json::to_string(result).unwrap();
      self.conn.execute(
          "INSERT INTO scans (scan_id, timestamp, duration_ms, health_score, speed_score, scan_data) 
           VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
          params![result.scan_id, result.timestamp, result.duration_ms, 
                  result.scores.health, result.scores.speed, json]
      )?;
      Ok(())
  }
  ```
- [ ] **Time:** 1 hour

### 2.3 Wire Up Storage in Scanner
- [ ] **File:** `agent/src/lib.rs`
- [ ] **Action:** Add database field to `ScannerEngine`, save results after scan
- [ ] **Time:** 1 hour

### 2.4 Implement Scan History Retrieval
- [ ] **File:** `agent/src/database.rs`
- [ ] **Action:** Add query methods for history
- [ ] **Update:** `ui/src-tauri/src/main.rs` `get_scan_history()` function
- [ ] **Time:** 2 hours

### 2.5 Test Database Integration
- [ ] **Action:** Create integration test
- [ ] **Verify:** Results persist, queries work
- [ ] **Time:** 1 hour

**Phase 2 Total Time:** ~7 hours

---

## Phase 3: Fix Broken Implementations (HIGH PRIORITY)

### 3.1 Fix Network Checker HTTP
- [ ] **Issue:** Manual HTTP implementation broken
- [ ] **File:** `agent/src/checkers/network.rs:48-85`
- [ ] **Action:** Replace with proper HTTP library
  - [ ] Option A: Add `reqwest` dependency (async)
  - [ ] Option B: Use `ureq` (sync, lighter)
- [ ] **Recommendation:** `ureq` if keeping sync, `reqwest` if async
- [ ] **Code Template:**
  ```rust
  use ureq::get;
  
  fn test_download_speed(&self) -> Option<f64> {
      let start = Instant::now();
      if let Ok(response) = get("http://speedtest.ftp.otenet.gr/files/test1Mb.db").call() {
          let mut bytes = Vec::new();
          let _ = response.into_reader().read_to_end(&mut bytes);
          let duration = start.elapsed().as_secs_f64();
          let mbps = (bytes.len() as f64 * 8.0) / (duration * 1_000_000.0);
          Some(mbps)
      } else {
          None
      }
  }
  ```
- [ ] **Time:** 2 hours

### 3.2 Fix Process Monitor CPU
- [ ] **Issue:** CPU usage always 0.0
- [ ] **File:** `agent/src/checkers/process.rs:307-349`
- [ ] **Action:** Use `sysinfo` crate properly
  ```rust
  use sysinfo::{System, SystemExt, PidExt, ProcessExt};
  
  fn get_top_cpu_processes(limit: usize) -> Result<Vec<ProcessInfo>, String> {
      let mut sys = System::new_all();
      sys.refresh_all();
      
      let mut processes: Vec<_> = sys.processes()
          .iter()
          .map(|(pid, proc)| ProcessInfo {
              pid: pid.as_u32(),
              name: proc.name().to_string(),
              cpu_percent: proc.cpu_usage() as f32,
              memory_mb: (proc.memory() as f32) / 1024.0,
          })
          .collect();
      
      processes.sort_by(|a, b| b.cpu_percent.partial_cmp(&a.cpu_percent).unwrap());
      processes.truncate(limit);
      Ok(processes)
  }
  ```
- [ ] **Time:** 1 hour

### 3.3 Implement Real Event Streaming
- [ ] **Issue:** Progress is simulated
- [ ] **Files:** `agent/src/lib.rs`, `ui/src-tauri/src/main.rs`
- [ ] **Action:** Add progress sender to ScanContext
  ```rust
  pub struct ScanContext {
      pub options: ScanOptions,
      pub progress: Option<mpsc::Sender<ProgressEvent>>,
  }
  
  // In scan loop:
  if let Some(ref sender) = context.progress {
      let _ = sender.send(ProgressEvent::TaskChanged {
          message: format!("Running {}", checker.name())
      }).await;
  }
  ```
- [ ] **Wire up in Tauri:** Use `tauri::Window::emit` for events
- [ ] **Time:** 3 hours

### 3.4 Fix Time Math Error
- [ ] **Issue:** Network checker has broken time calculation
- [ ] **File:** `agent/src/checkers/network.rs:27-34`
- [ ] **Action:** Fix to use `Instant` correctly
  ```rust
  let start = Instant::now();
  if TcpStream::connect_timeout(..., Duration::from_secs(2)).is_ok() {
      let latency = start.elapsed().as_millis();
  }
  ```
- [ ] **Time:** 15 minutes

**Phase 3 Total Time:** ~6.5 hours

---

## Phase 4: Error Handling Improvements (MEDIUM PRIORITY)

### 4.1 Define Error Types
- [ ] **File:** Create `agent/src/error.rs`
- [ ] **Action:** Define proper error types
  ```rust
  use thiserror::Error;
  
  #[derive(Error, Debug)]
  pub enum HealthCheckerError {
      #[error("Checker failed: {0}")]
      CheckerFailed(String),
      #[error("Database error: {0}")]
      Database(#[from] rusqlite::Error),
      #[error("Serialization error: {0}")]
      Serialization(#[from] serde_json::Error),
      #[error("IO error: {0}")]
      Io(#[from] std::io::Error),
      #[error("Invalid input: {0}")]
      InvalidInput(String),
  }
  
  pub type Result<T> = std::result::Result<T, HealthCheckerError>;
  ```
- [ ] **Time:** 1 hour

### 4.2 Convert String Errors
- [ ] **Files:** All checker files
- [ ] **Action:** Replace `Result<T, String>` with proper error types
- [ ] **Time:** 3 hours

### 4.3 Add Context to Errors
- [ ] **Action:** Use `.context()` for error chains
  ```rust
  Command::new("netsh").output()
      .context("Failed to check firewall")?;
  ```
- [ ] **Time:** 2 hours

**Phase 4 Total Time:** ~6 hours

---

## Phase 5: Testing (HIGH PRIORITY)

### 5.1 Unit Tests for Missing Checkers
- [ ] **Files:** `agent/src/checkers/firewall.rs`, `startup.rs`, `process.rs`, `os_update.rs`, `ports.rs`
- [ ] **Action:** Add tests similar to bloatware/storage
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;
      
      #[test]
      fn test_checker_name() {
          let checker = FirewallChecker;
          assert_eq!(checker.name(), "firewall_checker");
      }
      
      #[test]
      fn test_checker_category() {
          let checker = FirewallChecker;
          assert_eq!(checker.category(), CheckCategory::Security);
      }
      
      #[test]
      fn test_run_returns_issues() {
          let checker = FirewallChecker;
          let context = ScanContext { options: Default::default() };
          let issues = checker.run(&context);
          // Verify issues structure even if empty
          assert!(issues.iter().all(|i| !i.id.is_empty()));
      }
  }
  ```
- [ ] **Time:** 4 hours

### 5.2 Integration Tests
- [ ] **File:** `agent/tests/integration_test.rs`
- [ ] **Action:** Implement scan flow test
  ```rust
  #[tokio::test]
  async fn test_full_scan() {
      let mut engine = ScannerEngine::new();
      engine.register(Box::new(checkers::FirewallChecker));
      
      let options = ScanOptions::default();
      let result = engine.scan(options).await;
      
      assert!(result.health_score <= 100);
      assert!(result.speed_score <= 100);
      assert!(!result.scan_id.is_empty());
  }
  ```
- [ ] **Time:** 3 hours

### 5.3 Test Coverage
- [ ] **Action:** Install `cargo-tarpaulin`
- [ ] **Action:** Run `cargo tarpaulin --out Html`
- [ ] **Goal:** Achieve 80%+ coverage
- [ ] **Time:** On-going

**Phase 5 Total Time:** ~7 hours

---

## Phase 6: Code Quality (MEDIUM PRIORITY)

### 6.1 Add Documentation Comments
- [ ] **Files:** All public APIs
- [ ] **Action:** Add `///` doc comments
  ```rust
  /// ScannerEngine orchestrates security and performance checks
  /// 
  /// # Example
  /// ```
  /// let mut engine = ScannerEngine::new();
  /// let result = engine.scan(ScanOptions::default()).await?;
  /// ```
  pub struct ScannerEngine { }
  ```
- [ ] **Target:** All public types, functions, methods
- [ ] **Time:** 4 hours

### 6.2 Run Clippy
- [ ] **Action:** `cargo clippy -- -D warnings`
- [ ] **Action:** Fix all warnings
- [ ] **Time:** 2 hours

### 6.3 Format Code
- [ ] **Action:** `cargo fmt` for Rust
- [ ] **Action:** Add Prettier for TypeScript
- [ ] **Action:** Run on all files
- [ ] **Time:** 30 minutes

### 6.4 Add CI Checks
- [ ] **File:** `.github/workflows/ci.yml` (create)
- [ ] **Action:** Run tests, clippy, fmt on every PR
- [ ] **Time:** 2 hours

**Phase 6 Total Time:** ~8.5 hours

---

## Phase 7: Upgrade & Modernize (LOW PRIORITY)

### 7.1 Migrate to Tauri v2
- [ ] **Issue:** v1 is end-of-life
- [ ] **Action:** Follow migration guide
- [ ] **Breaking Changes:** API updates needed
- [ ] **Time:** 8 hours

### 7.2 Update Dependencies
- [ ] **File:** `agent/Cargo.toml`
- [ ] **Action:** Update versions
  - `tracing` 0.1 → latest
  - `systemstat` 0.2 → latest
  - `chrono` 0.4 → latest
- [ ] **Action:** Remove `tokio = ["full"]`, specify needed features
- [ ] **Time:** 2 hours

### 7.3 Update NPM Dependencies
- [ ] **File:** `ui/package.json`
- [ ] **Action:** `npm outdated`, update carefully
- [ ] **Time:** 1 hour

**Phase 7 Total Time:** ~11 hours

---

## Phase 8: Missing Features (MEDIUM PRIORITY)

### 8.1 Implement Restore Points
- [ ] **File:** Create `agent/src/restore.rs`
- [ ] **Action:** Windows VSS API integration
- [ ] **Code:** Use `windows` crate VSS bindings
- [ ] **Time:** 8 hours

### 8.2 Add PDF Export
- [ ] **File:** `agent/src/export.rs`
- [ ] **Action:** Add PDF generation (use `printpdf` or `pdf-lib`)
- [ ] **Time:** 4 hours

### 8.3 Persist Ignored Issues
- [ ] **Action:** Wire up UI → database for ignore functionality
- [ ] **Time:** 2 hours

### 8.4 Implement Scheduled Scans
- [ ] **Action:** Background daemon with scheduler
- [ ] **Time:** 12 hours

**Phase 8 Total Time:** ~26 hours

---

## Summary

| Phase | Priority | Time Estimate | Dependencies |
|-------|----------|---------------|--------------|
| Phase 1: Compile | P0 | 1 hour | None |
| Phase 2: Database | P0 | 7 hours | Phase 1 |
| Phase 3: Fix Bugs | P0 | 6.5 hours | Phase 1 |
| Phase 4: Errors | P1 | 6 hours | Phase 1 |
| Phase 5: Tests | P0 | 7 hours | Phase 1 |
| Phase 6: Quality | P1 | 8.5 hours | Phase 5 |
| Phase 7: Modernize | P2 | 11 hours | Phase 6 |
| Phase 8: Features | P1 | 26 hours | Phase 2-3 |

**Total Time to Full Functionality:** ~72 hours (9 working days)

**Critical Path:** Phase 1 → Phase 2 → Phase 3 → Phase 5

**Minimum Viable:** Phase 1 + Phase 2 (8 hours) gets you a working MVP

---

## Quick Reference

**Compilation fixes only:**
```bash
# 1. Edit checkers/mod.rs - remove module declarations OR create files
# 2. Make scan() async OR remove .await
# 3. Add hostname dependency
# 4. cargo check
```

**Get MVP working:**
```bash
# After compilation fixes:
# 1. Implement database module (2 hours)
# 2. Wire up storage (1 hour)
# 3. cargo build --release
# 4. Test scan
```

---

**Use this checklist systematically. Check off items as completed.**

