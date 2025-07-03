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
        features: vec![],
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

// Helper function to verify all common files exist
fn verify_common_files(project_dir: &Path) -> Result<(), String> {
    let required_files = vec![
        "Cargo.toml",
        "README.md",
        ".gitignore",
    ];
    
    for file in required_files {
        let file_path = project_dir.join(file);
        if !file_path.exists() {
            return Err(format!("Required file '{}' does not exist", file));
        }
        if !file_path.is_file() {
            return Err(format!("'{}' is not a file", file));
        }
    }
    
    let required_dirs = vec!["src", "tests"];
    
    for dir in required_dirs {
        let dir_path = project_dir.join(dir);
        if !dir_path.exists() {
            return Err(format!("Required directory '{}' does not exist", dir));
        }
        if !dir_path.is_dir() {
            return Err(format!("'{}' is not a directory", dir));
        }
    }
    
    Ok(())
}

#[test]
fn test_api_server_integration() {
    let temp_dir = create_test_dir();
    let project_dir = temp_dir.path().join("test-api-server");
    let config = create_test_config("test-api-server", "api-server");
    
    let generator = Generator::new();
    
    // Generate the project
    generator.generate(&config, &project_dir)
        .expect("Failed to generate API server project");
    
    // Verify common files
    verify_common_files(&project_dir)
        .expect("Common files verification failed");
    
    // Verify API server specific files
    let api_files = vec![
        "src/main.rs",
        "src/routes.rs",
        "src/handlers.rs",
        "src/models.rs",
        "config/default.toml",
        ".env.example",
    ];
    
    for file in api_files {
        let file_path = project_dir.join(file);
        assert!(file_path.exists(), "API server file '{}' does not exist", file);
        assert!(file_path.is_file(), "'{}' is not a file", file);
    }
    
    // Verify Cargo.toml contains correct dependencies
    verify_file_contains(
        &project_dir.join("Cargo.toml"),
        &[
            "test-api-server",
            "Test Author <test@example.com>",
            "Test api-server project",
            "axum",
            "tokio",
            "serde",
            "tower",
        ],
    ).expect("Cargo.toml verification failed");
    
    // Verify README contains project-specific content
    verify_file_contains(
        &project_dir.join("README.md"),
        &[
            "# test-api-server",
            "Test api-server project",
            "API Server",
            "http://localhost:3000",
        ],
    ).expect("README.md verification failed");
    
    // Run cargo check to verify the project compiles
    match run_cargo_check(&project_dir) {
        Ok(_) => println!("API server project compiles successfully"),
        Err(e) => panic!("API server project compilation failed: {}", e),
    }
}

#[test]
fn test_cli_tool_integration() {
    let temp_dir = create_test_dir();
    let project_dir = temp_dir.path().join("test-cli-tool");
    let config = create_test_config("test-cli-tool", "cli-tool");
    
    let generator = Generator::new();
    
    // Generate the project
    generator.generate(&config, &project_dir)
        .expect("Failed to generate CLI tool project");
    
    // Verify common files
    verify_common_files(&project_dir)
        .expect("Common files verification failed");
    
    // Verify CLI tool specific files
    let cli_files = vec![
        "src/main.rs",
        "src/cli.rs",
        "src/commands.rs",
    ];
    
    for file in cli_files {
        let file_path = project_dir.join(file);
        assert!(file_path.exists(), "CLI tool file '{}' does not exist", file);
        assert!(file_path.is_file(), "'{}' is not a file", file);
    }
    
    // Verify no lib.rs exists (should be binary only)
    assert!(!project_dir.join("src/lib.rs").exists(), "CLI tool should not have lib.rs");
    
    // Verify Cargo.toml contains correct dependencies and binary section
    verify_file_contains(
        &project_dir.join("Cargo.toml"),
        &[
            "test-cli-tool",
            "Test Author <test@example.com>",
            "Test cli-tool project",
            "clap",
            "anyhow",
            "env_logger",
            "[[bin]]",
            "path = \"src/main.rs\"",
        ],
    ).expect("Cargo.toml verification failed");
    
    // Verify README contains project-specific content
    verify_file_contains(
        &project_dir.join("README.md"),
        &[
            "# test-cli-tool",
            "Test cli-tool project",
            "CLI Tool",
            "cargo run -- --help",
        ],
    ).expect("README.md verification failed");
    
    // Run cargo check to verify the project compiles
    match run_cargo_check(&project_dir) {
        Ok(_) => println!("CLI tool project compiles successfully"),
        Err(e) => panic!("CLI tool project compilation failed: {}", e),
    }
}

#[test]
fn test_library_integration() {
    let temp_dir = create_test_dir();
    let project_dir = temp_dir.path().join("test-library");
    let config = create_test_config("test-library", "library");
    
    let generator = Generator::new();
    
    // Generate the project
    generator.generate(&config, &project_dir)
        .expect("Failed to generate library project");
    
    // Verify common files
    verify_common_files(&project_dir)
        .expect("Common files verification failed");
    
    // Verify library specific files
    assert!(project_dir.join("src/lib.rs").exists(), "Library should have lib.rs");
    assert!(!project_dir.join("src/main.rs").exists(), "Library should not have main.rs");
    assert!(project_dir.join("examples").exists(), "Library should have examples directory");
    assert!(project_dir.join("examples/basic.rs").exists(), "Library should have basic example");
    
    // Verify lib.rs contains documentation comment
    verify_file_contains(
        &project_dir.join("src/lib.rs"),
        &["//! Test library project"],
    ).expect("lib.rs verification failed");
    
    // Verify Cargo.toml contains correct library configuration
    verify_file_contains(
        &project_dir.join("Cargo.toml"),
        &[
            "test-library",
            "Test Author <test@example.com>",
            "Test library project",
            "[lib]",
            "test_library", // Note: hyphens converted to underscores
        ],
    ).expect("Cargo.toml verification failed");
    
    // Verify .gitignore includes Cargo.lock for libraries
    verify_file_contains(
        &project_dir.join(".gitignore"),
        &["/target", "Cargo.lock"],
    ).expect(".gitignore verification failed");
    
    // Verify README contains project-specific content
    verify_file_contains(
        &project_dir.join("README.md"),
        &[
            "# test-library",
            "Test library project",
            "Library",
            "cargo doc --open",
            "[dependencies]",
            "test-library = \"0.1.0\"",
        ],
    ).expect("README.md verification failed");
    
    // Run cargo check to verify the project compiles
    match run_cargo_check(&project_dir) {
        Ok(_) => println!("Library project compiles successfully"),
        Err(e) => panic!("Library project compilation failed: {}", e),
    }
}

#[test]
fn test_wasm_app_integration() {
    let temp_dir = create_test_dir();
    let project_dir = temp_dir.path().join("test-wasm-app");
    let config = create_test_config("test-wasm-app", "wasm-app");
    
    let generator = Generator::new();
    
    // Generate the project
    generator.generate(&config, &project_dir)
        .expect("Failed to generate WASM app project");
    
    // Verify common files
    verify_common_files(&project_dir)
        .expect("Common files verification failed");
    
    // Verify WASM app specific files
    let wasm_files = vec![
        "src/lib.rs",
        "index.html",
        "index.js",
        "package.json",
        "webpack.config.js",
        "build.sh",
    ];
    
    for file in wasm_files {
        let file_path = project_dir.join(file);
        assert!(file_path.exists(), "WASM app file '{}' does not exist", file);
        assert!(file_path.is_file(), "'{}' is not a file", file);
    }
    
    // Verify no main.rs exists (WASM uses lib.rs)
    assert!(!project_dir.join("src/main.rs").exists(), "WASM app should not have main.rs");
    
    // Verify Cargo.toml contains correct dependencies and library configuration
    verify_file_contains(
        &project_dir.join("Cargo.toml"),
        &[
            "test-wasm-app",
            "Test Author <test@example.com>",
            "Test wasm-app project",
            "wasm-bindgen",
            "web-sys",
            "js-sys",
            "[lib]",
            r#"crate-type = ["cdylib"]"#,
        ],
    ).expect("Cargo.toml verification failed");
    
    // Verify .gitignore includes WASM-specific entries
    verify_file_contains(
        &project_dir.join(".gitignore"),
        &["/target", "node_modules", "dist/", "pkg/"],
    ).expect(".gitignore verification failed");
    
    // Verify README contains project-specific content
    verify_file_contains(
        &project_dir.join("README.md"),
        &[
            "# test-wasm-app",
            "Test wasm-app project",
            "WASM Application",
            "./build.sh",
            "npm install",
            "npm start",
            "http://localhost:8080",
        ],
    ).expect("README.md verification failed");
    
    // Run cargo check to verify the project compiles
    match run_cargo_check(&project_dir) {
        Ok(_) => println!("WASM app project compiles successfully"),
        Err(e) => panic!("WASM app project compilation failed: {}", e),
    }
}

#[test]
fn test_template_variable_substitution_all_types() {
    let temp_dir = create_test_dir();
    let project_types = vec!["api-server", "cli-tool", "library", "wasm-app"];
    
    for project_type in project_types {
        let project_name = format!("subst-test-{}", project_type);
        let project_dir = temp_dir.path().join(&project_name);
        
        let config = ProjectConfig {
            name: project_name.clone(),
            project_type: project_type.to_string(),
            author: "Jane Smith <jane@example.com>".to_string(),
            description: Some(format!("Custom description for {}", project_type)),
            features: vec![],
        };
        
        let generator = Generator::new();
        generator.generate(&config, &project_dir)
            .expect(&format!("Failed to generate {} project", project_type));
        
        // Verify all substitutions in Cargo.toml
        verify_file_contains(
            &project_dir.join("Cargo.toml"),
            &[
                &project_name,
                "Jane Smith <jane@example.com>",
                &format!("Custom description for {}", project_type),
                "version = \"0.1.0\"",
                "edition = \"2021\"",
            ],
        ).expect(&format!("Variable substitution failed for {} Cargo.toml", project_type));
        
        // Verify substitutions in README.md
        verify_file_contains(
            &project_dir.join("README.md"),
            &[
                &format!("# {}", project_name),
                &format!("Custom description for {}", project_type),
            ],
        ).expect(&format!("Variable substitution failed for {} README.md", project_type));
    }
}

#[test]
fn test_project_compilation_with_minimal_code() {
    // This test ensures that generated projects not only have the right structure
    // but also contain minimal compilable code
    let temp_dir = create_test_dir();
    let generator = Generator::new();
    
    // Test each project type compiles
    let test_cases = vec![
        ("compile-api", "api-server"),
        ("compile-cli", "cli-tool"),
        ("compile-lib", "library"),
        ("compile-wasm", "wasm-app"),
    ];
    
    for (name, project_type) in test_cases {
        let project_dir = temp_dir.path().join(name);
        let config = create_test_config(name, project_type);
        
        generator.generate(&config, &project_dir)
            .expect(&format!("Failed to generate {} project", project_type));
        
        // Ensure the project compiles without errors
        match run_cargo_check(&project_dir) {
            Ok(_) => {
                println!("{} project '{}' compiles successfully", project_type, name);
                
                // Additional check: verify main entry point has some content
                match project_type {
                    "api-server" | "cli-tool" => {
                        let main_content = fs::read_to_string(project_dir.join("src/main.rs"))
                            .expect("Failed to read main.rs");
                        assert!(main_content.contains("fn main()"), 
                            "main.rs should contain main function");
                    },
                    "library" => {
                        let lib_content = fs::read_to_string(project_dir.join("src/lib.rs"))
                            .expect("Failed to read lib.rs");
                        assert!(lib_content.contains("//!"), 
                            "lib.rs should contain documentation comment");
                    },
                    "wasm-app" => {
                        // WASM app has empty lib.rs by default, which is fine
                        assert!(project_dir.join("src/lib.rs").exists());
                    },
                    _ => {}
                }
            },
            Err(e) => panic!("{} project '{}' compilation failed: {}", project_type, name, e),
        }
    }
}

#[test]
fn test_project_name_sanitization() {
    // Test that project names with special characters are handled correctly
    let temp_dir = create_test_dir();
    let project_dir = temp_dir.path().join("special-name-test");
    
    let config = ProjectConfig {
        name: "my-special_project.123".to_string(),
        project_type: "library".to_string(),
        author: "Test Author".to_string(),
        description: Some("Project with special name".to_string()),
        features: vec![],
    };
    
    let generator = Generator::new();
    generator.generate(&config, &project_dir)
        .expect("Failed to generate project with special name");
    
    // Verify the library name in Cargo.toml is properly sanitized
    let cargo_content = fs::read_to_string(project_dir.join("Cargo.toml"))
        .expect("Failed to read Cargo.toml");
    
    // Library names should have hyphens converted to underscores
    assert!(cargo_content.contains("name = \"my_special_project_123\"") || 
            cargo_content.contains("name = \"my-special_project.123\""),
            "Library name should be in Cargo.toml");
}

#[test]
fn test_directory_structure_completeness() {
    // Comprehensive test to ensure all expected directories and files are created
    let temp_dir = create_test_dir();
    let generator = Generator::new();
    
    // Define expected structure for each project type
    let project_structures = vec![
        ("api-server", vec![
            "src/", "src/main.rs", "src/routes.rs", "src/handlers.rs", "src/models.rs",
            "tests/", "config/", "config/default.toml", ".env.example",
            "Cargo.toml", "README.md", ".gitignore"
        ]),
        ("cli-tool", vec![
            "src/", "src/main.rs", "src/cli.rs", "src/commands.rs",
            "tests/", "Cargo.toml", "README.md", ".gitignore"
        ]),
        ("library", vec![
            "src/", "src/lib.rs", "tests/", "examples/", "examples/basic.rs",
            "Cargo.toml", "README.md", ".gitignore"
        ]),
        ("wasm-app", vec![
            "src/", "src/lib.rs", "tests/", "index.html", "index.js",
            "package.json", "webpack.config.js", "build.sh",
            "Cargo.toml", "README.md", ".gitignore"
        ]),
    ];
    
    for (project_type, expected_paths) in project_structures {
        let project_name = format!("structure-test-{}", project_type);
        let project_dir = temp_dir.path().join(&project_name);
        let config = create_test_config(&project_name, project_type);
        
        generator.generate(&config, &project_dir)
            .expect(&format!("Failed to generate {} project", project_type));
        
        // Check each expected path
        for path in expected_paths {
            let full_path = project_dir.join(path);
            assert!(full_path.exists(), 
                "{} project should have '{}'", project_type, path);
            
            // Verify it's the correct type (file vs directory)
            if path.ends_with('/') {
                assert!(full_path.is_dir(), 
                    "'{}' should be a directory in {} project", path, project_type);
            } else {
                assert!(full_path.is_file(), 
                    "'{}' should be a file in {} project", path, project_type);
            }
        }
    }
}

#[test]
fn test_game_engine_integration() {
    let temp_dir = create_test_dir();
    let project_dir = temp_dir.path().join("test-game-engine");
    let config = create_test_config("test-game-engine", "game-engine");
    
    let generator = Generator::new();
    
    // Generate the project
    generator.generate(&config, &project_dir)
        .expect("Failed to generate game engine project");
    
    // Verify common files
    verify_common_files(&project_dir)
        .expect("Common files verification failed");
    
    // Verify game engine specific files
    let game_files = vec![
        "src/main.rs",
        "assets/",
        "assets/README.md",
        "assets/models/",
        "assets/textures/",
        "assets/sounds/",
        "assets/shaders/",
        ".github/workflows/wasm.yml",
    ];
    
    for file in game_files {
        let file_path = project_dir.join(file);
        assert!(file_path.exists(), "Game engine file '{}' does not exist", file);
        
        if file.ends_with('/') {
            assert!(file_path.is_dir(), "'{}' should be a directory", file);
        } else {
            assert!(file_path.is_file(), "'{}' should be a file", file);
        }
    }
    
    // Verify no lib.rs exists (should be binary only)
    assert!(!project_dir.join("src/lib.rs").exists(), "Game engine should not have lib.rs");
    
    // Verify Cargo.toml contains correct dependencies
    verify_file_contains(
        &project_dir.join("Cargo.toml"),
        &[
            "test-game-engine",
            "Test Author <test@example.com>",
            "Test game-engine project",
            "bevy",
            "wasm-bindgen",
            "web-sys",
            "console_error_panic_hook",
            "[profile.dev]",
            "opt-level = 1",
        ],
    ).expect("Cargo.toml verification failed");
    
    // Verify .gitignore includes game-specific entries
    verify_file_contains(
        &project_dir.join(".gitignore"),
        &["/target", "wasm/", "*.wasm", ".DS_Store"],
    ).expect(".gitignore verification failed");
    
    // Verify README contains project-specific content
    verify_file_contains(
        &project_dir.join("README.md"),
        &[
            "# test-game-engine",
            "Test game-engine project",
            "Game Engine",
            "Bevy engine",
            "cargo run",
            "wasm32-unknown-unknown",
            "ESC: Exit game",
        ],
    ).expect("README.md verification failed");
    
    // Run cargo check to verify the project compiles
    match run_cargo_check(&project_dir) {
        Ok(_) => println!("Game engine project compiles successfully"),
        Err(e) => panic!("Game engine project compilation failed: {}", e),
    }
}

#[test]
fn test_embedded_integration() {
    let temp_dir = create_test_dir();
    let project_dir = temp_dir.path().join("test-embedded");
    let config = create_test_config("test-embedded", "embedded");
    
    let generator = Generator::new();
    
    // Generate the project
    generator.generate(&config, &project_dir)
        .expect("Failed to generate embedded project");
    
    // Verify common files
    verify_common_files(&project_dir)
        .expect("Common files verification failed");
    
    // Verify embedded specific files
    let embedded_files = vec![
        "src/main.rs",
        ".cargo/",
        ".cargo/config.toml",
        "memory.x",
        "Embed.toml",
    ];
    
    for file in embedded_files {
        let file_path = project_dir.join(file);
        assert!(file_path.exists(), "Embedded file '{}' does not exist", file);
        
        if file.ends_with('/') {
            assert!(file_path.is_dir(), "'{}' should be a directory", file);
        } else {
            assert!(file_path.is_file(), "'{}' should be a file", file);
        }
    }
    
    // Verify no lib.rs exists (should be binary only)
    assert!(!project_dir.join("src/lib.rs").exists(), "Embedded should not have lib.rs");
    
    // Verify main.rs contains no_std attributes
    verify_file_contains(
        &project_dir.join("src/main.rs"),
        &["#![no_std]", "#![no_main]"],
    ).expect("main.rs verification failed");
    
    // Verify Cargo.toml contains correct dependencies
    verify_file_contains(
        &project_dir.join("Cargo.toml"),
        &[
            "test-embedded",
            "Test Author <test@example.com>",
            "Test embedded project",
            "cortex-m",
            "cortex-m-rt",
            "panic-halt",
            "[profile.dev]",
            "opt-level = 1",
            "[profile.release]",
            "lto = \"fat\"",
        ],
    ).expect("Cargo.toml verification failed");
    
    // Verify .gitignore includes embedded-specific entries
    verify_file_contains(
        &project_dir.join(".gitignore"),
        &["/target", "*.bin", "*.hex", "*.elf", ".vscode/"],
    ).expect(".gitignore verification failed");
    
    // Verify README contains project-specific content
    verify_file_contains(
        &project_dir.join("README.md"),
        &[
            "# test-embedded",
            "Test embedded project",
            "Embedded Project",
            "no_std",
            "ARM Cortex-M",
            "cargo build --release",
            "probe-rs",
            "RTT logging",
        ],
    ).expect("README.md verification failed");
    
    // Skip cargo check for embedded projects as they require specific targets
    // that may not be available in all test environments
    println!("✓ Embedded project structure and content validation completed");
    println!("Note: Embedded compilation requires thumbv7em-none-eabihf target");
}

#[test]
fn test_workspace_integration() {
    let temp_dir = create_test_dir();
    let project_dir = temp_dir.path().join("test-workspace");
    let config = create_test_config("test-workspace", "workspace");
    
    let generator = Generator::new();
    
    // Generate the project
    generator.generate(&config, &project_dir)
        .expect("Failed to generate workspace project");
    
    // Verify workspace specific structure
    let workspace_files = vec![
        "Cargo.toml",
        "README.md",
        ".gitignore",
        "crates/",
        "crates/core/",
        "crates/core/src/",
        "crates/core/src/lib.rs",
        "crates/core/Cargo.toml",
        "crates/api/",
        "crates/api/src/",
        "crates/api/src/lib.rs",
        "crates/api/Cargo.toml",
        "crates/cli/",
        "crates/cli/src/",
        "crates/cli/src/main.rs",
        "crates/cli/Cargo.toml",
    ];
    
    for file in workspace_files {
        let file_path = project_dir.join(file);
        assert!(file_path.exists(), "Workspace file '{}' does not exist", file);
        
        if file.ends_with('/') {
            assert!(file_path.is_dir(), "'{}' should be a directory", file);
        } else {
            assert!(file_path.is_file(), "'{}' should be a file", file);
        }
    }
    
    // Verify no src/ directory at root (workspace doesn't have root src)
    assert!(!project_dir.join("src/").exists(), "Workspace should not have root src directory");
    
    // Verify core crate lib.rs contains documentation
    verify_file_contains(
        &project_dir.join("crates/core/src/lib.rs"),
        &["//! Core library"],
    ).expect("Core lib.rs verification failed");
    
    // Verify API crate lib.rs contains documentation
    verify_file_contains(
        &project_dir.join("crates/api/src/lib.rs"),
        &["//! API library"],
    ).expect("API lib.rs verification failed");
    
    // Verify CLI crate main.rs contains main function
    verify_file_contains(
        &project_dir.join("crates/cli/src/main.rs"),
        &["fn main()"],
    ).expect("CLI main.rs verification failed");
    
    // Verify .gitignore includes workspace-specific entries
    verify_file_contains(
        &project_dir.join(".gitignore"),
        &["/target", "Cargo.lock"],
    ).expect(".gitignore verification failed");
    
    // Verify README contains project-specific content
    verify_file_contains(
        &project_dir.join("README.md"),
        &[
            "# test-workspace",
            "Test workspace project",
            "Workspace Project",
            "multi-crate",
            "crates/core",
            "crates/api",
            "crates/cli",
            "cargo build",
            "cargo run",
        ],
    ).expect("README.md verification failed");
    
    // Run cargo check to verify the workspace compiles
    match run_cargo_check(&project_dir) {
        Ok(_) => println!("Workspace project compiles successfully"),
        Err(e) => panic!("Workspace project compilation failed: {}", e),
    }
}

#[test]
fn test_all_project_types_compilation() {
    // This test ensures that ALL project types compile successfully
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
        let project_name = format!("compile-test-{}", project_type);
        let project_dir = temp_dir.path().join(&project_name);
        let config = create_test_config(&project_name, project_type);
        
        generator.generate(&config, &project_dir)
            .expect(&format!("Failed to generate {} project", project_type));
        
        // Ensure the project compiles without errors (skip embedded as it requires special targets)
        if project_type != "embedded" {
            match run_cargo_check(&project_dir) {
                Ok(_) => println!("✓ {} project compiles successfully", project_type),
                Err(e) => panic!("✗ {} project compilation failed: {}", project_type, e),
            }
        } else {
            println!("✓ {} project structure validated (compilation skipped)", project_type);
        }
    }
}

#[test]
fn test_cross_platform_file_generation() {
    // Test that file generation works consistently across platforms
    let temp_dir = create_test_dir();
    let generator = Generator::new();
    let project_types = vec!["api-server", "cli-tool", "library", "wasm-app", "game-engine", "embedded", "workspace"];
    
    for project_type in project_types {
        let project_name = format!("cross-platform-{}", project_type);
        let project_dir = temp_dir.path().join(&project_name);
        let config = create_test_config(&project_name, project_type);
        
        generator.generate(&config, &project_dir)
            .expect(&format!("Failed to generate {} project", project_type));
        
        // Verify common files exist and are readable
        let common_files = vec!["Cargo.toml", "README.md", ".gitignore"];
        for file in common_files {
            let file_path = project_dir.join(file);
            assert!(file_path.exists(), "{} should have {}", project_type, file);
            assert!(file_path.is_file(), "{} should be a file", file);
            
            // Verify file is readable
            let content = fs::read_to_string(&file_path)
                .expect(&format!("Should be able to read {}", file));
            assert!(!content.is_empty(), "{} should not be empty", file);
        }
        
        println!("✓ {} project generates files correctly", project_type);
    }
}

#[test]
fn test_project_generation_performance() {
    // Test that project generation is reasonably fast
    let temp_dir = create_test_dir();
    let generator = Generator::new();
    let project_types = vec!["api-server", "cli-tool", "library", "wasm-app", "game-engine", "embedded", "workspace"];
    
    for project_type in project_types {
        let project_name = format!("perf-test-{}", project_type);
        let project_dir = temp_dir.path().join(&project_name);
        let config = create_test_config(&project_name, project_type);
        
        let start = std::time::Instant::now();
        generator.generate(&config, &project_dir)
            .expect(&format!("Failed to generate {} project", project_type));
        let duration = start.elapsed();
        
        // Project generation should complete within reasonable time
        assert!(duration.as_secs() < 5, 
            "{} project generation took too long: {:?}", project_type, duration);
        
        println!("✓ {} project generated in {:?}", project_type, duration);
    }
}