use cargo_forge::Forge;
use tempfile::TempDir;

#[test]
fn test_project_name_validation_comprehensive() {
    let temp_dir = TempDir::new().unwrap();
    let forge = Forge::new(temp_dir.path());
    
    // Valid project names
    let valid_names = vec![
        "my-project",
        "my_project", 
        "myproject123",
        "a",
        "_project",
        "underscore_project",
        "dash-project",
        "alpha123numeric",
    ];
    
    for name in valid_names {
        assert!(
            forge.validate_project_name(name).is_ok(),
            "Expected '{}' to be valid, but it was rejected",
            name
        );
    }
    
    // Invalid project names  
    let invalid_names = vec![
        ("", "Project name cannot be empty"),
        ("My-Project", "Project name must be lowercase"),
        ("my project", "Project name cannot contain spaces"),
        ("my/project", "Project name cannot contain slashes"),
        ("123project", "Project name cannot start with a number"),
        ("-project", "Project name cannot start with '-' or '_'"),
        ("project-", "Project name cannot end with '-' or '_'"),
        ("my--project", "Project name cannot contain consecutive dashes or underscores"),
        ("my__project", "Project name cannot contain consecutive dashes or underscores"),
        ("test", "'test' is a reserved name"),
        ("main", "'main' is a reserved name"),
        ("cargo", "'cargo' is a reserved name"),
        ("src", "'src' is a reserved name"),
        ("this-is-a-very-long-project-name-that-exceeds-the-maximum-allowed-length-of-64-chars", "Project name is too long (max 64 characters)"),
        ("my!project", "Project name can only contain letters, numbers, '-', and '_'"),
        ("my@project", "Project name can only contain letters, numbers, '-', and '_'"),
        ("my#project", "Project name can only contain letters, numbers, '-', and '_'"),
    ];
    
    for (name, expected_error) in invalid_names {
        let result = forge.validate_project_name(name);
        assert!(
            result.is_err(),
            "Expected '{}' to be invalid, but it was accepted", 
            name
        );
        
        if result.is_err() {
            let error_msg = result.unwrap_err().to_string();
            println!("Testing '{}': {}", name, error_msg);
            // Check that we get an appropriate error message
            assert!(
                error_msg.contains(expected_error) || 
                error_msg.to_lowercase().contains(&expected_error.to_lowercase()),
                "For '{}', expected error containing '{}', but got '{}'",
                name,
                expected_error,
                error_msg
            );
        }
    }
}

#[test]
fn test_validation_called_in_non_interactive_mode() {
    let temp_dir = TempDir::new().unwrap();
    let forge = Forge::new(temp_dir.path());
    
    // Test that invalid names are rejected in non-interactive mode
    let result = forge.run_non_interactive(
        Some("123-invalid".to_string()),
        Some("cli-tool".to_string()),
        None,
        None,
        None
    );
    
    assert!(result.is_err(), "Should reject invalid project name in non-interactive mode");
    assert!(result.unwrap_err().to_string().contains("cannot start with a number"));
}

#[test] 
fn test_validation_called_in_with_args_mode() {
    let temp_dir = TempDir::new().unwrap();
    let forge = Forge::new(temp_dir.path());
    
    // Test that invalid names are rejected when using args
    let result = forge.run_with_args(
        Some("MY-PROJECT".to_string()),
        Some("cli-tool".to_string()),
        None,
        None
    );
    
    assert!(result.is_err(), "Should reject invalid project name in with_args mode");
    assert!(result.unwrap_err().to_string().contains("must be lowercase"));
}

#[test]
fn test_validation_called_in_from_config_mode() {
    let temp_dir = TempDir::new().unwrap();
    let forge = Forge::new(temp_dir.path());
    
    // Create a dummy config file
    let config_path = temp_dir.path().join("config.json");
    std::fs::write(&config_path, r#"{"default_author":"test","default_license":"MIT","preferred_project_types":["cli-tool"],"default_features":{},"edition":"2021"}"#).unwrap();
    
    // Test that invalid names are rejected when using config
    let result = forge.run_from_config(
        config_path,
        Some("project-".to_string()),
        None,
        None,
        None
    );
    
    assert!(result.is_err(), "Should reject invalid project name in from_config mode");
    assert!(result.unwrap_err().to_string().contains("cannot end with"));
}

#[test]
fn test_validation_called_in_dry_run_mode() {
    let temp_dir = TempDir::new().unwrap();
    let forge = Forge::new(temp_dir.path());
    
    // Test that invalid names are rejected in dry run mode
    let result = forge.run_dry_run(
        Some("my/project".to_string()),
        Some("cli-tool".to_string()),
        None,
        None,
        true, // non_interactive
        None
    );
    
    assert!(result.is_err(), "Should reject invalid project name in dry_run mode");
    assert!(result.unwrap_err().to_string().contains("cannot contain slashes"));
}