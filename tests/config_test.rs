use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use cargo_forge::config::Config;

#[test]
fn test_config_new_with_defaults() {
    let config = Config::new();
    
    assert_eq!(config.default_author, None);
    assert_eq!(config.default_license, None);
    assert_eq!(config.default_ci, None);
    assert!(config.custom_template_dirs.is_empty());
    assert_eq!(config.remember_choices, true);
}

#[test]
fn test_config_with_values() {
    let mut custom_dirs = Vec::new();
    custom_dirs.push(PathBuf::from("/custom/templates"));
    
    let config = Config {
        default_author: Some("Jane Doe".to_string()),
        default_license: Some("MIT".to_string()),
        default_ci: Some("github".to_string()),
        custom_template_dirs: custom_dirs,
        remember_choices: false,
    };
    
    assert_eq!(config.default_author, Some("Jane Doe".to_string()));
    assert_eq!(config.default_license, Some("MIT".to_string()));
    assert_eq!(config.default_ci, Some("github".to_string()));
    assert_eq!(config.custom_template_dirs.len(), 1);
    assert_eq!(config.remember_choices, false);
}

#[test]
fn test_config_load_from_file() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");
    
    let config_content = r#"
default_author = "John Doe"
default_license = "Apache-2.0"
default_ci = "gitlab"
custom_template_dirs = ["/home/user/templates", "/opt/templates"]
remember_choices = false
"#;
    
    fs::write(&config_path, config_content).unwrap();
    
    let config = Config::load_from_file(&config_path).unwrap();
    
    assert_eq!(config.default_author, Some("John Doe".to_string()));
    assert_eq!(config.default_license, Some("Apache-2.0".to_string()));
    assert_eq!(config.default_ci, Some("gitlab".to_string()));
    assert_eq!(config.custom_template_dirs.len(), 2);
    assert_eq!(config.custom_template_dirs[0], PathBuf::from("/home/user/templates"));
    assert_eq!(config.custom_template_dirs[1], PathBuf::from("/opt/templates"));
    assert_eq!(config.remember_choices, false);
}

#[test]
fn test_config_load_from_nonexistent_file() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("nonexistent.toml");
    
    let config = Config::load_from_file(&config_path).unwrap();
    
    // Should return default config when file doesn't exist
    assert_eq!(config.default_author, None);
    assert_eq!(config.default_license, None);
    assert_eq!(config.default_ci, None);
    assert!(config.custom_template_dirs.is_empty());
    assert_eq!(config.remember_choices, true);
}

#[test]
fn test_config_save_to_file() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");
    
    let mut custom_dirs = Vec::new();
    custom_dirs.push(PathBuf::from("/home/user/my-templates"));
    
    let config = Config {
        default_author: Some("Alice Smith".to_string()),
        default_license: Some("MIT".to_string()),
        default_ci: Some("github".to_string()),
        custom_template_dirs: custom_dirs,
        remember_choices: true,
    };
    
    config.save_to_file(&config_path).unwrap();
    
    let saved_content = fs::read_to_string(&config_path).unwrap();
    assert!(saved_content.contains("default_author = \"Alice Smith\""));
    assert!(saved_content.contains("default_license = \"MIT\""));
    assert!(saved_content.contains("default_ci = \"github\""));
    assert!(saved_content.contains("/home/user/my-templates"));
    assert!(saved_content.contains("remember_choices = true"));
}

#[test]
fn test_config_load_from_home_directory() {
    let temp_dir = TempDir::new().unwrap();
    let home_dir = temp_dir.path();
    
    // Create .cargo-forge directory
    let cargo_forge_dir = home_dir.join(".cargo-forge");
    fs::create_dir_all(&cargo_forge_dir).unwrap();
    
    let config_path = cargo_forge_dir.join("config.toml");
    let config_content = r#"
default_author = "Home User"
default_license = "BSD-3-Clause"
"#;
    
    fs::write(&config_path, config_content).unwrap();
    
    let config = Config::load_from_home_with_path(home_dir).unwrap();
    
    assert_eq!(config.default_author, Some("Home User".to_string()));
    assert_eq!(config.default_license, Some("BSD-3-Clause".to_string()));
}

#[test]
fn test_config_merge_with_cli_args() {
    let config = Config {
        default_author: Some("Config Author".to_string()),
        default_license: Some("MIT".to_string()),
        default_ci: Some("github".to_string()),
        custom_template_dirs: vec![PathBuf::from("/config/templates")],
        remember_choices: true,
    };
    
    let cli_author = Some("CLI Author".to_string());
    let cli_license = None;
    let cli_ci = Some("gitlab".to_string());
    
    let merged = config.merge_with_cli(cli_author, cli_license, cli_ci);
    
    // CLI args should override config values
    assert_eq!(merged.default_author, Some("CLI Author".to_string()));
    assert_eq!(merged.default_license, Some("MIT".to_string())); // unchanged from config
    assert_eq!(merged.default_ci, Some("gitlab".to_string())); // overridden by CLI
    assert_eq!(merged.custom_template_dirs, config.custom_template_dirs); // unchanged
    assert_eq!(merged.remember_choices, true); // unchanged
}

#[test]
fn test_config_merge_with_empty_cli_args() {
    let config = Config {
        default_author: Some("Config Author".to_string()),
        default_license: Some("MIT".to_string()),
        default_ci: Some("github".to_string()),
        custom_template_dirs: vec![PathBuf::from("/config/templates")],
        remember_choices: false,
    };
    
    let merged = config.merge_with_cli(None, None, None);
    
    // Config values should remain unchanged when CLI args are None
    assert_eq!(merged.default_author, Some("Config Author".to_string()));
    assert_eq!(merged.default_license, Some("MIT".to_string()));
    assert_eq!(merged.default_ci, Some("github".to_string()));
    assert_eq!(merged.custom_template_dirs, config.custom_template_dirs);
    assert_eq!(merged.remember_choices, false);
}

#[test]
fn test_config_add_custom_template_directory() {
    let mut config = Config::new();
    let template_dir = PathBuf::from("/new/template/dir");
    
    config.add_custom_template_directory(template_dir.clone());
    
    assert_eq!(config.custom_template_dirs.len(), 1);
    assert_eq!(config.custom_template_dirs[0], template_dir);
}

#[test]
fn test_config_add_duplicate_template_directory() {
    let mut config = Config::new();
    let template_dir = PathBuf::from("/template/dir");
    
    config.add_custom_template_directory(template_dir.clone());
    config.add_custom_template_directory(template_dir.clone()); // duplicate
    
    // Should not add duplicates
    assert_eq!(config.custom_template_dirs.len(), 1);
    assert_eq!(config.custom_template_dirs[0], template_dir);
}

#[test]
fn test_config_remember_choice_functionality() {
    let mut config = Config::new();
    
    // Test remembering author choice
    config.remember_choice("author", "Remembered Author");
    assert_eq!(config.default_author, Some("Remembered Author".to_string()));
    
    // Test remembering license choice
    config.remember_choice("license", "Apache-2.0");
    assert_eq!(config.default_license, Some("Apache-2.0".to_string()));
    
    // Test remembering CI choice
    config.remember_choice("ci", "gitlab");
    assert_eq!(config.default_ci, Some("gitlab".to_string()));
}

#[test]
fn test_config_remember_choice_disabled() {
    let mut config = Config {
        remember_choices: false,
        ..Config::new()
    };
    
    config.remember_choice("author", "Should Not Remember");
    
    // Should not remember when remember_choices is false
    assert_eq!(config.default_author, None);
}

#[test]
fn test_config_get_effective_value() {
    let config = Config {
        default_author: Some("Config Author".to_string()),
        default_license: None,
        default_ci: Some("github".to_string()),
        custom_template_dirs: vec![],
        remember_choices: true,
    };
    
    // Test getting existing value
    assert_eq!(config.get_effective_author(None), Some("Config Author".to_string()));
    
    // Test CLI override
    assert_eq!(config.get_effective_author(Some("CLI Author".to_string())), Some("CLI Author".to_string()));
    
    // Test None when no value
    assert_eq!(config.get_effective_license(None), None);
    
    // Test CLI value when config is None
    assert_eq!(config.get_effective_license(Some("MIT".to_string())), Some("MIT".to_string()));
}