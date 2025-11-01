# AI Agent Instructions for Health & Speed Checker

**CRITICAL**: All AI agents (Copilot, Cursor, Claude, GPT, etc.) working on this project MUST follow these instructions.

---

## Project Overview

**Health & Speed Checker** is a desktop security and performance monitoring tool built with:
- **Backend**: Rust (agent library + checkers)
- **Frontend**: React + TypeScript + Tailwind CSS
- **Desktop Framework**: Tauri
- **Database**: rusqlite (synchronous)

**Purpose**: Scan Windows/macOS/Linux systems for security vulnerabilities and performance issues, provide actionable fixes.

---

## CRITICAL ARCHITECTURE DECISIONS

### 1. **SYNCHRONOUS CODE ONLY** ⚠️

**The entire checker system is SYNCHRONOUS. DO NOT introduce async/await.**

```rust
// ✅ CORRECT
pub trait Checker: Send + Sync {
    fn run(&self, context: &ScanContext) -> Vec<Issue>;
    fn fix(&self, issue_id: &str, params: &serde_json::Value) -> Result<FixResult, String>;
}

// ❌ WRONG - DO NOT DO THIS
pub trait Checker: Send + Sync {
    async fn run(&self, context: &ScanContext) -> Vec<Issue>;  // NO ASYNC!
}
```

**Why**: rusqlite is synchronous, checkers perform fast blocking I/O, async adds unnecessary complexity.

**If you see async/await in checker code, REMOVE IT immediately.**

---

## FROZEN TYPE SCHEMAS (DO NOT MODIFY)

### Issue Struct Schema

```rust
pub struct Issue {
    pub id: String,                          // ✅ Required
    pub severity: IssueSeverity,             // ✅ Required (Critical, Warning, Info)
    pub title: String,                       // ✅ Required
    pub description: String,                 // ✅ Required
    pub impact_category: ImpactCategory,     // ✅ Required (Security, Performance, Privacy, Both)
    pub fix: Option<FixAction>,              // ✅ Required (None if not fixable)
}
```

**❌ NEVER use these (old schema)**:
- `category: CheckCategory` → Use `impact_category: ImpactCategory`
- `fixable: bool` → Use `fix: Option<FixAction>`
- `affected_item: String` → REMOVED, don't use

### IssueSeverity Mapping

```rust
// ✅ CORRECT severity values
IssueSeverity::Critical  // Red, urgent action required
IssueSeverity::Warning   // Yellow, should fix soon
IssueSeverity::Info      // Blue, informational

// ❌ WRONG - these don't exist
IssueSeverity::High      // NO
IssueSeverity::Medium    // NO
IssueSeverity::Low       // NO
```

### FixResult Schema

```rust
pub struct FixResult {
    pub success: bool,
    pub message: String,
    pub rollback_available: bool,      // ✅ Use this
    pub restore_point_id: Option<String>,  // ✅ Use this
}

// ❌ WRONG - don't use
// pub requires_restart: bool  // This field doesn't exist!
```

---

## SECURITY REQUIREMENTS

### 1. **Input Validation & Sanitization**

**ALWAYS validate user input before using in commands:**

```rust
// ✅ CORRECT - validate against whitelist
fn fix(&self, issue_id: &str, params: &serde_json::Value) -> Result<FixResult, String> {
    if let Some(pattern) = issue_id.strip_prefix("bloatware_") {
        // Validate against whitelist
        let valid_patterns = Self::bloatware_patterns();
        if !valid_patterns.contains_key(pattern) {
            return Err(format!("Invalid pattern: {}", pattern));
        }

        // Additional character sanitization
        if !pattern.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err("Pattern contains invalid characters".to_string());
        }

        // NOW safe to use in command
        Command::new("reg").args(&["delete", "...", pattern, "/f"]).output();
    }
}

// ❌ WRONG - command injection vulnerability
fn fix(&self, issue_id: &str, params: &serde_json::Value) -> Result<FixResult, String> {
    let pattern = issue_id.strip_prefix("bloatware_").unwrap();
    // Directly using pattern in command = SECURITY HOLE
    Command::new("reg").args(&["delete", "...", pattern, "/f"]).output();
}
```

### 2. **NEVER Trust External Data**

- Validate ALL data from system commands before using
- Sanitize file paths, registry keys, process names
- Use whitelists, not blacklists
- Escape special characters in shell commands

### 3. **Principle of Least Privilege**

- Don't request admin/sudo unless absolutely necessary
- Clearly document when elevated privileges are needed
- Provide graceful fallback for non-admin users

---

## CODE QUALITY STANDARDS

### 1. **Error Handling**

```rust
// ✅ CORRECT - descriptive errors
fn check_firewall() -> Result<bool, String> {
    let output = Command::new("netsh")
        .args(&["advfirewall", "show", "currentprofile", "state"])
        .output()
        .map_err(|e| format!("Failed to check firewall status: {}. Ensure netsh is available.", e))?;

    Ok(String::from_utf8_lossy(&output.stdout).contains("ON"))
}

// ❌ WRONG - vague errors
fn check_firewall() -> Result<bool, String> {
    let output = Command::new("netsh")
        .args(&["advfirewall", "show", "currentprofile", "state"])
        .output()
        .map_err(|e| format!("Error: {}", e))?;  // Too vague!

    Ok(String::from_utf8_lossy(&output.stdout).contains("ON"))
}
```

### 2. **No Unwrap/Expect in Production Code**

```rust
// ✅ CORRECT
let config = load_config().unwrap_or_default();

// ❌ WRONG
let config = load_config().unwrap();  // Will panic!
```

### 3. **Descriptive Naming**

```rust
// ✅ CORRECT
const MAX_STARTUP_ITEMS_THRESHOLD: usize = 15;
const CRITICAL_DISK_SPACE_PERCENT: u8 = 5;
const WARNING_DISK_SPACE_PERCENT: u8 = 10;

// ❌ WRONG - magic numbers
if startup_items.len() > 15 { ... }
if disk_free_percent < 5 { ... }
```

---

## TYPESCRIPT/REACT STANDARDS

### 1. **Type Safety**

```typescript
// ✅ CORRECT - full type definitions
interface ScanResult {
  scan_id: string;
  timestamp: number;
  duration_ms: number;
  scores: {
    health: number;
    speed: number;
    health_delta?: number;
    speed_delta?: number;
  };
  issues: Issue[];
  details: ScanDetails;  // NOT 'any'
}

// ❌ WRONG
interface ScanResult {
  scan_id: string;
  // ...
  details: any;  // Don't use 'any'!
}
```

### 2. **User Feedback**

```typescript
// ✅ CORRECT - toast notifications, loading states
const [errorMessage, setErrorMessage] = useState<string | null>(null);
const [fixingIssueId, setFixingIssueId] = useState<string | null>(null);

const handleFix = async (actionId: string, issueId: string) => {
  setFixingIssueId(issueId);
  try {
    const result = await invoke('fix_action', { actionId });
    setSuccessMessage(result.message);
  } catch (error) {
    setErrorMessage(error.message);
  } finally {
    setFixingIssueId(null);
  }
};

// ❌ WRONG - console.log and alert()
const handleFix = async (actionId: string) => {
  try {
    const result = await invoke('fix_action', { actionId });
    alert('Fixed!');  // NO!
  } catch (error) {
    console.error(error);  // NO!
  }
};
```

### 3. **No Direct DOM Manipulation**

Use React state and props. Don't use `document.getElementById()` or jQuery.

---

## PLATFORM-SPECIFIC CODE

```rust
// ✅ CORRECT - use cfg attributes
#[cfg(target_os = "windows")]
fn check_windows_firewall() -> Result<bool, String> {
    use std::process::Command;
    // Windows-specific code
}

#[cfg(target_os = "macos")]
fn check_macos_firewall() -> Result<bool, String> {
    // macOS-specific code
}

#[cfg(target_os = "linux")]
fn check_linux_firewall() -> Result<bool, String> {
    // Linux-specific code
}

// Always provide fallback
impl Checker for FirewallChecker {
    fn run(&self, _context: &ScanContext) -> Vec<Issue> {
        #[cfg(target_os = "windows")]
        return check_windows();

        #[cfg(target_os = "macos")]
        return check_macos();

        #[cfg(target_os = "linux")]
        return check_linux();

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        Vec::new()  // Fallback for unsupported OS
    }
}
```

---

## TESTING REQUIREMENTS

### 1. **Unit Tests for All Checkers**

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
    fn test_sanitization() {
        // Test that malicious input is rejected
        let result = sanitize_input("../../etc/passwd");
        assert!(result.is_err());
    }
}
```

### 2. **Integration Tests**

Test full scan flow: `cargo test --workspace`

### 3. **Security Tests**

- Fuzz test input validation
- Test command injection prevention
- Test privilege escalation attempts

---

## COMMON MISTAKES TO AVOID

### ❌ 1. Using Async in Checkers
```rust
// WRONG
async fn run(&self, context: &ScanContext) -> Vec<Issue>
```

### ❌ 2. Old Issue Schema
```rust
// WRONG
Issue {
    severity: IssueSeverity::High,  // Use Critical, Warning, or Info
    category: CheckCategory::Security,  // Use impact_category
    fixable: true,  // Use fix: Some(FixAction {...})
}
```

### ❌ 3. Console.log in Production
```typescript
// WRONG
console.log('Scan result:', result);
alert('Export complete!');
```

### ❌ 4. Unwrap in Production
```rust
// WRONG
let config = load_config().unwrap();  // Will panic!
```

### ❌ 5. No Input Validation
```rust
// WRONG - command injection vulnerability
Command::new("reg").args(&["delete", user_input]).output();
```

---

## FILE STRUCTURE

```
Health & Speed Checker/
├── agent/                    # Rust backend
│   ├── src/
│   │   ├── lib.rs           # Core types, Checker trait, ScannerEngine
│   │   ├── checkers/
│   │   │   ├── mod.rs       # Checker exports + inline checkers
│   │   │   ├── network.rs   # Network checker
│   │   │   ├── bloatware.rs # Bloatware detector
│   │   │   ├── smart_disk.rs # S.M.A.R.T. checker
│   │   │   └── storage.rs   # Storage checker
│   │   └── db.rs            # Database layer (Jules' work)
│   └── Cargo.toml
├── ui/
│   ├── src/
│   │   ├── App.tsx          # Main React component
│   │   ├── components/
│   │   │   ├── QuickActions.tsx
│   │   │   ├── ExportDialog.tsx
│   │   │   └── TrendsChart.tsx
│   │   └── hooks/
│   │       └── useKeyboardShortcuts.ts
│   └── src-tauri/
│       └── src/
│           └── main.rs      # Tauri commands (UI ↔ Rust bridge)
└── AI_AGENT_INSTRUCTIONS.md  # This file
```

---

## WHEN IN DOUBT

1. **Read `agent/src/lib.rs`** - It defines the core types and architecture
2. **Check existing checkers** - Use them as templates
3. **Maintain type safety** - If TypeScript/Rust complains, fix the types, don't use `any` or `unwrap`
4. **Security first** - Validate ALL external input
5. **No async** - Keep everything synchronous unless it's UI code

---

## COMMIT GUIDELINES

```
feat: Add network latency checker
fix: Prevent command injection in bloatware detector
refactor: Extract magic numbers to constants
security: Add input validation to registry deletion
perf: Optimize process enumeration query
```

Use conventional commits. Always mention security fixes in commit messages.

---

## FINAL CHECKLIST BEFORE COMMITTING

- [ ] No `async fn` in checkers
- [ ] All `Issue` structs use correct schema
- [ ] Input validation on all external data
- [ ] No `console.log` or `alert()` in production code
- [ ] No `.unwrap()` in production code
- [ ] Error messages are user-friendly
- [ ] Tests pass: `cargo test && npm test`
- [ ] Type errors resolved (no `any` types)
- [ ] Security review: no command injection, XSS, SQL injection

---

**Last Updated**: 2025-10-31
**Maintained By**: AI agents working on Health & Speed Checker

**Remember**: This is a security tool. Users trust it with system access. Every line of code must be secure, reliable, and well-tested.
