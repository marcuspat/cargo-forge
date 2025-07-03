use crate::{ProjectType, Generator, ProjectConfig, Config};
use anyhow::{anyhow, Result};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use inquire::{Confirm, MultiSelect, Select, Text};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::fs;

/// Context for project creation containing all user inputs
#[derive(Debug, Clone, Serialize)]
pub struct ProjectContext {
    pub name: String,
    pub project_type: ProjectType,
    pub features: Vec<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub edition: String,
}

impl ProjectContext {
    /// Build template context for Tera
    pub fn build_template_context(&self) -> HashMap<String, serde_json::Value> {
        let mut context = HashMap::new();
        context.insert("project_name".to_string(), serde_json::json!(self.name));
        context.insert("project_type".to_string(), serde_json::json!(self.project_type.to_string()));
        context.insert("features".to_string(), serde_json::json!(self.features));
        
        if let Some(author) = &self.author {
            context.insert("author".to_string(), serde_json::json!(author));
        }
        if let Some(description) = &self.description {
            context.insert("description".to_string(), serde_json::json!(description));
        }
        if let Some(license) = &self.license {
            context.insert("license".to_string(), serde_json::json!(license));
        }
        
        context.insert("edition".to_string(), serde_json::json!(self.edition));
        context
    }

    /// Convert to ProjectConfig for generator
    pub fn to_project_config(&self) -> ProjectConfig {
        ProjectConfig {
            name: self.name.clone(),
            project_type: self.project_type.to_string(),
            author: self.author.clone().unwrap_or_else(|| "Unknown".to_string()),
            description: self.description.clone(),
            features: self.features.clone(),
        }
    }
}

/// Configuration structure for saving user preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeConfig {
    pub default_author: Option<String>,
    pub default_license: Option<String>,
    pub preferred_project_types: Vec<String>,
    pub default_features: HashMap<String, Vec<String>>,
    pub edition: Option<String>,
}

impl Default for ForgeConfig {
    fn default() -> Self {
        Self {
            default_author: None,
            default_license: Some("MIT".to_string()),
            preferred_project_types: vec!["cli-tool".to_string()],
            default_features: HashMap::new(),
            edition: Some("2021".to_string()),
        }
    }
}

impl ForgeConfig {
    /// Load configuration from file
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: ForgeConfig = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    /// Get the default configuration file path
    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow!("Could not find config directory"))?;
        Ok(config_dir.join("cargo-forge").join("config.json"))
    }

    /// Load configuration from custom path
    pub fn load_from<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: ForgeConfig = serde_json::from_str(&content)?;
        Ok(config)
    }
}

pub struct Forge {
    base_path: PathBuf,
    config: Config,
}

impl Forge {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        let config = Config::load_from_home().unwrap_or_else(|_| Config::new());
        Self {
            base_path: base_path.as_ref().to_path_buf(),
            config,
        }
    }

    pub fn run(&self) -> Result<()> {
        println!("{}", "🔨 Welcome to Cargo-Forge!".bright_cyan().bold());
        println!("{}", "Let's create your new Rust project.\n".bright_white());

        // Collect project context through interactive prompts
        let context = self.collect_project_context()?;
        
        // Create project with progress indicators
        self.create_project(context)?;
        
        Ok(())
    }

    pub fn run_interactive<R: Read>(&self, _reader: &mut R) -> Result<()> {
        // This method is kept for testing purposes
        // In production, use run() for the full interactive experience
        let mut input = String::new();
        _reader.read_to_string(&mut input)?;
        
        // Parse the input (simplified for testing)
        let lines: Vec<&str> = input.trim().split('\n').collect();
        if lines.len() >= 2 {
            let project_name = lines[1];
            let project_path = self.base_path.join(project_name);
            std::fs::create_dir_all(&project_path)?;
        }
        
        Ok(())
    }

    /// Collect all project information through interactive prompts
    fn collect_project_context(&self) -> Result<ProjectContext> {
        // Make a mutable copy of config for saving choices
        let mut config = self.config.clone();
        
        // Project name with validation
        let name = self.prompt_project_name()?;
        
        // Project type selection
        let project_type = self.prompt_project_type_interactive()?;
        
        // Feature selection based on project type
        let features = self.prompt_features(&project_type)?;
        
        // Optional fields with config defaults
        let author = self.prompt_author_with_config(&mut config)?;
        let description = self.prompt_optional_field("Description", "A new Rust project")?;
        let license = self.prompt_license_with_config(&mut config)?;
        
        // Save config if any choices were remembered
        if config.remember_choices {
            let _ = config.save_to_home(); // Ignore errors for user experience
        }
        
        Ok(ProjectContext {
            name,
            project_type,
            features,
            author,
            description,
            license,
            edition: "2021".to_string(),
        })
    }

    /// Prompt for project name with validation
    fn prompt_project_name(&self) -> Result<String> {
        let name = Text::new("Project name:")
            .with_placeholder("my-awesome-project")
            .with_validator(|input: &str| {
                if input.is_empty() {
                    Ok(inquire::validator::Validation::Invalid("Project name cannot be empty".into()))
                } else if input.len() > 64 {
                    Ok(inquire::validator::Validation::Invalid("Project name is too long (max 64 characters)".into()))
                } else if input != input.to_lowercase() {
                    Ok(inquire::validator::Validation::Invalid("Project name must be lowercase".into()))
                } else if input.starts_with(|c: char| c.is_numeric()) {
                    Ok(inquire::validator::Validation::Invalid("Project name cannot start with a number".into()))
                } else if !input.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
                    Ok(inquire::validator::Validation::Invalid("Project name can only contain letters, numbers, '-', and '_'".into()))
                } else {
                    Ok(inquire::validator::Validation::Valid)
                }
            })
            .with_help_message("Must be a valid Rust package name (lowercase, no spaces)")
            .prompt()?;

        Ok(name)
    }

    /// Interactive project type selection
    fn prompt_project_type_interactive(&self) -> Result<ProjectType> {
        let options = vec![
            ("API Server", "RESTful API with Axum framework", ProjectType::ApiServer),
            ("CLI Tool", "Command-line application with Clap", ProjectType::CliTool),
            ("Library", "Reusable Rust library", ProjectType::Library),
            ("WASM App", "WebAssembly application", ProjectType::WasmApp),
            ("Game Engine", "Game development with Bevy", ProjectType::GameEngine),
            ("Embedded", "No-std embedded development", ProjectType::Embedded),
            ("Workspace", "Multi-crate workspace project", ProjectType::Workspace),
        ];

        let selection = Select::new("Project type:", options.iter().map(|(name, desc, _)| format!("{} - {}", name, desc)).collect())
            .with_help_message("Choose the type of project you want to create")
            .prompt()?;

        let project_type = options.iter()
            .find(|(name, desc, _)| format!("{} - {}", name, desc) == selection)
            .map(|(_, _, pt)| *pt)
            .ok_or_else(|| anyhow!("Invalid project type selection"))?;

        Ok(project_type)
    }

    /// Prompt for features based on project type
    fn prompt_features(&self, project_type: &ProjectType) -> Result<Vec<String>> {
        let available_features = match project_type {
            ProjectType::ApiServer => vec![
                ("axum", "Web framework", true),
                ("tokio", "Async runtime", true),
                ("serde", "Serialization", true),
                ("tower", "Middleware framework", true),
                ("sqlx", "SQL toolkit", false),
                ("jwt", "JWT authentication", false),
                ("cors", "CORS support", false),
                ("tracing", "Structured logging", false),
            ],
            ProjectType::CliTool => vec![
                ("clap", "CLI argument parsing", true),
                ("anyhow", "Error handling", true),
                ("env_logger", "Logging", true),
                ("tokio", "Async runtime", false),
                ("serde", "Serialization", false),
                ("indicatif", "Progress bars", false),
                ("colored", "Colored output", false),
            ],
            ProjectType::Library => vec![
                ("serde", "Serialization", false),
                ("thiserror", "Error types", false),
                ("async-trait", "Async traits", false),
                ("criterion", "Benchmarking", false),
            ],
            ProjectType::WasmApp => vec![
                ("wasm-bindgen", "JS bindings", true),
                ("web-sys", "Web APIs", true),
                ("js-sys", "JS APIs", true),
                ("wee_alloc", "Small allocator", false),
                ("console_error_panic_hook", "Better panic messages", false),
            ],
            ProjectType::GameEngine => vec![
                ("bevy", "Game engine framework", true),
                ("audio", "Audio support", false),
                ("networking", "Multiplayer networking", false),
                ("physics", "Physics simulation", false),
                ("ui", "UI framework", false),
            ],
            ProjectType::Embedded => vec![
                ("cortex-m", "ARM Cortex-M support", true),
                ("cortex-m-rt", "Runtime support", true),
                ("panic-halt", "Panic handler", true),
                ("stm32f4", "STM32F4 HAL", false),
                ("stm32f1", "STM32F1 HAL", false),
                ("rp2040", "Raspberry Pi Pico", false),
                ("esp32", "ESP32 HAL", false),
                ("rtt", "Real-time transfer", false),
                ("semihosting", "Semihosting debug", false),
            ],
            ProjectType::Workspace => vec![
                ("tokio", "Async runtime", true),
                ("serde", "Serialization", true),
                ("anyhow", "Error handling", true),
                ("database", "Database support", false),
                ("web", "Web framework", false),
                ("clap", "CLI support", false),
                ("testing", "Advanced testing", false),
            ],
        };

        let _default_features: Vec<String> = available_features.iter()
            .filter(|(_, _, default)| *default)
            .map(|(name, _, _)| name.to_string())
            .collect();

        let options: Vec<String> = available_features.iter()
            .map(|(name, desc, _)| format!("{} - {}", name, desc))
            .collect();

        let default_indices: Vec<usize> = available_features.iter()
            .enumerate()
            .filter(|(_, (_, _, default))| *default)
            .map(|(i, _)| i)
            .collect();

        let selections = MultiSelect::new("Select features:", options)
            .with_default(&default_indices)
            .with_help_message("Space to select/deselect, Enter to confirm")
            .prompt()?;

        let features: Vec<String> = selections.iter()
            .filter_map(|selection| {
                available_features.iter()
                    .find(|(name, desc, _)| format!("{} - {}", name, desc) == *selection)
                    .map(|(name, _, _)| name.to_string())
            })
            .collect();

        Ok(features)
    }

    /// Prompt for optional fields
    fn prompt_optional_field(&self, field_name: &str, placeholder: &str) -> Result<Option<String>> {
        let include = Confirm::new(&format!("Include {}?", field_name.to_lowercase()))
            .with_default(false)
            .prompt()?;

        if include {
            let value = Text::new(&format!("{}:", field_name))
                .with_placeholder(placeholder)
                .prompt()?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    /// Prompt for license selection
    fn prompt_license(&self) -> Result<Option<String>> {
        let include_license = Confirm::new("Include license?")
            .with_default(true)
            .prompt()?;

        if include_license {
            let licenses = vec!["MIT", "Apache-2.0", "GPL-3.0", "BSD-3-Clause", "Unlicense", "Other"];
            let license = Select::new("License:", licenses)
                .prompt()?;
            
            if license == "Other" {
                let custom = Text::new("Custom license:")
                    .with_placeholder("AGPL-3.0")
                    .prompt()?;
                Ok(Some(custom))
            } else {
                Ok(Some(license.to_string()))
            }
        } else {
            Ok(None)
        }
    }

    /// Prompt for author with config defaults and remember choice functionality
    fn prompt_author_with_config(&self, config: &mut Config) -> Result<Option<String>> {
        // Use config default if available
        let default_author = config.default_author.as_deref();
        
        let include = if default_author.is_some() {
            // If we have a config default, ask if they want to use it or change it
            let use_default = Confirm::new(&format!(
                "Use saved author '{}'?", 
                default_author.unwrap()
            ))
            .with_default(true)
            .prompt()?;
            
            if use_default {
                return Ok(config.default_author.clone());
            } else {
                true // They want to change it, so include author field
            }
        } else {
            // No default, ask if they want to include author
            Confirm::new("Include author?")
                .with_default(false)
                .prompt()?
        };

        if include {
            let author = Text::new("Author:")
                .with_placeholder("your-name")
                .prompt()?;
            
            // Ask if they want to remember this choice
            if config.remember_choices {
                let remember = Confirm::new("Remember this choice for future projects?")
                    .with_default(true)
                    .prompt()?;
                
                if remember {
                    config.remember_choice("author", &author);
                }
            }
            
            Ok(Some(author))
        } else {
            Ok(None)
        }
    }

    /// Prompt for license with config defaults and remember choice functionality
    fn prompt_license_with_config(&self, config: &mut Config) -> Result<Option<String>> {
        // Use config default if available
        let default_license = config.default_license.as_deref();
        
        let include_license = if default_license.is_some() {
            // If we have a config default, ask if they want to use it or change it
            let use_default = Confirm::new(&format!(
                "Use saved license '{}'?", 
                default_license.unwrap()
            ))
            .with_default(true)
            .prompt()?;
            
            if use_default {
                return Ok(config.default_license.clone());
            } else {
                true // They want to change it, so include license selection
            }
        } else {
            // No default, ask if they want to include license
            Confirm::new("Include license?")
                .with_default(true)
                .prompt()?
        };

        if include_license {
            let licenses = vec!["MIT", "Apache-2.0", "GPL-3.0", "BSD-3-Clause", "Unlicense", "Other"];
            let license = Select::new("License:", licenses)
                .prompt()?;
            
            let final_license = if license == "Other" {
                let custom = Text::new("Custom license:")
                    .with_placeholder("AGPL-3.0")
                    .prompt()?;
                custom
            } else {
                license.to_string()
            };
            
            // Ask if they want to remember this choice
            if config.remember_choices {
                let remember = Confirm::new("Remember this choice for future projects?")
                    .with_default(true)
                    .prompt()?;
                
                if remember {
                    config.remember_choice("license", &final_license);
                }
            }
            
            Ok(Some(final_license))
        } else {
            Ok(None)
        }
    }

    /// Create the project with progress indicators
    fn create_project(&self, context: ProjectContext) -> Result<()> {
        let project_path = self.base_path.join(&context.name);
        
        // Check if directory already exists
        if project_path.exists() {
            return Err(anyhow!("Project directory '{}' already exists", context.name));
        }

        println!("\n{}", "Creating your project...".bright_yellow());
        
        // Progress bar for project generation
        let pb = ProgressBar::new(100);
        pb.set_style(ProgressStyle::default_bar()
            .template("{prefix:.bold.dim} {bar:40.cyan/blue} {percent}% {msg}")
            .unwrap()
            .progress_chars("##-"));
        pb.set_prefix("Progress");
        
        // Initialize project structure
        pb.set_message("Creating project directory...");
        std::fs::create_dir_all(&project_path)?;
        pb.set_position(20);
        std::thread::sleep(Duration::from_millis(100));
        
        // Generate project using generator
        pb.set_message("Generating project files...");
        let config = context.to_project_config();
        let generator = Generator::new();
        
        // Simulate progress during generation
        pb.set_position(40);
        std::thread::sleep(Duration::from_millis(100));
        
        generator.generate(&config, &project_path)?;
        
        pb.set_position(80);
        pb.set_message("Finalizing project setup...");
        std::thread::sleep(Duration::from_millis(100));
        
        pb.set_position(100);
        pb.finish_and_clear();
        
        // Enhanced success message
        println!("\n{} {}", "✓".bright_green().bold(), "Project created successfully!".bright_green());
        self.show_next_steps(&context, false)?;
        
        Ok(())
    }

    pub fn prompt_project_type<R: Read>(&self, reader: &mut R) -> Result<ProjectType> {
        let mut input = String::new();
        reader.read_to_string(&mut input)?;
        
        let choice = input.trim();
        match choice {
            "1" => Ok(ProjectType::ApiServer),
            "2" => Ok(ProjectType::CliTool),
            "3" => Ok(ProjectType::Library),
            "4" => Ok(ProjectType::WasmApp),
            _ => Err(anyhow!("Invalid project type selection")),
        }
    }

    pub fn validate_project_name(&self, name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(anyhow!("Project name cannot be empty"));
        }
        
        // Check length
        if name.len() > 64 {
            return Err(anyhow!("Project name is too long (max 64 characters)"));
        }
        
        // Check for reserved names
        let reserved_names = ["test", "main", "build", "cargo", "rust", "src", "target", "bin", "lib"];
        if reserved_names.contains(&name) {
            return Err(anyhow!("'{}' is a reserved name", name));
        }
        
        // Check for invalid characters and patterns
        if name.contains(' ') {
            return Err(anyhow!("Project name cannot contain spaces"));
        }
        
        if name.contains('/') || name.contains('\\') {
            return Err(anyhow!("Project name cannot contain slashes"));
        }
        
        // Must be lowercase
        if name != name.to_lowercase() {
            return Err(anyhow!("Project name must be lowercase"));
        }
        
        // Cannot start with a number
        if name.starts_with(|c: char| c.is_numeric()) {
            return Err(anyhow!("Project name cannot start with a number"));
        }
        
        // Cannot start or end with dash/underscore
        if name.starts_with('-') || name.starts_with('_') {
            return Err(anyhow!("Project name cannot start with '-' or '_'"));
        }
        
        if name.ends_with('-') || name.ends_with('_') {
            return Err(anyhow!("Project name cannot end with '-' or '_'"));
        }
        
        // Check for valid characters (alphanumeric, dash, underscore)
        if !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(anyhow!("Project name can only contain letters, numbers, '-', and '_'"));
        }
        
        // Check for double dashes or underscores
        if name.contains("--") || name.contains("__") {
            return Err(anyhow!("Project name cannot contain consecutive dashes or underscores"));
        }
        
        Ok(())
    }

    /// Run in non-interactive mode with defaults
    pub fn run_non_interactive(
        &self, 
        name: Option<String>, 
        project_type: Option<String>, 
        author: Option<String>, 
        description: Option<String>,
        from_config: Option<PathBuf>
    ) -> Result<()> {
        println!("{}", "🤖 Non-interactive mode".bright_blue().bold());
        
        let config = if let Some(config_path) = from_config {
            ForgeConfig::load_from(config_path)?
        } else {
            ForgeConfig::load()?
        };

        let project_name = name.unwrap_or_else(|| "my-project".to_string());
        
        // Validate project name before doing anything else
        self.validate_project_name(&project_name)?;
        
        let project_type_str = project_type.unwrap_or_else(|| "cli-tool".to_string());
        let project_type = self.parse_project_type(&project_type_str)?;
        
        let context = ProjectContext {
            name: project_name,
            project_type,
            features: config.default_features.get(&project_type_str).cloned().unwrap_or_default(),
            author: author.or(config.default_author),
            description,
            license: config.default_license,
            edition: config.edition.unwrap_or_else(|| "2021".to_string()),
        };

        self.create_project(context)?;
        Ok(())
    }

    /// Run with command line arguments
    pub fn run_with_args(
        &self,
        name: Option<String>,
        project_type: Option<String>,
        author: Option<String>,
        description: Option<String>
    ) -> Result<()> {
        let project_name = name.ok_or_else(|| anyhow!("Project name is required"))?;
        
        // Validate project name before doing anything else
        self.validate_project_name(&project_name)?;
        
        let project_type_str = project_type.ok_or_else(|| anyhow!("Project type is required"))?;
        let project_type = self.parse_project_type(&project_type_str)?;
        
        let context = ProjectContext {
            name: project_name,
            project_type,
            features: vec![], // Default features
            author,
            description,
            license: Some("MIT".to_string()),
            edition: "2021".to_string(),
        };

        self.create_project(context)?;
        Ok(())
    }

    /// Run from configuration file
    pub fn run_from_config(
        &self,
        config_path: PathBuf,
        name: Option<String>,
        project_type: Option<String>,
        author: Option<String>,
        description: Option<String>
    ) -> Result<()> {
        println!("{}", "📁 Loading configuration...".bright_cyan());
        
        let config = ForgeConfig::load_from(config_path)?;
        
        let project_name = name.unwrap_or_else(|| "my-project".to_string());
        
        // Validate project name before doing anything else
        self.validate_project_name(&project_name)?;
        
        let project_type_str = project_type.or_else(|| config.preferred_project_types.first().cloned())
            .unwrap_or_else(|| "cli-tool".to_string());
        let project_type = self.parse_project_type(&project_type_str)?;
        
        let context = ProjectContext {
            name: project_name,
            project_type,
            features: config.default_features.get(&project_type_str).cloned().unwrap_or_default(),
            author: author.or(config.default_author),
            description,
            license: config.default_license,
            edition: config.edition.unwrap_or_else(|| "2021".to_string()),
        };

        self.create_project(context)?;
        Ok(())
    }

    /// Run in dry-run mode
    pub fn run_dry_run(
        &self,
        name: Option<String>,
        project_type: Option<String>,
        author: Option<String>,
        description: Option<String>,
        non_interactive: bool,
        from_config: Option<PathBuf>
    ) -> Result<()> {
        if non_interactive {
            let config = if let Some(config_path) = from_config {
                ForgeConfig::load_from(config_path)?
            } else {
                ForgeConfig::load()?
            };

            let project_name = name.unwrap_or_else(|| "my-project".to_string());
            
            // Validate project name before doing anything else
            self.validate_project_name(&project_name)?;
            
            let project_type_str = project_type.unwrap_or_else(|| "cli-tool".to_string());
            let project_type = self.parse_project_type(&project_type_str)?;
            
            let context = ProjectContext {
                name: project_name,
                project_type,
                features: config.default_features.get(&project_type_str).cloned().unwrap_or_default(),
                author: author.or(config.default_author),
                description,
                license: config.default_license,
                edition: config.edition.unwrap_or_else(|| "2021".to_string()),
            };

            self.preview_project(&context)
        } else {
            let context = self.collect_project_context()?;
            self.preview_project(&context)
        }
    }

    /// Initialize project in current directory (non-interactive)
    pub fn run_init_non_interactive(
        &self,
        project_type: Option<String>,
        from_config: Option<PathBuf>
    ) -> Result<()> {
        println!("{}", "🤖 Initializing in current directory (non-interactive)".bright_blue().bold());
        
        let config = if let Some(config_path) = from_config {
            ForgeConfig::load_from(config_path)?
        } else {
            ForgeConfig::load()?
        };

        let current_dir = std::env::current_dir()?;
        let project_name = current_dir.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("my-project")
            .to_string();

        let project_type_str = project_type.unwrap_or_else(|| "cli-tool".to_string());
        let project_type = self.parse_project_type(&project_type_str)?;
        
        let context = ProjectContext {
            name: project_name,
            project_type,
            features: config.default_features.get(&project_type_str).cloned().unwrap_or_default(),
            author: config.default_author,
            description: None,
            license: config.default_license,
            edition: config.edition.unwrap_or_else(|| "2021".to_string()),
        };

        self.init_project_in_current_dir(context)?;
        Ok(())
    }

    /// Initialize with regular interactive prompts
    pub fn run_init(&self, project_type: Option<String>) -> Result<()> {
        println!("{}", "🔨 Initializing project in current directory".bright_cyan().bold());
        
        let current_dir = std::env::current_dir()?;
        let project_name = current_dir.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("my-project")
            .to_string();

        let project_type = if let Some(pt) = project_type {
            self.parse_project_type(&pt)?
        } else {
            self.prompt_project_type_interactive()?
        };
        
        let features = self.prompt_features(&project_type)?;
        let author = self.prompt_optional_field("Author", "your-name")?;
        let description = self.prompt_optional_field("Description", "A new Rust project")?;
        let license = self.prompt_license()?;
        
        let context = ProjectContext {
            name: project_name,
            project_type,
            features,
            author,
            description,
            license,
            edition: "2021".to_string(),
        };

        self.init_project_in_current_dir(context)?;
        Ok(())
    }

    /// Initialize from config file
    pub fn run_init_from_config(&self, config_path: PathBuf, project_type: Option<String>) -> Result<()> {
        println!("{}", "📁 Initializing from configuration...".bright_cyan());
        
        let config = ForgeConfig::load_from(config_path)?;
        
        let current_dir = std::env::current_dir()?;
        let project_name = current_dir.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("my-project")
            .to_string();

        let project_type_str = project_type.or_else(|| config.preferred_project_types.first().cloned())
            .unwrap_or_else(|| "cli-tool".to_string());
        let project_type = self.parse_project_type(&project_type_str)?;
        
        let context = ProjectContext {
            name: project_name,
            project_type,
            features: config.default_features.get(&project_type_str).cloned().unwrap_or_default(),
            author: config.default_author,
            description: None,
            license: config.default_license,
            edition: config.edition.unwrap_or_else(|| "2021".to_string()),
        };

        self.init_project_in_current_dir(context)?;
        Ok(())
    }

    /// Dry run for init command
    pub fn run_init_dry_run(
        &self,
        project_type: Option<String>,
        non_interactive: bool,
        from_config: Option<PathBuf>
    ) -> Result<()> {
        let current_dir = std::env::current_dir()?;
        let project_name = current_dir.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("my-project")
            .to_string();

        if non_interactive {
            let config = if let Some(config_path) = from_config {
                ForgeConfig::load_from(config_path)?
            } else {
                ForgeConfig::load()?
            };

            let project_type_str = project_type.unwrap_or_else(|| "cli-tool".to_string());
            let project_type = self.parse_project_type(&project_type_str)?;
            
            let context = ProjectContext {
                name: project_name,
                project_type,
                features: config.default_features.get(&project_type_str).cloned().unwrap_or_default(),
                author: config.default_author,
                description: None,
                license: config.default_license,
                edition: config.edition.unwrap_or_else(|| "2021".to_string()),
            };

            self.preview_init(&context)
        } else {
            let project_type = if let Some(pt) = project_type {
                self.parse_project_type(&pt)?
            } else {
                self.prompt_project_type_interactive()?
            };
            
            let features = self.prompt_features(&project_type)?;
            let author = self.prompt_optional_field("Author", "your-name")?;
            let description = self.prompt_optional_field("Description", "A new Rust project")?;
            let license = self.prompt_license()?;
            
            let context = ProjectContext {
                name: project_name,
                project_type,
                features,
                author,
                description,
                license,
                edition: "2021".to_string(),
            };

            self.preview_init(&context)
        }
    }

    /// Helper method to parse project type string
    fn parse_project_type(&self, project_type_str: &str) -> Result<ProjectType> {
        match project_type_str.to_lowercase().as_str() {
            "api-server" => Ok(ProjectType::ApiServer),
            "cli-tool" => Ok(ProjectType::CliTool),
            "library" => Ok(ProjectType::Library),
            "wasm-app" => Ok(ProjectType::WasmApp),
            "game-engine" => Ok(ProjectType::GameEngine),
            "embedded" => Ok(ProjectType::Embedded),
            "workspace" => Ok(ProjectType::Workspace),
            _ => Err(anyhow!("Invalid project type: {}", project_type_str)),
        }
    }

    /// Preview project structure without creating files
    fn preview_project(&self, context: &ProjectContext) -> Result<()> {
        println!("\n{}", "📋 Project Preview".bright_white().bold());
        println!("{}", "─".repeat(50).bright_black());
        
        println!("{} {}", "📦 Name:".bright_cyan(), context.name.bright_white());
        println!("{} {}", "🏗️  Type:".bright_cyan(), context.project_type.to_string().bright_white());
        
        if let Some(author) = &context.author {
            println!("{} {}", "👤 Author:".bright_cyan(), author.bright_white());
        }
        
        if let Some(description) = &context.description {
            println!("{} {}", "📝 Description:".bright_cyan(), description.bright_white());
        }
        
        if let Some(license) = &context.license {
            println!("{} {}", "⚖️  License:".bright_cyan(), license.bright_white());
        }
        
        println!("{} {}", "📅 Edition:".bright_cyan(), context.edition.bright_white());
        
        if !context.features.is_empty() {
            println!("{} {}", "🎯 Features:".bright_cyan(), context.features.join(", ").bright_white());
        }
        
        println!("\n{}", "📁 Directory Structure:".bright_white().bold());
        self.preview_directory_structure(context);
        
        println!("\n{}", "Next steps (if this were real):".bright_green().bold());
        println!("  {} cd {}", "→".bright_cyan(), context.name);
        println!("  {} cargo build", "→".bright_cyan());
        println!("  {} cargo run\n", "→".bright_cyan());
        
        Ok(())
    }

    /// Preview init structure
    fn preview_init(&self, context: &ProjectContext) -> Result<()> {
        println!("\n{}", "📋 Initialization Preview".bright_white().bold());
        println!("{}", "─".repeat(50).bright_black());
        
        println!("{} {}", "📦 Name:".bright_cyan(), context.name.bright_white());
        println!("{} {}", "🏗️  Type:".bright_cyan(), context.project_type.to_string().bright_white());
        println!("{} {}", "📁 Location:".bright_cyan(), "Current directory".bright_white());
        
        if let Some(author) = &context.author {
            println!("{} {}", "👤 Author:".bright_cyan(), author.bright_white());
        }
        
        if let Some(license) = &context.license {
            println!("{} {}", "⚖️  License:".bright_cyan(), license.bright_white());
        }
        
        if !context.features.is_empty() {
            println!("{} {}", "🎯 Features:".bright_cyan(), context.features.join(", ").bright_white());
        }
        
        println!("\n{}", "📁 Files to be created:".bright_white().bold());
        self.preview_directory_structure(context);
        
        println!("\n{}", "Next steps (if this were real):".bright_green().bold());
        println!("  {} cargo build", "→".bright_cyan());
        println!("  {} cargo run\n", "→".bright_cyan());
        
        Ok(())
    }

    /// Preview directory structure
    fn preview_directory_structure(&self, context: &ProjectContext) {
        println!("  {}/", context.name.bright_yellow());
        println!("  ├── {}", "Cargo.toml".bright_green());
        println!("  ├── {}", "README.md".bright_green());
        
        if context.license.is_some() {
            println!("  ├── {}", "LICENSE".bright_green());
        }
        
        println!("  ├── {}/ ", "src".bright_blue());
        
        match context.project_type {
            ProjectType::Library => {
                println!("  │   └── {}", "lib.rs".bright_green());
            }
            _ => {
                println!("  │   └── {}", "main.rs".bright_green());
            }
        }
        
        if context.features.contains(&"testing".to_string()) {
            println!("  ├── {}/ ", "tests".bright_blue());
            println!("  │   └── {}", "integration_tests.rs".bright_green());
        }
        
        if context.project_type == ProjectType::WasmApp {
            println!("  └── {}", "index.html".bright_green());
        }
        
        if context.project_type == ProjectType::GameEngine {
            println!("  └── {}/ ", "assets".bright_blue());
            println!("      ├── {}/ ", "models".bright_blue());
            println!("      ├── {}/ ", "shaders".bright_blue());
            println!("      ├── {}/ ", "sounds".bright_blue());
            println!("      └── {}/ ", "textures".bright_blue());
        }
    }

    /// Initialize project in current directory
    fn init_project_in_current_dir(&self, context: ProjectContext) -> Result<()> {
        let current_dir = std::env::current_dir()?;
        
        println!("\n{}", "Creating project files...".bright_yellow());
        
        let pb = ProgressBar::new(100);
        pb.set_style(ProgressStyle::default_bar()
            .template("{prefix:.bold.dim} {bar:40.cyan/blue} {percent}% {msg}")
            .unwrap()
            .progress_chars("##-"));
        pb.set_prefix("Progress");
        
        pb.set_message("Generating project files...");
        let config = context.to_project_config();
        let generator = Generator::new();
        
        pb.set_position(50);
        generator.generate(&config, &current_dir)?;
        
        pb.set_position(100);
        pb.finish_and_clear();
        
        println!("\n{} {}", "✓".bright_green().bold(), "Project initialized successfully!".bright_green());
        self.show_next_steps(&context, true)?;
        
        Ok(())
    }

    /// Enhanced next steps with better formatting
    fn show_next_steps(&self, context: &ProjectContext, is_init: bool) -> Result<()> {
        println!("\n{}", "🎉 Project Setup Complete!".bright_green().bold());
        println!("{}", "─".repeat(50).bright_black());
        
        println!("\n{}", "📋 Project Summary:".bright_white().bold());
        println!("  {} {}", "Name:".bright_cyan(), context.name.bright_white());
        println!("  {} {}", "Type:".bright_cyan(), context.project_type.to_string().bright_white());
        if !context.features.is_empty() {
            println!("  {} {}", "Features:".bright_cyan(), context.features.join(", ").bright_white());
        }
        
        println!("\n{}", "🚀 Next Steps:".bright_white().bold());
        
        if !is_init {
            println!("  {} cd {}", "1.".bright_yellow(), context.name.bright_white());
        }
        
        let step_num = if is_init { 1 } else { 2 };
        println!("  {} cargo build", format!("{}.", step_num).bright_yellow());
        println!("  {} cargo run", format!("{}.", step_num + 1).bright_yellow());
        
        if context.features.contains(&"testing".to_string()) {
            println!("  {} cargo test", format!("{}.", step_num + 2).bright_yellow());
        }
        
        match context.project_type {
            ProjectType::ApiServer => {
                println!("\n{}", "💡 API Server Tips:".bright_blue().bold());
                println!("  • Edit src/main.rs to define your API routes");
                println!("  • Run with: cargo run");
                println!("  • Test endpoints at: http://localhost:3000");
            }
            ProjectType::CliTool => {
                println!("\n{}", "💡 CLI Tool Tips:".bright_blue().bold());
                println!("  • Edit src/main.rs to define your CLI commands");
                println!("  • Build release version: cargo build --release");
                println!("  • Install globally: cargo install --path .");
            }
            ProjectType::Library => {
                println!("\n{}", "💡 Library Tips:".bright_blue().bold());
                println!("  • Edit src/lib.rs to define your public API");
                println!("  • Publish to crates.io: cargo publish");
                println!("  • Generate docs: cargo doc --open");
            }
            ProjectType::WasmApp => {
                println!("\n{}", "💡 WASM App Tips:".bright_blue().bold());
                println!("  • Build WASM: wasm-pack build --target web");
                println!("  • Serve locally: python -m http.server 8000");
                println!("  • Open: http://localhost:8000");
            }
            ProjectType::GameEngine => {
                println!("\n{}", "💡 Game Development Tips:".bright_blue().bold());
                println!("  • Add assets to the assets/ directory");
                println!("  • Edit src/main.rs to create your game systems");
                println!("  • Run with: cargo run");
            }
            ProjectType::Embedded => {
                println!("\n{}", "💡 Embedded Tips:".bright_blue().bold());
                println!("  • Configure your target in .cargo/config.toml");
                println!("  • Flash to device: cargo embed");
                println!("  • Debug with RTT: cargo embed --release");
            }
            ProjectType::Workspace => {
                println!("\n{}", "💡 Workspace Tips:".bright_blue().bold());
                println!("  • Add new crates: cargo new crates/new-crate");
                println!("  • Build all: cargo build");
                println!("  • Test all: cargo test");
            }
        }
        
        println!("\n{}", "📚 Resources:".bright_white().bold());
        println!("  • Rust Book: https://doc.rust-lang.org/book/");
        println!("  • Cargo Guide: https://doc.rust-lang.org/cargo/");
        println!("  • Crates.io: https://crates.io/");
        
        Ok(())
    }
}