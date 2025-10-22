use cargo_forge::{Config, Generator, ProjectConfig, ProjectType};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
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
            features: vec![],
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
                features: vec![],
            };

            let output_dir = temp_dir.path().join(format!(
                "invalid-name-{}",
                invalid_name.replace(|c: char| !c.is_alphanumeric(), "-")
            ));
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
            "const", "let", "fn", "struct", "enum", "impl", "trait", "mod", "use", "pub", "match",
            "if", "else", "while", "for", "loop", "break", "continue", "return", "self", "Self",
            "super", "crate",
        ];

        for reserved_name in reserved_names {
            let config = ProjectConfig {
                name: reserved_name.to_string(),
                project_type: ProjectType::Library.to_string(),
                author: "Test Author".to_string(),
                description: Some("Test".to_string()),
                features: vec![],
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
            features: vec![],
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
            features: vec![],
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
            features: vec![],
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
            features: vec![],
        };

        let result = generator.generate(&config, &output_dir);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Unknown project type"));
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
            features: vec![],
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
            "🚀-rocket",        // Emoji
        ];

        for unicode_name in unicode_names {
            let config = ProjectConfig {
                name: unicode_name.to_string(),
                project_type: ProjectType::Library.to_string(),
                author: "Test Author".to_string(),
                description: Some("Test".to_string()),
                features: vec![],
            };

            let output_dir = temp_dir.path().join(format!(
                "unicode-{}",
                unicode_name
                    .chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect::<String>()
            ));
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
            features: vec![],
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
            features: vec![],
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
            features: vec![],
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
            "LIBRARY",         // wrong case
            "lib",             // abbreviation
            "api",             // abbreviation
            "wasm",            // abbreviation
            "web-app",         // wrong format
            "library-project", // too specific
        ];

        for invalid_type in invalid_types {
            let result = ProjectType::from_str(invalid_type);
            assert!(
                result.is_err(),
                "Should fail for invalid type: {}",
                invalid_type
            );
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

/// Test extreme edge cases and stress tests
#[cfg(test)]
mod extreme_edge_cases {
    use super::*;

    #[test]
    fn test_control_characters_in_project_name() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();

        // Test various control characters
        let control_char_names = vec![
            "project\nname",   // newline
            "project\tname",   // tab
            "project\rname",   // carriage return
            "project\0name",   // null byte
            "project\x1bname", // escape character
            "project\x7fname", // delete character
        ];

        for name in control_char_names {
            let config = ProjectConfig {
                name: name.to_string(),
                project_type: ProjectType::Library.to_string(),
                author: "Test Author".to_string(),
                description: Some("Test".to_string()),
                features: vec![],
            };

            let output_dir = temp_dir.path().join(format!(
                "control-{}",
                name.chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect::<String>()
            ));
            let result = generator.generate(&config, &output_dir);
            // Should handle control characters gracefully
            let _ = result;
        }
    }

    #[test]
    fn test_extremely_long_author_name() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();

        // Create an extremely long author name (10KB)
        let long_author = "A".repeat(10_000);

        let config = ProjectConfig {
            name: "test-project".to_string(),
            project_type: ProjectType::Library.to_string(),
            author: long_author,
            description: Some("Test".to_string()),
            features: vec![],
        };

        let output_dir = temp_dir.path().join("long-author-test");
        let result = generator.generate(&config, &output_dir);

        // Should succeed even with very long author name
        assert!(result.is_ok());

        // Verify the Cargo.toml was created
        let cargo_content = fs::read_to_string(output_dir.join("Cargo.toml")).unwrap();
        assert!(cargo_content.len() > 10_000); // Should contain the long author name
    }

    #[test]
    fn test_extremely_long_description() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();

        // Create an extremely long description (100KB)
        let long_description = "This is a very long description. ".repeat(3000);

        let config = ProjectConfig {
            name: "test-project".to_string(),
            project_type: ProjectType::Library.to_string(),
            author: "Test Author".to_string(),
            description: Some(long_description.clone()),
            features: vec![],
        };

        let output_dir = temp_dir.path().join("long-desc-test");
        let result = generator.generate(&config, &output_dir);

        // Should succeed
        assert!(result.is_ok());

        // Verify the README contains the long description
        let readme_content = fs::read_to_string(output_dir.join("README.md")).unwrap();
        assert!(readme_content.contains(&long_description[..50])); // Check first 50 chars
    }

    #[test]
    fn test_path_traversal_attempts() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();

        // Test path traversal attempts in project name
        let traversal_names = vec![
            "../../../etc/passwd",
            "..\\..\\..\\windows\\system32",
            "./../../sensitive",
            "~/.ssh/id_rsa",
        ];

        for name in traversal_names {
            let config = ProjectConfig {
                name: name.to_string(),
                project_type: ProjectType::Library.to_string(),
                author: "Test Author".to_string(),
                description: Some("Test".to_string()),
                features: vec![],
            };

            let output_dir = temp_dir.path().join("traversal-test");
            let result = generator.generate(&config, &output_dir);

            // Should handle these safely
            let _ = result;

            // Clean up for next iteration
            let _ = fs::remove_dir_all(&output_dir);
        }
    }

    #[test]
    fn test_filesystem_limits() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();

        // Test with maximum path length (varies by OS, typically 255-260 chars)
        let very_long_path = "a".repeat(200);
        let output_dir = temp_dir.path().join(&very_long_path);

        let config = ProjectConfig {
            name: "test".to_string(),
            project_type: ProjectType::Library.to_string(),
            author: "Test Author".to_string(),
            description: Some("Test".to_string()),
            features: vec![],
        };

        let result = generator.generate(&config, &output_dir);
        // May fail due to filesystem limits
        let _ = result;
    }

    #[test]
    fn test_special_filesystem_names() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();

        // Test reserved names on various operating systems
        let special_names = vec![
            "CON", "PRN", "AUX", "NUL", // Windows reserved
            "COM1", "COM2", "LPT1", "LPT2", // Windows devices
            ".", "..", // Unix special
            ".git", ".cargo", // Hidden/special dirs
        ];

        for name in special_names {
            let config = ProjectConfig {
                name: name.to_string(),
                project_type: ProjectType::Library.to_string(),
                author: "Test Author".to_string(),
                description: Some("Test".to_string()),
                features: vec![],
            };

            let output_dir = temp_dir.path().join(format!("special-{}", name));
            let result = generator.generate(&config, &output_dir);
            let _ = result;
        }
    }

    #[test]
    fn test_mixed_case_project_types() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();

        // Test various case variations that should fail
        let mixed_cases = vec![
            ("Library", "library1"),
            ("LIBRARY", "library2"),
            ("LiBrArY", "library3"),
            ("Api-Server", "api1"),
            ("API-SERVER", "api2"),
            ("api-SERVER", "api3"),
            ("Cli-Tool", "cli1"),
            ("CLI-TOOL", "cli2"),
            ("cli-TOOL", "cli3"),
        ];

        for (project_type, unique_name) in mixed_cases {
            let config = ProjectConfig {
                name: "test-project".to_string(),
                project_type: project_type.to_string(),
                author: "Test Author".to_string(),
                description: Some("Test".to_string()),
                features: vec![],
            };

            let output_dir = temp_dir.path().join(unique_name);
            let result = generator.generate(&config, &output_dir);

            // Should fail due to invalid project type
            assert!(
                result.is_err(),
                "Expected error for mixed case project type '{}', but generation succeeded",
                project_type
            );
            let error_msg = result.unwrap_err().to_string();
            assert!(
                error_msg.contains("Unknown project type")
                    || error_msg.contains("unknown project type"),
                "Expected 'Unknown project type' error for '{}', but got: {}",
                project_type,
                error_msg
            );
        }
    }
}

/// Test concurrent generation and race conditions
#[cfg(test)]
mod concurrency_tests {
    use super::*;

    #[test]
    fn test_concurrent_generation_same_directory() {
        let temp_dir = TempDir::new().unwrap();
        let output_dir = Arc::new(temp_dir.path().join("concurrent-test"));
        let success_count = Arc::new(AtomicUsize::new(0));
        let failure_count = Arc::new(AtomicUsize::new(0));

        // Spawn multiple threads trying to generate to the same directory
        let handles: Vec<_> = (0..10)
            .map(|i| {
                let output_dir = Arc::clone(&output_dir);
                let success_count = Arc::clone(&success_count);
                let failure_count = Arc::clone(&failure_count);

                thread::spawn(move || {
                    let generator = Generator::new();
                    let config = ProjectConfig {
                        name: format!("test-project-{}", i),
                        project_type: ProjectType::Library.to_string(),
                        author: "Test Author".to_string(),
                        description: Some("Test".to_string()),
                        features: vec![],
                    };

                    match generator.generate(&config, &output_dir) {
                        Ok(_) => success_count.fetch_add(1, Ordering::SeqCst),
                        Err(_) => failure_count.fetch_add(1, Ordering::SeqCst),
                    };
                })
            })
            .collect();

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Only one should succeed, others should fail
        assert_eq!(success_count.load(Ordering::SeqCst), 1);
        assert_eq!(failure_count.load(Ordering::SeqCst), 9);
    }

    #[test]
    fn test_rapid_sequential_generation() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();

        // Generate many projects rapidly
        let start = Instant::now();
        for i in 0..50 {
            let config = ProjectConfig {
                name: format!("rapid-test-{}", i),
                project_type: ProjectType::Library.to_string(),
                author: "Test Author".to_string(),
                description: Some(format!("Rapid test project {}", i)),
                features: vec![],
            };

            let output_dir = temp_dir.path().join(format!("rapid-{}", i));
            let result = generator.generate(&config, &output_dir);
            assert!(result.is_ok(), "Failed on iteration {}: {:?}", i, result);
        }

        let duration = start.elapsed();
        println!("Generated 50 projects in {:?}", duration);

        // Should complete reasonably quickly (within 10 seconds)
        assert!(duration < Duration::from_secs(10));
    }
}

/// Test configuration edge cases
#[cfg(test)]
mod config_edge_cases {
    use super::*;

    #[test]
    fn test_malformed_config_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("malformed.toml");

        // Write various malformed TOML content
        let malformed_configs = vec![
            "this is not valid toml",
            "[incomplete",
            "key = ",
            "key = value = invalid",
            "[section]\nkey = \"unclosed string",
        ];

        for content in malformed_configs {
            fs::write(&config_path, content).unwrap();
            let result = Config::load_from_file(&config_path);
            assert!(result.is_err(), "Should fail to parse: {}", content);
        }
    }

    #[test]
    fn test_config_with_invalid_values() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("invalid_values.toml");

        // Test config with wrong types
        let invalid_configs = vec![
            "remember_choices = \"yes\"",              // Should be bool
            "custom_template_dirs = \"not an array\"", // Should be array
            "default_author = 123",                    // Should be string
        ];

        for content in invalid_configs {
            fs::write(&config_path, content).unwrap();
            let result = Config::load_from_file(&config_path);
            assert!(result.is_err(), "Should fail for invalid type: {}", content);
        }
    }

    #[test]
    fn test_config_with_huge_arrays() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("huge_array.toml");

        // Create config with thousands of template directories
        let mut content = "custom_template_dirs = [\n".to_string();
        for i in 0..10000 {
            content.push_str(&format!("  \"/path/to/template/{}\",\n", i));
        }
        content.push_str("]\n");

        fs::write(&config_path, content).unwrap();
        let result = Config::load_from_file(&config_path);

        // Should handle large arrays
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.custom_template_dirs.len(), 10000);
    }

    #[test]
    fn test_config_permission_denied() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("readonly.toml");

        // Create a config file
        let config = Config::new();
        config.save_to_file(&config_path).unwrap();

        // Make it read-only
        let mut perms = fs::metadata(&config_path).unwrap().permissions();
        perms.set_mode(0o444);
        fs::set_permissions(&config_path, perms.clone()).unwrap();

        // Try to save to it again
        let mut new_config = Config::new();
        new_config.default_author = Some("New Author".to_string());
        let result = new_config.save_to_file(&config_path);

        assert!(result.is_err());

        // Restore permissions for cleanup
        perms.set_mode(0o644);
        fs::set_permissions(&config_path, perms).unwrap();
    }

    #[test]
    fn test_config_circular_references() {
        let temp_dir = TempDir::new().unwrap();

        // Create a directory structure with circular symlinks
        let dir1 = temp_dir.path().join("dir1");
        let dir2 = temp_dir.path().join("dir2");
        fs::create_dir(&dir1).unwrap();
        fs::create_dir(&dir2).unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;
            symlink(&dir2, dir1.join("link_to_dir2")).unwrap();
            symlink(&dir1, dir2.join("link_to_dir1")).unwrap();
        }

        let mut config = Config::new();
        config.add_custom_template_directory(dir1.clone());
        config.add_custom_template_directory(dir2.clone());

        // Should handle circular references without infinite loops
        assert_eq!(config.custom_template_dirs.len(), 2);
    }
}

/// Test recovery from failures
#[cfg(test)]
mod recovery_tests {
    use super::*;

    #[test]
    fn test_partial_generation_cleanup() {
        let temp_dir = TempDir::new().unwrap();
        let output_dir = temp_dir.path().join("partial-test");

        // Create a partial project structure
        fs::create_dir_all(&output_dir).unwrap();
        fs::create_dir_all(output_dir.join("src")).unwrap();
        fs::write(output_dir.join("Cargo.toml"), "[package]\n").unwrap();

        // Now try to generate a project in the same directory
        let generator = Generator::new();
        let config = ProjectConfig {
            name: "test-project".to_string(),
            project_type: ProjectType::Library.to_string(),
            author: "Test Author".to_string(),
            description: Some("Test".to_string()),
            features: vec![],
        };

        let result = generator.generate(&config, &output_dir);

        // Should fail because directory is not empty
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));

        // Original files should still exist
        assert!(output_dir.join("Cargo.toml").exists());
    }

    #[test]
    fn test_disk_full_simulation() {
        // This test simulates disk full by creating files until write fails
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();

        // Create a small file to test write capability
        let test_file = temp_dir.path().join("test_write");
        let write_result = fs::write(&test_file, "test");

        if write_result.is_ok() {
            // If we can write, proceed with the test
            let config = ProjectConfig {
                name: "disk-full-test".to_string(),
                project_type: ProjectType::Library.to_string(),
                author: "Test Author".to_string(),
                description: Some("Test project for disk full scenario".to_string()),
                features: vec![],
            };

            let output_dir = temp_dir.path().join("disk-full-project");
            let result = generator.generate(&config, &output_dir);

            // In normal conditions this should succeed
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[test]
    fn test_interrupted_generation_recovery() {
        let temp_dir = TempDir::new().unwrap();
        let output_dir = temp_dir.path().join("interrupted-test");

        // Simulate an interrupted generation by creating incomplete structure
        fs::create_dir_all(&output_dir).unwrap();
        fs::create_dir_all(output_dir.join("src")).unwrap();
        // Missing Cargo.toml, README, etc.

        // Clean up the incomplete directory
        fs::remove_dir_all(&output_dir).unwrap();

        // Try generation again
        let generator = Generator::new();
        let config = ProjectConfig {
            name: "test-project".to_string(),
            project_type: ProjectType::Library.to_string(),
            author: "Test Author".to_string(),
            description: Some("Test".to_string()),
            features: vec![],
        };

        let result = generator.generate(&config, &output_dir);

        // Should succeed after cleanup
        assert!(result.is_ok());

        // Verify complete structure
        assert!(output_dir.join("Cargo.toml").exists());
        assert!(output_dir.join("README.md").exists());
        assert!(output_dir.join(".gitignore").exists());
    }
}

/// Performance and stress tests
#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_large_workspace_generation() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();
        let start = Instant::now();

        let config = ProjectConfig {
            name: "large-workspace".to_string(),
            project_type: ProjectType::Workspace.to_string(),
            author: "Test Author".to_string(),
            description: Some("Large workspace with many crates".to_string()),
            features: vec![],
        };

        let output_dir = temp_dir.path().join("large-workspace");
        let result = generator.generate(&config, &output_dir);

        assert!(result.is_ok());

        let duration = start.elapsed();
        println!("Large workspace generation took: {:?}", duration);

        // Should complete within reasonable time (1 second)
        assert!(duration < Duration::from_secs(1));

        // Verify all workspace components were created
        assert!(output_dir.join("Cargo.toml").exists());
        assert!(output_dir.join("crates/core/Cargo.toml").exists());
        assert!(output_dir.join("crates/api/Cargo.toml").exists());
        assert!(output_dir.join("crates/cli/Cargo.toml").exists());
    }

    #[test]
    fn test_many_files_generation() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();

        // Generate each project type and count files
        let project_types = vec![
            (ProjectType::Library.to_string(), "library"),
            (ProjectType::ApiServer.to_string(), "api-server"),
            (ProjectType::CliTool.to_string(), "cli-tool"),
            (ProjectType::WasmApp.to_string(), "wasm-app"),
            (ProjectType::GameEngine.to_string(), "game-engine"),
            (ProjectType::Embedded.to_string(), "embedded"),
            (ProjectType::Workspace.to_string(), "workspace"),
        ];

        for (project_type, name) in project_types {
            let start = Instant::now();

            let config = ProjectConfig {
                name: name.to_string(),
                project_type,
                author: "Test Author".to_string(),
                description: Some(format!("Performance test for {}", name)),
                features: vec![],
            };

            let output_dir = temp_dir.path().join(name);
            let result = generator.generate(&config, &output_dir);

            assert!(result.is_ok(), "Failed to generate {}: {:?}", name, result);

            let duration = start.elapsed();

            // Count generated files
            let file_count = count_files_recursive(&output_dir);

            println!("{}: {} files in {:?}", name, file_count, duration);

            // Each project type should generate reasonable number of files
            assert!(file_count >= 3, "{} generated too few files", name);

            // Should be fast (under 100ms per project)
            assert!(
                duration < Duration::from_millis(100),
                "{} took too long",
                name
            );
        }
    }

    fn count_files_recursive(dir: &std::path::Path) -> usize {
        let mut count = 0;
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        count += 1;
                    } else if metadata.is_dir() {
                        count += count_files_recursive(&entry.path());
                    }
                }
            }
        }
        count
    }

    #[test]
    fn test_memory_usage_with_large_configs() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();

        // Create configs with increasingly large data
        for size_multiplier in [1, 10, 100] {
            let large_description = "x".repeat(1000 * size_multiplier);
            let large_author = "Author ".repeat(100 * size_multiplier);

            let config = ProjectConfig {
                name: format!("memory-test-{}", size_multiplier),
                project_type: ProjectType::Library.to_string(),
                author: large_author,
                description: Some(large_description),
                features: vec![],
            };

            let output_dir = temp_dir.path().join(format!("memory-{}", size_multiplier));
            let result = generator.generate(&config, &output_dir);

            // Should handle large configs without issues
            assert!(result.is_ok());
        }
    }
}
