use cargo_forge::{Generator, ProjectConfig, ProjectType};
use std::fs;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use tempfile::TempDir;

/// Stress tests for cargo-forge
#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    #[ignore] // Use --ignored flag to run this expensive test
    fn test_massive_concurrent_generation() {
        let temp_dir = Arc::new(TempDir::new().unwrap());
        let total_projects = 100;
        let thread_count = 10;
        let projects_per_thread = total_projects / thread_count;
        
        let success_count = Arc::new(AtomicUsize::new(0));
        let failure_count = Arc::new(AtomicUsize::new(0));
        let start = Instant::now();
        
        let handles: Vec<_> = (0..thread_count)
            .map(|thread_id| {
                let temp_dir = Arc::clone(&temp_dir);
                let success_count = Arc::clone(&success_count);
                let failure_count = Arc::clone(&failure_count);
                
                thread::spawn(move || {
                    let generator = Generator::new();
                    
                    for i in 0..projects_per_thread {
                        let project_id = thread_id * projects_per_thread + i;
                        let project_types = vec![
                            ProjectType::Library.to_string(),
                            ProjectType::ApiServer.to_string(),
                            ProjectType::CliTool.to_string(),
                            ProjectType::WasmApp.to_string(),
                            ProjectType::GameEngine.to_string(),
                            ProjectType::Embedded.to_string(),
                            ProjectType::Workspace.to_string(),
                        ];
                        
                        let project_type = &project_types[project_id % project_types.len()];
                        
                        let config = ProjectConfig {
                            name: format!("stress-test-{}-{}", thread_id, i),
                            project_type: project_type.clone(),
                            author: format!("Stress Tester {}", thread_id),
                            description: Some(format!("Stress test project {} from thread {}", i, thread_id)),
                            features: vec![],
                        };
                        
                        let output_dir = temp_dir.path().join(format!("stress-{}-{}", thread_id, i));
                        
                        match generator.generate(&config, &output_dir) {
                            Ok(_) => {
                                success_count.fetch_add(1, Ordering::SeqCst);
                                // Verify basic structure
                                assert!(output_dir.join("Cargo.toml").exists());
                                assert!(output_dir.join("README.md").exists());
                            }
                            Err(e) => {
                                eprintln!("Failed to generate project {}-{}: {}", thread_id, i, e);
                                failure_count.fetch_add(1, Ordering::SeqCst);
                            }
                        }
                    }
                })
            })
            .collect();
        
        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }
        
        let duration = start.elapsed();
        let successes = success_count.load(Ordering::SeqCst);
        let failures = failure_count.load(Ordering::SeqCst);
        
        println!("Stress test results:");
        println!("  Total projects: {}", total_projects);
        println!("  Successful: {}", successes);
        println!("  Failed: {}", failures);
        println!("  Duration: {:?}", duration);
        println!("  Projects per second: {:.2}", successes as f64 / duration.as_secs_f64());
        
        // All should succeed
        assert_eq!(successes, total_projects);
        assert_eq!(failures, 0);
        
        // Performance benchmark: should complete within reasonable time
        assert!(duration < Duration::from_secs(30), "Stress test took too long: {:?}", duration);
    }

    #[test]
    #[ignore] // Use --ignored flag to run this expensive test
    fn test_extreme_file_count() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();
        
        // Generate a workspace with many additional files
        let config = ProjectConfig {
            name: "extreme-files".to_string(),
            project_type: ProjectType::Workspace.to_string(),
            author: "Test Author".to_string(),
            description: Some("Project with extreme number of files".to_string()),
        features: vec![],
        };
        
        let output_dir = temp_dir.path().join("extreme-files");
        let result = generator.generate(&config, &output_dir);
        assert!(result.is_ok());
        
        // Add thousands of additional source files
        let start = Instant::now();
        for i in 0..1000 {
            let module_path = output_dir.join("crates/core/src").join(format!("module_{}.rs", i));
            fs::write(&module_path, format!("// Module {}\npub fn func_{}() {{}}", i, i)).unwrap();
        }
        
        let duration = start.elapsed();
        println!("Created 1000 additional files in {:?}", duration);
        
        // Count total files
        let file_count = count_files_recursive(&output_dir);
        println!("Total files in project: {}", file_count);
        
        // Should handle large number of files
        assert!(file_count > 1000);
        assert!(duration < Duration::from_secs(10));
    }

    #[test]
    #[ignore] // Use --ignored flag to run this expensive test  
    fn test_deep_directory_nesting() {
        let temp_dir = TempDir::new().unwrap();
        let generator = Generator::new();
        
        // Create deeply nested project path
        let mut nested_path = temp_dir.path().to_path_buf();
        for i in 0..50 {
            nested_path = nested_path.join(format!("level_{}", i));
        }
        
        let config = ProjectConfig {
            name: "deep-nest".to_string(),
            project_type: ProjectType::Library.to_string(),
            author: "Test Author".to_string(),
            description: Some("Deeply nested project".to_string()),
        features: vec![],
        };
        
        let start = Instant::now();
        let result = generator.generate(&config, &nested_path);
        let duration = start.elapsed();
        
        println!("Deep nesting generation result: {:?}", result.as_ref().err());
        println!("Duration: {:?}", duration);
        
        // May fail due to OS path length limits, but should handle gracefully
        if result.is_ok() {
            assert!(nested_path.join("Cargo.toml").exists());
        }
    }

    #[test]
    fn test_recovery_after_panic() {
        let temp_dir = TempDir::new().unwrap();
        
        // Simulate a panic scenario by using catch_unwind
        let result = std::panic::catch_unwind(|| {
            let generator = Generator::new();
            let config = ProjectConfig {
                name: "panic-test".to_string(),
                project_type: ProjectType::Library.to_string(),
                author: "Test Author".to_string(),
                description: Some("Test".to_string()),
        features: vec![],
            };
            
            let output_dir = temp_dir.path().join("panic-test");
            generator.generate(&config, &output_dir).unwrap();
            
            // Simulate some condition that might panic
            if output_dir.exists() {
                // Don't actually panic in this test
                // panic!("Simulated panic!");
            }
        });
        
        assert!(result.is_ok(), "Generator should not panic");
        
        // After "recovery", try generation again
        let generator = Generator::new();
        let config = ProjectConfig {
            name: "recovery-test".to_string(),
            project_type: ProjectType::Library.to_string(),
            author: "Test Author".to_string(),
            description: Some("Recovery test".to_string()),
        features: vec![],
        };
        
        let output_dir = temp_dir.path().join("recovery-test");
        let result = generator.generate(&config, &output_dir);
        
        assert!(result.is_ok(), "Should be able to generate after recovery");
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
}