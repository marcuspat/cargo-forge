# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.8] - 2026-10-14
### Fixed (honesty pass: claims now match behavior)
- `rust-version` raised 1.70 → 1.88 — the committed Cargo.lock cannot
  build on 1.70 (clap 4.6 line requires a newer toolchain; floor verified
  empirically: 1.85 fails on the `home` crate, 1.88 compiles clean)
- workspace `--license` now reaches the member crates, not just
  `[workspace.package]` in the root manifest
- feature-plugin dependencies (e.g. `sqlx` for the database feature) are
  now written into the generated manifest — they were collected but never
  rendered, so feature files referenced crates the project did not depend on
- concurrent generation into the same directory is now exclusive via an
  O_EXCL claim file (previously a TOCTOU race allowed multiple writers);
  the claim is cleaned up on every exit path via a Drop guard, and the
  directory is re-checked for foreign files after the claim is taken, so a
  generator that acquires a just-released claim cannot overwrite the
  completed project that released it
- game-engine starter verified to compile against bevy 0.20 via manual
  `cargo check --target wasm32-unknown-unknown` (native desktop builds
  additionally need the Linux windowing libraries noted in the README)
- control characters in descriptions/authors/licenses can no longer
  produce invalid manifests (full TOML escaping); multi-line descriptions
  in libraries become multiple `//!` lines instead of broken code
- `--license` on `new` is no longer silently discarded — it writes a
  `license = "..."` field into `Cargo.toml` (and `[workspace.package]` for
  workspaces), the same convention as `cargo new --license`; on every
  path, including the interactive prompts (`--author` and
  `--description` too, which pre-answer their prompts)
- `init --author` and `init --license` are no longer silently discarded
- `--from-config` now accepts TOML (the documented format) in addition to
  JSON; invalid files report both parse errors instead of a cryptic one
- non-interactive mode honors `preferred_project_types` from the config
  file (previously only the dedicated from-config path did)
- dry-run preview no longer claims a `LICENSE` file is created
- interactive feature selection now offers only features the generator
  actually applies (docker / CI / database flavors); unknown features
  print a warning instead of silently vanishing
- the database feature family (database / sqlite / mysql) is only offered
  — and only accepted — for project types where sqlx links: api-server,
  cli-tool and library. embedded (no_std), wasm-app (wasm32 target),
  game-engine (wasm32 target table) and workspace (virtual manifest)
  projects get an explicit error instead of a project that cannot build
- feature-plugin dependencies land inside the generated manifest's
  `[dependencies]` table (previously they were appended after the last
  table, which for game-engine/embedded/cli-tool/library/wasm-app meant
  they ended up under `[profile.*]`/`[target.*]`/`[[bin]]`/`[lib]`)
- the api-server `.env.example` now documents what actually takes effect
  (the `PORT` environment variable) instead of describing a dotenv
  workflow and config-override relationship the starter does not have

### Changed
- Templates generate working starter code instead of empty stubs:
  api-server ships a running Axum server with a `GET /` health check,
  cli-tool a Clap skeleton with a `hello` command and an integration
  test, library a `greet` API with a unit test, wasm-app a
  wasm-bindgen export matching its documented wasm-pack flow,
  game-engine a minimal Bevy app
- game-engine: bevy 0.12 -> 0.20 (the old pin no longer compiled with
  current transitive dependencies)
- Empty scaffolding removed from wasm-app (webpack/package.json/
  build.sh) and game-engine (empty wasm.yml workflow)
- devcontainer.json is now a cargo-forge dev container (Rust + SSHD
  features) instead of a personal agent workspace; it no longer pins
  Rust 1.70, which could not build the current dependency tree

### Removed
- Dead Tera template engine: `src/templates`, the `templates/` tree,
  `tera`/`include_dir` dependencies, and `docs/TEMPLATE_SYNTAX.md` —
  generation has always been code-driven, not template-file-driven
- Test files that exercised mock structs defined inside the tests
  themselves rather than the product
- `install.sh` — it downloaded release binaries that were never published
  (the repository has no release assets; `cargo install cargo-forge` is the
  supported install path)

## [0.1.5] - 2025-11-26
### Added
- ESP32 support in interactive mode with separate options for ESP32 and Cortex-M based chips
- Enhanced project generation capabilities for embedded systems
- Devcontainer configuration for Claude workspace integration
- Comprehensive development environment setup

### Changed
- Improved embedded project templates with better target selection
- Updated dependency management for better compatibility

## [0.1.4] - 2025-11-26
### Fixed
- Internal development environment improvements
- Updated development tooling and documentation

## [0.1.3] - 2025-07-03
### Fixed
- Fixed all command examples to use correct `cargo-forge` syntax instead of `cargo forge`
- Updated installation documentation for consistency
- Ensured all documentation uses the proper binary name format

## [0.1.2] - 2025-07-03
### Fixed
- Removed marketing language and fluff from all documentation
- Removed placeholder "coming soon" references
- Simplified descriptions to be factual and accurate
- Updated Cargo.toml description to match simplified language

## [0.1.1] - 2025-07-03
### Fixed
- Corrected all "cargo forge" references to "cargo-forge" for consistency
- Updated all repository URLs to correct GitHub path
- Documentation accuracy improvements to ensure proper crate installation and usage

## [0.1.0] - 2025-01-03
### Added
- Initial release of cargo-forge
- Interactive project creation with beautiful TUI
- Support for 7 project types:
  - CLI Tool
  - Library
  - API Server
  - WebAssembly Application
  - Game Engine
  - Embedded System
  - Workspace
- Optional features:
  - CI/CD integration (GitHub Actions, GitLab CI)
  - Database support (PostgreSQL, MySQL, SQLite)
  - Authentication (JWT, OAuth, Password)
  - Docker support
  - Testing frameworks
  - Documentation templates
- Non-interactive mode for CI environments
- Dry-run mode for previewing project structure
- Configuration file support
- Shell completions for bash, zsh, and fish
- Comprehensive test suite with 45.84% coverage
- End-to-end testing framework
- Professional documentation

### Changed
- N/A (initial release)

### Deprecated
- N/A (initial release)

### Removed
- N/A (initial release)

### Fixed
- N/A (initial release)

### Security
- N/A (initial release)

## Version History

### [0.1.1] - 2025-07-03
- Documentation fixes and repository URL corrections

### [0.1.0] - 2025-01-03
- Initial release

[Unreleased]: https://github.com/marcuspat/cargo-forge/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/marcuspat/cargo-forge/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/marcuspat/cargo-forge/releases/tag/v0.1.0