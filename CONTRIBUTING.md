# Contributing to Cargo Forge

First off, thank you for considering contributing to Cargo Forge! It's people like you that make Cargo Forge such a great tool for the Rust community.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Documentation](#documentation)
- [Submitting Changes](#submitting-changes)
- [Style Guidelines](#style-guidelines)
- [Template Development](#template-development)
- [Release Process](#release-process)

## Code of Conduct

This project and everyone participating in it is governed by the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). By participating, you are expected to uphold this code.

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally
3. Create a new branch for your feature/fix
4. Make your changes
5. Run tests
6. Submit a pull request

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates. When creating a bug report, include:

- A clear and descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- System information (OS, Rust version)
- Any error messages or logs

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, include:

- A clear and descriptive title
- A detailed description of the proposed enhancement
- Any possible drawbacks
- Examples of how it would be used

### Contributing Code

#### First Time Contributors

Look for issues labeled `good first issue` or `help wanted`. These are great starting points for new contributors.

#### Pull Requests

1. Follow the [style guidelines](#style-guidelines)
2. Include tests for new functionality
3. Update documentation as needed
4. Ensure all tests pass
5. Write a clear commit message

## Development Setup

### Prerequisites

- Rust 1.70.0 or later
- Git
- A code editor (VS Code, IntelliJ IDEA with Rust plugin, etc.)

### Building from Source

```bash
# Clone your fork
git clone https://github.com/yourusername/cargo-forge
cd cargo-forge

# Build the project
cargo build

# Run the project
cargo run -- new test-project

# Run in release mode
cargo build --release
```

### Development Tools

```bash
# Install development tools
cargo install cargo-watch cargo-tarpaulin cargo-audit

# Watch for changes and rebuild
cargo watch -x build

# Run tests on file change
cargo watch -x test

# Check code coverage
cargo tarpaulin

# Security audit
cargo audit
```

## Project Structure

```
cargo-forge/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library root
│   ├── forge.rs             # Core forge logic
│   ├── generator.rs         # Project generation
│   ├── project_types.rs     # Project type definitions
│   ├── config.rs            # Configuration handling
│   ├── features/            # Optional features
│   │   ├── mod.rs
│   │   ├── ci.rs            # CI/CD integration
│   │   ├── database.rs      # Database features
│   │   └── docker.rs        # Docker support
│   └── templates/           # Template handling
│       ├── mod.rs
│       └── conditional.rs   # Conditional rendering
├── templates/               # Project templates
│   ├── api_server/
│   ├── cli_tool/
│   ├── library/
│   ├── wasm_app/
│   ├── game_engine/
│   ├── embedded/
│   └── workspace/
├── tests/                   # Test suite
│   ├── integration.rs       # Integration tests
│   ├── e2e_*.rs            # End-to-end tests
│   └── fixtures/           # Test fixtures
└── docs/                   # Additional documentation
```

## Making Changes

### Adding a New Project Type

1. Add the type to `ProjectType` enum in `src/project_types.rs`
2. Create a new template directory in `templates/`
3. Add template files (use `.tera` extension)
4. Update the `default_features()` method
5. Add tests for the new project type
6. Update documentation

Example:
```rust
// In src/project_types.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ProjectType {
    // ... existing types ...
    MyNewType,
}

impl ProjectType {
    pub fn default_features(&self) -> Vec<&'static str> {
        match self {
            // ... existing matches ...
            ProjectType::MyNewType => vec!["some-crate", "another-crate"],
        }
    }
}
```

### Adding a New Feature

1. Create a new module in `src/features/`
2. Implement the `Plugin` trait
3. Add integration logic
4. Create templates in `templates/features/`
5. Add tests
6. Update documentation

Example:
```rust
// In src/features/my_feature.rs
use crate::features::{Plugin, ProjectContext};
use std::error::Error;

pub struct MyFeaturePlugin;

impl Plugin for MyFeaturePlugin {
    fn name(&self) -> &str {
        "my-feature"
    }

    fn configure(&self, context: &mut ProjectContext) -> Result<(), Box<dyn Error>> {
        // Add dependencies
        context.add_dependency("my-crate", "1.0");
        
        // Create directories
        context.create_directory("src/my_feature");
        
        // Add template files
        context.add_template_file(
            "src/my_feature/mod.rs",
            include_str!("../../templates/features/my_feature/mod.rs.tera")
        );
        
        Ok(())
    }
}
```

### Template Syntax

Cargo Forge uses [Tera](https://tera.netlify.app/) templates. Key features:

```rust
// Variables
{{ name }}
{{ author | default(value="Anonymous") }}

// Conditionals
{% if feature_enabled %}
use some_crate::SomeType;
{% endif %}

// Loops
{% for dep in dependencies %}
{{ dep.name }} = "{{ dep.version }}"
{% endfor %}

// Filters
{{ project_name | snake_case }}
{{ struct_name | pascal_case }}

// Comments
{# This is a comment #}
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests in parallel
cargo test -- --test-threads=4
```

### Test Categories

1. **Unit Tests**: Test individual functions and modules
2. **Integration Tests**: Test component interactions
3. **E2E Tests**: Test complete project generation
4. **Feature Tests**: Test optional features

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_generation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let forge = Forge::new(temp_dir.path());
        
        let result = forge.generate_project(
            "test-project",
            ProjectType::Library,
            None,
        );
        
        assert!(result.is_ok());
        assert!(temp_dir.path().join("test-project/Cargo.toml").exists());
    }
}
```

### Test Coverage

We aim for >80% test coverage. Check coverage with:

```bash
cargo tarpaulin --out Html
# Open tarpaulin-report.html in your browser
```

## Documentation

### Code Documentation

- Add doc comments to all public items
- Include examples in doc comments
- Use `cargo doc --open` to preview

```rust
/// Creates a new Forge instance
/// 
/// # Arguments
/// 
/// * `base_path` - The base directory for project generation
/// 
/// # Examples
/// 
/// ```
/// use cargo_forge::Forge;
/// 
/// let forge = Forge::new(".");
/// ```
pub fn new(base_path: &str) -> Self {
    // ...
}
```

### README Updates

Update the README.md when:
- Adding new features
- Changing CLI interface
- Adding new project types
- Modifying configuration options

## Submitting Changes

### Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
feat: add new project type for GraphQL APIs
fix: correct template rendering for Windows paths
docs: update README with new CLI options
test: add E2E tests for workspace generation
refactor: simplify feature plugin system
```

### Pull Request Process

1. Update documentation
2. Add tests for new functionality
3. Ensure all tests pass
4. Update CHANGELOG.md
5. Request review from maintainers

### PR Title Format

```
[Type] Brief description

Examples:
[Feature] Add GraphQL API project type
[Fix] Handle spaces in project names correctly
[Docs] Add troubleshooting guide
```

## Style Guidelines

### Rust Code Style

We use the standard Rust style guide with some additions:

```rust
// Use explicit imports for clarity
use std::path::PathBuf;
use std::fs;

// Prefer const over static for constants
const DEFAULT_PORT: u16 = 3000;

// Use descriptive variable names
let project_config = load_config()?;

// Group related functionality
mod database {
    pub mod connection;
    pub mod migrations;
}

// Error handling - use anyhow for applications
use anyhow::{Result, Context};

fn do_something() -> Result<()> {
    std::fs::read_to_string("file.txt")
        .context("Failed to read configuration file")?;
    Ok(())
}
```

### Formatting

Always run `cargo fmt` before committing:

```bash
# Format all files
cargo fmt

# Check formatting without changes
cargo fmt -- --check
```

### Linting

Use Clippy for additional checks:

```bash
# Run clippy
cargo clippy

# Run clippy with all features
cargo clippy --all-features

# Run clippy and fail on warnings
cargo clippy -- -D warnings
```

## Template Development

### Best Practices

1. **Use meaningful variable names**: `{{ project_name }}` not `{{ name }}`
2. **Provide sensible defaults**: `{{ port | default(value=3000) }}`
3. **Add helpful comments**: Explain complex template logic
4. **Consider cross-platform**: Use path separators correctly
5. **Validate inputs**: Check for edge cases

### Template Testing

Create test fixtures for templates:

```rust
#[test]
fn test_template_rendering() {
    let mut context = Context::new();
    context.insert("name", "test-project");
    context.insert("author", "Test Author");
    
    let template = include_str!("../templates/library/Cargo.toml.tera");
    let rendered = Tera::one_off(template, &context, false).unwrap();
    
    assert!(rendered.contains("name = \"test-project\""));
    assert!(rendered.contains("authors = [\"Test Author\"]"));
}
```

## Release Process

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):

- MAJOR: Incompatible API changes
- MINOR: New functionality (backwards compatible)
- PATCH: Bug fixes (backwards compatible)

### Release Checklist

1. [ ] Update version in `Cargo.toml`
2. [ ] Update CHANGELOG.md
3. [ ] Run full test suite
4. [ ] Build release binaries
5. [ ] Create GitHub release
6. [ ] Publish to crates.io

```bash
# Prepare release
cargo release --dry-run

# Create release
cargo release patch # or minor, major
```

## Getting Help

- **Discord**: Join our [Discord server](https://discord.gg/rust-forge)
- **Discussions**: Use GitHub Discussions for questions
- **Issues**: Report bugs via GitHub Issues

## Recognition

Contributors are recognized in:
- CHANGELOG.md (for each release)
- GitHub contributors page
- Annual contributor spotlight blog posts

Thank you for contributing to Cargo Forge!