<div align="center">

# ⚒️ Cargo-Forge

**An interactive Rust project generator with templates and common features**

[![Crates.io](https://img.shields.io/crates/v/cargo-forge.svg)](https://crates.io/crates/cargo-forge)
[![Documentation](https://docs.rs/cargo-forge/badge.svg)](https://docs.rs/cargo-forge)
[![CI](https://github.com/marcuspat/cargo-forge/workflows/CI/badge.svg)](https://github.com/marcuspat/cargo-forge/actions)
[![License](https://img.shields.io/crates/l/cargo-forge.svg)](https://github.com/marcuspat/cargo-forge#license)
[![Test Coverage](https://img.shields.io/badge/coverage-45.84%25-yellow.svg)](https://github.com/marcuspat/cargo-forge)

*Generate Rust projects with templates and optional features*

</div>

## 🚀 Quick Start

```bash
# Install cargo-forge
cargo install cargo-forge

# Create a new project interactively
cargo-forge new

# Or specify project type directly
cargo-forge new my-api --project-type api-server

# Initialize in current directory
cargo-forge init --project-type library
```

📖 **[Detailed Installation Guide](INSTALLATION.md)** - Including shell completions, pre-built binaries, and platform-specific instructions.

## ✨ Features

### 🎯 Project Types

Cargo-Forge supports 7 project types with templates:

| Type | Description | Key Features |
|------|-------------|--------------|
| **cli-tool** | Command-line applications | • Clap argument parsing<br>• Colored output<br>• Progress indicators<br>• Error handling |
| **library** | Rust library crates | • Documentation templates<br>• Example code<br>• Benchmark setup<br>• CI/CD ready |
| **api-server** | REST API servers | • Axum web framework<br>• JWT authentication<br>• Database integration<br>• OpenAPI docs |
| **wasm-app** | WebAssembly applications | • wasm-bindgen setup<br>• Web-sys integration<br>• Build scripts<br>• HTML template |
| **game-engine** | Game development | • Bevy engine<br>• Asset pipeline<br>• ECS architecture<br>• Dev tools |
| **embedded** | Embedded systems | • no_std setup<br>• Memory configuration<br>• HAL integration<br>• Debug configs |
| **workspace** | Multi-crate projects | • Organized structure<br>• Shared dependencies<br>• Cross-crate testing<br>• Unified CI |

### 🛠️ Optional Features

Enable features during project creation:

#### **CI/CD Integration**
- **GitHub Actions**: Automated testing, releases, and deployment
- **GitLab CI**: Complete pipeline configuration with caching
- **Custom CI**: Template for other CI systems

#### **Database Support**
- **PostgreSQL**: SQLx integration with migrations
- **MySQL**: Full MySQL/MariaDB support
- **SQLite**: Embedded database with migrations

#### **Authentication**
- **JWT**: JSON Web Token authentication
- **OAuth**: OAuth2 with popular providers
- **Password**: Bcrypt-based password authentication

#### **Additional Features**
- **Docker**: Multi-stage Dockerfile and docker-compose
- **Testing Frameworks**: Property testing, benchmarks, integration tests
- **Documentation**: Auto-generated docs with examples
- **Logging**: Structured logging with tracing

## 📋 Comparison with cargo-generate

| Feature | cargo-forge | cargo-generate |
|---------|------------|----------------|
| **Interactive Mode** | ✅ Built-in with beautiful TUI | ❌ Requires manual input |
| **Project Types** | ✅ 7 specialized types | ⚠️ Generic templates |
| **Defaults** | ✅ Pre-configured options | ❌ Manual configuration |
| **Feature Combinations** | ✅ Validated combinations | ⚠️ No validation |
| **Dry Run Mode** | ✅ Preview before creation | ❌ Not available |
| **Config Files** | ✅ Save/load preferences | ⚠️ Limited support |
| **Non-interactive Mode** | ✅ CI-friendly with defaults | ✅ Available |
| **Custom Templates** | ✅ Tera templates | ✅ Various engines |
| **Conditional Logic** | ✅ Smart conditionals | ✅ Basic support |
| **Post-generation Hooks** | ✅ Automatic setup | ⚠️ Manual scripts |
| **Error Recovery** | ✅ Graceful handling | ⚠️ Basic errors |
| **Performance** | ✅ ~1.5s generation | ⚠️ Varies by template |

## 🎮 Usage Examples

### Interactive Mode (Recommended)

```bash
$ cargo-forge new
⚒️ Welcome to Cargo-Forge!
? What's your project name? › my-awesome-api
? Select project type › API Server
? Add authentication? › JWT
? Include database? › PostgreSQL
? Add Docker support? › Yes
? Setup CI/CD? › GitHub Actions

✨ Project created successfully at ./my-awesome-api
```

### Command-Line Mode

```bash
# Create an API server with PostgreSQL and JWT auth
cargo-forge new my-api \
  --project-type api-server \
  --author "Jane Doe" \
  --description "My awesome API"

# Create a CLI tool in non-interactive mode (great for CI)
cargo-forge new my-cli \
  --project-type cli-tool \
  --non-interactive

# Initialize a library in current directory
cargo-forge init --project-type library

# Dry run to preview what will be created
cargo-forge new my-project --dry-run

# Use saved configuration
cargo-forge new my-project --from-config ~/.forge/defaults.toml
```

### Advanced Usage

```bash
# Create a workspace with multiple crates
cargo-forge new my-workspace --project-type workspace

# Generate a game with Bevy engine
cargo-forge new my-game --project-type game-engine

# Create an embedded project for STM32
cargo-forge new my-firmware --project-type embedded
```

## 📁 Generated Project Structure

### Example: API Server

```
my-api/
├── src/
│   ├── main.rs           # Application entry point
│   ├── routes/           # HTTP route handlers
│   │   ├── mod.rs
│   │   ├── health.rs
│   │   └── users.rs
│   ├── models/           # Data models
│   │   ├── mod.rs
│   │   └── user.rs
│   ├── middleware/       # HTTP middleware
│   │   ├── mod.rs
│   │   └── auth.rs
│   └── utils/            # Utility functions
│       ├── mod.rs
│       └── config.rs
├── migrations/           # Database migrations
│   └── 001_initial.sql
├── tests/                # Integration tests
│   └── api_tests.rs
├── .github/              # GitHub Actions CI
│   └── workflows/
│       └── ci.yml
├── Dockerfile            # Multi-stage Docker build
├── docker-compose.yml    # Local development setup
├── .env.example          # Environment variables template
├── Cargo.toml            # Project manifest
└── README.md             # Project documentation
```

## 🔧 Configuration

### Global Configuration

Save your preferences for future projects:

```toml
# ~/.config/cargo-forge/config.toml
[defaults]
author = "Your Name"
license = "MIT OR Apache-2.0"
vcs = "git"

[preferences]
always_add_ci = true
default_ci = "github"
prefer_workspace = false
```

### Project Configuration

Each project type supports specific configuration:

```toml
# forge.toml in your project
[project]
type = "api-server"
features = ["database", "auth", "docker"]

[database]
type = "postgresql"
migrations_dir = "./migrations"

[auth]
type = "jwt"
expires_in = "24h"
```

## 📊 Template Syntax

Cargo-Forge uses Tera templates with custom helpers:

```rust
// Conditional compilation based on features
{% if database %}
use sqlx::{PgPool, postgres::PgPoolOptions};
{% endif %}

// Smart defaults with fallbacks
const PORT: u16 = {{ port | default(value=3000) }};

// Case transformations
mod {{ name | snake_case }};
struct {{ name | pascal_case }};

// Feature combinations
{% if auth and database %}
// Authentication with database backend
{% endif %}
```

## 🧪 Testing

Generated projects include comprehensive test setups:

```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin

# Benchmarks (if enabled)
cargo bench

# Property tests (if enabled)
cargo test --features proptest
```

## 🚢 CI/CD Integration

All project types can include CI/CD configuration:

### GitHub Actions
- Multi-platform testing (Windows, Mac, Linux)
- Rust version matrix (stable, beta, nightly)
- Security audits and dependency checks
- Release automation with cargo-release
- Code coverage with Codecov

### GitLab CI
- Cached dependencies for faster builds
- Parallel job execution
- Deploy stages for different environments
- Container registry integration

## 🐳 Docker Support

Generated Dockerfiles use multi-stage builds for optimal image size:

```dockerfile
# Build stage
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
COPY --from=builder /app/target/release/app /usr/local/bin/
CMD ["app"]
```

## 🔒 Security

Cargo-Forge security features:

- No hardcoded secrets in templates
- Secure default configurations
- Environment variable usage for sensitive data
- Security audit integration in CI
- OWASP compliance for web projects

## 🤝 Contributing

We love contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/cargo-forge
cd cargo-forge

# Run tests
cargo test

# Run with coverage
cargo tarpaulin

# Build for release
cargo build --release
```

## 📈 Performance

Cargo-Forge is optimized for speed:

- Project generation: ~1.5 seconds
- Template rendering: <100ms
- Feature validation: <50ms
- Cross-platform: Works on Windows, Mac, and Linux

## 🐛 Troubleshooting

### Common Issues

**Q: Command not found after installation**
```bash
# Ensure cargo bin directory is in PATH
export PATH="$HOME/.cargo/bin:$PATH"
```

**Q: Permission denied errors**
```bash
# Check directory permissions
ls -la .
# Use sudo if needed (not recommended)
```

**Q: Template rendering fails**
```bash
# Validate your input
cargo-forge new --dry-run
# Check for special characters in project name
```

### Getting Help

- 📖 [Documentation](https://docs.rs/cargo-forge)
- 🐛 [Issue Tracker](https://github.com/marcuspat/cargo-forge/issues)

## 📚 Documentation

- **[Quick Reference](docs/QUICK_REFERENCE.md)** - Command cheat sheet and quick examples
- **[Project Types Guide](docs/PROJECT_TYPES.md)** - Detailed guide for each project type
- **[Template Syntax](docs/TEMPLATE_SYNTAX.md)** - Tera template documentation
- **[FAQ](docs/FAQ.md)** - Frequently asked questions
- **[Troubleshooting](docs/TROUBLESHOOTING.md)** - Common issues and solutions
- **[Contributing](CONTRIBUTING.md)** - Development guidelines
- **[Changelog](CHANGELOG.md)** - Version history and roadmap

## 📜 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🙏 Acknowledgments

- The Rust community for feedback
- Contributors who help make Cargo-Forge better
- Similar projects: cargo-generate, create-react-app

---

<div align="center">

**Built with ❤️ by the Rust community**

[Documentation](https://docs.rs/cargo-forge) • [Crates.io](https://crates.io/crates/cargo-forge)

</div>
