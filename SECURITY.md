# Security Policy

## Supported Versions

We release patches for security vulnerabilities in the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

We take the security of Health & Speed Checker seriously. If you believe you have found a security vulnerability, please report it to us as described below.

### Please do NOT:

- Open a public GitHub issue for security vulnerabilities
- Post about the vulnerability on social media or other public forums

### Please DO:

1. **Email us directly** at: security@healthspeedchecker.com
2. **Include the following information:**
   - Type of vulnerability
   - Full paths of source file(s) related to the vulnerability
   - Location of the affected source code (tag/branch/commit or direct URL)
   - Step-by-step instructions to reproduce the issue
   - Proof-of-concept or exploit code (if possible)
   - Impact of the issue, including how an attacker might exploit it

### What to expect:

- **Acknowledgment**: We will acknowledge receipt of your vulnerability report within 48 hours
- **Initial Assessment**: We will provide an initial assessment within 5 business days
- **Updates**: We will keep you informed of our progress throughout the resolution process
- **Credit**: We will credit you (if desired) in the security advisory once the issue is resolved

## Security Best Practices

When using Health & Speed Checker:

### For Users:

- ✅ Always download from official sources (GitHub releases or official website)
- ✅ Verify release signatures when available
- ✅ Keep the application updated to the latest version
- ✅ Review the permissions requested by the application
- ✅ Run scans regularly to detect security issues

### For Developers:

- ✅ Follow secure coding practices outlined in CONTRIBUTING.md
- ✅ Run `cargo audit` regularly to check for vulnerable dependencies
- ✅ Use `cargo clippy` to catch potential security issues
- ✅ Never commit secrets or API keys
- ✅ Review all third-party dependencies before adding them
- ✅ Write security-focused tests for critical functionality

## Known Security Considerations

### Local Data Storage

- All scan results and configuration are stored locally in `~/.healthchecker/`
- On Windows, sensitive data uses DPAPI for encryption
- Database files should have appropriate file permissions

### Privileged Operations

Some checkers and fixes require elevated permissions:
- Firewall modifications
- System restore point creation
- Registry modifications (Windows)

The application will request these permissions only when needed.

### Network Communication

By default, the application operates 100% offline:
- No telemetry or analytics by default
- Optional update checks can be disabled in settings
- No data leaves your machine without explicit opt-in

## Security Features

- 🔒 **Local-first architecture**: No cloud dependencies by default
- 🔐 **Encrypted storage**: Sensitive data protected with OS-native encryption
- 📝 **Audit trail**: All fixes logged in the database
- ↩️ **Reversible actions**: System restore points created before modifications
- 🔍 **Open source**: All code is auditable on GitHub
- ✅ **Signed binaries**: Windows and macOS releases are code-signed

## Dependency Security

We use automated tools to monitor our dependencies:

- **Rust**: `cargo-audit` runs on every CI build
- **Node.js**: Dependabot alerts enabled
- **GitHub Actions**: Security scanning enabled

## Updates and Patches

Security patches are released as soon as possible after discovery:

- **Critical vulnerabilities**: Patches within 24-48 hours
- **High severity**: Patches within 1 week
- **Medium/Low severity**: Patches in next regular release

## Bug Bounty Program

We currently do not have a formal bug bounty program, but we greatly appreciate responsible disclosure and will publicly credit researchers (with permission) in our security advisories.

## Security Contacts

- **Email**: security@healthspeedchecker.com
- **PGP Key**: [Link to PGP key] (for encrypted communications)
- **Response Time**: Within 48 hours

## Acknowledgments

We would like to thank the following security researchers who have responsibly disclosed vulnerabilities:

- [List will be updated as reports are received and resolved]

---

**Thank you for helping keep Health & Speed Checker and its users safe!**
