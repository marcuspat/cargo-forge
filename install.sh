#!/bin/sh
# Cargo-Forge Installation Script
# This script downloads and installs the latest version of cargo-forge

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
REPO="marcuspat/cargo-forge"
INSTALL_DIR="${CARGO_HOME:-$HOME/.cargo}/bin"
BINARY_NAME="cargo-forge"

# Detect OS and architecture
detect_platform() {
  OS=$(uname -s | tr '[:upper:]' '[:lower:]')
  ARCH=$(uname -m)

  case "$OS" in
  linux)
    case "$ARCH" in
    x86_64) PLATFORM="x86_64-linux" ;;
    aarch64) PLATFORM="aarch64-linux" ;;
    *)
      echo "${RED}Unsupported architecture: $ARCH${NC}"
      exit 1
      ;;
    esac
    ;;
  darwin)
    case "$ARCH" in
    x86_64) PLATFORM="x86_64-macos" ;;
    arm64) PLATFORM="aarch64-macos" ;;
    *)
      echo "${RED}Unsupported architecture: $ARCH${NC}"
      exit 1
      ;;
    esac
    ;;
  *)
    echo "${RED}Unsupported OS: $OS${NC}"
    exit 1
    ;;
  esac
}

# Get latest release version
get_latest_version() {
  curl -s "https://api.github.com/repos/$REPO/releases/latest" |
    grep '"tag_name":' |
    sed -E 's/.*"([^"]+)".*/\1/'
}

# Download and install
install_cargo_forge() {
  echo "${GREEN}Installing cargo-forge...${NC}"

  # Detect platform
  detect_platform
  echo "Detected platform: $PLATFORM"

  # Get latest version
  VERSION=$(get_latest_version)
  if [ -z "$VERSION" ]; then
    echo "${RED}Failed to get latest version${NC}"
    exit 1
  fi
  echo "Latest version: $VERSION"

  # Construct download URL
  DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/cargo-forge-$PLATFORM.tar.gz"
  echo "Download URL: $DOWNLOAD_URL"

  # Create temporary directory
  TMP_DIR=$(mktemp -d)
  cd "$TMP_DIR"

  # Download binary
  echo "Downloading cargo-forge..."
  if ! curl -sL "$DOWNLOAD_URL" -o cargo-forge.tar.gz; then
    echo "${RED}Failed to download cargo-forge${NC}"
    exit 1
  fi

  # Extract binary
  echo "Extracting..."
  tar xzf cargo-forge.tar.gz

  # Create install directory if it doesn't exist
  mkdir -p "$INSTALL_DIR"

  # Install binary
  echo "Installing to $INSTALL_DIR..."
  if ! mv cargo-forge "$INSTALL_DIR/"; then
    echo "${RED}Failed to install cargo-forge. Try running with sudo or check permissions.${NC}"
    exit 1
  fi

  # Make executable
  chmod +x "$INSTALL_DIR/$BINARY_NAME"

  # Cleanup
  cd ..
  rm -rf "$TMP_DIR"

  # Verify installation
  if command -v cargo-forge >/dev/null 2>&1; then
    echo "${GREEN}✓ cargo-forge installed successfully!${NC}"
    echo "Version: $(cargo-forge --version)"
    echo ""
    echo "To get started, run:"
    echo "  ${YELLOW}cargo-forge new${NC}"
    echo ""
    echo "For shell completions, see:"
    echo "  ${YELLOW}cargo-forge completions --help${NC}"
  else
    echo "${YELLOW}cargo-forge installed to $INSTALL_DIR${NC}"
    echo "Make sure $INSTALL_DIR is in your PATH:"
    echo "  ${YELLOW}export PATH=\"$INSTALL_DIR:\$PATH\"${NC}"
  fi
}

# Main
main() {
  echo "${GREEN}⚒️  Cargo-Forge Installer${NC}"
  echo ""

  # Check for curl
  if ! command -v curl >/dev/null 2>&1; then
    echo "${RED}Error: curl is required but not installed.${NC}"
    echo "Please install curl and try again."
    exit 1
  fi

  # Run installation
  install_cargo_forge
}

# Run main function
main

