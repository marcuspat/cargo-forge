use cargo_forge::{Generator, ProjectConfig, ProjectType};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::str::FromStr;
use tempfile::TempDir;

/// Test comprehensive error cases for the generator
#[cfg(test)]
mod generator_error_tests {
    use super::*;

    #[test]
    fn test_invalid_project_names() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();
        
        // Test empty project name
        let config = ProjectConfig {
            name: "".to_string(),
            project_type: ProjectType::Library.to_string(),
            author: "Test Author".to_string(),
            description: Some("Test".to_string()),
        };
        
        let output_dir = temp_dir.path().join("empty-name-test");
        let result = generator.generate(&config, &output_dir);
        // Empty name should work but create minimal content
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_characters_in_project_name() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();
        
        // Test project names with invalid characters
        let invalid_names = vec![
            "project@name",  // @ symbol
            "project#name",  // # symbol  
            "project$name",  // $ symbol
            "project name",  // space
            "project/name",  // slash
            "project\\name", // backslash
            "project:name",  // colon
        ];
        
        for invalid_name in invalid_names {
            let config = ProjectConfig {
                name: invalid_name.to_string(),
                project_type: ProjectType::Library.to_string(),
                author: "Test Author".to_string(),
                description: Some("Test".to_string()),
            };
            
            let output_dir = temp_dir.path().join(format!("invalid-name-{}", invalid_name.replace(|c: char| !c.is_alphanumeric(), "-")));
            let result = generator.generate(&config, &output_dir);
            // May succeed or fail depending on implementation - just test the code path
            let _ = result;
        }
    }

    #[test]
    fn test_reserved_project_names() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();
        
        // Test reserved Rust keywords as project names
        let reserved_names = vec![
            "const", "let", "fn", "struct", "enum", "impl", 
            "trait", "mod", "use", "pub", "match", "if",
            "else", "while", "for", "loop", "break", "continue",
            "return", "self", "Self", "super", "crate"
        ];
        
        for reserved_name in reserved_names {
            let config = ProjectConfig {
                name: reserved_name.to_string(),
                project_type: ProjectType::Library.to_string(),
                author: "Test Author".to_string(),
                description: Some("Test".to_string()),
            };
            
            let output_dir = temp_dir.path().join(format!("reserved-{}", reserved_name));
            let result = generator.generate(&config, &output_dir);
            // May succeed or fail depending on implementation - just test the code path
            let _ = result;
        }
    }

    #[test]
    fn test_directory_already_exists_with_files() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();
        let output_dir = temp_dir.path().join("existing-dir");
        
        // Create directory with a file
        fs::create_dir_all(&output_dir).unwrap();
        fs::write(output_dir.join("existing-file.txt"), "content").unwrap();
        
        let config = ProjectConfig {
            name: "test-project".to_string(),
            project_type: ProjectType::Library.to_string(),
            author: "Test Author".to_string(),
            description: Some("Test".to_string()),
        };
        
        let result = generator.generate(&config, &output_dir);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));
    }

    #[test] 
    fn test_directory_already_exists_empty() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();
        let output_dir = temp_dir.path().join("empty-existing-dir");
        
        // Create empty directory
        fs::create_dir_all(&output_dir).unwrap();
        
        let config = ProjectConfig {
            name: "test-project".to_string(),
            project_type: ProjectType::Library.to_string(),
            author: "Test Author".to_string(),
            description: Some("Test".to_string()),
        };
        
        let result = generator.generate(&config, &output_dir);
        // Empty directory should be fine
        assert!(result.is_ok());
    }

    #[test]
    #[cfg(unix)]
    fn test_permission_denied_directory() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();
        let readonly_dir = temp_dir.path().join("readonly");
        
        // Create directory and make it read-only
        fs::create_dir_all(&readonly_dir).unwrap();
        let mut perms = fs::metadata(&readonly_dir).unwrap().permissions();
        perms.set_mode(0o444); // read-only
        fs::set_permissions(&readonly_dir, perms).unwrap();
        
        let output_dir = readonly_dir.join("new-project");
        
        let config = ProjectConfig {
            name: "test-project".to_string(),
            project_type: ProjectType::Library.to_string(),
            author: "Test Author".to_string(),
            description: Some("Test".to_string()),
        };
        
        let result = generator.generate(&config, &output_dir);
        assert!(result.is_err());
        
        // Restore permissions for cleanup
        let mut perms = fs::metadata(&readonly_dir).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&readonly_dir, perms).unwrap();
    }

    #[test]
    fn test_invalid_project_type() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();
        let output_dir = temp_dir.path().join("invalid-type-test");
        
        let config = ProjectConfig {
            name: "test-project".to_string(),
            project_type: "invalid-type".to_string(),
            author: "Test Author".to_string(),
            description: Some("Test".to_string()),
        };
        
        let result = generator.generate(&config, &output_dir);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown project type"));
    }

    #[test]
    fn test_very_long_project_name() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();
        
        // Create a very long project name (255+ characters)
        let long_name = "a".repeat(300);
        
        let config = ProjectConfig {
            name: long_name.clone(),
            project_type: ProjectType::Library.to_string(),
            author: "Test Author".to_string(),
            description: Some("Test".to_string()),
        };
        
        let output_dir = temp_dir.path().join("long-name-test");
        let result = generator.generate(&config, &output_dir);
        
        // May fail due to filesystem limitations
        // Don't assert either way since it depends on the filesystem
        let _ = result;
    }

    #[test]
    fn test_unicode_project_name() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();
        
        let unicode_names = vec![
            "测试项目",         // Chinese
            "тестовый-проект",  // Russian 
            "プロジェクト",     // Japanese
            "proyecto-español", // Spanish with accents
            "🚀-rocket",       // Emoji
        ];
        
        for unicode_name in unicode_names {
            let config = ProjectConfig {
                name: unicode_name.to_string(),
                project_type: ProjectType::Library.to_string(),
                author: "Test Author".to_string(),
                description: Some("Test".to_string()),
            };
            
            let output_dir = temp_dir.path().join(format!("unicode-{}", unicode_name.chars().filter(|c| c.is_alphanumeric()).collect::<String>()));
            let result = generator.generate(&config, &output_dir);
            // Should succeed for most unicode names
            assert!(result.is_ok(), "Failed for unicode name: {}", unicode_name);
        }
    }

    #[test]
    fn test_null_description() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();
        let output_dir = temp_dir.path().join("null-desc-test");
        
        let config = ProjectConfig {
            name: "test-project".to_string(),
            project_type: ProjectType::Library.to_string(),
            author: "Test Author".to_string(),
            description: None,
        };
        
        let result = generator.generate(&config, &output_dir);
        assert!(result.is_ok());
        
        // Check that README is generated without description
        let readme_content = fs::read_to_string(output_dir.join("README.md")).unwrap();
        assert!(readme_content.contains("test-project"));
    }

    #[test]
    fn test_empty_author() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();
        let output_dir = temp_dir.path().join("empty-author-test");
        
        let config = ProjectConfig {
            name: "test-project".to_string(),
            project_type: ProjectType::Library.to_string(),
            author: "".to_string(),
            description: Some("Test".to_string()),
        };
        
        let result = generator.generate(&config, &output_dir);
        assert!(result.is_ok());
        
        // Check that Cargo.toml is generated with empty author
        let cargo_content = fs::read_to_string(output_dir.join("Cargo.toml")).unwrap();
        assert!(cargo_content.contains("authors = [\"\"]"));
    }

    #[test]
    fn test_malformed_path_characters() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();
        
        // Test paths with potentially problematic characters
        let output_dir = temp_dir.path().join("test..project");
        
        let config = ProjectConfig {
            name: "test-project".to_string(),
            project_type: ProjectType::Library.to_string(),
            author: "Test Author".to_string(),
            description: Some("Test".to_string()),
        };
        
        let result = generator.generate(&config, &output_dir);
        assert!(result.is_ok());
    }
}

/// Test project type validation and edge cases
#[cfg(test)]
mod project_type_error_tests {
    use super::*;

    #[test]
    fn test_project_type_from_invalid_string() {
        use std::str::FromStr;
        
        let invalid_types = vec![
            "",
            "invalid",
            "LIBRARY",  // wrong case
            "lib",      // abbreviation
            "api",      // abbreviation
            "wasm",     // abbreviation
            "web-app",  // wrong format
            "library-project", // too specific
        ];
        
        for invalid_type in invalid_types {
            let result = ProjectType::from_str(invalid_type);
            assert!(result.is_err(), "Should fail for invalid type: {}", invalid_type);
        }
    }

    #[test]
    fn test_project_type_display_consistency() {
        // Test that display and from_str are consistent
        let types = vec![
            ProjectType::Library,
            ProjectType::ApiServer,
            ProjectType::CliTool,
            ProjectType::WasmApp,
            ProjectType::GameEngine,
            ProjectType::Embedded,
            ProjectType::Workspace,
        ];
        
        for project_type in types {
            let type_string = project_type.to_string();
            let parsed_type = ProjectType::from_str(&type_string).unwrap();
            assert_eq!(project_type, parsed_type);
        }
    }
}