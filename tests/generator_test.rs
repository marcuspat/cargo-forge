use cargo_forge::{Generator, ProjectConfig};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use tempfile::TempDir;

#[test]
fn test_file_generation_functionality() {
    // This test should fail as Generator doesn't exist yet
    let temp_dir = TempDir::new().unwrap();
    let output_dir = temp_dir.path().join("test-project");
    
    let config = ProjectConfig {
        name: "test-project".to_string(),
        project_type: "api-server".to_string(),
        author: "Test Author".to_string(),
        description: Some("Test description".to_string()),
        features: vec![],
    };
    
    let generator = Generator::new();
    let result = generator.generate(&config, &output_dir);
    
    assert!(result.is_ok(), "File generation should succeed");
    assert!(output_dir.exists(), "Output directory should be created");
}

#[test]
fn test_directory_creation() {
    // Test that all necessary directories are created
    let temp_dir = TempDir::new().unwrap();
    let output_dir = temp_dir.path().join("test-project");
    
    let config = ProjectConfig {
        name: "test-project".to_string(),
        project_type: "library".to_string(),
        author: "Test Author".to_string(),
        description: None,
        features: vec![],
    };
    
    let generator = Generator::new();
    generator.generate(&config, &output_dir).unwrap();
    
    // Check standard Rust project structure
    assert!(output_dir.join("src").exists(), "src directory should exist");
    assert!(output_dir.join("src").is_dir(), "src should be a directory");
    
    assert!(output_dir.join("tests").exists(), "tests directory should exist");
    assert!(output_dir.join("tests").is_dir(), "tests should be a directory");
    
    assert!(output_dir.join("Cargo.toml").exists(), "Cargo.toml should exist");
    assert!(output_dir.join("Cargo.toml").is_file(), "Cargo.toml should be a file");
}

#[test]
fn test_proper_permissions() {
    // Test that files and directories have proper permissions
    let temp_dir = TempDir::new().unwrap();
    let output_dir = temp_dir.path().join("test-project");
    
    let config = ProjectConfig {
        name: "test-project".to_string(),
        project_type: "cli-tool".to_string(),
        author: "Test Author".to_string(),
        description: Some("A CLI tool".to_string()),
        features: vec![],
    };
    
    let generator = Generator::new();
    generator.generate(&config, &output_dir).unwrap();
    
    // Check directory permissions (should be readable and executable)
    let src_metadata = fs::metadata(output_dir.join("src")).unwrap();
    let src_permissions = src_metadata.permissions();
    assert!(src_permissions.mode() & 0o755 >= 0o755, "src directory should have proper permissions");
    
    // Check file permissions (should be readable)
    let cargo_metadata = fs::metadata(output_dir.join("Cargo.toml")).unwrap();
    let cargo_permissions = cargo_metadata.permissions();
    assert!(cargo_permissions.mode() & 0o644 >= 0o644, "Cargo.toml should have proper permissions");
}

#[test]
fn test_overwrite_protection() {
    // Test that generator doesn't overwrite existing projects by default
    let temp_dir = TempDir::new().unwrap();
    let output_dir = temp_dir.path().join("test-project");
    
    // Create directory with existing file
    fs::create_dir_all(&output_dir).unwrap();
    fs::write(output_dir.join("existing.txt"), "existing content").unwrap();
    
    let config = ProjectConfig {
        name: "test-project".to_string(),
        project_type: "library".to_string(),
        author: "Test Author".to_string(),
        description: None,
        features: vec![],
    };
    
    let generator = Generator::new();
    let result = generator.generate(&config, &output_dir);
    
    // Should fail or warn about existing directory
    assert!(result.is_err() || fs::read_to_string(output_dir.join("existing.txt")).unwrap() == "existing content");
}

#[test]
fn test_template_variable_injection() {
    // Test that template variables are properly injected into generated files
    let temp_dir = TempDir::new().unwrap();
    let output_dir = temp_dir.path().join("variable-test");
    
    let config = ProjectConfig {
        name: "variable-test".to_string(),
        project_type: "api-server".to_string(),
        author: "John Doe".to_string(),
        description: Some("A test API server".to_string()),
        features: vec![],
    };
    
    let generator = Generator::new();
    generator.generate(&config, &output_dir).unwrap();
    
    // Read generated Cargo.toml and check for injected variables
    let cargo_content = fs::read_to_string(output_dir.join("Cargo.toml")).unwrap();
    assert!(cargo_content.contains("variable-test"), "Project name should be in Cargo.toml");
    assert!(cargo_content.contains("John Doe"), "Author should be in Cargo.toml");
    assert!(cargo_content.contains("A test API server"), "Description should be in Cargo.toml");
}

#[test]
fn test_binary_vs_library_generation() {
    // Test different file generation for binary vs library projects
    let temp_dir = TempDir::new().unwrap();
    
    // Test library generation
    let lib_dir = temp_dir.path().join("lib-project");
    let lib_config = ProjectConfig {
        name: "lib-project".to_string(),
        project_type: "library".to_string(),
        author: "Test Author".to_string(),
        description: None,
        features: vec![],
    };
    
    let generator = Generator::new();
    generator.generate(&lib_config, &lib_dir).unwrap();
    
    assert!(lib_dir.join("src/lib.rs").exists(), "Library should have lib.rs");
    assert!(!lib_dir.join("src/main.rs").exists(), "Library should not have main.rs");
    
    // Test binary generation
    let bin_dir = temp_dir.path().join("bin-project");
    let bin_config = ProjectConfig {
        name: "bin-project".to_string(),
        project_type: "cli-tool".to_string(),
        author: "Test Author".to_string(),
        description: None,
        features: vec![],
    };
    
    generator.generate(&bin_config, &bin_dir).unwrap();
    
    assert!(bin_dir.join("src/main.rs").exists(), "Binary should have main.rs");
    assert!(!bin_dir.join("src/lib.rs").exists(), "Binary should not have lib.rs by default");
}

#[test]
fn test_gitignore_generation() {
    // Test that .gitignore is properly generated
    let temp_dir = TempDir::new().unwrap();
    let output_dir = temp_dir.path().join("git-test");
    
    let config = ProjectConfig {
        name: "git-test".to_string(),
        project_type: "library".to_string(),
        author: "Test Author".to_string(),
        description: None,
        features: vec![],
    };
    
    let generator = Generator::new();
    generator.generate(&config, &output_dir).unwrap();
    
    let gitignore_path = output_dir.join(".gitignore");
    assert!(gitignore_path.exists(), ".gitignore should be generated");
    
    let gitignore_content = fs::read_to_string(gitignore_path).unwrap();
    assert!(gitignore_content.contains("/target"), ".gitignore should contain /target");
    assert!(gitignore_content.contains("Cargo.lock"), ".gitignore should contain Cargo.lock for libraries");
}

#[test]
fn test_readme_generation() {
    // Test that README.md is properly generated
    let temp_dir = TempDir::new().unwrap();
    let output_dir = temp_dir.path().join("readme-test");
    
    let config = ProjectConfig {
        name: "readme-test".to_string(),
        project_type: "api-server".to_string(),
        author: "Test Author".to_string(),
        description: Some("An awesome API server".to_string()),
        features: vec![],
    };
    
    let generator = Generator::new();
    generator.generate(&config, &output_dir).unwrap();
    
    let readme_path = output_dir.join("README.md");
    assert!(readme_path.exists(), "README.md should be generated");
    
    let readme_content = fs::read_to_string(readme_path).unwrap();
    assert!(readme_content.contains("readme-test"), "README should contain project name");
    assert!(readme_content.contains("An awesome API server"), "README should contain description");
}