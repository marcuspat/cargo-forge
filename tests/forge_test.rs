use cargo_forge::{Forge, ProjectType};
use std::io::Cursor;
use tempfile::TempDir;

#[test]
fn test_interactive_flow() {
    // This test should fail as Forge struct doesn't exist yet
    let temp_dir = TempDir::new().unwrap();
    let forge = Forge::new(temp_dir.path());

    // Simulate user input for interactive flow
    let input = "1\nmy-awesome-project\n";
    let mut cursor = Cursor::new(input);

    let result = forge.run_interactive(&mut cursor);
    assert!(
        result.is_ok(),
        "Interactive flow should complete successfully"
    );
}

#[test]
fn test_user_can_select_project_type() {
    // This test should fail as the functionality doesn't exist yet
    let temp_dir = TempDir::new().unwrap();
    let forge = Forge::new(temp_dir.path());

    // Test selecting API Server (option 1)
    let input = "1\n";
    let mut cursor = Cursor::new(input);

    let project_type = forge.prompt_project_type(&mut cursor).unwrap();
    assert_eq!(project_type, ProjectType::ApiServer);

    // Test selecting CLI Tool (option 2)
    let input = "2\n";
    let mut cursor = Cursor::new(input);

    let project_type = forge.prompt_project_type(&mut cursor).unwrap();
    assert_eq!(project_type, ProjectType::CliTool);

    // Test selecting Library (option 3)
    let input = "3\n";
    let mut cursor = Cursor::new(input);

    let project_type = forge.prompt_project_type(&mut cursor).unwrap();
    assert_eq!(project_type, ProjectType::Library);

    // Test selecting WASM App (option 4)
    let input = "4\n";
    let mut cursor = Cursor::new(input);

    let project_type = forge.prompt_project_type(&mut cursor).unwrap();
    assert_eq!(project_type, ProjectType::WasmApp);
}

#[test]
fn test_project_name_validation() {
    // This test should fail as the validation doesn't exist yet
    let temp_dir = TempDir::new().unwrap();
    let forge = Forge::new(temp_dir.path());

    // Test valid project names
    assert!(forge.validate_project_name("my-project").is_ok());
    assert!(forge.validate_project_name("my_project").is_ok());
    assert!(forge.validate_project_name("myproject123").is_ok());
    assert!(forge.validate_project_name("a").is_ok());

    // Test invalid project names
    assert!(forge.validate_project_name("").is_err());
    assert!(forge.validate_project_name("My-Project").is_err()); // No capitals
    assert!(forge.validate_project_name("my project").is_err()); // No spaces
    assert!(forge.validate_project_name("my/project").is_err()); // No slashes
    assert!(forge.validate_project_name("123project").is_err()); // Can't start with number
    assert!(forge.validate_project_name("-project").is_err()); // Can't start with dash
    assert!(forge.validate_project_name("project-").is_err()); // Can't end with dash
}

#[test]
fn test_interactive_flow_with_invalid_input() {
    // Test handling of invalid inputs
    let temp_dir = TempDir::new().unwrap();
    let forge = Forge::new(temp_dir.path());

    // Invalid project type selection
    let input = "99\n1\nvalid-project\n";
    let mut cursor = Cursor::new(input);

    let result = forge.run_interactive(&mut cursor);
    assert!(result.is_ok(), "Should recover from invalid input");
}

#[test]
fn test_interactive_flow_creates_project_directory() {
    // Test that the interactive flow creates the project directory
    let temp_dir = TempDir::new().unwrap();
    let forge = Forge::new(temp_dir.path());

    let input = "1\ntest-project\n";
    let mut cursor = Cursor::new(input);

    forge.run_interactive(&mut cursor).unwrap();

    let project_path = temp_dir.path().join("test-project");
    assert!(project_path.exists(), "Project directory should be created");
    assert!(project_path.is_dir(), "Project path should be a directory");
}
