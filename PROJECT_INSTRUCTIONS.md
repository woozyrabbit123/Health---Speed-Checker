# 🧭 Project Instructions — Health & Speed Checker

## 1 · Mission & Mindset

**Goal**: Deliver a local-first, privacy-respecting PC health tool that gives users answers, not telemetry.

**First milestone**: A working command-line prototype (the "10-hour proof") that prints real scores and issues.

**Principle**: Ship small, ship safe, iterate weekly.

---

## 2 · Environment Setup

### Required Toolchain

| Component | Recommendation |
|-----------|----------------|
| Rust | latest stable (`rustup update`) |
| Node.js | ≥ 18 LTS |
| Tauri CLI | `cargo install tauri-cli` |
| SQLite | built-in or `sqlite3` CLI |
| Git | version control |
| Code signing | Windows SDK `signtool` (later) |

### Repository Layout

```
health-speed-checker/
├── agent/           # Rust core
├── ui/              # Tauri frontend
├── cli/             # Command-line interface
├── db/              # SQLite schema & migrations
├── docs/            # Design & API specs
└── tests/
```

---

## 3 · Implementation Phases

### Phase 1 – Foundation (Week 1–2)

**Objective**: Bootstrapped, compiling skeleton.

- [ ] Scaffold Tauri project (`tauri init`)
- [ ] Create `Checker` trait + registration system
- [ ] Build `EventBus` for progress streaming
- [ ] Stub the frozen API (`scan_start`, `get_scan_result`, `fix_action`)
- [ ] Create `ScanResult`, `Issue`, and `FixAction` structs
- [ ] Verify end-to-end JSON round-trip between UI ↔ Agent

**✅ Exit Criteria**: Calling `scan_start` returns a `scan_id`; dummy progress events stream successfully.

### Phase 2 – 10-Hour Proof (Week 2–3)

**Objective**: Working scanner in terminal.

- [ ] Implement three checkers:
  - FirewallChecker
  - StartupAnalyzer
  - ProcessMonitor
- [ ] Add simple scoring function + CLI output:

```bash
cargo run -- scan --quick
# → Health: 72/100  Speed: 85/100  Top issues listed
```

**✅ Exit Criteria**: Real data printed, no crash on Windows 10/11.

### Phase 3 – MVP Checkers & Scoring (Week 3–4)

- [ ] Add `OsUpdateChecker` and `PortScanner`
- [ ] Implement real scoring weights + deltas
- [ ] Store results in SQLite (`scans` table)
- [ ] Add restore-point stub (Windows API call only)

**✅ Exit Criteria**: Five checkers run sequentially; DB stores results; scores appear in history.

### Phase 4 – UI & CLI Integration (Week 5–6)

- [ ] Tauri dashboard with health/speed scores
- [ ] Real-time progress bar using event stream
- [ ] "Fix Now" buttons calling `fix_action`
- [ ] CLI parity with UI

**✅ Exit Criteria**: Identical results in UI and CLI; at least one fix executes correctly.

### Phase 5 – Safety & Polish (Week 7–8)

- [ ] Implement auto-restore points
- [ ] Add "Ignore Issue" + whitelist logic
- [ ] Quick-scan mode
- [ ] PDF/JSON export
- [ ] Sign Windows binary

**✅ Exit Criteria**: Installer + portable ZIP build tested on 3 machines.

---

## 4 · Coding Guidelines

- **Language**: Rust 2021 edition; `async`/`await` only
- **Error Handling**: `anyhow` + graceful fallbacks (`Result<Option<T>>`)
- **Logging**: `tracing` crate; level configurable
- **Concurrency**: `tokio` runtime; never block main thread
- **Security**: Minimal privileges, no unsafe I/O
- **UI IPC**: Serialize → `serde_json`
- **Testing**: `cargo test` for each checker; integration tests for CLI commands

---

## 5 · Data and Storage

- **DB**: Single SQLite file under `~/.healthchecker/`
- **Tables**: `scans`, `user_config`, `fix_history`
- **Retention**: Keep last 20 scans; purge older automatically
- **Telemetry**: Disabled by default; opt-in JSON upload (hashed only)

---

## 6 · Quality & Release Checklist

Before release:

- [ ] Code signed (`signtool sign …`)
- [ ] Restore-point verified (Win 10/11)
- [ ] Common dev ports whitelisted (3000, 8080, 5432)
- [ ] Privacy policy page built into "About"
- [ ] Crash handler logs locally only
- [ ] Beta tests: 3 users complete scan and understand output

---

## 7 · Launch Plan

- **Channels**: GitHub Releases · Product Hunt · Reddit r/pcmasterrace / r/privacy
- **Tagline**: "No cloud. No account. Just answers."
- **Assets**: Screenshots, GIF of scan progress, "local-first" badge

**First 48 hours:**
- Monitor crash telemetry
- Update whitelist
- Patch within 24h if reproducible bug found

---

## 8 · Post-Launch Roadmap

| Month | Focus | Features |
|-------|-------|----------|
| 1 | Stabilization | Quick-scan, bugfixes |
| 2 | Pro tier | Browser cleanup, scheduler, gaming mode |
| 3 | Business tier | Fleet dashboard, compliance export |
| 6 | Ecosystem | Plugin store, privacy & firmware checkers |

---

## 9 · Project Governance

- **Branch strategy**: `main` → release; `dev` → integration; feature branches per module
- **Versioning**: Semantic `vMAJOR.MINOR.PATCH`
- **CI/CD**: GitHub Actions (build · lint · test · package Windows artifact)
- **Docs**: Keep `docs/` updated with each module added

---

## 10 · Daily Workflow

1. Pull latest `main`
2. Run `cargo clippy` and tests
3. Implement 1 checker or 1 UI screen max per day
4. Commit with semantic message
5. Push · open PR · review · merge

**Simple rule**: "One improvement, one merge."

---

## 11 · Milestone Definition of Done

| Milestone | DoD |
|-----------|-----|
| Prototype | CLI prints Health/Speed scores |
| MVP | GUI + CLI parity + restore point |
| Beta | Signed build + public testers |
| Launch | ProductHunt release + docs complete |

---

## Next Step

**Now build the 10-hour proof first.**

Once you see `Health: 72/100` in the terminal, lock the API, tag `v0.1.0`, and sprint toward the UI.

---

## Quick Commands Reference

```bash
# Development
cargo run -- scan --quick
cargo test
cargo clippy

# Building
cargo build --release
tauri build

# Packaging
# Windows: creates .msi installer
tauri build --target x86_64-pc-windows-msvc

# Portable version
# Just zip the target/release folder
```

---

## Contact & Support

- GitHub Issues: `[your-repo]/issues`
- Discord: `[if applicable]`
- Email: `[your-email]`

---

*This document is a living guide. Update it as the project evolves.*