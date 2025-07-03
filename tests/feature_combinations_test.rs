use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

/// Represents a project with various features enabled
struct ProjectWithFeatures {
    name: String,
    features: HashSet<String>,
    generated_files: HashMap<String, String>,
    dependencies: HashMap<String, String>,
}

impl ProjectWithFeatures {
    fn new(name: &str, features: Vec<&str>) -> Self {
        Self {
            name: name.to_string(),
            features: features.into_iter().map(|s| s.to_string()).collect(),
            generated_files: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }
    
    fn has_feature(&self, feature: &str) -> bool {
        self.features.contains(feature)
    }
    
    fn generate_project_structure(&mut self) -> Vec<PathBuf> {
        let mut files = vec![];
        
        // Base files (always generated)
        files.push(PathBuf::from("Cargo.toml"));
        files.push(PathBuf::from("src/main.rs"));
        files.push(PathBuf::from("src/lib.rs"));
        files.push(PathBuf::from("README.md"));
        files.push(PathBuf::from(".gitignore"));
        
        // API feature files
        if self.has_feature("api") {
            files.push(PathBuf::from("src/routes/mod.rs"));
            files.push(PathBuf::from("src/handlers/mod.rs"));
            files.push(PathBuf::from("src/middleware/mod.rs"));
            files.push(PathBuf::from("src/config.rs"));
            
            self.dependencies.insert("axum".to_string(), "0.7".to_string());
            self.dependencies.insert("tower".to_string(), "0.4".to_string());
            self.dependencies.insert("tower-http".to_string(), "0.5".to_string());
            self.dependencies.insert("tokio".to_string(), "1".to_string());
        }
        
        // Database feature files
        if self.has_feature("database") {
            files.push(PathBuf::from("src/db.rs"));
            files.push(PathBuf::from("src/models.rs"));
            files.push(PathBuf::from("src/config/database.rs"));
            files.push(PathBuf::from("migrations/001_initial.sql"));
            files.push(PathBuf::from("migrations/002_sessions.sql"));
            files.push(PathBuf::from("src/bin/migrate.rs"));
            files.push(PathBuf::from(".env.database.example"));
            
            self.dependencies.insert("sqlx".to_string(), "0.7".to_string());
            self.dependencies.insert("chrono".to_string(), "0.4".to_string());
            self.dependencies.insert("uuid".to_string(), "1".to_string());
        }
        
        // Auth feature files
        if self.has_feature("auth") {
            files.push(PathBuf::from("src/auth/mod.rs"));
            files.push(PathBuf::from("src/auth/jwt.rs"));
            files.push(PathBuf::from("src/auth/password.rs"));
            files.push(PathBuf::from("src/auth/middleware.rs"));
            files.push(PathBuf::from("src/auth/routes.rs"));
            files.push(PathBuf::from(".env.auth.example"));
            
            self.dependencies.insert("jsonwebtoken".to_string(), "9".to_string());
            self.dependencies.insert("argon2".to_string(), "0.5".to_string());
            self.dependencies.insert("validator".to_string(), "0.16".to_string());
            
            if self.has_feature("oauth") {
                files.push(PathBuf::from("src/auth/oauth.rs"));
                self.dependencies.insert("oauth2".to_string(), "4".to_string());
                self.dependencies.insert("reqwest".to_string(), "0.11".to_string());
            }
        }
        
        // Docker feature files
        if self.has_feature("docker") {
            files.push(PathBuf::from("Dockerfile"));
            files.push(PathBuf::from(".dockerignore"));
            files.push(PathBuf::from("docker-compose.yml"));
            
            if self.has_feature("database") {
                files.push(PathBuf::from("docker-compose.dev.yml"));
            }
        }
        
        // CI feature files
        if self.has_feature("ci") {
            files.push(PathBuf::from(".github/workflows/ci.yml"));
            files.push(PathBuf::from(".github/workflows/release.yml"));
            
            if self.has_feature("docker") {
                files.push(PathBuf::from(".github/workflows/docker.yml"));
            }
        }
        
        files
    }
    
    fn validate_dependencies(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // Check for dependency conflicts
        if self.has_feature("api") && !self.dependencies.contains_key("tokio") {
            errors.push("API feature requires tokio dependency".to_string());
        }
        
        if self.has_feature("database") && self.has_feature("auth") {
            // These features work well together
            if !self.dependencies.contains_key("sqlx") {
                errors.push("Database feature should add sqlx dependency".to_string());
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    
    fn generate_docker_compose(&self) -> String {
        let mut compose = String::from("version: '3.8'\n\nservices:\n");
        
        // Main application service
        compose.push_str(&format!(
            "  {}:\n    build: .\n    ports:\n      - \"3000:3000\"\n",
            self.name
        ));
        
        if self.has_feature("database") {
            compose.push_str("    depends_on:\n      - postgres\n");
            compose.push_str("    environment:\n");
            compose.push_str(&format!("      DATABASE_URL: postgresql://postgres:password@postgres/{}\n", self.name));
        }
        
        compose.push_str("\n");
        
        // Database service
        if self.has_feature("database") {
            compose.push_str("  postgres:\n");
            compose.push_str("    image: postgres:15\n");
            compose.push_str("    environment:\n");
            compose.push_str("      POSTGRES_PASSWORD: password\n");
            compose.push_str(&format!("      POSTGRES_DB: {}\n", self.name));
            compose.push_str("    volumes:\n");
            compose.push_str("      - postgres_data:/var/lib/postgresql/data\n");
            compose.push_str("\n");
        }
        
        // Redis service for auth sessions
        if self.has_feature("auth") && self.has_feature("database") {
            compose.push_str("  redis:\n");
            compose.push_str("    image: redis:7-alpine\n");
            compose.push_str("    ports:\n");
            compose.push_str("      - \"6379:6379\"\n");
            compose.push_str("\n");
        }
        
        // Volumes
        if self.has_feature("database") {
            compose.push_str("volumes:\n");
            compose.push_str("  postgres_data:\n");
        }
        
        compose
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_api_only_project() {
        let mut project = ProjectWithFeatures::new("api-service", vec!["api"]);
        let files = project.generate_project_structure();
        
        // Should have API-specific files
        assert!(files.iter().any(|f| f.to_str().unwrap().contains("routes")));
        assert!(files.iter().any(|f| f.to_str().unwrap().contains("handlers")));
        assert!(files.iter().any(|f| f.to_str().unwrap().contains("middleware")));
        
        // Should have API dependencies
        assert!(project.dependencies.contains_key("axum"));
        assert!(project.dependencies.contains_key("tower"));
        
        // Should not have database files
        assert!(!files.iter().any(|f| f.to_str().unwrap().contains("migrations")));
    }
    
    #[test]
    fn test_api_plus_database_combination() {
        let mut project = ProjectWithFeatures::new("full-api", vec!["api", "database"]);
        let files = project.generate_project_structure();
        
        // Should have both API and database files
        assert!(files.iter().any(|f| f.to_str().unwrap().contains("routes")));
        assert!(files.iter().any(|f| f.to_str().unwrap().contains("db.rs")));
        assert!(files.iter().any(|f| f.to_str().unwrap().contains("migrations")));
        assert!(files.iter().any(|f| f.to_str().unwrap().contains("migrate.rs")));
        
        // Should have combined dependencies
        assert!(project.dependencies.contains_key("axum"));
        assert!(project.dependencies.contains_key("sqlx"));
        assert!(project.dependencies.contains_key("chrono"));
    }
    
    #[test]
    fn test_api_database_auth_combination() {
        let mut project = ProjectWithFeatures::new("secure-api", vec!["api", "database", "auth"]);
        let files = project.generate_project_structure();
        
        // Should have all feature files
        assert!(files.iter().any(|f| f.to_str().unwrap().contains("routes")));
        assert!(files.iter().any(|f| f.to_str().unwrap().contains("db.rs")));
        assert!(files.iter().any(|f| f.to_str().unwrap().contains("auth/jwt.rs")));
        assert!(files.iter().any(|f| f.to_str().unwrap().contains("auth/middleware.rs")));
        
        // Should have all dependencies
        assert!(project.dependencies.contains_key("axum"));
        assert!(project.dependencies.contains_key("sqlx"));
        assert!(project.dependencies.contains_key("jsonwebtoken"));
        assert!(project.dependencies.contains_key("argon2"));
    }
    
    #[test]
    fn test_auth_with_oauth_sub_feature() {
        let mut project = ProjectWithFeatures::new("oauth-app", vec!["api", "auth", "oauth"]);
        let files = project.generate_project_structure();
        
        // Should have OAuth files
        assert!(files.iter().any(|f| f.to_str().unwrap().contains("auth/oauth.rs")));
        
        // Should have OAuth dependencies
        assert!(project.dependencies.contains_key("oauth2"));
        assert!(project.dependencies.contains_key("reqwest"));
    }
    
    #[test]
    fn test_docker_integration() {
        let mut project = ProjectWithFeatures::new("dockerized-app", vec!["api", "database", "docker"]);
        let files = project.generate_project_structure();
        
        // Should have Docker files
        assert!(files.iter().any(|f| f.to_str().unwrap() == "Dockerfile"));
        assert!(files.iter().any(|f| f.to_str().unwrap() == "docker-compose.yml"));
        assert!(files.iter().any(|f| f.to_str().unwrap() == ".dockerignore"));
        assert!(files.iter().any(|f| f.to_str().unwrap() == "docker-compose.dev.yml"));
    }
    
    #[test]
    fn test_docker_compose_generation() {
        let project = ProjectWithFeatures::new("test-app", vec!["api", "database", "auth", "docker"]);
        let compose = project.generate_docker_compose();
        
        // Should have main app service
        assert!(compose.contains("test-app:"));
        assert!(compose.contains("build: ."));
        
        // Should have postgres service
        assert!(compose.contains("postgres:"));
        assert!(compose.contains("postgres:15"));
        assert!(compose.contains("POSTGRES_PASSWORD"));
        
        // Should have redis service for auth
        assert!(compose.contains("redis:"));
        assert!(compose.contains("redis:7-alpine"));
        
        // Should have volumes
        assert!(compose.contains("volumes:"));
        assert!(compose.contains("postgres_data:"));
    }
    
    #[test]
    fn test_ci_integration() {
        let mut project = ProjectWithFeatures::new("ci-app", vec!["api", "ci"]);
        let files = project.generate_project_structure();
        
        // Should have CI files
        assert!(files.iter().any(|f| f.to_str().unwrap().contains(".github/workflows/ci.yml")));
        assert!(files.iter().any(|f| f.to_str().unwrap().contains(".github/workflows/release.yml")));
    }
    
    #[test]
    fn test_ci_with_docker() {
        let mut project = ProjectWithFeatures::new("ci-docker-app", vec!["api", "ci", "docker"]);
        let files = project.generate_project_structure();
        
        // Should have Docker CI workflow
        assert!(files.iter().any(|f| f.to_str().unwrap().contains(".github/workflows/docker.yml")));
    }
    
    #[test]
    fn test_complete_feature_combination() {
        let mut project = ProjectWithFeatures::new("complete-app", vec![
            "api", "database", "auth", "oauth", "docker", "ci"
        ]);
        let files = project.generate_project_structure();
        
        // Count total files - should have many
        assert!(files.len() > 25, "Complete project should have many files");
        
        // Verify all major components are present
        let file_paths: Vec<String> = files.iter()
            .map(|p| p.to_str().unwrap().to_string())
            .collect();
        
        // API files
        assert!(file_paths.iter().any(|f| f.contains("routes")));
        assert!(file_paths.iter().any(|f| f.contains("handlers")));
        
        // Database files
        assert!(file_paths.iter().any(|f| f.contains("migrations")));
        assert!(file_paths.iter().any(|f| f.contains("db.rs")));
        
        // Auth files
        assert!(file_paths.iter().any(|f| f.contains("auth/jwt.rs")));
        assert!(file_paths.iter().any(|f| f.contains("auth/oauth.rs")));
        
        // Docker files
        assert!(file_paths.iter().any(|f| f == "Dockerfile"));
        assert!(file_paths.iter().any(|f| f == "docker-compose.yml"));
        
        // CI files
        assert!(file_paths.iter().any(|f| f.contains(".github/workflows")));
        
        // Validate dependencies
        assert!(project.validate_dependencies().is_ok());
    }
    
    #[test]
    fn test_minimal_vs_maximal_projects() {
        let minimal = ProjectWithFeatures::new("minimal", vec![]);
        let maximal = ProjectWithFeatures::new("maximal", vec![
            "api", "database", "auth", "oauth", "docker", "ci"
        ]);
        
        let mut minimal_mut = minimal;
        let mut maximal_mut = maximal;
        
        let minimal_files = minimal_mut.generate_project_structure();
        let maximal_files = maximal_mut.generate_project_structure();
        
        // Minimal should have only base files
        assert!(minimal_files.len() < 10);
        
        // Maximal should have many more files
        assert!(maximal_files.len() > 25);
        
        // Maximal should have way more dependencies
        assert!(minimal_mut.dependencies.is_empty());
        assert!(maximal_mut.dependencies.len() > 10);
    }
}