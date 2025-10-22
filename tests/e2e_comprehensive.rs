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

// Helper function to run cargo build in a directory
fn run_cargo_build(project_dir: &Path) -> Result<bool, String> {
    let output = Command::new("cargo")
        .arg("build")
        .current_dir(project_dir)
        .output()
        .map_err(|e| format!("Failed to execute cargo build: {}", e))?;

    if output.status.success() {
        Ok(true)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Cargo build failed: {}", stderr))
    }
}

// Helper function to run cargo test in a directory
fn run_cargo_test(project_dir: &Path) -> Result<bool, String> {
    let output = Command::new("cargo")
        .arg("test")
        .current_dir(project_dir)
        .output()
        .map_err(|e| format!("Failed to execute cargo test: {}", e))?;

    if output.status.success() {
        Ok(true)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Cargo test failed: {}", stderr))
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

// Helper function to verify file doesn't contain specific strings
fn verify_file_not_contains(file_path: &Path, forbidden_contents: &[&str]) -> Result<(), String> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file {:?}: {}", file_path, e))?;

    for forbidden in forbidden_contents {
        if content.contains(forbidden) {
            return Err(format!(
                "File {:?} should not contain: '{}'",
                file_path, forbidden
            ));
        }
    }

    Ok(())
}

// Helper function to count lines in a file
fn count_lines(file_path: &Path) -> Result<usize, String> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file {:?}: {}", file_path, e))?;
    Ok(content.lines().count())
}

// Helper function to get file size
fn get_file_size(file_path: &Path) -> Result<u64, String> {
    let metadata = fs::metadata(file_path)
        .map_err(|e| format!("Failed to get metadata for {:?}: {}", file_path, e))?;
    Ok(metadata.len())
}

#[test]
fn test_comprehensive_project_matrix() {
    // Test ALL project types with different configurations
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

    let authors = vec![
        "John Doe <john@example.com>",
        "Jane Smith <jane@test.org>",
        "Bob Wilson <bob@dev.net>",
    ];

    let descriptions = vec![
        Some("A test project for validation".to_string()),
        Some("Another test project".to_string()),
        None,
    ];

    for (i, project_type) in project_types.iter().enumerate() {
        let project_name = format!("matrix-test-{}-{}", project_type, i);
        let project_dir = temp_dir.path().join(&project_name);

        let config = ProjectConfig {
            name: project_name.clone(),
            project_type: project_type.to_string(),
            author: authors[i % authors.len()].to_string(),
            description: descriptions[i % descriptions.len()].clone(),
            features: vec![],
        };

        generator
            .generate(&config, &project_dir)
            .expect(&format!("Failed to generate {} project", project_type));

        // Verify the project compiles (skip embedded which needs special targets)
        if *project_type != "embedded" {
            match run_cargo_check(&project_dir) {
                Ok(_) => println!("✓ {} project with config {} compiles", project_type, i),
                Err(e) => panic!("✗ {} project with config {} failed: {}", project_type, i, e),
            }
        } else {
            println!(
                "✓ {} project with config {} structure validated",
                project_type, i
            );
        }

        // Verify author is correctly set
        verify_file_contains(
            &project_dir.join("Cargo.toml"),
            &[authors[i % authors.len()]],
        )
        .expect("Author verification failed");

        // Verify description handling
        if let Some(desc) = &descriptions[i % descriptions.len()] {
            verify_file_contains(&project_dir.join("Cargo.toml"), &[desc])
                .expect("Description verification failed");
        }
    }
}

#[test]
fn test_project_dependency_verification() {
    // Test that each project type has the correct dependencies
    let temp_dir = create_test_dir();
    let generator = Generator::new();

    let dependency_tests = vec![
        ("api-server", vec!["axum", "tokio", "serde", "tower"]),
        ("cli-tool", vec!["clap", "anyhow", "env_logger"]),
        ("library", vec![]), // Library has no default dependencies
        ("wasm-app", vec!["wasm-bindgen", "web-sys", "js-sys"]),
        (
            "game-engine",
            vec![
                "bevy",
                "wasm-bindgen",
                "web-sys",
                "console_error_panic_hook",
            ],
        ),
        ("embedded", vec!["cortex-m", "cortex-m-rt", "panic-halt"]),
        ("workspace", vec![]), // Workspace has no root dependencies
    ];

    for (project_type, expected_deps) in dependency_tests {
        let project_name = format!("dep-test-{}", project_type);
        let project_dir = temp_dir.path().join(&project_name);
        let config = create_test_config(&project_name, project_type);

        generator
            .generate(&config, &project_dir)
            .expect(&format!("Failed to generate {} project", project_type));

        if !expected_deps.is_empty() {
            verify_file_contains(&project_dir.join("Cargo.toml"), &expected_deps).expect(&format!(
                "Dependencies verification failed for {}",
                project_type
            ));
        }

        println!("✓ {} project has correct dependencies", project_type);
    }
}

#[test]
fn test_project_structure_validation() {
    // Test that each project type has the correct file structure
    let temp_dir = create_test_dir();
    let generator = Generator::new();

    let structure_tests = vec![
        (
            "api-server",
            vec![
                "src/main.rs",
                "src/routes.rs",
                "src/handlers.rs",
                "src/models.rs",
                "config/default.toml",
                ".env.example",
            ],
        ),
        (
            "cli-tool",
            vec!["src/main.rs", "src/cli.rs", "src/commands.rs"],
        ),
        ("library", vec!["src/lib.rs", "examples/basic.rs"]),
        (
            "wasm-app",
            vec![
                "src/lib.rs",
                "index.html",
                "index.js",
                "package.json",
                "webpack.config.js",
                "build.sh",
            ],
        ),
        (
            "game-engine",
            vec![
                "src/main.rs",
                "assets/README.md",
                "assets/models/",
                "assets/textures/",
                "assets/sounds/",
                "assets/shaders/",
                ".github/workflows/wasm.yml",
            ],
        ),
        (
            "embedded",
            vec![
                "src/main.rs",
                ".cargo/config.toml",
                "memory.x",
                "Embed.toml",
            ],
        ),
        (
            "workspace",
            vec![
                "crates/core/src/lib.rs",
                "crates/api/src/lib.rs",
                "crates/cli/src/main.rs",
                "crates/core/Cargo.toml",
                "crates/api/Cargo.toml",
                "crates/cli/Cargo.toml",
            ],
        ),
    ];

    for (project_type, expected_files) in structure_tests {
        let project_name = format!("struct-test-{}", project_type);
        let project_dir = temp_dir.path().join(&project_name);
        let config = create_test_config(&project_name, project_type);

        generator
            .generate(&config, &project_dir)
            .expect(&format!("Failed to generate {} project", project_type));

        for file in expected_files {
            let file_path = project_dir.join(file);
            assert!(
                file_path.exists(),
                "{} project should have {}",
                project_type,
                file
            );

            if file.ends_with('/') {
                assert!(file_path.is_dir(), "{} should be a directory", file);
            } else {
                assert!(file_path.is_file(), "{} should be a file", file);
            }
        }

        println!("✓ {} project has correct structure", project_type);
    }
}

#[test]
fn test_gitignore_correctness() {
    // Test that .gitignore files are appropriate for each project type
    let temp_dir = create_test_dir();
    let generator = Generator::new();

    let gitignore_tests = vec![
        ("api-server", vec!["/target"], vec![]),
        ("cli-tool", vec!["/target"], vec!["Cargo.lock"]),
        ("library", vec!["/target", "Cargo.lock"], vec![]),
        (
            "wasm-app",
            vec!["/target", "node_modules", "dist/", "pkg/"],
            vec!["Cargo.lock"],
        ),
        (
            "game-engine",
            vec!["/target", "wasm/", "*.wasm", ".DS_Store"],
            vec!["Cargo.lock"],
        ),
        (
            "embedded",
            vec!["/target", "*.bin", "*.hex", "*.elf", ".vscode/"],
            vec!["Cargo.lock"],
        ),
        ("workspace", vec!["/target", "Cargo.lock"], vec![]),
    ];

    for (project_type, must_have, must_not_have) in gitignore_tests {
        let project_name = format!("gitignore-test-{}", project_type);
        let project_dir = temp_dir.path().join(&project_name);
        let config = create_test_config(&project_name, project_type);

        generator
            .generate(&config, &project_dir)
            .expect(&format!("Failed to generate {} project", project_type));

        let gitignore_path = project_dir.join(".gitignore");
        assert!(
            gitignore_path.exists(),
            "{} should have .gitignore",
            project_type
        );

        // Verify required entries
        verify_file_contains(&gitignore_path, &must_have).expect(&format!(
            ".gitignore verification failed for {}",
            project_type
        ));

        // Verify forbidden entries
        if !must_not_have.is_empty() {
            verify_file_not_contains(&gitignore_path, &must_not_have).expect(&format!(
                ".gitignore should not contain forbidden entries for {}",
                project_type
            ));
        }

        println!("✓ {} project has correct .gitignore", project_type);
    }
}

#[test]
fn test_readme_quality() {
    // Test that README files are comprehensive and helpful
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
        let project_name = format!("readme-test-{}", project_type);
        let project_dir = temp_dir.path().join(&project_name);
        let config = create_test_config(&project_name, project_type);

        generator
            .generate(&config, &project_dir)
            .expect(&format!("Failed to generate {} project", project_type));

        let readme_path = project_dir.join("README.md");
        assert!(
            readme_path.exists(),
            "{} should have README.md",
            project_type
        );

        // Verify basic structure
        verify_file_contains(
            &readme_path,
            &[
                &format!("# {}", project_name),
                &format!("Test {} project", project_type),
            ],
        )
        .expect(&format!(
            "README basic structure failed for {}",
            project_type
        ));

        // Verify README has reasonable content
        let line_count = count_lines(&readme_path).expect(&format!(
            "Failed to count lines in README for {}",
            project_type
        ));
        assert!(
            line_count > 10,
            "{} README should have substantial content",
            project_type
        );

        let file_size = get_file_size(&readme_path)
            .expect(&format!("Failed to get README size for {}", project_type));
        assert!(
            file_size > 100,
            "{} README should have reasonable size",
            project_type
        );

        println!("✓ {} project has quality README", project_type);
    }
}

#[test]
fn test_cargo_toml_quality() {
    // Test that Cargo.toml files are well-formed and complete
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
        let project_name = format!("cargo-test-{}", project_type);
        let project_dir = temp_dir.path().join(&project_name);
        let config = create_test_config(&project_name, project_type);

        generator
            .generate(&config, &project_dir)
            .expect(&format!("Failed to generate {} project", project_type));

        let cargo_toml_path = project_dir.join("Cargo.toml");
        assert!(
            cargo_toml_path.exists(),
            "{} should have Cargo.toml",
            project_type
        );

        // Verify basic metadata
        let required_fields = vec!["name =", "version =", "edition =", "authors ="];

        verify_file_contains(&cargo_toml_path, &required_fields).expect(&format!(
            "Cargo.toml basic fields failed for {}",
            project_type
        ));

        // Verify project-specific configuration
        match project_type {
            "cli-tool" => {
                verify_file_contains(&cargo_toml_path, &["[[bin]]", "path = \"src/main.rs\""])
                    .expect("CLI tool should have bin configuration");
            }
            "library" => {
                verify_file_contains(&cargo_toml_path, &["[lib]"])
                    .expect("Library should have lib configuration");
            }
            "wasm-app" => {
                verify_file_contains(&cargo_toml_path, &["[lib]", "crate-type = [\"cdylib\"]"])
                    .expect("WASM app should have cdylib configuration");
            }
            "game-engine" => {
                verify_file_contains(&cargo_toml_path, &["[profile.dev]", "opt-level = 1"])
                    .expect("Game engine should have profile optimization");
            }
            "embedded" => {
                verify_file_contains(&cargo_toml_path, &["[profile.release]", "lto = \"fat\""])
                    .expect("Embedded should have release optimization");
            }
            _ => {}
        }

        println!("✓ {} project has quality Cargo.toml", project_type);
    }
}

#[test]
fn test_project_compilation_modes() {
    // Test that projects compile in different modes
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
        let project_name = format!("compile-modes-{}", project_type);
        let project_dir = temp_dir.path().join(&project_name);
        let config = create_test_config(&project_name, project_type);

        generator
            .generate(&config, &project_dir)
            .expect(&format!("Failed to generate {} project", project_type));

        // Test cargo check (skip for embedded which needs special targets)
        if project_type != "embedded" {
            match run_cargo_check(&project_dir) {
                Ok(_) => println!("✓ {} project passes cargo check", project_type),
                Err(e) => panic!("✗ {} project fails cargo check: {}", project_type, e),
            }
        } else {
            println!(
                "✓ {} project structure validated (check requires embedded target)",
                project_type
            );
        }

        // Test cargo build (skip for embedded and wasm which need special setup)
        if !matches!(project_type, "embedded" | "wasm-app") {
            match run_cargo_build(&project_dir) {
                Ok(_) => println!("✓ {} project builds successfully", project_type),
                Err(e) => panic!("✗ {} project fails to build: {}", project_type, e),
            }
        } else {
            println!(
                "✓ {} project generation completed (build requires special setup)",
                project_type
            );
        }

        // Test cargo test (skip for embedded and wasm which may need special setup)
        if !matches!(project_type, "embedded" | "wasm-app") {
            match run_cargo_test(&project_dir) {
                Ok(_) => println!("✓ {} project passes tests", project_type),
                Err(e) => panic!("✗ {} project fails tests: {}", project_type, e),
            }
        }
    }
}

#[test]
fn test_project_name_edge_cases() {
    // Test project generation with various name formats
    let temp_dir = create_test_dir();
    let generator = Generator::new();

    let name_tests = vec![
        ("simple-name", "library"),
        ("name_with_underscores", "library"),
        ("name-with-hyphens", "library"),
        ("name123with456numbers", "library"),
        ("very-long-project-name-that-should-still-work", "library"),
        ("a", "library"), // Single character
        (
            "project_name_with_multiple_underscores_and_hyphens",
            "library",
        ),
    ];

    for (project_name, project_type) in name_tests {
        let project_dir = temp_dir.path().join(project_name);
        let config = create_test_config(project_name, project_type);

        generator.generate(&config, &project_dir).expect(&format!(
            "Failed to generate project with name '{}'",
            project_name
        ));

        // Verify the project compiles
        match run_cargo_check(&project_dir) {
            Ok(_) => println!("✓ Project with name '{}' compiles", project_name),
            Err(e) => panic!("✗ Project with name '{}' failed: {}", project_name, e),
        }

        // Verify name is correctly set in Cargo.toml
        let cargo_toml_path = project_dir.join("Cargo.toml");
        let content = fs::read_to_string(&cargo_toml_path).expect("Failed to read Cargo.toml");

        // For libraries, name might be converted (hyphens to underscores)
        if project_type == "library" {
            let expected_lib_name = project_name.replace('-', "_");
            assert!(
                content.contains(&expected_lib_name) || content.contains(project_name),
                "Library name should be properly handled in Cargo.toml"
            );
        } else {
            assert!(
                content.contains(project_name),
                "Project name should be in Cargo.toml"
            );
        }
    }
}

#[test]
fn test_cross_platform_compatibility() {
    // Test that generated projects work across different platforms
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
        let project_name = format!("cross-platform-{}", project_type);
        let project_dir = temp_dir.path().join(&project_name);
        let config = create_test_config(&project_name, project_type);

        generator
            .generate(&config, &project_dir)
            .expect(&format!("Failed to generate {} project", project_type));

        // Test that all files use correct line endings and are readable
        let files_to_check = vec!["Cargo.toml", "README.md", ".gitignore"];
        for file in files_to_check {
            let file_path = project_dir.join(file);
            if file_path.exists() {
                let content = fs::read_to_string(&file_path)
                    .expect(&format!("Should be able to read {}", file));

                // Verify file is not empty and has reasonable content
                assert!(!content.is_empty(), "{} should not be empty", file);
                assert!(
                    content.len() > 10,
                    "{} should have reasonable content",
                    file
                );

                // Verify file doesn't have obvious encoding issues
                assert!(
                    !content.contains('\0'),
                    "{} should not contain null bytes",
                    file
                );
            }
        }

        println!("✓ {} project is cross-platform compatible", project_type);
    }
}

#[test]
fn test_project_generation_stress() {
    // Stress test by generating many projects quickly
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

    let num_iterations = 50;
    let start_time = std::time::Instant::now();

    for i in 0..num_iterations {
        let project_type = &project_types[i % project_types.len()];
        let project_name = format!("stress-test-{}-{}", project_type, i);
        let project_dir = temp_dir.path().join(&project_name);
        let config = create_test_config(&project_name, project_type);

        generator.generate(&config, &project_dir).expect(&format!(
            "Failed to generate {} project in stress test",
            project_type
        ));

        // Quick verification
        assert!(project_dir.join("Cargo.toml").exists());
        assert!(project_dir.join("README.md").exists());
        assert!(project_dir.join(".gitignore").exists());
    }

    let elapsed = start_time.elapsed();
    println!("✓ Generated {} projects in {:?}", num_iterations, elapsed);

    // Verify performance is reasonable
    let avg_time = elapsed.as_millis() / num_iterations as u128;
    assert!(
        avg_time < 1000,
        "Average project generation time should be under 1 second"
    );
}
