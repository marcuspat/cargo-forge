# Release Checklist for Cargo Forge

This checklist ensures a smooth release process for cargo-forge. Follow these steps in order.

## Pre-Release Preparation

### Code Quality
- [ ] All tests pass: `cargo test --all-features`
- [ ] Test coverage is acceptable (current: 45.84%): `cargo tarpaulin`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Code is formatted: `cargo fmt --check`
- [ ] Documentation builds: `cargo doc --no-deps`
- [ ] E2E tests pass: `cargo test --test e2e_tests`

### Version Management
- [ ] Update version in `Cargo.toml`
- [ ] Update version in README.md badges
- [ ] Update CHANGELOG.md with release date
- [ ] Move "Unreleased" items to new version section
- [ ] Create new "Unreleased" section in CHANGELOG

### Documentation Review
- [ ] README.md is up to date
- [ ] INSTALLATION.md reflects current process
- [ ] API documentation is complete
- [ ] All examples work correctly
- [ ] Shell completion instructions are accurate

### Dependency Audit
- [ ] Run `cargo audit`
- [ ] Update dependencies if needed: `cargo update`
- [ ] Check for security advisories
- [ ] Verify minimum Rust version (1.70.0)

## Release Process

### Local Verification
- [ ] Clean build: `cargo clean && cargo build --release`
- [ ] Test installation: `cargo install --path .`
- [ ] Verify all subcommands work:
  - [ ] `cargo-forge new --dry-run`
  - [ ] `cargo-forge init --dry-run`
  - [ ] `cargo-forge completions bash`
- [ ] Test shell completions generation

### Git Preparation
- [ ] All changes committed
- [ ] Branch is up to date with main: `git pull origin main`
- [ ] No uncommitted files: `git status`
- [ ] Create release branch: `git checkout -b release-v0.1.0`

### Crates.io Verification
- [ ] Login to crates.io: `cargo login`
- [ ] Dry run publish: `cargo publish --dry-run`
- [ ] Verify package size is reasonable
- [ ] Check included/excluded files

### GitHub Release Preparation
- [ ] Ensure GitHub Actions secrets are set:
  - [ ] `CARGO_REGISTRY_TOKEN` for crates.io
  - [ ] Default `GITHUB_TOKEN` is available
- [ ] Verify release workflow is correct
- [ ] Check binary build matrix covers all platforms

## Release Execution

### Using cargo-release (Recommended)
```bash
# Dry run first
cargo release --dry-run

# Actual release
cargo release --execute
```

### Manual Release Process (Alternative)
1. [ ] Create and push version tag:
   ```bash
   git tag -a v0.1.0 -m "Release version 0.1.0"
   git push origin v0.1.0
   ```

2. [ ] Wait for GitHub Actions to:
   - [ ] Create GitHub release
   - [ ] Build and upload binaries
   - [ ] Publish to crates.io
   - [ ] Generate shell completions

3. [ ] Monitor the release workflow

## Post-Release Verification

### Crates.io
- [ ] Package appears on crates.io
- [ ] Documentation builds on docs.rs
- [ ] Installation works: `cargo install cargo-forge`
- [ ] Verify installed version: `cargo-forge --version`

### GitHub Release
- [ ] Release appears on GitHub
- [ ] All binaries are uploaded:
  - [ ] Linux x86_64
  - [ ] Linux ARM64
  - [ ] macOS x86_64
  - [ ] macOS ARM64
  - [ ] Windows x86_64
- [ ] Shell completions archive is attached
- [ ] Release notes are properly formatted

### Binary Testing
- [ ] Download and test at least one binary per OS
- [ ] Verify binary version matches release
- [ ] Test basic functionality

### Documentation
- [ ] Update website/landing page (if applicable)
- [ ] Tweet/announce the release
- [ ] Update Discord/community channels
- [ ] Create blog post for major releases

## Rollback Plan

If issues are discovered:

1. [ ] Delete the release on GitHub (keeps tag)
2. [ ] Yank from crates.io if critical: `cargo yank --vers 0.1.0`
3. [ ] Fix issues on a hotfix branch
4. [ ] Create patch release (e.g., 0.1.1)
5. [ ] Document lessons learned

## Future Releases

After successful release:
- [ ] Create milestone for next version
- [ ] Update project board
- [ ] Plan features for next release
- [ ] Update development version in Cargo.toml

## Release Communication Template

```markdown
🎉 Cargo Forge v0.1.0 Released!

We're excited to announce the first release of Cargo Forge, a powerful Rust project generator.

✨ Highlights:
- Interactive project creation
- 7 specialized project types
- Smart feature detection
- Shell completions
- Beautiful TUI

📦 Install: `cargo install cargo-forge`
📖 Docs: https://docs.rs/cargo-forge
🐛 Issues: https://github.com/yourusername/cargo-forge/issues

Full changelog: https://github.com/yourusername/cargo-forge/blob/main/CHANGELOG.md
```

## Notes

- Always do a dry run before actual release
- Keep credentials secure and use GitHub secrets
- Test on multiple platforms when possible
- Document any deviations from this process
- Update this checklist based on lessons learned