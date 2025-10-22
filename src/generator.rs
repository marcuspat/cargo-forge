use crate::external_generators;
use crate::features::ci::CIPlugin;
use crate::features::database::DatabasePlugin;
use crate::features::docker::{DockerBuildStage, DockerPlugin};
use crate::features::{PluginManager, ProjectContext as FeatureContext};
use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub project_type: String,
    pub author: String,
    pub description: Option<String>,
    pub features: Vec<String>,
    pub target: Option<String>,
    pub esp32_chip: Option<String>,
}

pub struct Generator;

impl Generator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, config: &ProjectConfig, output_dir: &Path) -> Result<()> {
        // For ESP32 projects do not create the directory structure. esp-generate will handle it
        if let Some(target) = &config.target {
            if target == "esp32" {
                if let Some(parent) = output_dir.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent)?;
                    }
                }
                // Call esp-generate directory without creating project structure
                return self.generate_embedded(config, output_dir);
            }
        }

        // Create directory if it doesn't exist, but check for conflicts first
        if output_dir.exists() {
            if !output_dir.is_dir() {
                return Err(anyhow!(
                    "Output path exists but is not a directory: {}",
                    output_dir.display()
                ));
            }
            // Directory exists and is a directory - check if it's empty for safety
            if output_dir.read_dir()?.next().is_some() {
                return Err(anyhow!("Directory '{}' is not empty", output_dir.display()));
            }
        } else {
            // Create the directory
            fs::create_dir_all(output_dir)?;
        }

        // Only create src and tests directories for non-workspace projects
        if config.project_type != "workspace" {
            fs::create_dir_all(output_dir.join("src"))?;
            fs::create_dir_all(output_dir.join("tests"))?;
        }

        // Generate files based on project type
        match config.project_type.as_str() {
            "api-server" => self.generate_api_server(config, output_dir)?,
            "cli-tool" => self.generate_cli_tool(config, output_dir)?,
            "library" => self.generate_library(config, output_dir)?,
            "wasm-app" => self.generate_wasm_app(config, output_dir)?,
            "game-engine" => self.generate_game_engine(config, output_dir)?,
            "embedded" => self.generate_embedded(config, output_dir)?,
            "workspace" => self.generate_workspace(config, output_dir)?,
            _ => return Err(anyhow!("Unknown project type: {}", config.project_type)),
        }

        // Create feature context and apply plugins before generating common files
        let mut feature_context = FeatureContext::new(&config.name);
        if !config.features.is_empty() {
            let mut plugin_manager = PluginManager::new();

            // Register plugins based on selected features
            for feature in &config.features {
                match feature.as_str() {
                    "docker" => {
                        let port = match config.project_type.as_str() {
                            "api-server" => Some(3000),
                            "wasm-app" => Some(8080),
                            _ => None,
                        };
                        let mut docker_plugin =
                            DockerPlugin::new().with_build_stage(DockerBuildStage::MultiStage);
                        if let Some(p) = port {
                            docker_plugin = docker_plugin.expose_port(p);
                        }
                        plugin_manager.register(Box::new(docker_plugin));
                    }
                    "ci" | "github-actions" => {
                        use crate::features::ci::CIPlatform;
                        plugin_manager.register(Box::new(CIPlugin::new(CIPlatform::GitHubActions)));
                    }
                    "database" => {
                        use crate::features::database::DatabaseType;
                        plugin_manager
                            .register(Box::new(DatabasePlugin::new(DatabaseType::PostgreSQL)));
                    }
                    "postgres" => {
                        use crate::features::database::DatabaseType;
                        plugin_manager
                            .register(Box::new(DatabasePlugin::new(DatabaseType::PostgreSQL)));
                    }
                    "sqlite" => {
                        use crate::features::database::DatabaseType;
                        plugin_manager
                            .register(Box::new(DatabasePlugin::new(DatabaseType::SQLite)));
                    }
                    "mysql" => {
                        use crate::features::database::DatabaseType;
                        plugin_manager.register(Box::new(DatabasePlugin::new(DatabaseType::MySQL)));
                    }
                    _ => {
                        // Unknown features are ignored
                    }
                }
            }

            // Apply all plugins
            plugin_manager
                .configure_all(&mut feature_context)
                .map_err(|e| anyhow!("Plugin configuration failed: {}", e))?;
        }

        // Generate common files with feature integration
        self.generate_cargo_toml(config, output_dir)?;
        self.generate_gitignore_with_features(config, output_dir, &feature_context)?;
        self.generate_readme_with_features(config, output_dir, &feature_context)?;

        // Generate feature-specific files
        if !config.features.is_empty() {
            self.generate_feature_files(&feature_context, output_dir)?;
        }

        Ok(())
    }

    fn generate_feature_files(
        &self,
        feature_context: &FeatureContext,
        output_dir: &Path,
    ) -> Result<()> {
        // Create directories specified by plugins
        for dir in &feature_context.directories {
            fs::create_dir_all(output_dir.join(dir))?;
        }

        // Write template files from plugins
        for (path, content) in &feature_context.template_files {
            let file_path = output_dir.join(path);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&file_path, content)?;

            // Make scripts executable on Unix
            #[cfg(unix)]
            if path.starts_with("scripts/") && path.ends_with(".sh") {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&file_path)?.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&file_path, perms)?;
            }
        }

        Ok(())
    }

    fn generate_gitignore_with_features(
        &self,
        config: &ProjectConfig,
        output_dir: &Path,
        feature_context: &FeatureContext,
    ) -> Result<()> {
        let mut content = String::from("/target\n");

        // Add Cargo.lock for libraries
        if config.project_type == "library" {
            content.push_str("Cargo.lock\n");
        }

        // Add project-type specific ignores
        match config.project_type.as_str() {
            "wasm-app" => {
                content.push_str("node_modules\n");
                content.push_str("dist/\n");
                content.push_str("pkg/\n");
            }
            "game-engine" => {
                content.push_str("wasm/\n");
                content.push_str("*.wasm\n");
                content.push_str(".DS_Store\n");
            }
            "embedded" => {
                content.push_str("*.bin\n");
                content.push_str("*.hex\n");
                content.push_str("*.elf\n");
                content.push_str(".vscode/\n");
            }
            "workspace" => {
                content.push_str("Cargo.lock\n");
            }
            _ => {}
        }

        // Add feature-specific gitignore entries
        for entry in &feature_context.gitignore_entries {
            content.push_str(entry);
            content.push('\n');
        }

        fs::write(output_dir.join(".gitignore"), content)?;
        Ok(())
    }

    fn generate_readme_with_features(
        &self,
        config: &ProjectConfig,
        output_dir: &Path,
        feature_context: &FeatureContext,
    ) -> Result<()> {
        let mut content = format!("# {}\n\n", config.name);

        if let Some(desc) = &config.description {
            content.push_str(desc);
            content.push_str("\n\n");
        }

        // Add project-type specific content
        match config.project_type.as_str() {
            "api-server" => {
                content.push_str("## API Server\n\n");
                content.push_str("This is a REST API server built with Axum.\n\n");
                content.push_str("### Endpoints\n\n");
                content.push_str("- `GET /` - Health check endpoint\n");
                content.push_str("- More endpoints coming soon...\n\n");
                content.push_str("### Running\n\n");
                content.push_str("```bash\ncargo run\n```\n\n");
                content.push_str("The server will start on `http://localhost:3000`\n");
            }
            "cli-tool" => {
                content.push_str("## CLI Tool\n\n");
                content.push_str("### Usage\n\n");
                content.push_str("```bash\ncargo run -- --help\n```\n\n");
                content.push_str("### Commands\n\n");
                content.push_str("Available commands and arguments will be shown in help.\n");
            }
            "library" => {
                content.push_str("## Library\n\n");
                content.push_str("### Usage\n\n");
                content.push_str("Add this to your `Cargo.toml`:\n\n");
                content.push_str("```toml\n[dependencies]\n");
                content.push_str(&format!("{} = \"0.1.0\"\n", config.name));
                content.push_str("```\n\n");
                content.push_str("### Example\n\n");
                content.push_str("```rust\n// Example usage\n```\n\n");
                content.push_str("### API Documentation\n\n");
                content.push_str("Run `cargo doc --open` to view the documentation.\n");
            }
            _ => {}
        }

        // Add feature-specific readme sections
        for section in &feature_context.readme_sections {
            content.push_str(section);
            content.push_str("\n");
        }

        fs::write(output_dir.join("README.md"), content)?;
        Ok(())
    }

    fn generate_api_server(&self, _config: &ProjectConfig, output_dir: &Path) -> Result<()> {
        // Create API server specific files
        fs::write(output_dir.join("src/main.rs"), "fn main() {}\n")?;
        fs::write(output_dir.join("src/routes.rs"), "")?;
        fs::write(output_dir.join("src/handlers.rs"), "")?;
        fs::write(output_dir.join("src/models.rs"), "")?;

        fs::create_dir_all(output_dir.join("config"))?;
        fs::write(output_dir.join("config/default.toml"), "")?;
        fs::write(output_dir.join(".env.example"), "")?;

        Ok(())
    }

    fn generate_cli_tool(&self, _config: &ProjectConfig, output_dir: &Path) -> Result<()> {
        // Create CLI tool specific files
        fs::write(output_dir.join("src/main.rs"), "fn main() {}\n")?;
        fs::write(output_dir.join("src/cli.rs"), "")?;
        fs::write(output_dir.join("src/commands.rs"), "")?;

        Ok(())
    }

    fn generate_library(&self, config: &ProjectConfig, output_dir: &Path) -> Result<()> {
        // Create library specific files
        let lib_content = format!(
            "//! {}\n\n",
            config.description.as_deref().unwrap_or("A Rust library")
        );
        fs::write(output_dir.join("src/lib.rs"), lib_content)?;

        fs::create_dir_all(output_dir.join("examples"))?;
        fs::write(output_dir.join("examples/basic.rs"), "fn main() {}\n")?;

        Ok(())
    }

    fn generate_wasm_app(&self, _config: &ProjectConfig, output_dir: &Path) -> Result<()> {
        // Create WASM app specific files
        fs::write(output_dir.join("src/lib.rs"), "")?;
        fs::write(output_dir.join("index.html"), "")?;
        fs::write(output_dir.join("index.js"), "")?;
        fs::write(output_dir.join("package.json"), "{}")?;
        fs::write(output_dir.join("webpack.config.js"), "")?;
        fs::write(output_dir.join("build.sh"), "#!/bin/bash\n")?;

        Ok(())
    }

    fn generate_game_engine(&self, _config: &ProjectConfig, output_dir: &Path) -> Result<()> {
        // Create game engine specific files
        fs::write(output_dir.join("src/main.rs"), "fn main() {}\n")?;

        // Create assets directory structure
        fs::create_dir_all(output_dir.join("assets/models"))?;
        fs::create_dir_all(output_dir.join("assets/textures"))?;
        fs::create_dir_all(output_dir.join("assets/sounds"))?;
        fs::create_dir_all(output_dir.join("assets/shaders"))?;

        // Create basic asset README
        fs::write(
            output_dir.join("assets/README.md"),
            "# Assets\n\nPlace your game assets here.",
        )?;

        // Create GitHub Actions for WASM builds
        fs::create_dir_all(output_dir.join(".github/workflows"))?;
        fs::write(output_dir.join(".github/workflows/wasm.yml"), "")?;

        Ok(())
    }

    fn generate_embedded(&self, config: &ProjectConfig, output_dir: &Path) -> Result<()> {
        // Check if it is an esp32 project
        if let Some(target) = &config.target {
            if target == "esp32" {
                let chip = config.esp32_chip.as_deref().unwrap_or("esp32");
                println!("🔧 Generating project for chip : {}", chip);
                return external_generators::generate_esp32_project(&config.name, chip, output_dir);
            }
        }

        // Default to Cortex-M embedded project (existing logic)
        println!("🔧 Generating Cortex-M embedded project");
        self.generate_cortex_m_embedded(config, output_dir)
    }

    fn generate_cortex_m_embedded(&self, _config: &ProjectConfig, output_dir: &Path) -> Result<()> {
        // Create embedded specific files with proper no_std setup
        let main_content = r#"#![no_std]
#![no_main]

use panic_halt as _; // panic handler

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use the heap
    
    // Main application logic
    loop {
        // Your code here
    }
}
"#;
        fs::write(output_dir.join("src/main.rs"), main_content)?;

        // Create cargo config
        fs::create_dir_all(output_dir.join(".cargo"))?;
        let cargo_config_content = r#"[target.thumbv7em-none-eabihf]
runner = "probe-rs-cli run --chip STM32F401RETx"

[build]
target = "thumbv7em-none-eabihf"

[env]
DEFMT_LOG = "debug"
"#;
        fs::write(output_dir.join(".cargo/config.toml"), cargo_config_content)?;

        // Create memory layout file
        let memory_x_content = r#"/* Linker script for the STM32F401RET6 */
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K
  RAM : ORIGIN = 0x20000000, LENGTH = 96K
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* You may want to use this variable to locate the call stack and static
   variables in different memory regions. Below is shown the default value */
/* _stack_start = ORIGIN(RAM) + LENGTH(RAM); */

/* You can use this symbol to customize the location of the .text section */
/* If omitted the .text section will be placed right after the .vector_table
   section */
/* This can be useful if you want to move the firmware to some address other
   than the default one (= 0x00000000 in thumb mode, 0x00000008 in non-thumb mode) */
/* ENTRY_POINT = 0x08000000; */
"#;
        fs::write(output_dir.join("memory.x"), memory_x_content)?;

        // Create probe-rs config
        let embed_toml_content = r#"[default.probe]
protocol = "Swd"

[default.flashing]
enabled = true

[default.reset]
enabled = true

[default.general]
chip = "STM32F401RETx"

[default.rtt]
enabled = true
up_mode = "NoBlockSkip"
"#;
        fs::write(output_dir.join("Embed.toml"), embed_toml_content)?;

        Ok(())
    }

    fn generate_workspace(&self, config: &ProjectConfig, output_dir: &Path) -> Result<()> {
        // Create workspace structure
        fs::create_dir_all(output_dir.join("crates/core/src"))?;
        fs::create_dir_all(output_dir.join("crates/api/src"))?;
        fs::create_dir_all(output_dir.join("crates/cli/src"))?;

        // Create core crate
        fs::write(
            output_dir.join("crates/core/src/lib.rs"),
            "//! Core library\n\npub fn hello() {\n    println!(\"Hello from core!\");\n}\n",
        )?;
        fs::write(output_dir.join("crates/core/src/error.rs"), "//! Error types for the core library\n\nuse std::fmt;\n\n#[derive(Debug)]\npub enum CoreError {\n    Generic(String),\n}\n\nimpl fmt::Display for CoreError {\n    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {\n        match self {\n            CoreError::Generic(msg) => write!(f, \"Core error: {}\", msg),\n        }\n    }\n}\n\nimpl std::error::Error for CoreError {}\n")?;
        fs::write(output_dir.join("crates/core/src/lib.rs"), "//! Core library\n\npub mod error;\n\npub use error::CoreError;\n\npub fn hello() {\n    println!(\"Hello from core!\");\n}\n")?;
        fs::write(output_dir.join("crates/core/src/models.rs"), "//! Data models\n\n#[derive(Debug, Clone)]\npub struct User {\n    pub id: u64,\n    pub name: String,\n    pub email: String,\n}\n\nimpl User {\n    pub fn new(id: u64, name: String, email: String) -> Self {\n        Self { id, name, email }\n    }\n}\n")?;
        fs::write(output_dir.join("crates/core/src/utils.rs"), "//! Utility functions\n\npub fn format_name(first: &str, last: &str) -> String {\n    format!(\"{} {}\", first, last)\n}\n\npub fn validate_email(email: &str) -> bool {\n    email.contains('@') && email.contains('.')\n}\n")?;

        let core_cargo_toml = format!(
            r#"[package]
name = "{}-core"
version = "0.1.0"
edition = "2021"
authors = ["{}"]

[dependencies]
serde = {{ version = "1.0", features = ["derive"] }}
anyhow = "1.0"
"#,
            config.name, config.author
        );
        fs::write(output_dir.join("crates/core/Cargo.toml"), core_cargo_toml)?;

        // Create API crate
        fs::write(output_dir.join("crates/api/src/lib.rs"), "//! API library\n\nuse anyhow::Result;\n\npub fn start_server() -> Result<()> {\n    println!(\"Starting API server...\");\n    Ok(())\n}\n")?;
        fs::write(output_dir.join("crates/api/src/state.rs"), "//! Application state management\n\nuse std::sync::Arc;\nuse tokio::sync::RwLock;\n\n#[derive(Clone)]\npub struct AppState {\n    pub counter: Arc<RwLock<u64>>,\n}\n\nimpl AppState {\n    pub fn new() -> Self {\n        Self {\n            counter: Arc::new(RwLock::new(0)),\n        }\n    }\n}\n\nimpl Default for AppState {\n    fn default() -> Self {\n        Self::new()\n    }\n}\n")?;

        let api_cargo_toml = format!(
            r#"[package]
name = "{}-api"
version = "0.1.0"
edition = "2021"
authors = ["{}"]

[dependencies]
{}-core = {{ path = "../core" }}
tokio = {{ version = "1", features = ["full"] }}
anyhow = "1.0"
serde = {{ version = "1.0", features = ["derive"] }}

[lib]
name = "{}_api"
"#,
            config.name,
            config.author,
            config.name,
            config.name.replace('-', "_")
        );
        fs::write(output_dir.join("crates/api/Cargo.toml"), api_cargo_toml)?;

        // Create CLI crate
        fs::write(output_dir.join("crates/cli/src/main.rs"), &format!("use {}_core::hello;\nuse anyhow::Result;\n\nfn main() -> Result<()> {{\n    println!(\"Welcome to {}!\");\n    hello();\n    Ok(())\n}}\n", config.name.replace('-', "_"), config.name))?;

        let cli_cargo_toml = format!(
            r#"[package]
name = "{}-cli"
version = "0.1.0"
edition = "2021"
authors = ["{}"]

[[bin]]
name = "{}"
path = "src/main.rs"

[dependencies]
{}-core = {{ path = "../core" }}
clap = {{ version = "4", features = ["derive"] }}
anyhow = "1.0"
"#,
            config.name, config.author, config.name, config.name
        );
        fs::write(output_dir.join("crates/cli/Cargo.toml"), cli_cargo_toml)?;

        Ok(())
    }

    fn generate_cargo_toml(&self, config: &ProjectConfig, output_dir: &Path) -> Result<()> {
        // Skip generating Cargo.toml for esp32 projects
        if let Some(target) = &config.target {
            if target == "esp32" {
                return Ok(());
            }
        }
        let mut content = String::new();

        // Generate workspace Cargo.toml for workspace projects
        if config.project_type == "workspace" {
            // Add package section for workspace root
            content.push_str("[package]\n");
            content.push_str(&format!(r#"name = "{}""#, config.name));
            content.push('\n');
            content.push_str(r#"version = "0.1.0""#);
            content.push('\n');
            content.push_str(&format!(r#"authors = ["{}"]"#, config.author));
            content.push('\n');
            content.push_str(r#"edition = "2021""#);
            content.push('\n');
            if let Some(desc) = &config.description {
                content.push_str(&format!(r#"description = "{}""#, desc));
                content.push('\n');
            }
            content.push_str("\n");

            content.push_str("[workspace]\n");
            content.push_str("resolver = \"2\"\n");
            content.push_str("members = [\n");
            content.push_str("  \"crates/core\",\n");
            content.push_str("  \"crates/api\",\n");
            content.push_str("  \"crates/cli\",\n");
            content.push_str("]\n\n");

            content.push_str("[workspace.package]\n");
            content.push_str(&format!(r#"version = "0.1.0""#));
            content.push('\n');
            content.push_str(&format!(r#"authors = ["{}"]"#, config.author));
            content.push('\n');
            content.push_str(r#"edition = "2021""#);
            content.push('\n');
            if let Some(desc) = &config.description {
                content.push_str(&format!(r#"description = "{}""#, desc));
                content.push('\n');
            }
            content.push_str("\n");

            content.push_str("[workspace.dependencies]\n");
            content.push_str("tokio = { version = \"1\", features = [\"full\"] }\n");
            content.push_str("serde = { version = \"1\", features = [\"derive\"] }\n");
            content.push_str("anyhow = \"1\"\n");
            content.push_str("clap = { version = \"4\", features = [\"derive\"] }\n");

            fs::write(output_dir.join("Cargo.toml"), content)?;
            return Ok(());
        }

        // Generate regular package Cargo.toml for other project types
        content.push_str("[package]\n");
        content.push_str(&format!(r#"name = "{}""#, config.name));
        content.push('\n');
        content.push_str(r#"version = "0.1.0""#);
        content.push('\n');
        content.push_str(&format!(r#"authors = ["{}"]"#, config.author));
        content.push('\n');
        content.push_str(r#"edition = "2021""#);
        content.push('\n');

        if let Some(desc) = &config.description {
            content.push_str(&format!(r#"description = "{}""#, desc));
            content.push('\n');
        }

        content.push_str("\n[dependencies]\n");

        // Add project-type specific dependencies
        match config.project_type.as_str() {
            "api-server" => {
                content.push_str("axum = \"0.7\"\n");
                content.push_str("tokio = { version = \"1\", features = [\"full\"] }\n");
                content.push_str("serde = { version = \"1\", features = [\"derive\"] }\n");
                content.push_str("tower = \"0.4\"\n");
            }
            "cli-tool" => {
                content.push_str("clap = { version = \"4\", features = [\"derive\"] }\n");
                content.push_str("anyhow = \"1\"\n");
                content.push_str("env_logger = \"0.10\"\n");

                content.push_str("\n[[bin]]\n");
                content.push_str(&format!(r#"name = "{}""#, config.name));
                content.push('\n');
                content.push_str("path = \"src/main.rs\"\n");
            }
            "library" => {
                content.push_str("\n[lib]\n");
                content.push_str(&format!(r#"name = "{}""#, config.name.replace('-', "_")));
                content.push('\n');
            }
            "wasm-app" => {
                content.push_str("wasm-bindgen = \"0.2\"\n");
                content.push_str("web-sys = \"0.3\"\n");
                content.push_str("js-sys = \"0.3\"\n");

                content.push_str("\n[lib]\n");
                content.push_str(r#"crate-type = ["cdylib"]"#);
                content.push('\n');
            }
            "game-engine" => {
                content.push_str("bevy = \"0.12\"\n");

                content.push_str("\n[target.'cfg(target_arch = \"wasm32\")'.dependencies]\n");
                content.push_str("wasm-bindgen = \"0.2\"\n");
                content.push_str("web-sys = \"0.3\"\n");
                content.push_str("console_error_panic_hook = \"0.1\"\n");

                content.push_str("\n[profile.dev]\n");
                content.push_str("opt-level = 1\n");
                content.push_str("\n[profile.dev.package.\"*\"]\n");
                content.push_str("opt-level = 3\n");
            }
            "embedded" => {
                content.push_str("cortex-m = \"0.7\"\n");
                content.push_str("cortex-m-rt = \"0.7\"\n");
                content.push_str("panic-halt = \"0.2\"\n");

                content.push_str("\n[profile.dev]\n");
                content.push_str("opt-level = 1\n");
                content.push_str("\n[profile.release]\n");
                content.push_str("lto = \"fat\"\n");
                content.push_str("opt-level = 3\n");
            }
            "workspace" => {
                // Workspace projects have different structure - skip dependencies here
            }
            _ => {}
        }

        fs::write(output_dir.join("Cargo.toml"), content)?;
        Ok(())
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}
