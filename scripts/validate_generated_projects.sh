#!/bin/bash

# Project Generation Validation Script
# This script generates all project types and validates they compile successfully

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

print_success() {
    print_status "$GREEN" "✅ $1"
}

print_error() {
    print_status "$RED" "❌ $1"
}

print_warning() {
    print_status "$YELLOW" "⚠️  $1"
}

print_info() {
    print_status "$BLUE" "ℹ️  $1"
}

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to validate a generated project
validate_project() {
    local project_type=$1
    local project_name="validation-${project_type}"
    local temp_dir="$2"
    
    print_info "Validating ${project_type} project generation..."
    
    # Change to temp directory
    cd "$temp_dir"
    
    # Generate project
    if ! cargo-forge new "$project_name" \
        --project-type "$project_type" \
        --author "Validation Test <validation@test.com>" \
        --description "Generated for validation testing" \
        --non-interactive; then
        print_error "Failed to generate $project_type project"
        return 1
    fi
    
    # Change to project directory
    cd "$project_name"
    
    # Validate basic structure
    print_info "Checking basic project structure..."
    if [[ ! -f "Cargo.toml" ]]; then
        print_error "Missing Cargo.toml"
        return 1
    fi
    
    if [[ ! -f "README.md" ]]; then
        print_error "Missing README.md"
        return 1
    fi
    
    if [[ ! -f ".gitignore" ]]; then
        print_error "Missing .gitignore"
        return 1
    fi
    
    # Validate Cargo.toml content
    print_info "Validating Cargo.toml content..."
    if ! grep -q "name = \"$project_name\"" Cargo.toml; then
        print_error "Project name not found in Cargo.toml"
        return 1
    fi
    
    if ! grep -q "version = \"0.1.0\"" Cargo.toml; then
        print_error "Version not found in Cargo.toml"
        return 1
    fi
    
    if ! grep -q "edition = \"2021\"" Cargo.toml; then
        print_error "Edition not found in Cargo.toml"
        return 1
    fi
    
    # Validate README content
    print_info "Validating README.md content..."
    if ! grep -q "# $project_name" README.md; then
        print_error "Project name not found in README.md"
        return 1
    fi
    
    if ! grep -q "Generated for validation testing" README.md; then
        print_error "Description not found in README.md"
        return 1
    fi
    
    # Project-specific validations
    case "$project_type" in
        "cli-tool")
            if [[ ! -f "src/main.rs" ]]; then
                print_error "CLI tool missing src/main.rs"
                return 1
            fi
            if [[ ! -f "src/cli.rs" ]]; then
                print_error "CLI tool missing src/cli.rs"
                return 1
            fi
            if [[ ! -f "src/commands.rs" ]]; then
                print_error "CLI tool missing src/commands.rs"
                return 1
            fi
            if ! grep -q "clap" Cargo.toml; then
                print_error "CLI tool missing clap dependency"
                return 1
            fi
            ;;
        "library")
            if [[ ! -f "src/lib.rs" ]]; then
                print_error "Library missing src/lib.rs"
                return 1
            fi
            if [[ ! -d "examples" ]]; then
                print_error "Library missing examples directory"
                return 1
            fi
            if [[ ! -f "examples/basic.rs" ]]; then
                print_error "Library missing basic example"
                return 1
            fi
            ;;
        "api-server")
            if [[ ! -f "src/main.rs" ]]; then
                print_error "API server missing src/main.rs"
                return 1
            fi
            if [[ ! -f "src/routes.rs" ]]; then
                print_error "API server missing src/routes.rs"
                return 1
            fi
            if [[ ! -f "src/handlers.rs" ]]; then
                print_error "API server missing src/handlers.rs"
                return 1
            fi
            if ! grep -q "axum" Cargo.toml; then
                print_error "API server missing axum dependency"
                return 1
            fi
            ;;
        "wasm-app")
            if [[ ! -f "src/lib.rs" ]]; then
                print_error "WASM app missing src/lib.rs"
                return 1
            fi
            if [[ ! -f "index.html" ]]; then
                print_error "WASM app missing index.html"
                return 1
            fi
            if [[ ! -f "package.json" ]]; then
                print_error "WASM app missing package.json"
                return 1
            fi
            if ! grep -q "wasm-bindgen" Cargo.toml; then
                print_error "WASM app missing wasm-bindgen dependency"
                return 1
            fi
            ;;
        "game-engine")
            if [[ ! -f "src/main.rs" ]]; then
                print_error "Game engine missing src/main.rs"
                return 1
            fi
            if [[ ! -d "assets" ]]; then
                print_error "Game engine missing assets directory"
                return 1
            fi
            if ! grep -q "bevy" Cargo.toml; then
                print_error "Game engine missing bevy dependency"
                return 1
            fi
            ;;
        "embedded")
            if [[ ! -f "src/main.rs" ]]; then
                print_error "Embedded project missing src/main.rs"
                return 1
            fi
            if [[ ! -f "memory.x" ]]; then
                print_error "Embedded project missing memory.x"
                return 1
            fi
            if [[ ! -f ".cargo/config.toml" ]]; then
                print_error "Embedded project missing .cargo/config.toml"
                return 1
            fi
            if ! grep -q "cortex-m" Cargo.toml; then
                print_error "Embedded project missing cortex-m dependency"
                return 1
            fi
            ;;
        "workspace")
            if [[ ! -d "crates" ]]; then
                print_error "Workspace missing crates directory"
                return 1
            fi
            if [[ ! -d "crates/core" ]]; then
                print_error "Workspace missing core crate"
                return 1
            fi
            if [[ ! -d "crates/api" ]]; then
                print_error "Workspace missing api crate"
                return 1
            fi
            if [[ ! -d "crates/cli" ]]; then
                print_error "Workspace missing cli crate"
                return 1
            fi
            ;;
    esac
    
    # Test compilation
    print_info "Testing compilation with cargo check..."
    if ! cargo check --quiet; then
        print_error "Project failed cargo check"
        return 1
    fi
    
    # Test build for non-embedded/non-wasm projects
    if [[ "$project_type" != "embedded" && "$project_type" != "wasm-app" ]]; then
        print_info "Testing build with cargo build..."
        if ! cargo build --quiet; then
            print_error "Project failed cargo build"
            return 1
        fi
    fi
    
    # Test unit tests for applicable projects
    if [[ "$project_type" != "embedded" && "$project_type" != "wasm-app" ]]; then
        print_info "Testing with cargo test..."
        if ! cargo test --quiet; then
            print_error "Project failed cargo test"
            return 1
        fi
    fi
    
    print_success "$project_type project validation completed successfully"
    return 0
}

# Main execution
main() {
    print_info "Starting project generation validation..."
    
    # Check prerequisites
    if ! command_exists cargo-forge; then
        print_error "cargo-forge not found in PATH. Please build and install it first."
        exit 1
    fi
    
    if ! command_exists cargo; then
        print_error "cargo not found in PATH. Please install Rust."
        exit 1
    fi
    
    # Create temporary directory
    TEMP_DIR=$(mktemp -d)
    trap "rm -rf $TEMP_DIR" EXIT
    
    print_info "Using temporary directory: $TEMP_DIR"
    
    # List of project types to test
    PROJECT_TYPES=(
        "cli-tool"
        "library"
        "api-server"
        "wasm-app"
        "game-engine"
        "embedded"
        "workspace"
    )
    
    # Track results
    SUCCESSFUL=0
    FAILED=0
    FAILED_PROJECTS=()
    
    # Validate each project type
    for project_type in "${PROJECT_TYPES[@]}"; do
        if validate_project "$project_type" "$TEMP_DIR"; then
            ((SUCCESSFUL++))
        else
            ((FAILED++))
            FAILED_PROJECTS+=("$project_type")
        fi
        echo # Add blank line for readability
    done
    
    # Print summary
    echo "===========================================" 
    print_info "VALIDATION SUMMARY"
    echo "==========================================="
    print_success "Successful validations: $SUCCESSFUL"
    
    if [[ $FAILED -gt 0 ]]; then
        print_error "Failed validations: $FAILED"
        print_error "Failed project types: ${FAILED_PROJECTS[*]}"
        exit 1
    else
        print_success "All project types validated successfully!"
        exit 0
    fi
}

# Show help if requested
if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
    echo "Project Generation Validation Script"
    echo ""
    echo "This script validates that cargo-forge can generate all project types"
    echo "and that the generated projects compile successfully."
    echo ""
    echo "Prerequisites:"
    echo "  - cargo-forge must be built and available in PATH"
    echo "  - Rust toolchain must be installed"
    echo ""
    echo "Usage:"
    echo "  $0              # Run validation for all project types"
    echo "  $0 --help       # Show this help message"
    echo ""
    exit 0
fi

# Run main function
main "$@"