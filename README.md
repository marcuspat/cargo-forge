<p align="center"><img src="assets/banner.svg" alt="cargo-forge — animated banner" width="100%"></p>

<div align="center">

# ⚒️ Cargo-Forge

**A Rust project generator with templates and common features**

[![Crates.io](https://img.shields.io/crates/v/cargo-forge.svg)](https://crates.io/crates/cargo-forge)
[![Documentation](https://docs.rs/cargo-forge/badge.svg)](https://docs.rs/cargo-forge)
[![License](https://img.shields.io/crates/l/cargo-forge.svg)](https://github.com/marcuspat/cargo-forge#license)
[![Test Coverage](https://img.shields.io/badge/coverage-45.84%25-yellow.svg)](https://github.com/marcuspat/cargo-forge)

*Generate Rust projects with templates and optional features*

</div>

## 🎬 Demo

![cargo-forge generating an API server project](demo.gif)

*Recorded from the actual binary with [asciinema](https://asciinema.org) + [agg](https://github.com/asciinema/agg).*

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

📖 See [Quick Reference](docs/QUICK_REFERENCE.md) for shell completions and common commands, or [Troubleshooting](docs/TROUBLESHOOTING.md) for platform-specific install issues.

## ✨ Features

### 🎯 Project Types

Cargo-Forge supports 7 project types with templates:

| Type | Description | Key Features |
|------|-------------|--------------|
| **cli-tool** | Command-line applications | • Clap dependency setup<br>• Basic project structure<br>• Ready for CLI development |
| **library** | Rust library crates | • Library template<br>• Examples directory<br>• Documentation ready<br>• Tests structure |
| **api-server** | REST API servers | • Axum web framework<br>• Basic HTTP server setup<br>• Route handlers structure<br>• Ready for API development |
| **wasm-app** | WebAssembly applications | • wasm-bindgen setup<br>• Web-sys integration<br>• Build scripts<br>• HTML template |
| **game-engine** | Game development | • Bevy engine<br>• Asset pipeline structure<br>• Basic game setup<br>• Development ready |
| **embedded** | Embedded systems | • no_std setup<br>• Memory configuration<br>• HAL integration<br>• Debug configs |
| **workspace** | Multi-crate projects | • Organized structure<br>• Shared dependencies<br>• Cross-crate testing<br>• Unified configuration |

_Note: the templates are deliberately minimal starting points, not frameworks. `api-server` gives you a running server with one health-check route; the rest of the handlers/routes/models are yours to write. `game-engine` compiles a windowed Bevy app; real assets and systems are up to you._

### 🛠️ Core Features

Current features available in v0.1.8:

#### **Project Structure**
- **Working starters**: every template generates code that compiles and runs
  (an Axum health-check server, a Clap CLI with a `hello` command, a library
  with a unit-tested `greet` API, a wasm-bindgen export, a minimal Bevy app)
- **Dependency Management**: Appropriate dependencies for each project type
- **Documentation**: README files with project-specific instructions
- **Testing**: the library and cli-tool starters ship tests you can run with
  `cargo test` right after generation

#### **Development Tools**
- **Dry Run Mode**: Preview project structure before creation (add
  `--non-interactive` when running without a TTY, e.g. in CI)
- **Non-interactive Mode**: CI-friendly project generation
- **Name Validation**: Ensures valid Cargo package names
- **License Field**: `--license` writes the SPDX `license` field into
  `Cargo.toml` (same convention as `cargo new --license`; no LICENSE file
  is generated)
- **Shell Completions**: Bash, zsh, fish, and PowerShell support
- **Config File**: `--from-config` reads TOML (JSON also accepted for
  backward compatibility); `default_author`, `default_license`,
  `preferred_project_types` and per-type `default_features` are applied

#### **Optional Features** (select interactively or via config)
- **Docker**: Dockerfile (multi-stage) + .dockerignore
- **CI**: GitHub Actions workflow
- **Database**: PostgreSQL, MySQL, or SQLite scaffolding via sqlx

#### **Planned**
- Authentication scaffolding (JWT/OAuth)
- GitLab CI support

## 📋 Comparison with cargo-generate

| Feature | cargo-forge | cargo-generate |
|---------|------------|----------------|
| **Interactive Mode** | ✅ Prompt-based (inquire) | ⚠️ Requires manual input |
| **Project Types** | ✅ 7 built-in types | ✅ Arbitrary template repos |
| **Defaults** | ✅ Pre-configured options | ❌ Manual configuration |
| **Name Validation** | ✅ Built-in validation | ⚠️ Basic validation |
| **Dry Run Mode** | ✅ Preview before creation | ❌ Not available |
| **Non-interactive Mode** | ✅ CI-friendly with defaults | ✅ Available |
| **Custom Templates** | ❌ Built-in types only | ✅ Various engines |
| **Shell Completions** | ✅ All major shells | ⚠️ Manual setup |
| **Error Recovery** | ✅ Graceful handling | ⚠️ Basic errors |
| **Performance** | ✅ ~0.3s generation | ⚠️ Varies by template |

## 🎮 Usage Examples

### Command-Line Mode (Current)

```bash
# Create an API server (Axum starter with a GET / health check)
cargo-forge new my-api \
  --project-type api-server \
  --author "Jane Doe" \
  --description "My awesome API" \
  --license MIT

# Create a CLI tool in non-interactive mode (great for CI)
cargo-forge new my-cli \
  --project-type cli-tool \
  --non-interactive

# Initialize a library in current directory
cargo-forge init --project-type library

# Dry run to preview what will be created
cargo-forge new my-project --dry-run

# Use saved configuration (TOML; JSON is also accepted)
cargo-forge new my-project --from-config ~/.forge/defaults.toml
```

Example `defaults.toml`:

```toml
default_author = "Jane Doe"
default_license = "MIT"
preferred_project_types = ["api-server"]
default_features = { api-server = ["docker", "ci"] }
edition = "2021"
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
│   ├── main.rs           # Tokio main: binds 0.0.0.0:3000 (PORT env overrides)
│   ├── handlers.rs       # GET / health-check handler
│   ├── routes.rs         # Router definition
│   └── models.rs         # Health model
├── config/
│   └── default.toml      # Configuration template
├── .env.example          # Documented env vars
├── .gitignore            # Git ignore file
├── Cargo.toml            # Project manifest with Axum dependencies
└── README.md             # Project documentation
```

## 🔧 Configuration

### Command-Line Options

Cargo-Forge supports various command-line options:

```bash
# Non-interactive mode (great for CI/CD)
cargo-forge new my-project --project-type api-server --non-interactive

# Dry run to preview what will be created
# (add --non-interactive when there is no TTY, e.g. in CI)
cargo-forge new my-project --project-type library --dry-run --non-interactive

# Initialize in current directory
cargo-forge init --project-type cli-tool
```

### Project Customization

After project creation, you can customize:
- Add dependencies to `Cargo.toml`
- Modify source files to fit your needs
- Update configuration files as needed
- Add additional features and integrations

## 🧪 Testing

The library and cli-tool starters include tests that run immediately:

```bash
# In a freshly generated library or cli-tool project
cargo test
```

## 🐳 Docker Support (optional feature)

Select the `docker` feature (interactively, or via `default_features` in your config) to generate a multi-stage Dockerfile plus .dockerignore. For `api-server` projects the container exposes port 3000. The generated Dockerfile uses multi-stage builds for optimal image size:

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

Properties of the generated templates today:

- No hardcoded secrets in templates
- Secure default configurations
- Environment variable usage for sensitive data

**Planned:** security audit integration in CI and OWASP-aligned checks for the `api-server` template — both depend on the CI/CD integration above, which isn't implemented yet.

## 🤝 Contributing

We love contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/marcuspat/cargo-forge
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

- Project generation: ~0.3 seconds
- Name validation: <1ms
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
