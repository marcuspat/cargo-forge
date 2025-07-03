# Cargo Forge Troubleshooting Guide

This guide helps you resolve common issues when using Cargo Forge.

## Table of Contents

- [Installation Issues](#installation-issues)
- [Project Generation Errors](#project-generation-errors)
- [Template Rendering Problems](#template-rendering-problems)
- [Feature Compatibility Issues](#feature-compatibility-issues)
- [Platform-Specific Problems](#platform-specific-problems)
- [Performance Issues](#performance-issues)
- [CI/CD Problems](#cicd-problems)
- [Getting Help](#getting-help)

## Installation Issues

### cargo-forge: command not found

**Problem**: After installing with `cargo install cargo-forge`, the command isn't recognized.

**Solution**:
```bash
# Check if cargo bin directory is in PATH
echo $PATH | grep -q ".cargo/bin" || echo "Cargo bin not in PATH"

# Add to PATH (add to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.cargo/bin:$PATH"

# Verify installation
which cargo-forge
cargo forge --version
```

### Installation fails with linking errors

**Problem**: Compilation fails during installation.

**Solution**:
```bash
# Install system dependencies (Ubuntu/Debian)
sudo apt-get update
sudo apt-get install pkg-config libssl-dev

# macOS
brew install openssl

# Windows - Install Visual Studio Build Tools
# Download from: https://visualstudio.microsoft.com/downloads/

# Retry installation
cargo install cargo-forge --locked
```

### Permission denied during installation

**Problem**: Cannot write to cargo directory.

**Solution**:
```bash
# Check permissions
ls -la ~/.cargo

# Fix permissions
chmod -R u+w ~/.cargo

# Alternative: Install to custom location
CARGO_INSTALL_ROOT=/custom/path cargo install cargo-forge
```

## Project Generation Errors

### "Failed to create project directory"

**Problem**: Cannot create the project directory.

**Causes & Solutions**:

1. **Directory already exists**
   ```bash
   # Check if directory exists
   ls my-project
   
   # Remove or use different name
   rm -rf my-project  # Careful!
   cargo forge new my-project-v2
   ```

2. **No write permissions**
   ```bash
   # Check current directory permissions
   ls -la .
   
   # Create in writable location
   cd ~/projects
   cargo forge new my-project
   ```

3. **Invalid project name**
   ```bash
   # Project names must be valid Rust identifiers
   # Bad: my-project!, 123project, my project
   # Good: my_project, my-project, project123
   ```

### "Template not found" error

**Problem**: Cargo Forge cannot find template files.

**Solution**:
```bash
# Reinstall to ensure templates are included
cargo install cargo-forge --force

# Check installation integrity
cargo forge --version

# If persists, report issue with:
cargo forge new my-project --verbose
```

### "Failed to render template"

**Problem**: Template rendering fails with Tera errors.

**Common causes**:

1. **Special characters in input**
   ```bash
   # Avoid special characters in:
   # - Project name
   # - Author name (escape quotes)
   # - Description
   ```

2. **Missing required variables**
   ```bash
   # Use defaults or provide all required info
   cargo forge new my-project \
     --author "Your Name" \
     --description "Project description"
   ```

## Template Rendering Problems

### Variables not replaced in generated files

**Problem**: See `{{ variable }}` in generated files.

**Solution**:
1. Check for typos in variable names
2. Ensure using `.tera` extension for templates
3. Verify template syntax:
   ```tera
   {# Correct #}
   {{ project_name }}
   
   {# Incorrect #}
   {{project_name}}    # Missing spaces
   {{ project name }}  # Space in variable name
   ```

### Conditional blocks not working

**Problem**: `{% if %}` blocks appear in output.

**Solution**:
```tera
{# Ensure proper syntax #}
{% if database %}
use sqlx::PgPool;
{% endif %}

{# Not: #}
{% if database = true %}  {# Wrong! #}
```

### Build fails after generation

**Problem**: Generated project doesn't compile.

**Debug steps**:
```bash
# Check generated Cargo.toml
cat my-project/Cargo.toml

# Verify dependencies are available
cd my-project
cargo check

# Update dependencies
cargo update

# Check for version conflicts
cargo tree
```

## Feature Compatibility Issues

### "Incompatible features selected"

**Problem**: Some feature combinations don't work together.

**Common incompatibilities**:
- WASM + Database (WASM can't use system databases)
- Embedded + Docker (Different deployment models)
- No-std + certain async runtimes

**Solution**: Choose compatible features or use workspace for separation.

### Database connection fails

**Problem**: Generated project can't connect to database.

**Solutions**:

1. **PostgreSQL**
   ```bash
   # Ensure PostgreSQL is running
   sudo systemctl status postgresql
   
   # Create database
   createdb my_project_db
   
   # Update .env
   DATABASE_URL=postgresql://user:pass@localhost/my_project_db
   ```

2. **MySQL**
   ```bash
   # Check MySQL service
   sudo systemctl status mysql
   
   # Create database
   mysql -u root -p -e "CREATE DATABASE my_project_db;"
   ```

3. **SQLite**
   ```bash
   # SQLite creates file automatically
   # Just ensure directory is writable
   DATABASE_URL=sqlite://./my_project.db
   ```

### Authentication not working

**Problem**: JWT/OAuth setup fails.

**Solutions**:

1. **Missing JWT secret**
   ```bash
   # Generate secure secret
   openssl rand -hex 32
   
   # Add to .env
   JWT_SECRET=your_generated_secret_here
   ```

2. **OAuth redirect URLs**
   ```bash
   # Update OAuth provider settings with:
   # Callback URL: http://localhost:3000/auth/callback
   ```

## Platform-Specific Problems

### Windows

#### Path separators in generated files

**Problem**: Backslashes in paths cause issues.

**Solution**: Cargo Forge handles this automatically, but if issues persist:
```rust
// Use PathBuf for cross-platform paths
use std::path::PathBuf;
let path = PathBuf::from("assets").join("file.txt");
```

#### Line ending issues

**Problem**: CRLF vs LF conflicts.

**Solution**:
```bash
# Configure git
git config --global core.autocrlf true

# Or use .gitattributes
echo "* text=auto" > .gitattributes
```

### macOS

#### SSL certificate issues

**Problem**: Can't verify SSL certificates.

**Solution**:
```bash
# Update certificates
brew install ca-certificates

# Set environment variable
export SSL_CERT_FILE=/usr/local/etc/openssl/cert.pem
```

### Linux

#### Missing development headers

**Problem**: Compilation fails with missing headers.

**Solution**:
```bash
# Ubuntu/Debian
sudo apt-get install build-essential libssl-dev pkg-config

# Fedora
sudo dnf install gcc openssl-devel

# Arch
sudo pacman -S base-devel openssl
```

## Performance Issues

### Slow project generation

**Problem**: Takes too long to generate project.

**Solutions**:

1. **Disable virus scanner temporarily**
   - Windows Defender can slow file operations

2. **Use faster disk**
   ```bash
   # Generate in RAM disk if available
   cargo forge new /tmp/my-project
   ```

3. **Skip optional features**
   ```bash
   # Minimal generation
   cargo forge new my-project --non-interactive
   ```

### High memory usage

**Problem**: Cargo Forge uses too much memory.

**Solution**:
- Close other applications
- Use `--dry-run` to preview without generation
- Report issue if consistent problem

## CI/CD Problems

### GitHub Actions workflow fails

**Problem**: Generated CI workflow has errors.

**Common fixes**:

1. **Rust version issues**
   ```yaml
   # Update .github/workflows/ci.yml
   - uses: dtolnay/rust-toolchain@stable
     with:
       toolchain: stable  # or specific version
   ```

2. **Missing secrets**
   ```yaml
   # Add to repository settings:
   # - CARGO_REGISTRY_TOKEN
   # - DOCKER_HUB_TOKEN
   ```

### GitLab CI pipeline errors

**Problem**: Pipeline fails with cache or runner issues.

**Solution**:
```yaml
# Update .gitlab-ci.yml
variables:
  CARGO_HOME: ${CI_PROJECT_DIR}/.cargo

cache:
  key: ${CI_COMMIT_REF_SLUG}
  paths:
    - .cargo/
    - target/
```

## Common Error Messages

### "Error: Too many open files"

**Solution**:
```bash
# Increase file limit (temporary)
ulimit -n 4096

# Permanent (add to ~/.bashrc)
echo "ulimit -n 4096" >> ~/.bashrc
```

### "Error: Disk quota exceeded"

**Solution**:
```bash
# Check disk usage
df -h

# Clean cargo cache
cargo clean
rm -rf ~/.cargo/registry/cache
```

### "Error: Version conflict"

**Solution**:
```bash
# Update all dependencies
cargo update

# Or use specific versions
cargo update -p problematic-crate
```

## Debug Mode

Enable verbose output for troubleshooting:

```bash
# Set environment variable
export RUST_LOG=debug
export CARGO_FORGE_DEBUG=1

# Run with verbose flag
cargo forge new my-project --verbose

# Capture full output
cargo forge new my-project --verbose 2>&1 | tee forge-debug.log
```

## Reporting Issues

When reporting issues, include:

1. **System Information**
   ```bash
   cargo forge --version
   rustc --version
   cargo --version
   uname -a  # or OS version
   ```

2. **Complete Error Message**
   ```bash
   cargo forge new test-project 2>&1 | tee error.log
   ```

3. **Steps to Reproduce**
   - Exact commands used
   - Options selected
   - Expected vs actual behavior

4. **Debug Log**
   ```bash
   RUST_LOG=debug cargo forge new test-project 2>&1 > debug.log
   ```

## Getting Help

### Resources

1. **Documentation**
   - README.md
   - [Online Docs](https://docs.rs/cargo-forge)
   - Template examples

2. **Community**
   - [GitHub Discussions](https://github.com/cargo-forge/cargo-forge/discussions)
   - [Discord Server](https://discord.gg/cargo-forge)
   - [Stack Overflow](https://stackoverflow.com/questions/tagged/cargo-forge)

3. **Direct Support**
   - [Issue Tracker](https://github.com/cargo-forge/cargo-forge/issues)
   - Email: support@cargo-forge.rs

### Quick Fixes Checklist

- [ ] Rust toolchain up to date?
- [ ] Cargo Forge latest version?
- [ ] Valid project name?
- [ ] Sufficient disk space?
- [ ] Required system dependencies installed?
- [ ] Environment variables set correctly?
- [ ] File permissions correct?

### Emergency Recovery

If all else fails:

```bash
# Complete reinstall
cargo uninstall cargo-forge
rm -rf ~/.cargo/registry/cache/cargo-forge*
cargo install cargo-forge

# Manual template usage
git clone https://github.com/cargo-forge/templates
cd templates
# Manually copy and customize templates
```

Remember: Most issues have simple solutions. Check the basics first!