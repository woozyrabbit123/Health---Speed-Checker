# Contributing to Health & Speed Checker

Thank you for your interest in contributing to Health & Speed Checker! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for everyone.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues to avoid duplicates. When creating a bug report, include:

- **Clear title and description**
- **Steps to reproduce** the issue
- **Expected behavior** vs **actual behavior**
- **Screenshots** if applicable
- **OS version** and **app version**
- **Relevant logs** from `~/.healthchecker/logs/`

### Suggesting Features

Feature suggestions are welcome! Please:

- Use a clear and descriptive title
- Provide detailed explanation of the feature
- Explain why this feature would be useful
- Include mockups or examples if applicable

### Pull Requests

1. **Fork the repository** and create your branch from `main`
2. **Make your changes** following our coding standards
3. **Test thoroughly** - ensure all tests pass
4. **Update documentation** if needed
5. **Write clear commit messages**
6. **Submit the pull request**

## Development Setup

### Prerequisites

- Rust 1.75+ (`rustup update`)
- Node.js 18+ (`node --version`)
- Platform-specific dependencies:
  - **Windows**: Windows SDK
  - **macOS**: Xcode Command Line Tools
  - **Linux**: `libwebkit2gtk-4.0-dev`, `libgtk-3-dev`

### Getting Started

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/health-speed-checker.git
cd health-speed-checker

# Build the Rust agent
cd agent
cargo build
cargo test

# Build the UI
cd ../ui
npm install
npm run dev
```

## Coding Standards

### Rust

- Follow standard Rust formatting: `cargo fmt`
- Lint with clippy: `cargo clippy`
- Write tests for new functionality
- Document public APIs with doc comments
- Use meaningful variable and function names

### TypeScript/React

- Use TypeScript for type safety
- Follow ESLint rules: `npm run lint`
- Use functional components with hooks
- Keep components small and focused
- Write meaningful prop types

### Commit Messages

Follow the conventional commits format:

```
type(scope): subject

body (optional)

footer (optional)
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Test additions or changes
- `chore`: Build process or tooling changes

Examples:
```
feat(scanner): add S.M.A.R.T. drive health checker
fix(ui): resolve progress bar animation glitch
docs(readme): update installation instructions
```

## Project Structure

```
health-speed-checker/
├── agent/           # Rust backend
│   └── src/
│       ├── checkers/   # Individual checker modules
│       ├── lib.rs      # Core library
│       └── main.rs     # CLI entry point
├── ui/              # Tauri/React frontend
│   └── src/
│       ├── App.tsx     # Main component
│       └── App.css     # Styles
├── db/              # Database schemas
└── docs/            # Documentation
```

## Writing Checkers

New checkers should implement the `Checker` trait:

```rust
use async_trait::async_trait;

pub struct MyChecker;

#[async_trait]
impl Checker for MyChecker {
    fn name(&self) -> &'static str {
        "my_checker"
    }

    fn category(&self) -> CheckCategory {
        CheckCategory::Security // or Performance
    }

    async fn run(&self, context: &ScanContext) -> Vec<Issue> {
        let mut issues = Vec::new();

        // Your checking logic here

        issues
    }

    async fn fix(&self, issue_id: &str, params: &serde_json::Value)
        -> Result<FixResult, String> {
        // Optional: Implement fix functionality
        Err("Fix not implemented".to_string())
    }
}
```

Register your checker in `main.rs`:
```rust
engine.register(Box::new(MyChecker));
```

## Testing

### Rust Tests

```bash
# Run all tests
cd agent
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### UI Tests

```bash
cd ui
npm test
```

## Pull Request Process

1. Update README.md if needed
2. Update CHANGELOG.md with your changes
3. Ensure all tests pass
4. Request review from maintainers
5. Address any feedback
6. Merge after approval

## Release Process

(For maintainers only)

1. Update version in `Cargo.toml` and `package.json`
2. Update CHANGELOG.md
3. Create git tag: `git tag -a v0.1.0 -m "Release v0.1.0"`
4. Push tag: `git push origin v0.1.0`
5. GitHub Actions will build and create release

## Questions?

- Check the [documentation](docs/)
- Open a [discussion](https://github.com/yourusername/health-speed-checker/discussions)
- Join our [Discord](https://discord.gg/yourinvite)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing! 🎉
