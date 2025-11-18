use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub default_author: Option<String>,
    #[serde(default)]
    pub default_license: Option<String>,
    #[serde(default)]
    pub default_ci: Option<String>,
    #[serde(default)]
    pub custom_template_dirs: Vec<PathBuf>,
    #[serde(default = "default_remember_choices")]
    pub remember_choices: bool,
}

fn default_remember_choices() -> bool {
    true
}

pub trait ConfigDefaults {
    fn new() -> Self;
}

impl ConfigDefaults for Config {
    fn new() -> Self {
        Config {
            default_author: None,
            default_license: None,
            default_ci: None,
            custom_template_dirs: Vec::new(),
            remember_choices: true,
        }
    }
}

impl Config {
    /// Create a new Config with default values
    pub fn new() -> Self {
        <Self as ConfigDefaults>::new()
    }

    /// Load config from a specific file path
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        if !path.exists() {
            return Ok(Self::new());
        }

        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

        Ok(config)
    }

    /// Load config from ~/.cargo-forge/config.toml
    pub fn load_from_home() -> Result<Self> {
        if let Some(home_dir) = dirs::home_dir() {
            Self::load_from_home_with_path(&home_dir)
        } else {
            Ok(Self::new())
        }
    }

    /// Load config from specific home directory (for testing)
    pub fn load_from_home_with_path<P: AsRef<Path>>(home_dir: P) -> Result<Self> {
        let config_dir = home_dir.as_ref().join(".cargo-forge");
        let config_path = config_dir.join("config.toml");
        Self::load_from_file(&config_path)
    }

    /// Save config to a specific file path
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create config directory: {}", parent.display())
            })?;
        }

        let content = toml::to_string_pretty(self).context("Failed to serialize config to TOML")?;

        fs::write(path, content)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;

        Ok(())
    }

    /// Save config to ~/.cargo-forge/config.toml
    pub fn save_to_home(&self) -> Result<()> {
        if let Some(home_dir) = dirs::home_dir() {
            let config_dir = home_dir.join(".cargo-forge");
            let config_path = config_dir.join("config.toml");
            self.save_to_file(&config_path)
        } else {
            anyhow::bail!("Could not determine home directory")
        }
    }

    /// Merge config with CLI arguments (CLI args take precedence)
    pub fn merge_with_cli(
        &self,
        cli_author: Option<String>,
        cli_license: Option<String>,
        cli_ci: Option<String>,
    ) -> Self {
        Config {
            default_author: cli_author.or_else(|| self.default_author.clone()),
            default_license: cli_license.or_else(|| self.default_license.clone()),
            default_ci: cli_ci.or_else(|| self.default_ci.clone()),
            custom_template_dirs: self.custom_template_dirs.clone(),
            remember_choices: self.remember_choices,
        }
    }

    /// Add a custom template directory (avoid duplicates)
    pub fn add_custom_template_directory(&mut self, path: PathBuf) {
        if !self.custom_template_dirs.contains(&path) {
            self.custom_template_dirs.push(path);
        }
    }

    /// Remember a choice if remember_choices is enabled
    pub fn remember_choice(&mut self, choice_type: &str, value: &str) {
        if !self.remember_choices {
            return;
        }

        match choice_type {
            "author" => self.default_author = Some(value.to_string()),
            "license" => self.default_license = Some(value.to_string()),
            "ci" => self.default_ci = Some(value.to_string()),
            _ => {}
        }
    }

    /// Get effective author value (CLI overrides config)
    pub fn get_effective_author(&self, cli_author: Option<String>) -> Option<String> {
        cli_author.or_else(|| self.default_author.clone())
    }

    /// Get effective license value (CLI overrides config)
    pub fn get_effective_license(&self, cli_license: Option<String>) -> Option<String> {
        cli_license.or_else(|| self.default_license.clone())
    }

    /// Get effective CI value (CLI overrides config)
    pub fn get_effective_ci(&self, cli_ci: Option<String>) -> Option<String> {
        cli_ci.or_else(|| self.default_ci.clone())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
