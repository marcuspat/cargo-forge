# Cargo-Forge Quick Reference

## Command Overview

```bash
cargo-forge <COMMAND> [OPTIONS]
```

### Commands

| Command | Description |
|---------|-------------|
| `new [NAME]` | Create a new project |
| `init` | Initialize in current directory |
| `help` | Show help information |

## Common Usage Patterns

### Interactive Mode (Recommended)
```bash
cargo-forge new
# Follow the interactive prompts
```

### Quick Project Creation
```bash
# API Server
cargo-forge new my-api --project-type api-server

# CLI Tool
cargo-forge new my-cli --project-type cli-tool

# Library
cargo-forge new my-lib --project-type library

# WASM App
cargo-forge new my-app --project-type wasm-app

# Game
cargo-forge new my-game --project-type game-engine

# Embedded
cargo-forge new my-firmware --project-type embedded

# Workspace
cargo-forge new my-workspace --project-type workspace
```

### Non-Interactive Mode (CI/Scripts)
```bash
cargo-forge new my-project \
  --project-type api-server \
  --author "Jane Doe" \
  --non-interactive
```

### Dry Run (Preview)
```bash
cargo-forge new my-project --dry-run
```

### From Configuration
```bash
cargo-forge new my-project --from-config ~/.forge/defaults.toml
```

## Options Reference

### Global Options

| Option | Short | Description |
|--------|-------|-------------|
| `--help` | `-h` | Show help |
| `--version` | `-V` | Show version |

### `new` Command Options

| Option | Short | Description | Example |
|--------|-------|-------------|---------|
| `--project-type` | `-p` | Project type | `api-server` |
| `--author` | `-a` | Author name | `"Jane Doe"` |
| `--description` | `-d` | Project description | `"My API"` |
| `--license` | `-l` | License | `MIT` |
| `--non-interactive` | | Use defaults | |
| `--from-config` | | Config file path | `~/.forge/config.toml` |
| `--dry-run` | | Preview only | |

### `init` Command Options

| Option | Short | Description |
|--------|-------|-------------|
| `--project-type` | `-p` | Project type |
| `--author` | `-a` | Author name |
| `--license` | `-l` | License |
| `--non-interactive` | | Use defaults |
| `--from-config` | | Config file path |
| `--dry-run` | | Preview only |

## Project Types

| Type | Use Case | Key Dependencies |
|------|----------|------------------|
| `cli-tool` | Command-line apps | clap, colored, indicatif |
| `library` | Reusable crates | - |
| `api-server` | REST APIs | axum, tokio, tower |
| `wasm-app` | Browser apps | wasm-bindgen, web-sys |
| `game-engine` | Games | bevy |
| `embedded` | IoT/MCU | cortex-m, panic-halt |
| `workspace` | Multi-crate | - |

## Feature Matrix

| Feature | API | CLI | Lib | WASM | Game | Embedded | Workspace |
|---------|-----|-----|-----|------|------|----------|-----------|
| Database | ✅ | ✅ | ✅ | ❌ | ✅ | ❌ | ✅ |
| Auth | ✅ | ❌ | ⚠️ | ⚠️ | ❌ | ❌ | ✅ |
| Docker | ✅ | ✅ | ❌ | ⚠️ | ✅ | ❌ | ✅ |
| CI/CD | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |

Legend: ✅ Full support | ⚠️ Partial | ❌ Not applicable

## Configuration File Format

```toml
# ~/.config/cargo-forge/config.toml

[defaults]
author = "Your Name"
email = "your.email@example.com"
license = "MIT OR Apache-2.0"
vcs = "git"

[preferences]
always_add_ci = true
default_ci = "github"  # github, gitlab, both
prefer_workspace = false
default_features = ["docker", "database"]

[project_defaults.api-server]
database = "postgresql"
auth = "jwt"
port = 3000

[project_defaults.cli-tool]
style = "subcommands"  # simple, subcommands
```

## Template Variables

### Common Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `{{ name }}` | Project name | `my-project` |
| `{{ author }}` | Author name | `Jane Doe` |
| `{{ description }}` | Description | `My awesome project` |
| `{{ license }}` | License | `MIT` |
| `{{ year }}` | Current year | `2024` |
| `{{ date }}` | Current date | `2024-01-15` |

### Filters

| Filter | Description | Example |
|--------|-------------|---------|
| `snake_case` | Convert to snake_case | `{{ name \| snake_case }}` |
| `pascal_case` | Convert to PascalCase | `{{ name \| pascal_case }}` |
| `kebab_case` | Convert to kebab-case | `{{ name \| kebab_case }}` |
| `shouty_case` | Convert to SHOUTY_CASE | `{{ name \| shouty_case }}` |

## Quick Fixes

### Command not found
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### Update Cargo-Forge
```bash
cargo install cargo-forge --force
```

### Debug mode
```bash
RUST_LOG=debug cargo-forge new my-project
```

### Clean install
```bash
cargo uninstall cargo-forge
cargo install cargo-forge
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `CARGO_FORGE_CONFIG` | Config file path |
| `CARGO_FORGE_DEBUG` | Enable debug output |
| `CARGO_FORGE_NO_COLOR` | Disable colored output |
| `RUST_LOG` | Log level (debug, info, warn, error) |

## File Structure Generated

### Minimal Structure
```
my-project/
├── src/
│   └── main.rs or lib.rs
├── Cargo.toml
├── README.md
└── .gitignore
```

### Full Structure (with features)
```
my-project/
├── src/
│   ├── main.rs
│   ├── routes/
│   ├── models/
│   └── utils/
├── tests/
├── benches/
├── examples/
├── migrations/
├── .github/
│   └── workflows/
├── docker/
│   └── Dockerfile
├── Cargo.toml
├── README.md
├── CHANGELOG.md
├── LICENSE
└── .env.example
```

## Common Workflows

### New API with all features
```bash
cargo-forge new my-api \
  --project-type api-server
# Select: PostgreSQL, JWT auth, Docker, GitHub Actions
```

### Library for crates.io
```bash
cargo-forge new my-crate \
  --project-type library \
  --license "MIT OR Apache-2.0"
```

### Quick CLI tool
```bash
cargo-forge new my-tool \
  --project-type cli-tool \
  --non-interactive
```

### WASM app with webpack
```bash
cargo-forge new my-wasm-app \
  --project-type wasm-app
# The template includes webpack config
```

## Post-Generation Steps

1. **Enter directory**
   ```bash
   cd my-project
   ```

2. **Initialize git** (if not done)
   ```bash
   git init
   git add .
   git commit -m "Initial commit from Cargo-Forge"
   ```

3. **Check compilation**
   ```bash
   cargo check
   cargo test
   ```

4. **Set up environment** (if using features)
   ```bash
   cp .env.example .env
   # Edit .env with your values
   ```

5. **Run the project**
   ```bash
   cargo run
   ```

## Tips & Tricks

- Use `--dry-run` to preview before creating
- Save preferences in config file for consistency
- Combine with `direnv` for automatic environment setup
- Use workspace type for microservices
- Enable CI from the start for best practices
- Check generated README for project-specific instructions

## Getting Help

- Run: `cargo-forge help`
- Docs: `https://docs.rs/cargo-forge`
- Issues: `https://github.com/marcuspat/cargo-forge/issues`

---

*Quick Reference v0.1.0 - For detailed information, see the full documentation*