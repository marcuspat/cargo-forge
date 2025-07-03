use cargo_forge::{Generator, ProjectConfig};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;
use std::collections::HashMap;

/// Feature matrix testing for comprehensive validation of project generation
/// This module tests various combinations of project configurations, edge cases,
/// and feature interactions to ensure robust project generation.

/// Test matrix configuration
#[derive(Debug, Clone)]
pub struct MatrixTestConfig {
    pub name: String,
    pub project_type: String,
    pub author: String,
    pub description: Option<String>,
    pub features: Vec<String>,
    pub test_category: String,
    pub expected_behavior: TestExpectation,
    pub validation_rules: Vec<ValidationRule>,
}

/// Expected behavior for tests
#[derive(Debug, Clone)]
pub enum TestExpectation {
    Success,
    Failure(String),
    ConditionalSuccess(String),
}

/// Validation rules for different aspects of generated projects
#[derive(Debug, Clone)]
pub enum ValidationRule {
    FileExists(String),
    FileContains(String, String),
    FileNotContains(String, String),
    CargoCheckPasses,
    CargoBuildPasses,
    CargoTestPasses,
    DirectoryExists(String),
    FileExecutable(String),
    FileSize(String, u64, u64), // file, min_size, max_size
    LineCount(String, usize, usize), // file, min_lines, max_lines
}

/// Feature matrix test suite
pub struct FeatureMatrixTestSuite {
    generator: Generator,
    temp_dir: TempDir,
    test_results: HashMap<String, Result<(), String>>,
}

impl FeatureMatrixTestSuite {
    pub fn new() -> Self {
        Self {
            generator: Generator::new(),
            temp_dir: TempDir::new().expect("Failed to create temp directory"),
            test_results: HashMap::new(),
        }
    }

    /// Generate comprehensive test matrix covering various scenarios
    fn generate_test_matrix() -> Vec<MatrixTestConfig> {
        let mut matrix = Vec::new();

        // Project name edge cases
        matrix.extend(vec![
            MatrixTestConfig {
                name: "a".to_string(),
                project_type: "library".to_string(),
                author: "Test <test@example.com>".to_string(),
                description: Some("Single character project name".to_string()),
                features: vec![],
                test_category: "Edge Case - Project Names".to_string(),
                expected_behavior: TestExpectation::Success,
                validation_rules: vec![
                    ValidationRule::FileExists("Cargo.toml".to_string()),
                    ValidationRule::FileContains("Cargo.toml".to_string(), "name = \"a\"".to_string()),
                    ValidationRule::CargoCheckPasses,
                ],
            },
            MatrixTestConfig {
                name: "very-long-project-name-that-tests-the-limits-of-reasonable-naming-conventions-and-filesystem-compatibility".to_string(),
                project_type: "library".to_string(),
                author: "Test <test@example.com>".to_string(),
                description: Some("Very long project name".to_string()),
                features: vec![],
                test_category: "Edge Case - Project Names".to_string(),
                expected_behavior: TestExpectation::Success,
                validation_rules: vec![
                    ValidationRule::FileExists("Cargo.toml".to_string()),
                    ValidationRule::CargoCheckPasses,
                ],
            },
            MatrixTestConfig {
                name: "project_with_underscores".to_string(),
                project_type: "library".to_string(),
                author: "Test <test@example.com>".to_string(),
                description: Some("Project with underscores".to_string()),
                features: vec![],
                test_category: "Edge Case - Project Names".to_string(),
                expected_behavior: TestExpectation::Success,
                validation_rules: vec![
                    ValidationRule::FileExists("Cargo.toml".to_string()),
                    ValidationRule::FileContains("Cargo.toml".to_string(), "name = \"project_with_underscores\"".to_string()),
                    ValidationRule::CargoCheckPasses,
                ],
            },
            MatrixTestConfig {
                name: "project-with-hyphens".to_string(),
                project_type: "library".to_string(),
                author: "Test <test@example.com>".to_string(),
                description: Some("Project with hyphens".to_string()),
                features: vec![],
                test_category: "Edge Case - Project Names".to_string(),
                expected_behavior: TestExpectation::Success,
                validation_rules: vec![
                    ValidationRule::FileExists("Cargo.toml".to_string()),
                    ValidationRule::FileContains("Cargo.toml".to_string(), "name = \"project-with-hyphens\"".to_string()),
                    ValidationRule::CargoCheckPasses,
                ],
            },
        ]);

        // Author variations
        matrix.extend(vec![
            MatrixTestConfig {
                name: "author-test-simple".to_string(),
                project_type: "library".to_string(),
                author: "Simple Name".to_string(),
                description: Some("Simple author name".to_string()),
                features: vec![],
                test_category: "Author Variations".to_string(),
                expected_behavior: TestExpectation::Success,
                validation_rules: vec![
                    ValidationRule::FileContains("Cargo.toml".to_string(), "authors = [\"Simple Name\"]".to_string()),
                    ValidationRule::CargoCheckPasses,
                ],
            },
            MatrixTestConfig {
                name: "author-test-email".to_string(),
                project_type: "library".to_string(),
                author: "John Doe <john@example.com>".to_string(),
                description: Some("Author with email".to_string()),
                features: vec![],
                test_category: "Author Variations".to_string(),
                expected_behavior: TestExpectation::Success,
                validation_rules: vec![
                    ValidationRule::FileContains("Cargo.toml".to_string(), "authors = [\"John Doe <john@example.com>\"]".to_string()),
                    ValidationRule::CargoCheckPasses,
                ],
            },
            MatrixTestConfig {
                name: "author-test-unicode".to_string(),
                project_type: "library".to_string(),
                author: "José García <josé@example.com>".to_string(),
                description: Some("Author with unicode characters".to_string()),
                features: vec![],
                test_category: "Author Variations".to_string(),
                expected_behavior: TestExpectation::Success,
                validation_rules: vec![
                    ValidationRule::FileContains("Cargo.toml".to_string(), "José García".to_string()),
                    ValidationRule::CargoCheckPasses,
                ],
            },
        ]);

        // Description variations
        matrix.extend(vec![
            MatrixTestConfig {
                name: "no-description".to_string(),
                project_type: "library".to_string(),
                author: "Test <test@example.com>".to_string(),
                description: None,
                features: vec![],
                test_category: "Description Variations".to_string(),
                expected_behavior: TestExpectation::Success,
                validation_rules: vec![
                    ValidationRule::FileExists("Cargo.toml".to_string()),
                    ValidationRule::FileNotContains("Cargo.toml".to_string(), "description =".to_string()),
                    ValidationRule::CargoCheckPasses,
                ],
            },
            MatrixTestConfig {
                name: "long-description".to_string(),
                project_type: "library".to_string(),
                author: "Test <test@example.com>".to_string(),
                description: Some("This is a very long description that tests how the project generator handles descriptions with multiple sentences and various punctuation marks! It should handle this gracefully and properly escape any special characters that might cause issues in the TOML format.".to_string()),
                features: vec![],
                test_category: "Description Variations".to_string(),
                expected_behavior: TestExpectation::Success,
                validation_rules: vec![
                    ValidationRule::FileExists("Cargo.toml".to_string()),
                    ValidationRule::FileContains("Cargo.toml".to_string(), "description =".to_string()),
                    ValidationRule::CargoCheckPasses,
                ],
            },
            MatrixTestConfig {
                name: "special-chars-desc".to_string(),
                project_type: "library".to_string(),
                author: "Test <test@example.com>".to_string(),
                description: Some("Description with \"quotes\", 'apostrophes', and other special chars: @#$%^&*()".to_string()),
                features: vec![],
                test_category: "Description Variations".to_string(),
                expected_behavior: TestExpectation::Success,
                validation_rules: vec![
                    ValidationRule::FileExists("Cargo.toml".to_string()),
                    ValidationRule::FileContains("Cargo.toml".to_string(), "description =".to_string()),
                    ValidationRule::CargoCheckPasses,
                ],
            },
        ]);

        // Project type combinations with different configurations
        matrix.extend(vec![
            MatrixTestConfig {
                name: "cli-minimal".to_string(),
                project_type: "cli-tool".to_string(),
                author: "CLI Test <cli@example.com>".to_string(),
                description: Some("Minimal CLI tool".to_string()),
                features: vec![],
                test_category: "CLI Variations".to_string(),
                expected_behavior: TestExpectation::Success,
                validation_rules: vec![
                    ValidationRule::FileExists("src/main.rs".to_string()),
                    ValidationRule::FileExists("src/cli.rs".to_string()),
                    ValidationRule::FileExists("src/commands.rs".to_string()),
                    ValidationRule::FileContains("Cargo.toml".to_string(), "clap".to_string()),
                    ValidationRule::FileContains("Cargo.toml".to_string(), "[[bin]]".to_string()),
                    ValidationRule::CargoCheckPasses,
                    ValidationRule::CargoBuildPasses,
                ],
            },
            MatrixTestConfig {
                name: "api-minimal".to_string(),
                project_type: "api-server".to_string(),
                author: "API Test <api@example.com>".to_string(),
                description: Some("Minimal API server".to_string()),
                features: vec![],
                test_category: "API Variations".to_string(),
                expected_behavior: TestExpectation::Success,
                validation_rules: vec![
                    ValidationRule::FileExists("src/main.rs".to_string()),
                    ValidationRule::FileExists("src/routes.rs".to_string()),
                    ValidationRule::FileExists("src/handlers.rs".to_string()),
                    ValidationRule::FileExists("src/models.rs".to_string()),
                    ValidationRule::FileExists("config/default.toml".to_string()),
                    ValidationRule::FileExists(".env.example".to_string()),
                    ValidationRule::FileContains("Cargo.toml".to_string(), "axum".to_string()),
                    ValidationRule::FileContains("Cargo.toml".to_string(), "tokio".to_string()),
                    ValidationRule::CargoCheckPasses,
                    ValidationRule::CargoBuildPasses,
                ],
            },
            MatrixTestConfig {
                name: "wasm-minimal".to_string(),
                project_type: "wasm-app".to_string(),
                author: "WASM Test <wasm@example.com>".to_string(),
                description: Some("Minimal WASM app".to_string()),
                features: vec![],
                test_category: "WASM Variations".to_string(),
                expected_behavior: TestExpectation::Success,
                validation_rules: vec![
                    ValidationRule::FileExists("src/lib.rs".to_string()),
                    ValidationRule::FileExists("index.html".to_string()),
                    ValidationRule::FileExists("index.js".to_string()),
                    ValidationRule::FileExists("package.json".to_string()),
                    ValidationRule::FileExists("webpack.config.js".to_string()),
                    ValidationRule::FileExists("build.sh".to_string()),
                    ValidationRule::FileExecutable("build.sh".to_string()),
                    ValidationRule::FileContains("Cargo.toml".to_string(), "wasm-bindgen".to_string()),
                    ValidationRule::FileContains("Cargo.toml".to_string(), "crate-type = [\"cdylib\"]".to_string()),
                    ValidationRule::CargoCheckPasses,
                ],
            },
        ]);

        // File size and content quality tests
        matrix.extend(vec![
            MatrixTestConfig {
                name: "readme-quality".to_string(),
                project_type: "library".to_string(),
                author: "README Test <readme@example.com>".to_string(),
                description: Some("Testing README quality and content".to_string()),
                features: vec![],
                test_category: "Content Quality".to_string(),
                expected_behavior: TestExpectation::Success,
                validation_rules: vec![
                    ValidationRule::FileExists("README.md".to_string()),
                    ValidationRule::FileSize("README.md".to_string(), 100, 10000),
                    ValidationRule::LineCount("README.md".to_string(), 10, 100),
                    ValidationRule::FileContains("README.md".to_string(), "# readme-quality".to_string()),
                    ValidationRule::FileContains("README.md".to_string(), "Testing README quality".to_string()),
                    ValidationRule::CargoCheckPasses,
                ],
            },
            MatrixTestConfig {
                name: "cargo-toml-quality".to_string(),
                project_type: "workspace".to_string(),
                author: "Cargo Test <cargo@example.com>".to_string(),
                description: Some("Testing Cargo.toml quality and structure".to_string()),
                features: vec![],
                test_category: "Content Quality".to_string(),
                expected_behavior: TestExpectation::Success,
                validation_rules: vec![
                    ValidationRule::FileExists("Cargo.toml".to_string()),
                    ValidationRule::FileContains("Cargo.toml".to_string(), "[workspace]".to_string()),
                    ValidationRule::FileContains("Cargo.toml".to_string(), "[workspace.package]".to_string()),
                    ValidationRule::FileContains("Cargo.toml".to_string(), "[workspace.dependencies]".to_string()),
                    ValidationRule::FileContains("Cargo.toml".to_string(), "members = [".to_string()),
                    ValidationRule::FileSize("Cargo.toml".to_string(), 100, 5000),
                    ValidationRule::CargoCheckPasses,
                ],
            },
        ]);

        // Cross-platform compatibility tests
        matrix.extend(vec![
            MatrixTestConfig {
                name: "cross-platform-paths".to_string(),
                project_type: "game-engine".to_string(),
                author: "Cross Platform <cross@example.com>".to_string(),
                description: Some("Testing cross-platform path handling".to_string()),
                features: vec![],
                test_category: "Cross-Platform".to_string(),
                expected_behavior: TestExpectation::Success,
                validation_rules: vec![
                    ValidationRule::DirectoryExists("assets/models".to_string()),
                    ValidationRule::DirectoryExists("assets/textures".to_string()),
                    ValidationRule::DirectoryExists("assets/sounds".to_string()),
                    ValidationRule::DirectoryExists("assets/shaders".to_string()),
                    ValidationRule::DirectoryExists(".github/workflows".to_string()),
                    ValidationRule::FileExists("assets/README.md".to_string()),
                    ValidationRule::FileExists(".github/workflows/wasm.yml".to_string()),
                    ValidationRule::CargoCheckPasses,
                ],
            },
        ]);

        // Performance and stress tests
        matrix.extend(vec![
            MatrixTestConfig {
                name: "stress-test-1".to_string(),
                project_type: "library".to_string(),
                author: "Stress Test <stress@example.com>".to_string(),
                description: Some("First stress test project".to_string()),
                features: vec![],
                test_category: "Performance".to_string(),
                expected_behavior: TestExpectation::Success,
                validation_rules: vec![
                    ValidationRule::FileExists("Cargo.toml".to_string()),
                    ValidationRule::FileExists("README.md".to_string()),
                    ValidationRule::CargoCheckPasses,
                ],
            },
            MatrixTestConfig {
                name: "stress-test-2".to_string(),
                project_type: "cli-tool".to_string(),
                author: "Stress Test <stress@example.com>".to_string(),
                description: Some("Second stress test project".to_string()),
                features: vec![],
                test_category: "Performance".to_string(),
                expected_behavior: TestExpectation::Success,
                validation_rules: vec![
                    ValidationRule::FileExists("Cargo.toml".to_string()),
                    ValidationRule::FileExists("README.md".to_string()),
                    ValidationRule::CargoCheckPasses,
                ],
            },
        ]);

        matrix
    }

    /// Apply a single validation rule
    fn apply_validation_rule(&self, rule: &ValidationRule, project_dir: &Path) -> Result<(), String> {
        match rule {
            ValidationRule::FileExists(file) => {
                let path = project_dir.join(file);
                if !path.exists() {
                    return Err(format!("File {} does not exist", file));
                }
            }
            ValidationRule::FileContains(file, content) => {
                let path = project_dir.join(file);
                let file_content = fs::read_to_string(&path)
                    .map_err(|e| format!("Failed to read {}: {}", file, e))?;
                if !file_content.contains(content) {
                    return Err(format!("File {} does not contain '{}'", file, content));
                }
            }
            ValidationRule::FileNotContains(file, content) => {
                let path = project_dir.join(file);
                let file_content = fs::read_to_string(&path)
                    .map_err(|e| format!("Failed to read {}: {}", file, e))?;
                if file_content.contains(content) {
                    return Err(format!("File {} should not contain '{}'", file, content));
                }
            }
            ValidationRule::CargoCheckPasses => {
                let output = Command::new("cargo")
                    .arg("check")
                    .current_dir(project_dir)
                    .output()
                    .map_err(|e| format!("Failed to run cargo check: {}", e))?;
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("Cargo check failed: {}", stderr));
                }
            }
            ValidationRule::CargoBuildPasses => {
                let output = Command::new("cargo")
                    .arg("build")
                    .current_dir(project_dir)
                    .output()
                    .map_err(|e| format!("Failed to run cargo build: {}", e))?;
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("Cargo build failed: {}", stderr));
                }
            }
            ValidationRule::CargoTestPasses => {
                let output = Command::new("cargo")
                    .arg("test")
                    .current_dir(project_dir)
                    .output()
                    .map_err(|e| format!("Failed to run cargo test: {}", e))?;
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(format!("Cargo test failed: {}", stderr));
                }
            }
            ValidationRule::DirectoryExists(dir) => {
                let path = project_dir.join(dir);
                if !path.exists() || !path.is_dir() {
                    return Err(format!("Directory {} does not exist", dir));
                }
            }
            ValidationRule::FileExecutable(file) => {
                let path = project_dir.join(file);
                if !path.exists() {
                    return Err(format!("File {} does not exist", file));
                }
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let metadata = fs::metadata(&path)
                        .map_err(|e| format!("Failed to get metadata for {}: {}", file, e))?;
                    let permissions = metadata.permissions();
                    if (permissions.mode() & 0o111) == 0 {
                        return Err(format!("File {} is not executable", file));
                    }
                }
            }
            ValidationRule::FileSize(file, min_size, max_size) => {
                let path = project_dir.join(file);
                let metadata = fs::metadata(&path)
                    .map_err(|e| format!("Failed to get metadata for {}: {}", file, e))?;
                let size = metadata.len();
                if size < *min_size || size > *max_size {
                    return Err(format!("File {} size {} is not in range {}-{}", file, size, min_size, max_size));
                }
            }
            ValidationRule::LineCount(file, min_lines, max_lines) => {
                let path = project_dir.join(file);
                let content = fs::read_to_string(&path)
                    .map_err(|e| format!("Failed to read {}: {}", file, e))?;
                let line_count = content.lines().count();
                if line_count < *min_lines || line_count > *max_lines {
                    return Err(format!("File {} line count {} is not in range {}-{}", file, line_count, min_lines, max_lines));
                }
            }
        }
        Ok(())
    }

    /// Run a single matrix test
    fn run_matrix_test(&mut self, config: &MatrixTestConfig) -> Result<(), String> {
        let project_dir = self.temp_dir.path().join(&config.name);
        
        // Generate project
        let basic_config = ProjectConfig {
            name: config.name.clone(),
            project_type: config.project_type.clone(),
            author: config.author.clone(),
            description: config.description.clone(),
            features: config.features.clone(),
        };

        self.generator.generate(&basic_config, &project_dir)
            .map_err(|e| format!("Failed to generate project: {}", e))?;

        // Apply validation rules
        for rule in &config.validation_rules {
            self.apply_validation_rule(rule, &project_dir)?;
        }

        Ok(())
    }

    /// Run all matrix tests
    pub fn run_all_matrix_tests(&mut self) -> HashMap<String, Result<(), String>> {
        let matrix = Self::generate_test_matrix();
        let mut results = HashMap::new();
        
        for config in matrix {
            let result = self.run_matrix_test(&config);
            match &result {
                Ok(()) => println!("✅ Matrix test {} passed", config.name),
                Err(e) => println!("❌ Matrix test {} failed: {}", config.name, e),
            }
            results.insert(config.name.clone(), result);
        }
        
        results
    }

    /// Generate test report by category
    pub fn generate_matrix_report(&self, results: &HashMap<String, Result<(), String>>) -> String {
        let mut report = String::new();
        report.push_str("=== Feature Matrix Test Report ===\n\n");
        
        // Group results by category
        let matrix = Self::generate_test_matrix();
        let mut categories: HashMap<String, Vec<(String, bool)>> = HashMap::new();
        
        for config in matrix {
            let category = config.test_category.clone();
            let test_name = config.name.clone();
            let success = results.get(&test_name).map_or(false, |r| r.is_ok());
            
            categories.entry(category).or_insert_with(Vec::new).push((test_name, success));
        }
        
        // Generate report by category
        for (category, tests) in categories {
            let total = tests.len();
            let passed = tests.iter().filter(|(_, success)| *success).count();
            let failed = total - passed;
            
            report.push_str(&format!("## {}\n", category));
            report.push_str(&format!("Total: {}, Passed: {}, Failed: {}\n", total, passed, failed));
            
            for (test_name, success) in tests {
                let status = if success { "PASS" } else { "FAIL" };
                report.push_str(&format!("  {} - {}\n", test_name, status));
            }
            report.push('\n');
        }
        
        let total_tests = results.len();
        let passed_tests = results.values().filter(|r| r.is_ok()).count();
        let failed_tests = total_tests - passed_tests;
        
        report.push_str(&format!("=== Summary ===\n"));
        report.push_str(&format!("Total Tests: {}\n", total_tests));
        report.push_str(&format!("Passed: {}\n", passed_tests));
        report.push_str(&format!("Failed: {}\n", failed_tests));
        report.push_str(&format!("Success Rate: {:.2}%\n", (passed_tests as f64 / total_tests as f64) * 100.0));
        
        report
    }
}

/// Main test for feature matrix validation
#[test]
fn test_feature_matrix_comprehensive() {
    let mut test_suite = FeatureMatrixTestSuite::new();
    let results = test_suite.run_all_matrix_tests();
    
    // Generate and print report
    let report = test_suite.generate_matrix_report(&results);
    println!("{}", report);
    
    // Ensure all tests passed
    let failed_tests: Vec<&String> = results.iter()
        .filter(|(_, result)| result.is_err())
        .map(|(name, _)| name)
        .collect();
    
    if !failed_tests.is_empty() {
        panic!("The following matrix tests failed: {:?}", failed_tests);
    }
}

/// Test specific edge cases
#[test]
fn test_edge_cases() {
    let mut test_suite = FeatureMatrixTestSuite::new();
    
    // Test single character project name
    let config = MatrixTestConfig {
        name: "x".to_string(),
        project_type: "library".to_string(),
        author: "Test <test@example.com>".to_string(),
        description: Some("Single char test".to_string()),
                features: vec![],
        test_category: "Edge Cases".to_string(),
        expected_behavior: TestExpectation::Success,
        validation_rules: vec![
            ValidationRule::CargoCheckPasses,
            ValidationRule::FileExists("Cargo.toml".to_string()),
        ],
    };
    
    test_suite.run_matrix_test(&config)
        .expect("Single character project name should work");
}

/// Test performance under stress
#[test]
fn test_matrix_performance() {
    let start_time = std::time::Instant::now();
    let mut test_suite = FeatureMatrixTestSuite::new();
    
    // Generate multiple projects rapidly
    let configs = vec![
        MatrixTestConfig {
            name: "perf-1".to_string(),
            project_type: "library".to_string(),
            author: "Perf Test <perf@example.com>".to_string(),
            description: Some("Performance test 1".to_string()),
                features: vec![],
            test_category: "Performance".to_string(),
            expected_behavior: TestExpectation::Success,
            validation_rules: vec![ValidationRule::CargoCheckPasses],
        },
        MatrixTestConfig {
            name: "perf-2".to_string(),
            project_type: "cli-tool".to_string(),
            author: "Perf Test <perf@example.com>".to_string(),
            description: Some("Performance test 2".to_string()),
                features: vec![],
            test_category: "Performance".to_string(),
            expected_behavior: TestExpectation::Success,
            validation_rules: vec![ValidationRule::CargoCheckPasses],
        },
        MatrixTestConfig {
            name: "perf-3".to_string(),
            project_type: "api-server".to_string(),
            author: "Perf Test <perf@example.com>".to_string(),
            description: Some("Performance test 3".to_string()),
                features: vec![],
            test_category: "Performance".to_string(),
            expected_behavior: TestExpectation::Success,
            validation_rules: vec![ValidationRule::CargoCheckPasses],
        },
    ];
    
    for config in configs {
        test_suite.run_matrix_test(&config)
            .expect(&format!("Performance test {} should pass", config.name));
    }
    
    let elapsed = start_time.elapsed();
    println!("Matrix performance test completed in {:?}", elapsed);
    
    // Should complete within reasonable time
    assert!(elapsed.as_secs() < 60, "Matrix performance test took too long: {:?}", elapsed);
}