# Installation Guide

## Table of Contents
- [Installing via Cargo](#installing-via-cargo)
- [Installing Pre-built Binaries](#installing-pre-built-binaries)
- [Building from Source](#building-from-source)
- [Shell Completions](#shell-completions)
- [Verifying Installation](#verifying-installation)
- [Updating](#updating)
- [Uninstalling](#uninstalling)

## Installing via Cargo

The easiest way to install `cargo-forge` is through cargo:

```bash
cargo install cargo-forge
```

This will:
- Download the latest version from crates.io
- Compile it with optimizations
- Install the binary to `~/.cargo/bin/`
- Make it available as both `cargo-forge` and `cargo forge`

### Requirements
- Rust 1.70.0 or later
- Cargo (comes with Rust)

## Installing Pre-built Binaries

Pre-compiled binaries are available for major platforms:

### Automatic Installation (Unix-like systems)

```bash
curl -sSf https://raw.githubusercontent.com/yourusername/cargo-forge/main/install.sh | sh
```

### Manual Installation

1. Download the appropriate binary from the [latest release](https://github.com/yourusername/cargo-forge/releases/latest):
   - Linux x86_64: `cargo-forge-x86_64-linux.tar.gz`
   - Linux ARM64: `cargo-forge-aarch64-linux.tar.gz`
   - macOS x86_64 (Intel): `cargo-forge-x86_64-macos.tar.gz`
   - macOS ARM64 (Apple Silicon): `cargo-forge-aarch64-macos.tar.gz`
   - Windows x86_64: `cargo-forge-x86_64-windows.zip`

2. Extract the archive:
   ```bash
   # Unix-like systems
   tar xzf cargo-forge-*.tar.gz
   
   # Windows (PowerShell)
   Expand-Archive cargo-forge-*.zip
   ```

3. Move the binary to a directory in your PATH:
   ```bash
   # Unix-like systems
   sudo mv cargo-forge /usr/local/bin/
   
   # Or to user directory (no sudo required)
   mkdir -p ~/.local/bin
   mv cargo-forge ~/.local/bin/
   # Add ~/.local/bin to PATH if not already there
   ```

## Building from Source

To build from the latest development version:

```bash
# Clone the repository
git clone https://github.com/yourusername/cargo-forge.git
cd cargo-forge

# Build and install
cargo install --path .
```

For development:
```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run directly without installing
cargo run -- new my-project
```

## Shell Completions

Cargo Forge supports shell completions for bash, zsh, and fish.

### Bash

Add to your `~/.bashrc` or `~/.bash_profile`:

```bash
# Cargo Forge completions
if command -v cargo-forge &> /dev/null; then
    eval "$(cargo-forge completions bash)"
fi
```

### Zsh

Add to your `~/.zshrc`:

```zsh
# Cargo Forge completions
if command -v cargo-forge &> /dev/null; then
    eval "$(cargo-forge completions zsh)"
fi
```

### Fish

Add to your `~/.config/fish/config.fish`:

```fish
# Cargo Forge completions
if command -v cargo-forge > /dev/null
    cargo-forge completions fish | source
end
```

### Alternative: Installing Completion Files

You can also install completion files manually:

```bash
# Download completions archive from release
wget https://github.com/yourusername/cargo-forge/releases/latest/download/shell-completions.tar.gz
tar xzf shell-completions.tar.gz

# Bash (requires bash-completion package)
sudo cp completions/cargo-forge.bash /usr/share/bash-completion/completions/cargo-forge

# Zsh
sudo cp completions/_cargo-forge /usr/share/zsh/site-functions/_cargo-forge

# Fish
cp completions/cargo-forge.fish ~/.config/fish/completions/cargo-forge.fish
```

After installation, restart your shell or source your configuration file:
```bash
# Bash
source ~/.bashrc

# Zsh
source ~/.zshrc

# Fish
source ~/.config/fish/config.fish
```

## Verifying Installation

After installation, verify that cargo-forge is working:

```bash
# Check version
cargo-forge --version

# Or using cargo subcommand syntax
cargo forge --version

# View help
cargo-forge --help

# Test shell completions (type and press TAB)
cargo-forge <TAB>
```

## Updating

### Via Cargo

```bash
cargo install cargo-forge --force
```

### Via Pre-built Binaries

Download and install the latest binary following the same steps as initial installation.

### Update Notifications

Cargo Forge will notify you when a new version is available (can be disabled in config).

## Uninstalling

### Installed via Cargo

```bash
cargo uninstall cargo-forge
```

### Installed via Binary

```bash
# Remove the binary
sudo rm /usr/local/bin/cargo-forge
# or
rm ~/.local/bin/cargo-forge

# Remove shell completions if installed manually
sudo rm /usr/share/bash-completion/completions/cargo-forge
sudo rm /usr/share/zsh/site-functions/_cargo-forge
rm ~/.config/fish/completions/cargo-forge.fish
```

### Remove Configuration and Cache

```bash
# Remove configuration files
rm -rf ~/.config/cargo-forge

# Remove any cached templates
rm -rf ~/.cache/cargo-forge
```

## Troubleshooting

### Command Not Found

If `cargo-forge` is not found after installation:

1. Ensure `~/.cargo/bin` is in your PATH:
   ```bash
   echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

2. For binary installations, verify the installation directory is in PATH:
   ```bash
   which cargo-forge
   echo $PATH
   ```

### Permission Denied

If you get permission errors:

1. For cargo installation, ensure you own the cargo directories:
   ```bash
   sudo chown -R $(whoami) ~/.cargo
   ```

2. For binary installation, use a user directory instead of system directories:
   ```bash
   mkdir -p ~/.local/bin
   # Add to PATH in your shell configuration
   ```

### Completions Not Working

1. Ensure your shell has completion support enabled
2. For bash, install `bash-completion` package:
   ```bash
   # Debian/Ubuntu
   sudo apt-get install bash-completion
   
   # macOS
   brew install bash-completion
   ```

3. Restart your shell after adding completions

### Building from Source Fails

1. Update Rust to the latest stable version:
   ```bash
   rustup update stable
   ```

2. Clear cargo cache if needed:
   ```bash
   cargo clean
   rm -rf ~/.cargo/registry/cache
   ```

## Platform-Specific Notes

### macOS

- If using Homebrew, ensure `/usr/local/bin` is in your PATH
- For Apple Silicon Macs, use the `aarch64-apple-darwin` binary

### Windows

- Add cargo's bin directory to your system PATH:
  - Default location: `%USERPROFILE%\.cargo\bin`
  - System Settings → Environment Variables → Path
- Use PowerShell or Command Prompt to run cargo-forge
- For completions, consider using PowerShell's tab completion

### Linux

- Some distributions may require additional development packages:
  ```bash
  # Debian/Ubuntu
  sudo apt-get install build-essential pkg-config libssl-dev
  
  # Fedora
  sudo dnf install gcc openssl-devel
  
  # Arch
  sudo pacman -S base-devel openssl
  ```

## Getting Help

If you encounter any issues:

1. Check the [FAQ](docs/FAQ.md)
2. Search [existing issues](https://github.com/yourusername/cargo-forge/issues)
3. Join our [Discord community](https://discord.gg/rust-forge)
4. Create a new issue with:
   - Your OS and version
   - Installation method used
   - Complete error message
   - Output of `cargo --version` and `rustc --version`