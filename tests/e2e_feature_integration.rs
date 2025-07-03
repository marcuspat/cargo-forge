use cargo_forge::{Generator, ProjectConfig};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

// Helper function to create a test project configuration
fn create_test_config(name: &str, project_type: &str) -> ProjectConfig {
    ProjectConfig {
        name: name.to_string(),
        project_type: project_type.to_string(),
        author: "Test Author <test@example.com>".to_string(),
        description: Some(format!("Test {} project", project_type)),
    }
}

// Helper function to create a temporary test directory
fn create_test_dir() -> TempDir {
    TempDir::new().expect("Failed to create temp directory")
}

// Helper function to run cargo check in a directory
fn run_cargo_check(project_dir: &Path) -> Result<bool, String> {
    let output = Command::new("cargo")
        .arg("check")
        .current_dir(project_dir)
        .output()
        .map_err(|e| format!("Failed to execute cargo check: {}", e))?;

    if output.status.success() {
        Ok(true)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Cargo check failed: {}", stderr))
    }
}

// Helper function to verify file contents contain expected strings
fn verify_file_contains(file_path: &Path, expected_contents: &[&str]) -> Result<(), String> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file {:?}: {}", file_path, e))?;
    
    for expected in expected_contents {
        if !content.contains(expected) {
            return Err(format!(
                "File {:?} does not contain expected content: '{}'",
                file_path, expected
            ));
        }
    }
    
    Ok(())
}

// Extended ProjectConfig that could support features in the future
#[derive(Debug, Clone)]
pub struct ExtendedProjectConfig {
    pub name: String,
    pub project_type: String,
    pub author: String,
    pub description: Option<String>,
    pub features: Vec<String>,
    pub database: Option<String>,
    pub auth_type: Option<String>,
    pub ci_provider: Option<String>,
    pub docker: bool,
    pub testing_framework: Option<String>,
}

impl ExtendedProjectConfig {
    pub fn new(name: &str, project_type: &str) -> Self {
        Self {
            name: name.to_string(),
            project_type: project_type.to_string(),
            author: "Test Author <test@example.com>".to_string(),
            description: Some(format!("Test {} project", project_type)),
            features: Vec::new(),
            database: None,
            auth_type: None,
            ci_provider: None,
            docker: false,
            testing_framework: None,
        }
    }
    
    pub fn with_feature(mut self, feature: &str) -> Self {
        self.features.push(feature.to_string());
        self
    }
    
    pub fn with_database(mut self, db: &str) -> Self {
        self.database = Some(db.to_string());
        self
    }
    
    pub fn with_auth(mut self, auth: &str) -> Self {
        self.auth_type = Some(auth.to_string());
        self
    }
    
    pub fn with_ci(mut self, ci: &str) -> Self {
        self.ci_provider = Some(ci.to_string());
        self
    }
    
    pub fn with_docker(mut self) -> Self {
        self.docker = true;
        self
    }
    
    pub fn with_testing(mut self, framework: &str) -> Self {
        self.testing_framework = Some(framework.to_string());
        self
    }
    
    // Convert to basic ProjectConfig for now
    pub fn to_basic_config(&self) -> ProjectConfig {
        ProjectConfig {
            name: self.name.clone(),
            project_type: self.project_type.clone(),
            author: self.author.clone(),
            description: self.description.clone(),
        }
    }
}

#[test]
fn test_basic_project_types_with_default_features() {
    // Test that all project types work with their default feature sets
    let temp_dir = create_test_dir();
    let generator = Generator::new();
    
    let project_types = vec![
        "api-server",
        "cli-tool",
        "library", 
        "wasm-app",
        "game-engine",
        "embedded",
        "workspace",
    ];
    
    for project_type in project_types {
        let project_name = format!("basic-{}", project_type);
        let project_dir = temp_dir.path().join(&project_name);
        let config = create_test_config(&project_name, project_type);
        
        generator.generate(&config, &project_dir)
            .expect(&format!("Failed to generate {} project", project_type));
        
        // Verify default dependencies are included
        let cargo_toml_path = project_dir.join("Cargo.toml");
        let cargo_content = fs::read_to_string(&cargo_toml_path)
            .expect("Failed to read Cargo.toml");
        
        match project_type {
            "api-server" => {
                assert!(cargo_content.contains("axum"), "API server should have axum");
                assert!(cargo_content.contains("tokio"), "API server should have tokio");
                assert!(cargo_content.contains("serde"), "API server should have serde");
                assert!(cargo_content.contains("tower"), "API server should have tower");
            }
            "cli-tool" => {
                assert!(cargo_content.contains("clap"), "CLI tool should have clap");
                assert!(cargo_content.contains("anyhow"), "CLI tool should have anyhow");
                assert!(cargo_content.contains("env_logger"), "CLI tool should have env_logger");
            }
            "wasm-app" => {
                assert!(cargo_content.contains("wasm-bindgen"), "WASM app should have wasm-bindgen");
                assert!(cargo_content.contains("web-sys"), "WASM app should have web-sys");
                assert!(cargo_content.contains("js-sys"), "WASM app should have js-sys");
            }
            "game-engine" => {
                assert!(cargo_content.contains("bevy"), "Game engine should have bevy");
            }
            "embedded" => {
                assert!(cargo_content.contains("cortex-m"), "Embedded should have cortex-m");
                assert!(cargo_content.contains("cortex-m-rt"), "Embedded should have cortex-m-rt");
                assert!(cargo_content.contains("panic-halt"), "Embedded should have panic-halt");
            }
            _ => {} // Library and workspace don't have specific dependencies
        }
        
        // Verify project compiles (skip embedded which needs special targets)
        if project_type != "embedded" {
            match run_cargo_check(&project_dir) {
                Ok(_) => println!("✓ {} project with default features compiles", project_type),
                Err(e) => panic!("✗ {} project with default features failed: {}", project_type, e),
            }
        } else {
            println!("✓ {} project structure validated (compilation requires embedded target)", project_type);
        }
    }
}

#[test]
fn test_template_feature_conditional_rendering() {
    // Test how the current template system handles conditional content
    let temp_dir = create_test_dir();
    let generator = Generator::new();
    
    // Test API server project - should have specific structure
    let api_project_dir = temp_dir.path().join("test-api-features");
    let api_config = create_test_config("test-api-features", "api-server");
    
    generator.generate(&api_config, &api_project_dir)
        .expect("Failed to generate API server project");
    
    // Verify API server specific files exist
    assert!(api_project_dir.join("src/routes.rs").exists(), "API server should have routes.rs");
    assert!(api_project_dir.join("src/handlers.rs").exists(), "API server should have handlers.rs");
    assert!(api_project_dir.join("src/models.rs").exists(), "API server should have models.rs");
    assert!(api_project_dir.join("config/default.toml").exists(), "API server should have config");
    assert!(api_project_dir.join(".env.example").exists(), "API server should have .env.example");
    
    // Test CLI tool project - should have different structure
    let cli_project_dir = temp_dir.path().join("test-cli-features");
    let cli_config = create_test_config("test-cli-features", "cli-tool");
    
    generator.generate(&cli_config, &cli_project_dir)
        .expect("Failed to generate CLI tool project");
    
    // Verify CLI tool specific files exist
    assert!(cli_project_dir.join("src/cli.rs").exists(), "CLI tool should have cli.rs");
    assert!(cli_project_dir.join("src/commands.rs").exists(), "CLI tool should have commands.rs");
    
    // Verify CLI tool doesn't have API server files
    assert!(!cli_project_dir.join("src/routes.rs").exists(), "CLI tool should not have routes.rs");
    assert!(!cli_project_dir.join("config/default.toml").exists(), "CLI tool should not have config");
}

#[test]
fn test_project_type_specific_configurations() {
    // Test that each project type has appropriate configurations
    let temp_dir = create_test_dir();
    let generator = Generator::new();
    
    // Test library configuration
    let lib_project_dir = temp_dir.path().join("test-lib-config");
    let lib_config = create_test_config("test-lib-config", "library");
    
    generator.generate(&lib_config, &lib_project_dir)
        .expect("Failed to generate library project");
    
    verify_file_contains(
        &lib_project_dir.join("Cargo.toml"),
        &["[lib]", "test_lib_config"], // Note: hyphens converted to underscores
    ).expect("Library should have lib configuration");
    
    verify_file_contains(
        &lib_project_dir.join(".gitignore"),
        &["Cargo.lock"], // Libraries should ignore Cargo.lock
    ).expect("Library should ignore Cargo.lock");
    
    // Test CLI tool binary configuration
    let cli_project_dir = temp_dir.path().join("test-cli-config");
    let cli_config = create_test_config("test-cli-config", "cli-tool");
    
    generator.generate(&cli_config, &cli_project_dir)
        .expect("Failed to generate CLI tool project");
    
    verify_file_contains(
        &cli_project_dir.join("Cargo.toml"),
        &["[[bin]]", "path = \"src/main.rs\""],
    ).expect("CLI tool should have bin configuration");
    
    // Test WASM app library configuration
    let wasm_project_dir = temp_dir.path().join("test-wasm-config");
    let wasm_config = create_test_config("test-wasm-config", "wasm-app");
    
    generator.generate(&wasm_config, &wasm_project_dir)
        .expect("Failed to generate WASM app project");
    
    verify_file_contains(
        &wasm_project_dir.join("Cargo.toml"),
        &["[lib]", r#"crate-type = ["cdylib"]"#],
    ).expect("WASM app should have cdylib configuration");
    
    verify_file_contains(
        &wasm_project_dir.join(".gitignore"),
        &["node_modules", "dist/", "pkg/"],
    ).expect("WASM app should ignore Node.js and build artifacts");
}

#[test]
fn test_cross_project_type_compatibility() {
    // Test that different project types can coexist and don't interfere
    let temp_dir = create_test_dir();
    let generator = Generator::new();
    
    let project_types = vec![
        "api-server",
        "cli-tool",
        "library",
        "wasm-app",
    ];
    
    let mut project_dirs = Vec::new();
    
    // Generate all project types in the same temp directory
    for project_type in &project_types {
        let project_name = format!("compat-{}", project_type);
        let project_dir = temp_dir.path().join(&project_name);
        let config = create_test_config(&project_name, project_type);
        
        generator.generate(&config, &project_dir)
            .expect(&format!("Failed to generate {} project", project_type));
        
        project_dirs.push((project_type, project_dir));
    }
    
    // Verify all projects can be checked simultaneously (skip embedded)
    for (project_type, project_dir) in project_dirs {
        if project_type != &"embedded" {
            match run_cargo_check(&project_dir) {
                Ok(_) => println!("✓ {} project compiles in multi-project environment", project_type),
                Err(e) => panic!("✗ {} project failed in multi-project environment: {}", project_type, e),
            }
        } else {
            println!("✓ {} project structure validated in multi-project environment", project_type);
        }
    }
}

#[test]
fn test_future_feature_system_design() {
    // This test documents and validates the design for a future feature system
    
    // Example of how an extended feature system could work
    let extended_configs = vec![
        ExtendedProjectConfig::new("api-with-db", "api-server")
            .with_database("postgresql")
            .with_auth("jwt")
            .with_docker(),
            
        ExtendedProjectConfig::new("cli-with-config", "cli-tool")
            .with_feature("config-file")
            .with_testing("integration"),
            
        ExtendedProjectConfig::new("lib-with-async", "library")
            .with_feature("async")
            .with_ci("github-actions"),
            
        ExtendedProjectConfig::new("wasm-with-workers", "wasm-app")
            .with_feature("web-workers")
            .with_testing("wasm-pack-test"),
    ];
    
    // For now, validate that the basic configs still work
    let temp_dir = create_test_dir();
    let generator = Generator::new();
    
    for extended_config in extended_configs {
        let basic_config = extended_config.to_basic_config();
        let project_dir = temp_dir.path().join(&basic_config.name);
        
        generator.generate(&basic_config, &project_dir)
            .expect(&format!("Failed to generate {} project", basic_config.name));
        
        // Validate that basic structure is created
        assert!(project_dir.join("Cargo.toml").exists());
        assert!(project_dir.join("README.md").exists());
        assert!(project_dir.join(".gitignore").exists());
        
        // Document what additional features would be added
        match extended_config.project_type.as_str() {
            "api-server" => {
                if extended_config.database.is_some() {
                    // Future: Should add database dependencies and migrations
                    println!("Future: Would add PostgreSQL dependencies and migrations");
                }
                if extended_config.auth_type.is_some() {
                    // Future: Should add auth middleware and JWT dependencies
                    println!("Future: Would add JWT auth middleware");
                }
                if extended_config.docker {
                    // Future: Should add Dockerfile and docker-compose.yml
                    println!("Future: Would add Docker configuration");
                }
            }
            "cli-tool" => {
                if extended_config.features.contains(&"config-file".to_string()) {
                    // Future: Should add config file handling
                    println!("Future: Would add config file handling");
                }
            }
            "library" => {
                if extended_config.features.contains(&"async".to_string()) {
                    // Future: Should add async dependencies
                    println!("Future: Would add async runtime dependencies");
                }
            }
            "wasm-app" => {
                if extended_config.features.contains(&"web-workers".to_string()) {
                    // Future: Should add web workers setup
                    println!("Future: Would add web workers configuration");
                }
            }
            _ => {}
        }
        
        // Verify current project compiles (skip embedded)
        if extended_config.project_type != "embedded" {
            match run_cargo_check(&project_dir) {
                Ok(_) => println!("✓ Extended config {} compiles with basic features", extended_config.name),
                Err(e) => panic!("✗ Extended config {} failed: {}", extended_config.name, e),
            }
        } else {
            println!("✓ Extended config {} structure validated", extended_config.name);
        }
    }
}

#[test]
fn test_template_system_extensibility() {
    // Test that the current template system could be extended for features
    let temp_dir = create_test_dir();
    let generator = Generator::new();
    
    // Test that we can generate multiple variations of the same project type
    let variations = vec![
        ("minimal-api", "api-server"),
        ("full-api", "api-server"),
        ("simple-cli", "cli-tool"),
        ("advanced-cli", "cli-tool"),
    ];
    
    for (project_name, project_type) in variations {
        let project_dir = temp_dir.path().join(project_name);
        let config = create_test_config(project_name, project_type);
        
        generator.generate(&config, &project_dir)
            .expect(&format!("Failed to generate {} project", project_name));
        
        // All variations should currently produce the same output
        // But in the future, they could produce different feature sets
        
        if !project_type.contains("embedded") {
            match run_cargo_check(&project_dir) {
                Ok(_) => println!("✓ {} variation compiles", project_name),
                Err(e) => panic!("✗ {} variation failed: {}", project_name, e),
            }
        } else {
            println!("✓ {} variation structure validated", project_name);
        }
    }
}

#[test]
fn test_dependency_management_patterns() {
    // Test how dependencies are managed across project types
    let temp_dir = create_test_dir();
    let generator = Generator::new();
    
    let project_types = vec![
        ("api-server", vec!["axum", "tokio", "serde", "tower"]),
        ("cli-tool", vec!["clap", "anyhow", "env_logger"]),
        ("wasm-app", vec!["wasm-bindgen", "web-sys", "js-sys"]),
        ("game-engine", vec!["bevy"]),
        ("embedded", vec!["cortex-m", "cortex-m-rt", "panic-halt"]),
    ];
    
    for (project_type, expected_deps) in project_types {
        let project_name = format!("deps-{}", project_type);
        let project_dir = temp_dir.path().join(&project_name);
        let config = create_test_config(&project_name, project_type);
        
        generator.generate(&config, &project_dir)
            .expect(&format!("Failed to generate {} project", project_type));
        
        let cargo_toml_path = project_dir.join("Cargo.toml");
        
        // Verify all expected dependencies are present
        for dep in expected_deps {
            verify_file_contains(&cargo_toml_path, &[dep])
                .expect(&format!("{} should have {} dependency", project_type, dep));
        }
        
        // Verify project compiles with these dependencies (skip embedded)
        if project_type != "embedded" {
            match run_cargo_check(&project_dir) {
                Ok(_) => println!("✓ {} with standard dependencies compiles", project_type),
                Err(e) => panic!("✗ {} with standard dependencies failed: {}", project_type, e),
            }
        } else {
            println!("✓ {} with standard dependencies structure validated", project_type);
        }
    }
}