use cargo_forge::{Generator, ProjectConfig};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_docker_feature_generates_files() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test-docker-api");

    let config = ProjectConfig {
        name: "test-docker-api".to_string(),
        project_type: "api-server".to_string(),
        author: "Test Author".to_string(),
        description: Some("Test API with Docker".to_string()),
        features: vec!["docker".to_string()],
        target: None,
        esp32_chip: None
    };

    let generator = Generator::new();
    generator.generate(&config, &project_path).unwrap();

    // Check that Docker files were created
    assert!(
        project_path.join("Dockerfile").exists(),
        "Dockerfile should be created"
    );
    assert!(
        project_path.join(".dockerignore").exists(),
        ".dockerignore should be created"
    );
    assert!(
        project_path.join("scripts/docker-build.sh").exists(),
        "docker-build.sh script should be created"
    );

    // Check that the README contains Docker section
    let readme_content = fs::read_to_string(project_path.join("README.md")).unwrap();
    assert!(
        readme_content.contains("Docker Support"),
        "README should contain Docker section"
    );
}

#[test]
fn test_ci_feature_generates_files() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test-ci-cli");

    let config = ProjectConfig {
        name: "test-ci-cli".to_string(),
        project_type: "cli-tool".to_string(),
        author: "Test Author".to_string(),
        description: Some("Test CLI with CI".to_string()),
        features: vec!["ci".to_string()],
        target: None,
        esp32_chip: None
    };

    let generator = Generator::new();
    generator.generate(&config, &project_path).unwrap();

    // Check that CI files were created
    assert!(
        project_path.join(".github/workflows/ci.yml").exists(),
        "GitHub Actions CI workflow should be created"
    );
}

#[test]
fn test_database_feature_generates_files() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test-database-api");

    let config = ProjectConfig {
        name: "test-database-api".to_string(),
        project_type: "api-server".to_string(),
        author: "Test Author".to_string(),
        description: Some("Test API with Database".to_string()),
        features: vec!["database".to_string()],
        target: None,
        esp32_chip: None
    };

    let generator = Generator::new();
    generator.generate(&config, &project_path).unwrap();

    // Check that database files were created
    assert!(
        project_path.join("src/database.rs").exists(),
        "database.rs should be created"
    );
    assert!(
        project_path.join("migrations").exists(),
        "migrations directory should be created"
    );
}

#[test]
fn test_multiple_features() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("test-full-api");

    let config = ProjectConfig {
        name: "test-full-api".to_string(),
        project_type: "api-server".to_string(),
        author: "Test Author".to_string(),
        description: Some("Test API with all features".to_string()),
        features: vec![
            "docker".to_string(),
            "ci".to_string(),
            "database".to_string(),
        ],
        target: None,
        esp32_chip: None
    };

    let generator = Generator::new();
    generator.generate(&config, &project_path).unwrap();

    // Check that all feature files were created
    assert!(
        project_path.join("Dockerfile").exists(),
        "Dockerfile should be created"
    );
    assert!(
        project_path.join(".github/workflows/ci.yml").exists(),
        "CI workflow should be created"
    );
    assert!(
        project_path.join("src/database.rs").exists(),
        "database.rs should be created"
    );
    assert!(
        project_path.join("migrations").exists(),
        "migrations directory should be created"
    );
}
