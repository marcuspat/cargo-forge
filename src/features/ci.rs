use crate::{ProjectContext, Plugin};
use std::error::Error;

#[derive(Debug, Clone, Copy)]
pub enum CIPlatform {
    GitHubActions,
    GitLabCI,
    Both,
}

pub struct CIPlugin {
    platform: CIPlatform,
    with_coverage: bool,
    with_release: bool,
    with_security_audit: bool,
}

impl CIPlugin {
    pub fn new(platform: CIPlatform) -> Self {
        Self {
            platform,
            with_coverage: true,
            with_release: true,
            with_security_audit: true,
        }
    }
    
    pub fn with_coverage(mut self, enabled: bool) -> Self {
        self.with_coverage = enabled;
        self
    }
    
    pub fn with_release(mut self, enabled: bool) -> Self {
        self.with_release = enabled;
        self
    }
    
    pub fn with_security_audit(mut self, enabled: bool) -> Self {
        self.with_security_audit = enabled;
        self
    }
    
    fn generate_github_actions_ci(&self) -> String {
        let mut workflow = r#"name: CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    name: Test
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, beta, nightly]
        
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@master
      with:
        toolchain: ${{ matrix.rust }}
        components: rustfmt, clippy
    
    - name: Cache cargo registry
      uses: actions/cache@v3
      with:
        path: ~/.cargo/registry
        key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Cache cargo index
      uses: actions/cache@v3
      with:
        path: ~/.cargo/git
        key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Cache cargo build
      uses: actions/cache@v3
      with:
        path: target
        key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Check formatting
      run: cargo fmt -- --check
      
    - name: Run clippy
      run: cargo clippy -- -D warnings
      
    - name: Build
      run: cargo build --verbose
      
    - name: Run tests
      run: cargo test --verbose"#.to_string();
      
        if self.with_coverage {
            workflow.push_str(r#"

  coverage:
    name: Code Coverage
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
      
    - name: Install tarpaulin
      run: cargo install cargo-tarpaulin
      
    - name: Generate code coverage
      run: cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out xml
      
    - name: Upload coverage to Codecov
      uses: codecov/codecov-action@v3
      with:
        file: ./cobertura.xml
        fail_ci_if_error: true"#);
        }
        
        if self.with_security_audit {
            workflow.push_str(r#"

  security_audit:
    name: Security Audit
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    
    - name: Run security audit
      uses: actions-rs/audit-check@v1
      with:
        token: ${{ secrets.GITHUB_TOKEN }}"#);
        }
        
        if self.with_release {
            workflow.push_str(r#"

  release:
    name: Release
    needs: [test]
    runs-on: ubuntu-latest
    if: startsWith(github.ref, 'refs/tags/')
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
      
    - name: Build release
      run: cargo build --release
      
    - name: Create GitHub Release
      uses: softprops/action-gh-release@v1
      with:
        files: target/release/${{ github.event.repository.name }}
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}"#);
        }
        
        workflow
    }
    
    fn generate_gitlab_ci(&self) -> String {
        let mut ci = r#"stages:
  - test
  - build
  - deploy

variables:
  CARGO_HOME: $CI_PROJECT_DIR/.cargo

cache:
  paths:
    - .cargo/
    - target/

test:cargo:
  stage: test
  image: rust:latest
  script:
    - rustc --version && cargo --version
    - cargo fmt -- --check
    - cargo clippy -- -D warnings
    - cargo test --verbose
  only:
    - branches
    - merge_requests"#.to_string();
    
        if self.with_coverage {
            ci.push_str(r#"

coverage:
  stage: test
  image: rust:latest
  script:
    - apt-get update && apt-get install -y pkg-config libssl-dev
    - cargo install cargo-tarpaulin
    - cargo tarpaulin --out Xml
  coverage: '/^\d+.\d+% coverage/'
  artifacts:
    reports:
      coverage_report:
        coverage_format: cobertura
        path: cobertura.xml
  only:
    - main
    - merge_requests"#);
        }
        
        if self.with_security_audit {
            ci.push_str(r#"

security:
  stage: test
  image: rust:latest
  script:
    - cargo install cargo-audit
    - cargo audit
  allow_failure: true
  only:
    - branches
    - merge_requests"#);
        }
        
        ci.push_str(r#"

build:
  stage: build
  image: rust:latest
  script:
    - cargo build --release
  artifacts:
    paths:
      - target/release/
    expire_in: 1 week
  only:
    - main
    - tags"#);
    
        if self.with_release {
            ci.push_str(r#"

release:
  stage: deploy
  image: registry.gitlab.com/gitlab-org/release-cli:latest
  script:
    - echo "Creating release for $CI_COMMIT_TAG"
  release:
    tag_name: $CI_COMMIT_TAG
    description: 'Release $CI_COMMIT_TAG'
    assets:
      links:
        - name: 'Binary'
          url: '$CI_PROJECT_URL/-/jobs/$CI_JOB_ID/artifacts/file/target/release/$CI_PROJECT_NAME'
  only:
    - tags"#);
        }
        
        ci
    }
}

impl Plugin for CIPlugin {
    fn name(&self) -> &str {
        "CI/CD"
    }
    
    fn configure(&self, context: &mut ProjectContext) -> Result<(), Box<dyn Error>> {
        match self.platform {
            CIPlatform::GitHubActions => {
                context.create_directory(".github/workflows");
                context.add_template_file(
                    ".github/workflows/ci.yml",
                    self.generate_github_actions_ci()
                );
            }
            CIPlatform::GitLabCI => {
                context.add_template_file(
                    ".gitlab-ci.yml",
                    self.generate_gitlab_ci()
                );
            }
            CIPlatform::Both => {
                context.create_directory(".github/workflows");
                context.add_template_file(
                    ".github/workflows/ci.yml",
                    self.generate_github_actions_ci()
                );
                context.add_template_file(
                    ".gitlab-ci.yml",
                    self.generate_gitlab_ci()
                );
            }
        }
        
        if self.with_coverage {
            context.add_to_gitignore("cobertura.xml");
            context.add_to_gitignore("tarpaulin-report.html");
            context.add_to_gitignore("coverage/");
        }
        
        let readme_section = match self.platform {
            CIPlatform::GitHubActions => r#"
## CI/CD

This project uses GitHub Actions for continuous integration.

[![CI](https://github.com/USERNAME/REPO/actions/workflows/ci.yml/badge.svg)](https://github.com/USERNAME/REPO/actions/workflows/ci.yml)

The CI pipeline runs:
- Tests on multiple OS (Ubuntu, Windows, macOS) and Rust versions
- Code formatting checks (rustfmt)
- Linting (clippy)"#,
            CIPlatform::GitLabCI => r#"
## CI/CD

This project uses GitLab CI for continuous integration.

The CI pipeline runs:
- Tests
- Code formatting checks (rustfmt)
- Linting (clippy)"#,
            CIPlatform::Both => r#"
## CI/CD

This project supports both GitHub Actions and GitLab CI for continuous integration.

### GitHub Actions
[![CI](https://github.com/USERNAME/REPO/actions/workflows/ci.yml/badge.svg)](https://github.com/USERNAME/REPO/actions/workflows/ci.yml)

### GitLab CI
The pipeline configuration is in `.gitlab-ci.yml`

Both pipelines run:
- Tests
- Code formatting checks (rustfmt)
- Linting (clippy)"#,
        };
        
        let mut full_readme = readme_section.to_string();
        
        if self.with_coverage {
            full_readme.push_str("\n- Code coverage reporting");
        }
        
        if self.with_security_audit {
            full_readme.push_str("\n- Security vulnerability scanning");
        }
        
        if self.with_release {
            full_readme.push_str("\n- Automatic releases on tags");
        }
        
        context.add_to_readme(&full_readme);
        
        Ok(())
    }
}