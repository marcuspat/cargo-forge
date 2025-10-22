use cargo_forge::{Generator, ProjectConfig, ProjectType};
use std::fs;
use std::str::FromStr;
use tempfile::TempDir;

#[test]
fn test_api_server_project_type() {
    // This test should fail as the implementation doesn't exist yet
    let temp_dir = TempDir::new().unwrap();
    let output_dir = temp_dir.path().join("api-server-test");

    let config = ProjectConfig {
        name: "api-server-test".to_string(),
        project_type: ProjectType::ApiServer.to_string(),
        author: "Test Author".to_string(),
        description: Some("Test API Server".to_string()),
        features: vec![],
    };

    let generator = Generator::new();
    generator.generate(&config, &output_dir).unwrap();

    // Check API server specific files and dependencies
    assert!(
        output_dir.join("src/main.rs").exists(),
        "API server should have main.rs"
    );
    assert!(
        output_dir.join("src/routes.rs").exists(),
        "API server should have routes module"
    );
    assert!(
        output_dir.join("src/handlers.rs").exists(),
        "API server should have handlers module"
    );
    assert!(
        output_dir.join("src/models.rs").exists(),
        "API server should have models module"
    );

    // Check Cargo.toml for API server dependencies
    let cargo_content = fs::read_to_string(output_dir.join("Cargo.toml")).unwrap();
    assert!(
        cargo_content.contains("axum"),
        "API server should have axum dependency"
    );
    assert!(
        cargo_content.contains("tokio"),
        "API server should have tokio dependency"
    );
    assert!(
        cargo_content.contains("serde"),
        "API server should have serde dependency"
    );
    assert!(
        cargo_content.contains("tower"),
        "API server should have tower dependency"
    );

    // Check for config files
    assert!(
        output_dir.join(".env.example").exists(),
        "API server should have .env.example"
    );
    assert!(
        output_dir.join("config/default.toml").exists(),
        "API server should have config file"
    );
}

#[test]
fn test_cli_tool_project_type() {
    // Test CLI tool project generation
    let temp_dir = TempDir::new().unwrap();
    let output_dir = temp_dir.path().join("cli-tool-test");

    let config = ProjectConfig {
        name: "cli-tool-test".to_string(),
        project_type: ProjectType::CliTool.to_string(),
        author: "Test Author".to_string(),
        description: Some("Test CLI Tool".to_string()),
        features: vec![],
    };

    let generator = Generator::new();
    generator.generate(&config, &output_dir).unwrap();

    // Check CLI tool specific files
    assert!(
        output_dir.join("src/main.rs").exists(),
        "CLI tool should have main.rs"
    );
    assert!(
        output_dir.join("src/cli.rs").exists(),
        "CLI tool should have cli module"
    );
    assert!(
        output_dir.join("src/commands.rs").exists(),
        "CLI tool should have commands module"
    );

    // Check Cargo.toml for CLI dependencies
    let cargo_content = fs::read_to_string(output_dir.join("Cargo.toml")).unwrap();
    assert!(
        cargo_content.contains("clap"),
        "CLI tool should have clap dependency"
    );
    assert!(
        cargo_content.contains("anyhow"),
        "CLI tool should have anyhow dependency"
    );
    assert!(
        cargo_content.contains("env_logger"),
        "CLI tool should have env_logger dependency"
    );

    // Check for CLI-specific features in Cargo.toml
    assert!(
        cargo_content.contains("[[bin]]"),
        "CLI tool should have binary target"
    );
    assert!(
        cargo_content.contains(r#"name = "cli-tool-test""#),
        "Binary name should match project"
    );
}

#[test]
fn test_library_project_type() {
    // Test library project generation
    let temp_dir = TempDir::new().unwrap();
    let output_dir = temp_dir.path().join("library-test");

    let config = ProjectConfig {
        name: "library-test".to_string(),
        project_type: ProjectType::Library.to_string(),
        author: "Test Author".to_string(),
        description: Some("Test Library".to_string()),
        features: vec![],
    };

    let generator = Generator::new();
    generator.generate(&config, &output_dir).unwrap();

    // Check library specific files
    assert!(
        output_dir.join("src/lib.rs").exists(),
        "Library should have lib.rs"
    );
    assert!(
        !output_dir.join("src/main.rs").exists(),
        "Library should not have main.rs"
    );

    // Check for examples directory
    assert!(
        output_dir.join("examples").exists(),
        "Library should have examples directory"
    );
    assert!(
        output_dir.join("examples/basic.rs").exists(),
        "Library should have basic example"
    );

    // Check Cargo.toml library configuration
    let cargo_content = fs::read_to_string(output_dir.join("Cargo.toml")).unwrap();
    assert!(
        cargo_content.contains("[lib]"),
        "Library should have [lib] section"
    );
    assert!(
        cargo_content.contains(r#"name = "library_test""#),
        "Library name should be snake_case"
    );

    // Check for documentation
    assert!(output_dir.join("src/lib.rs").exists());
    let lib_content = fs::read_to_string(output_dir.join("src/lib.rs")).unwrap();
    assert!(
        lib_content.contains("//!"),
        "Library should have crate-level documentation"
    );
}

#[test]
fn test_wasm_app_project_type() {
    // Test WASM application project generation
    let temp_dir = TempDir::new().unwrap();
    let output_dir = temp_dir.path().join("wasm-app-test");

    let config = ProjectConfig {
        name: "wasm-app-test".to_string(),
        project_type: ProjectType::WasmApp.to_string(),
        author: "Test Author".to_string(),
        description: Some("Test WASM App".to_string()),
        features: vec![],
    };

    let generator = Generator::new();
    generator.generate(&config, &output_dir).unwrap();

    // Check WASM specific files
    assert!(
        output_dir.join("src/lib.rs").exists(),
        "WASM app should have lib.rs"
    );
    assert!(
        output_dir.join("index.html").exists(),
        "WASM app should have index.html"
    );
    assert!(
        output_dir.join("index.js").exists(),
        "WASM app should have index.js"
    );
    assert!(
        output_dir.join("package.json").exists(),
        "WASM app should have package.json"
    );
    assert!(
        output_dir.join("webpack.config.js").exists(),
        "WASM app should have webpack config"
    );

    // Check Cargo.toml for WASM dependencies
    let cargo_content = fs::read_to_string(output_dir.join("Cargo.toml")).unwrap();
    assert!(
        cargo_content.contains("wasm-bindgen"),
        "WASM app should have wasm-bindgen"
    );
    assert!(
        cargo_content.contains("web-sys"),
        "WASM app should have web-sys"
    );
    assert!(
        cargo_content.contains("js-sys"),
        "WASM app should have js-sys"
    );
    assert!(
        cargo_content.contains("[lib]"),
        "WASM app should have [lib] section"
    );
    assert!(
        cargo_content.contains(r#"crate-type = ["cdylib"]"#),
        "WASM app should be cdylib"
    );

    // Check for build script
    assert!(
        output_dir.join("build.sh").exists() || output_dir.join("build.rs").exists(),
        "WASM app should have build script"
    );
}

#[test]
fn test_project_type_specific_readme() {
    // Test that each project type generates appropriate README content
    let temp_dir = TempDir::new().unwrap();
    let types = vec![
        (
            ProjectType::ApiServer,
            "API Server",
            vec!["endpoint", "axum", "rest"],
        ),
        (
            ProjectType::CliTool,
            "CLI Tool",
            vec!["command", "usage", "help"],
        ),
        (
            ProjectType::Library,
            "Library",
            vec!["example", "cargo", "documentation"],
        ),
        (
            ProjectType::WasmApp,
            "WASM App",
            vec!["webpack", "install", "build"],
        ),
    ];

    for (project_type, type_name, expected_words) in types {
        let output_dir = temp_dir.path().join(format!(
            "{}-readme-test",
            type_name.to_lowercase().replace(" ", "-")
        ));

        let config = ProjectConfig {
            name: format!("{}-test", type_name.to_lowercase().replace(" ", "-")),
            project_type: project_type.to_string(),
            author: "Test Author".to_string(),
            description: Some(format!("Test {}", type_name)),
            features: vec![],
        };

        let generator = Generator::new();
        generator.generate(&config, &output_dir).unwrap();

        let readme_content = fs::read_to_string(output_dir.join("README.md")).unwrap();

        // Check for type-specific content
        for word in expected_words {
            assert!(
                readme_content.to_lowercase().contains(word),
                "{} README should contain '{}'",
                type_name,
                word
            );
        }
    }
}

#[test]
fn test_project_type_specific_gitignore() {
    // Test that each project type has appropriate .gitignore entries
    let temp_dir = TempDir::new().unwrap();

    // Test WASM app specific gitignore
    let wasm_dir = temp_dir.path().join("wasm-gitignore-test");
    let wasm_config = ProjectConfig {
        name: "wasm-gitignore-test".to_string(),
        project_type: ProjectType::WasmApp.to_string(),
        author: "Test Author".to_string(),
        description: None,
        features: vec![],
    };

    let generator = Generator::new();
    generator.generate(&wasm_config, &wasm_dir).unwrap();

    let gitignore_content = fs::read_to_string(wasm_dir.join(".gitignore")).unwrap();
    assert!(
        gitignore_content.contains("node_modules"),
        "WASM .gitignore should exclude node_modules"
    );
    assert!(
        gitignore_content.contains("dist/"),
        "WASM .gitignore should exclude dist/"
    );
    assert!(
        gitignore_content.contains("pkg/"),
        "WASM .gitignore should exclude pkg/"
    );
}

#[test]
fn test_game_engine_project_type() {
    // Test game engine project generation
    let temp_dir = TempDir::new().unwrap();
    let output_dir = temp_dir.path().join("game-engine-test");

    let config = ProjectConfig {
        name: "game-engine-test".to_string(),
        project_type: ProjectType::GameEngine.to_string(),
        author: "Test Author".to_string(),
        description: Some("Test Game Engine".to_string()),
        features: vec![],
    };

    let generator = Generator::new();
    generator.generate(&config, &output_dir).unwrap();

    // Check game engine specific files
    assert!(
        output_dir.join("src/main.rs").exists(),
        "Game engine should have main.rs"
    );
    assert!(
        output_dir.join("assets").exists(),
        "Game engine should have assets directory"
    );
    assert!(
        output_dir.join("assets/models").exists(),
        "Game engine should have models directory"
    );
    assert!(
        output_dir.join("assets/textures").exists(),
        "Game engine should have textures directory"
    );
    assert!(
        output_dir.join("assets/sounds").exists(),
        "Game engine should have sounds directory"
    );
    assert!(
        output_dir.join("assets/shaders").exists(),
        "Game engine should have shaders directory"
    );

    // Check Cargo.toml for game engine dependencies
    let cargo_content = fs::read_to_string(output_dir.join("Cargo.toml")).unwrap();
    assert!(
        cargo_content.contains("bevy"),
        "Game engine should have bevy dependency"
    );
    assert!(
        cargo_content.contains("[profile.dev]"),
        "Game engine should have dev profile optimization"
    );

    // Check for WASM target support
    assert!(
        cargo_content.contains("wasm-bindgen"),
        "Game engine should support WASM target"
    );
}

#[test]
fn test_embedded_project_type() {
    // Test embedded project generation
    let temp_dir = TempDir::new().unwrap();
    let output_dir = temp_dir.path().join("embedded-test");

    let config = ProjectConfig {
        name: "embedded-test".to_string(),
        project_type: ProjectType::Embedded.to_string(),
        author: "Test Author".to_string(),
        description: Some("Test Embedded System".to_string()),
        features: vec![],
    };

    let generator = Generator::new();
    generator.generate(&config, &output_dir).unwrap();

    // Check embedded specific files
    assert!(
        output_dir.join("src/main.rs").exists(),
        "Embedded should have main.rs"
    );
    assert!(
        output_dir.join("memory.x").exists(),
        "Embedded should have memory.x linker script"
    );
    assert!(
        output_dir.join("Embed.toml").exists(),
        "Embedded should have Embed.toml config"
    );

    // Check Cargo.toml for embedded dependencies
    let cargo_content = fs::read_to_string(output_dir.join("Cargo.toml")).unwrap();
    assert!(
        cargo_content.contains("cortex-m"),
        "Embedded should have cortex-m dependency"
    );
    assert!(
        cargo_content.contains("cortex-m-rt"),
        "Embedded should have cortex-m-rt dependency"
    );
    assert!(
        cargo_content.contains("panic-halt"),
        "Embedded should have panic-halt dependency"
    );

    // Check for embedded-specific main.rs attributes
    let main_content = fs::read_to_string(output_dir.join("src/main.rs")).unwrap();
    assert!(
        main_content.contains("#![no_std]"),
        "Embedded main.rs should have no_std"
    );
    assert!(
        main_content.contains("#![no_main]"),
        "Embedded main.rs should have no_main"
    );
}

#[test]
fn test_workspace_project_type() {
    // Test workspace project generation
    let temp_dir = TempDir::new().unwrap();
    let output_dir = temp_dir.path().join("workspace-test");

    let config = ProjectConfig {
        name: "workspace-test".to_string(),
        project_type: ProjectType::Workspace.to_string(),
        author: "Test Author".to_string(),
        description: Some("Test Workspace".to_string()),
        features: vec![],
    };

    let generator = Generator::new();
    generator.generate(&config, &output_dir).unwrap();

    // Check workspace structure
    assert!(
        output_dir.join("Cargo.toml").exists(),
        "Workspace should have root Cargo.toml"
    );
    assert!(
        output_dir.join("crates").exists(),
        "Workspace should have crates directory"
    );
    assert!(
        output_dir.join("crates/core").exists(),
        "Workspace should have core crate"
    );
    assert!(
        output_dir.join("crates/api").exists(),
        "Workspace should have api crate"
    );
    assert!(
        output_dir.join("crates/cli").exists(),
        "Workspace should have cli crate"
    );

    // Check workspace Cargo.toml
    let cargo_content = fs::read_to_string(output_dir.join("Cargo.toml")).unwrap();
    assert!(
        cargo_content.contains("[workspace]"),
        "Should have workspace section"
    );
    assert!(
        cargo_content.contains("members ="),
        "Should have members list"
    );
    assert!(
        cargo_content.contains("crates/core"),
        "Should list core crate"
    );
    assert!(
        cargo_content.contains("crates/api"),
        "Should list api crate"
    );
    assert!(
        cargo_content.contains("crates/cli"),
        "Should list cli crate"
    );
    assert!(
        cargo_content.contains("[workspace.dependencies]"),
        "Should have workspace dependencies"
    );

    // Check individual crate files exist
    assert!(
        output_dir.join("crates/core/src/lib.rs").exists(),
        "Core crate should have lib.rs"
    );
    assert!(
        output_dir.join("crates/api/src/lib.rs").exists(),
        "API crate should have lib.rs"
    );
    assert!(
        output_dir.join("crates/cli/src/main.rs").exists(),
        "CLI crate should have main.rs"
    );
}

#[test]
fn test_project_type_enum_display() {
    // Test that ProjectType enum properly implements Display
    assert_eq!(ProjectType::ApiServer.to_string(), "api-server");
    assert_eq!(ProjectType::CliTool.to_string(), "cli-tool");
    assert_eq!(ProjectType::Library.to_string(), "library");
    assert_eq!(ProjectType::WasmApp.to_string(), "wasm-app");
    assert_eq!(ProjectType::GameEngine.to_string(), "game-engine");
    assert_eq!(ProjectType::Embedded.to_string(), "embedded");
    assert_eq!(ProjectType::Workspace.to_string(), "workspace");
}

#[test]
fn test_project_type_from_string() {
    // Test parsing ProjectType from string
    assert_eq!(
        ProjectType::from_str("api-server").unwrap(),
        ProjectType::ApiServer
    );
    assert_eq!(
        ProjectType::from_str("cli-tool").unwrap(),
        ProjectType::CliTool
    );
    assert_eq!(
        ProjectType::from_str("library").unwrap(),
        ProjectType::Library
    );
    assert_eq!(
        ProjectType::from_str("wasm-app").unwrap(),
        ProjectType::WasmApp
    );
    assert_eq!(
        ProjectType::from_str("game-engine").unwrap(),
        ProjectType::GameEngine
    );
    assert_eq!(
        ProjectType::from_str("embedded").unwrap(),
        ProjectType::Embedded
    );
    assert_eq!(
        ProjectType::from_str("workspace").unwrap(),
        ProjectType::Workspace
    );

    // Test invalid input
    assert!(ProjectType::from_str("invalid").is_err());
    assert!(ProjectType::from_str("").is_err());
}
