# Cargo-Forge - Frequently Asked Questions

## General Questions

### What is Cargo-Forge?

Cargo-Forge is an interactive, intelligent Rust project generator that creates production-ready project structures with best practices, optional features, and comprehensive tooling setup. It's designed to get you from idea to implementation faster than traditional boilerplate approaches.

### How does Cargo-Forge differ from cargo-generate?

While both tools generate Rust projects from templates, Cargo-Forge offers several unique advantages:

- **Interactive TUI**: Beautiful terminal interface for selecting options
- **Intelligent defaults**: Smart suggestions based on project type
- **Validated combinations**: Ensures selected features work together
- **Built-in templates**: No need to find external template repositories
- **Feature integration**: Seamlessly combines multiple features (auth, database, CI)
- **Dry-run mode**: Preview what will be generated before creation

### Is Cargo-Forge free to use?

Yes! Cargo-Forge is completely free and open source under the MIT/Apache-2.0 dual license. There are no paid tiers or premium features - everything is available to everyone.

### What are the system requirements?

- Rust 1.70.0 or later
- Cargo (comes with Rust)
- Git (optional but recommended)
- OS: Windows, macOS, or Linux

## Installation & Setup

### How do I install Cargo-Forge?

```bash
cargo install cargo-forge
```

That's it! The command will be available as `cargo forge`.

### Can I install from source?

Yes, you can build from source:

```bash
git clone https://github.com/marcuspat/cargo-forge
cd cargo-forge
cargo install --path .
```

### How do I update Cargo-Forge?

```bash
cargo install cargo-forge --force
```

### Can I use Cargo-Forge without installing it?

Yes, using cargo-run:

```bash
cargo run --git https://github.com/marcuspat/cargo-forge -- new my-project
```

## Usage Questions

### How do I create a new project?

The simplest way is to run:

```bash
cargo forge new
```

This starts the interactive mode where you can choose all options step by step.

### Can I use Cargo-Forge in CI/CD pipelines?

Yes! Use the `--non-interactive` flag:

```bash
cargo forge new my-project --project-type api-server --non-interactive
```

This uses sensible defaults without prompting for input.

### How do I save my preferences?

Create a configuration file at `~/.config/cargo-forge/config.toml`:

```toml
[defaults]
author = "Your Name"
license = "MIT"
always_add_ci = true
```

### Can I preview what will be generated?

Yes, use the `--dry-run` flag:

```bash
cargo forge new my-project --dry-run
```

This shows what would be created without actually generating files.

### How do I use Cargo-Forge with existing projects?

Use the `init` command in your project directory:

```bash
cd existing-project
cargo forge init --project-type library
```

## Project Types

### Which project type should I choose?

- **cli-tool**: For command-line applications and developer tools
- **library**: For reusable Rust crates
- **api-server**: For REST APIs and web services
- **wasm-app**: For browser-based applications
- **game-engine**: For game development with Bevy
- **embedded**: For microcontroller/IoT projects
- **workspace**: For multi-crate projects

### Can I change project type after generation?

Not automatically, but you can:
1. Generate a new project with the desired type
2. Copy your existing code over
3. Or manually add the required dependencies and structure

### Can I combine multiple project types?

Yes, use the workspace project type to combine multiple crates with different purposes in a single repository.

## Features & Customization

### What features can I add to my project?

Common features include:
- **Authentication**: JWT, OAuth, password-based
- **Database**: PostgreSQL, MySQL, SQLite
- **CI/CD**: GitHub Actions, GitLab CI
- **Docker**: Containerization support
- **Testing**: Property tests, benchmarks, E2E tests

### Can I add features after project creation?

Currently, features must be selected during generation. We're working on a `cargo forge add-feature` command for future releases.

### How do I create custom templates?

While Cargo-Forge uses built-in templates, you can:
1. Fork the repository
2. Modify templates in the `templates/` directory
3. Build and use your custom version

### Can I disable certain files from being generated?

Not directly, but you can:
- Use `--dry-run` to preview
- Delete unwanted files after generation
- Submit a feature request for more granular control

## Technical Questions

### What template engine does Cargo-Forge use?

Cargo-Forge uses [Tera](https://tera.netlify.app/), a Jinja2-like template engine for Rust.

### How are templates packaged?

Templates are embedded in the binary using `include_dir!`, making Cargo-Forge a single self-contained executable.

### Can I use environment variables in templates?

Not directly in templates, but you can pass values through the CLI:

```bash
cargo forge new my-project --author "$USER"
```

### Does Cargo-Forge support Windows paths?

Yes, Cargo-Forge automatically handles path conversions across platforms.

## Troubleshooting

### Why does project generation fail?

Common reasons:
1. Invalid project name (must be valid Rust identifier)
2. Directory already exists
3. No write permissions
4. Disk space issues

### Why are my templates not rendering correctly?

Check for:
- Typos in variable names
- Missing `.tera` extension
- Incorrect conditional syntax
- Special characters in input values

### How do I debug template issues?

Enable debug logging:

```bash
RUST_LOG=debug cargo forge new my-project
```

### Where are the error logs?

Cargo-Forge outputs errors directly to stderr. Capture them with:

```bash
cargo forge new my-project 2> error.log
```

## Best Practices

### Should I commit generated files to version control?

Yes! Commit all generated files so other developers can:
- See the complete project structure
- Build without running Cargo-Forge
- Track changes over time

### How often should I update project dependencies?

After generation:
1. Run `cargo update` monthly
2. Use `cargo audit` for security updates
3. Test thoroughly after updates

### Should I modify generated files?

Absolutely! Generated files are just a starting point. Modify them to fit your needs - that's the whole point!

### What's the recommended workflow?

1. Run `cargo forge new` interactively
2. Review generated files
3. Run tests to ensure everything works
4. Start implementing your features
5. Keep the README updated

## Contributing

### How can I contribute to Cargo-Forge?

We welcome contributions! See [CONTRIBUTING.md](../CONTRIBUTING.md) for:
- Code contributions
- Template improvements
- Documentation updates
- Bug reports
- Feature requests

### Where do I report bugs?

Use the [GitHub issue tracker](https://github.com/marcuspat/cargo-forge/issues) with:
- Clear description
- Steps to reproduce
- Expected vs actual behavior
- System information

### Can I add new project types?

Yes! New project types are welcome. See the contributing guide for details on adding project types.

### How do I suggest new features?

Open a discussion on [GitHub Discussions](https://github.com/marcuspat/cargo-forge/discussions) or create a feature request issue.

## Advanced Usage

### Can I script Cargo-Forge?

Yes, Cargo-Forge is designed to be scriptable:

```bash
#!/bin/bash
for proj in api cli lib; do
  cargo forge new "$proj-service" \
    --project-type "$proj" \
    --non-interactive
done
```

### How do I integrate with other tools?

Cargo-Forge generates standard Rust projects that work with:
- cargo-watch
- cargo-release
- cargo-audit
- rust-analyzer
- Any Cargo subcommand

### Can I use custom registries?

Yes, modify the generated `Cargo.toml`:

```toml
[registries]
my-registry = { index = "https://my-registry.com/index" }
```

### Is there an API or library interface?

Currently, Cargo-Forge is CLI-only. A library API is planned for future releases.

## Future Plans

### What features are planned?

Upcoming features include:
- Plugin system for custom generators
- Web interface option
- More project types (GraphQL, gRPC)
- Template marketplace
- Project update/migration commands

### When will X feature be available?

Check the [roadmap](https://github.com/marcuspat/cargo-forge/projects) for planned features and timelines.

### How can I stay updated?

- Watch the GitHub repository
- Join our Discord server
- Follow the blog
- Subscribe to release notifications

## More Help

### Where can I get more help?

- [Documentation](https://docs.rs/cargo-forge)
- [GitHub Discussions](https://github.com/marcuspat/cargo-forge/discussions)
- Discord Server (coming soon)
- Video Tutorials (coming soon)

### Is there commercial support available?

While Cargo-Forge is open source, commercial support may be available for enterprise users in the future.

### How do I become a Cargo-Forge expert?

1. Read all documentation
2. Try each project type
3. Experiment with features
4. Contribute to the project
5. Help others in the community

---

*Don't see your question? Open a [discussion](https://github.com/marcuspat/cargo-forge/discussions) on GitHub!*