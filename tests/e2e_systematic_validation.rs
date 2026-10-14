use cargo_forge::{Generator, ProjectConfig};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

/// Comprehensive end-to-end testing framework for project generation validation
/// This module provides systematic testing of all project types, feature combinations,
/// and cross-platform compatibility.

/// Test configuration for systematic project validation
#[derive(Debug, Clone)]
pub struct TestConfig {
    pub name: String,
    pub project_type: String,
    pub author: String,
    pub description: Option<String>,
    pub features: Vec<String>,
    pub expected_files: Vec<String>,
    pub expected_dependencies: Vec<String>,
    pub required_cargo_sections: Vec<String>,
    pub gitignore_must_have: Vec<String>,
    pub gitignore_must_not_have: Vec<String>,
    pub supports_cargo_check: bool,
    pub supports_cargo_build: bool,
    pub supports_cargo_test: bool,
    pub platform_specific_requirements: HashMap<String, Vec<String>>,
}

impl TestConfig {
    pub fn new(name: &str, project_type: &str) -> Self {
        Self {
            name: name.to_string(),
            project_type: project_type.to_string(),
            author: "E2E Test <e2e@test.com>".to_string(),
            description: Some(format!("Systematic E2E test for {} project", project_type)),
            features: Vec::new(),
            expected_files: Vec::new(),
            expected_dependencies: Vec::new(),
            required_cargo_sections: Vec::new(),
            gitignore_must_have: Vec::new(),
            gitignore_must_not_have: Vec::new(),
            supports_cargo_check: true,
            supports_cargo_build: true,
            supports_cargo_test: true,
            platform_specific_requirements: HashMap::new(),
        }
    }

    pub fn with_expected_files(mut self, files: Vec<&str>) -> Self {
        self.expected_files = files.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_expected_dependencies(mut self, deps: Vec<&str>) -> Self {
        self.expected_dependencies = deps.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_required_cargo_sections(mut self, sections: Vec<&str>) -> Self {
        self.required_cargo_sections = sections.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_gitignore_requirements(
        mut self,
        must_have: Vec<&str>,
        must_not_have: Vec<&str>,
    ) -> Self {
        self.gitignore_must_have = must_have.into_iter().map(|s| s.to_string()).collect();
        self.gitignore_must_not_have = must_not_have.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_cargo_support(mut self, check: bool, build: bool, test: bool) -> Self {
        self.supports_cargo_check = check;
        self.supports_cargo_build = build;
        self.supports_cargo_test = test;
        self
    }

    pub fn with_platform_requirements(mut self, platform: &str, requirements: Vec<&str>) -> Self {
        self.platform_specific_requirements.insert(
            platform.to_string(),
            requirements.into_iter().map(|s| s.to_string()).collect(),
        );
        self
    }
}

/// Comprehensive test suite for all project types
pub struct E2ETestSuite {
    generator: Generator,
    temp_dir: TempDir,
    test_results: HashMap<String, bool>,
}

impl E2ETestSuite {
    pub fn new() -> Self {
        Self {
            generator: Generator::new(),
            temp_dir: TempDir::new().expect("Failed to create temp directory"),
            test_results: HashMap::new(),
        }
    }

    /// Get the current platform identifier
    fn get_platform() -> String {
        if cfg!(target_os = "windows") {
            "windows".to_string()
        } else if cfg!(target_os = "macos") {
            "macos".to_string()
        } else if cfg!(target_os = "linux") {
            "linux".to_string()
        } else {
            "unknown".to_string()
        }
    }

    /// Generate all project type test configurations
    fn get_all_project_test_configs() -> Vec<TestConfig> {
        vec![
            // CLI Tool configuration
            TestConfig::new("e2e-cli-tool", "cli-tool")
                .with_expected_files(vec![
                    "src/main.rs",
                    "src/cli.rs",
                    "src/commands.rs",
                    "Cargo.toml",
                    "README.md",
                    ".gitignore",
                ])
                .with_expected_dependencies(vec!["clap", "anyhow", "env_logger"])
                .with_required_cargo_sections(vec!["[package]", "[[bin]]", "[dependencies]"])
                .with_gitignore_requirements(vec!["/target"], vec!["Cargo.lock"])
                .with_cargo_support(true, true, true),
            // Library configuration
            TestConfig::new("e2e-library", "library")
                .with_expected_files(vec![
                    "src/lib.rs",
                    "examples/basic.rs",
                    "Cargo.toml",
                    "README.md",
                    ".gitignore",
                ])
                .with_expected_dependencies(vec![])
                .with_required_cargo_sections(vec!["[package]", "[lib]"])
                .with_gitignore_requirements(vec!["/target", "Cargo.lock"], vec![])
                .with_cargo_support(true, true, true),
            // API Server configuration
            TestConfig::new("e2e-api-server", "api-server")
                .with_expected_files(vec![
                    "src/main.rs",
                    "src/routes.rs",
                    "src/handlers.rs",
                    "src/models.rs",
                    "config/default.toml",
                    ".env.example",
                    "Cargo.toml",
                    "README.md",
                    ".gitignore",
                ])
                .with_expected_dependencies(vec!["axum", "tokio", "serde", "tower"])
                .with_required_cargo_sections(vec!["[package]", "[dependencies]"])
                .with_gitignore_requirements(vec!["/target"], vec![])
                .with_cargo_support(true, true, true),
            // WASM App configuration
            TestConfig::new("e2e-wasm-app", "wasm-app")
                .with_expected_files(vec![
                    "src/lib.rs",
                    "index.html",
                    "index.js",
                    "package.json",
                    "webpack.config.js",
                    "build.sh",
                    "Cargo.toml",
                    "README.md",
                    ".gitignore",
                ])
                .with_expected_dependencies(vec!["wasm-bindgen", "web-sys", "js-sys"])
                .with_required_cargo_sections(vec!["[package]", "[lib]", "[dependencies]"])
                .with_gitignore_requirements(
                    vec!["/target", "node_modules", "dist/", "pkg/"],
                    vec![],
                )
                .with_cargo_support(true, false, false), // WASM needs special build setup
            // Game Engine configuration
            TestConfig::new("e2e-game-engine", "game-engine")
                .with_expected_files(vec![
                    "src/main.rs",
                    "assets/README.md",
                    "assets/models/",
                    "assets/textures/",
                    "assets/sounds/",
                    "assets/shaders/",
                    ".github/workflows/wasm.yml",
                    "Cargo.toml",
                    "README.md",
                    ".gitignore",
                ])
                .with_expected_dependencies(vec!["bevy"])
                .with_required_cargo_sections(vec!["[package]", "[dependencies]", "[profile.dev]"])
                .with_gitignore_requirements(
                    vec!["/target", "wasm/", "*.wasm", ".DS_Store"],
                    vec![],
                )
                .with_cargo_support(true, true, true),
            // Embedded configuration
            TestConfig::new("e2e-embedded", "embedded")
                .with_expected_files(vec![
                    "src/main.rs",
                    "memory.x",
                    ".cargo/config.toml",
                    "Embed.toml",
                    "Cargo.toml",
                    "README.md",
                    ".gitignore",
                ])
                .with_expected_dependencies(vec!["cortex-m", "cortex-m-rt", "panic-halt"])
                .with_required_cargo_sections(vec![
                    "[package]",
                    "[dependencies]",
                    "[profile.dev]",
                    "[profile.release]",
                ])
                .with_gitignore_requirements(
                    vec!["/target", "*.bin", "*.hex", "*.elf", ".vscode/"],
                    vec![],
                )
                .with_cargo_support(false, false, false), // Embedded needs special targets
            // Workspace configuration
            TestConfig::new("e2e-workspace", "workspace")
                .with_expected_files(vec![
                    "crates/core/src/lib.rs",
                    "crates/core/src/error.rs",
                    "crates/core/src/models.rs",
                    "crates/core/src/utils.rs",
                    "crates/core/Cargo.toml",
                    "crates/api/src/lib.rs",
                    "crates/api/src/state.rs",
                    "crates/api/Cargo.toml",
                    "crates/cli/src/main.rs",
                    "crates/cli/Cargo.toml",
                    "Cargo.toml",
                    "README.md",
                    ".gitignore",
                ])
                .with_expected_dependencies(vec!["tokio", "serde", "anyhow"])
                .with_required_cargo_sections(vec![
                    "[workspace]",
                    "[workspace.package]",
                    "[workspace.dependencies]",
                ])
                .with_gitignore_requirements(vec!["/target", "Cargo.lock"], vec![])
                .with_cargo_support(true, true, true),
        ]
    }

    /// Validate project structure according to test configuration
    fn validate_project_structure(
        &self,
        config: &TestConfig,
        project_dir: &Path,
    ) -> Result<(), String> {
        for file in &config.expected_files {
            let file_path = project_dir.join(file);
            if file.ends_with('/') {
                if !file_path.exists() || !file_path.is_dir() {
                    return Err(format!("Expected directory {} does not exist", file));
                }
            } else {
                if !file_path.exists() || !file_path.is_file() {
                    return Err(format!("Expected file {} does not exist", file));
                }
            }
        }
        Ok(())
    }

    /// Validate Cargo.toml content
    fn validate_cargo_toml(&self, config: &TestConfig, project_dir: &Path) -> Result<(), String> {
        let cargo_toml_path = project_dir.join("Cargo.toml");
        let content = fs::read_to_string(&cargo_toml_path)
            .map_err(|e| format!("Failed to read Cargo.toml: {}", e))?;

        // Check required sections
        for section in &config.required_cargo_sections {
            if !content.contains(section) {
                return Err(format!("Cargo.toml missing required section: {}", section));
            }
        }

        // Check dependencies
        for dep in &config.expected_dependencies {
            if !content.contains(dep) {
                return Err(format!("Cargo.toml missing expected dependency: {}", dep));
            }
        }

        // Validate basic metadata
        if !content.contains(&format!("name = \"{}\"", config.name)) {
            return Err("Cargo.toml missing correct project name".to_string());
        }

        if !content.contains("version = \"0.1.0\"") {
            return Err("Cargo.toml missing version".to_string());
        }

        if !content.contains("edition = \"2021\"") {
            return Err("Cargo.toml missing edition".to_string());
        }

        Ok(())
    }

    /// Validate .gitignore content
    fn validate_gitignore(&self, config: &TestConfig, project_dir: &Path) -> Result<(), String> {
        let gitignore_path = project_dir.join(".gitignore");
        let content = fs::read_to_string(&gitignore_path)
            .map_err(|e| format!("Failed to read .gitignore: {}", e))?;

        // Check required entries
        for entry in &config.gitignore_must_have {
            if !content.contains(entry) {
                return Err(format!(".gitignore missing required entry: {}", entry));
            }
        }

        // Check forbidden entries
        for entry in &config.gitignore_must_not_have {
            if content.contains(entry) {
                return Err(format!(".gitignore contains forbidden entry: {}", entry));
            }
        }

        Ok(())
    }

    /// Validate README.md content
    fn validate_readme(&self, config: &TestConfig, project_dir: &Path) -> Result<(), String> {
        let readme_path = project_dir.join("README.md");
        let content = fs::read_to_string(&readme_path)
            .map_err(|e| format!("Failed to read README.md: {}", e))?;

        // Check project name in title
        if !content.contains(&format!("# {}", config.name)) {
            return Err("README.md missing project name in title".to_string());
        }

        // Check description if provided
        if let Some(desc) = &config.description {
            if !content.contains(desc) {
                return Err("README.md missing project description".to_string());
            }
        }

        // Check minimum content length
        if content.len() < 100 {
            return Err("README.md content too short".to_string());
        }

        Ok(())
    }

    /// Run cargo command and return success/failure
    fn run_cargo_command(&self, project_dir: &Path, command: &str) -> Result<bool, String> {
        let output = Command::new("cargo")
            .arg(command)
            .current_dir(project_dir)
            .output()
            .map_err(|e| format!("Failed to execute cargo {}: {}", command, e))?;

        if output.status.success() {
            Ok(true)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Cargo {} failed: {}", command, stderr))
        }
    }

    /// Validate cargo operations
    fn validate_cargo_operations(
        &self,
        config: &TestConfig,
        project_dir: &Path,
    ) -> Result<(), String> {
        if config.supports_cargo_check {
            self.run_cargo_command(project_dir, "check")
                .map_err(|e| format!("Cargo check failed: {}", e))?;
        }

        if config.supports_cargo_build {
            self.run_cargo_command(project_dir, "build")
                .map_err(|e| format!("Cargo build failed: {}", e))?;
        }

        if config.supports_cargo_test {
            self.run_cargo_command(project_dir, "test")
                .map_err(|e| format!("Cargo test failed: {}", e))?;
        }

        Ok(())
    }

    /// Validate platform-specific requirements
    fn validate_platform_requirements(
        &self,
        config: &TestConfig,
        project_dir: &Path,
    ) -> Result<(), String> {
        let platform = Self::get_platform();
        if let Some(requirements) = config.platform_specific_requirements.get(&platform) {
            for requirement in requirements {
                // Platform-specific validation logic can be added here
                // For now, just check that the requirement is documented
                println!("Platform {} requirement: {}", platform, requirement);
            }
        }
        Ok(())
    }

    /// Run comprehensive validation for a single project
    fn validate_single_project(&mut self, config: &TestConfig) -> Result<(), String> {
        let project_dir = self.temp_dir.path().join(&config.name);

        // Generate project
        let basic_config = ProjectConfig {
            name: config.name.clone(),
            project_type: config.project_type.clone(),
            author: config.author.clone(),
            description: config.description.clone(),
            features: vec![],
            target: None,
            esp32_chip: None,
        };

        self.generator
            .generate(&basic_config, &project_dir)
            .map_err(|e| format!("Failed to generate project: {}", e))?;

        // Validate structure
        self.validate_project_structure(config, &project_dir)?;

        // Validate Cargo.toml
        self.validate_cargo_toml(config, &project_dir)?;

        // Validate .gitignore
        self.validate_gitignore(config, &project_dir)?;

        // Validate README.md
        self.validate_readme(config, &project_dir)?;

        // Validate cargo operations
        self.validate_cargo_operations(config, &project_dir)?;

        // Validate platform requirements
        self.validate_platform_requirements(config, &project_dir)?;

        Ok(())
    }

    /// Run validation for all project types
    pub fn run_all_validations(&mut self) -> HashMap<String, bool> {
        let configs = Self::get_all_project_test_configs();

        for config in configs {
            let result = self.validate_single_project(&config);
            match result {
                Ok(()) => {
                    println!("✅ {} validation passed", config.name);
                    self.test_results.insert(config.name.clone(), true);
                }
                Err(e) => {
                    println!("❌ {} validation failed: {}", config.name, e);
                    self.test_results.insert(config.name.clone(), false);
                }
            }
        }

        self.test_results.clone()
    }

    /// Generate test report
    pub fn generate_test_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== E2E Systematic Validation Report ===\n\n");

        let total_tests = self.test_results.len();
        let passed_tests = self.test_results.values().filter(|&&result| result).count();
        let failed_tests = total_tests - passed_tests;

        report.push_str(&format!("Total Tests: {}\n", total_tests));
        report.push_str(&format!("Passed: {}\n", passed_tests));
        report.push_str(&format!("Failed: {}\n", failed_tests));
        report.push_str(&format!(
            "Success Rate: {:.2}%\n\n",
            (passed_tests as f64 / total_tests as f64) * 100.0
        ));

        report.push_str("Test Results:\n");
        for (test_name, result) in &self.test_results {
            let status = if *result { "PASS" } else { "FAIL" };
            report.push_str(&format!("  {} - {}\n", test_name, status));
        }

        report.push_str(&format!("\nPlatform: {}\n", Self::get_platform()));
        report.push_str(&format!(
            "Timestamp: {}\n",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));

        report
    }
}

/// Main test for systematic validation of all project types
#[test]
fn test_systematic_e2e_validation() {
    let mut test_suite = E2ETestSuite::new();
    let results = test_suite.run_all_validations();

    // Generate and print report
    let report = test_suite.generate_test_report();
    println!("{}", report);

    // Ensure all tests passed
    let failed_tests: Vec<&String> = results
        .iter()
        .filter(|(_, &result)| !result)
        .map(|(name, _)| name)
        .collect();

    if !failed_tests.is_empty() {
        panic!("The following tests failed: {:?}", failed_tests);
    }
}

/// Test each project type individually for detailed feedback
#[test]
fn test_individual_project_validations() {
    let configs = E2ETestSuite::get_all_project_test_configs();

    for config in configs {
        let mut test_suite = E2ETestSuite::new();
        match test_suite.validate_single_project(&config) {
            Ok(()) => println!("✅ Individual test for {} passed", config.name),
            Err(e) => panic!("❌ Individual test for {} failed: {}", config.name, e),
        }
    }
}

/// Test cross-platform compatibility
#[test]
fn test_cross_platform_compatibility() {
    let mut test_suite = E2ETestSuite::new();
    let platform = E2ETestSuite::get_platform();

    println!("Running cross-platform compatibility test on: {}", platform);

    // Test a subset of project types for cross-platform compatibility
    let cross_platform_configs = vec![
        TestConfig::new("cross-platform-cli", "cli-tool"),
        TestConfig::new("cross-platform-lib", "library"),
        TestConfig::new("cross-platform-api", "api-server"),
    ];

    for config in cross_platform_configs {
        match test_suite.validate_single_project(&config) {
            Ok(()) => println!(
                "✅ Cross-platform test for {} passed on {}",
                config.name, platform
            ),
            Err(e) => panic!(
                "❌ Cross-platform test for {} failed on {}: {}",
                config.name, platform, e
            ),
        }
    }
}

/// Test project generation performance
#[test]
fn test_project_generation_performance() {
    let start_time = std::time::Instant::now();
    let mut test_suite = E2ETestSuite::new();

    // Generate multiple projects quickly
    let performance_configs = vec![
        TestConfig::new("perf-test-1", "library"),
        TestConfig::new("perf-test-2", "cli-tool"),
        TestConfig::new("perf-test-3", "api-server"),
    ];

    for config in performance_configs {
        test_suite
            .validate_single_project(&config)
            .expect(&format!("Performance test failed for {}", config.name));
    }

    let elapsed = start_time.elapsed();
    println!("Performance test completed in {:?}", elapsed);

    // Ensure reasonable performance (should complete in under 3 minutes for 3 projects)
    assert!(
        elapsed.as_secs() < 180,
        "Project generation performance test took too long: {:?}",
        elapsed
    );
}
