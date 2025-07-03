use std::collections::HashMap;
use std::path::Path;
use std::fs;
use anyhow::Result;

/// Test helper to create a mock template renderer that supports conditional rendering
struct ConditionalTemplateRenderer {
    features: Vec<String>,
    templates: HashMap<String, String>,
}

impl ConditionalTemplateRenderer {
    fn new(features: Vec<String>) -> Self {
        Self {
            features,
            templates: HashMap::new(),
        }
    }
    
    fn add_template(&mut self, name: &str, content: &str) {
        self.templates.insert(name.to_string(), content.to_string());
    }
    
    fn has_feature(&self, feature: &str) -> bool {
        self.features.contains(&feature.to_string())
    }
    
    fn render(&self, template_name: &str) -> Result<String> {
        let template = self.templates.get(template_name)
            .ok_or_else(|| anyhow::anyhow!("Template not found: {}", template_name))?;
        
        // Simple conditional rendering implementation for testing
        let mut result = String::new();
        let mut lines = template.lines();
        let mut skip_until_endif = false;
        
        while let Some(line) = lines.next() {
            if line.trim().starts_with("{{#if ") && line.trim().ends_with("}}") {
                let feature = line.trim()
                    .strip_prefix("{{#if ")
                    .unwrap()
                    .strip_suffix("}}")
                    .unwrap()
                    .trim();
                    
                skip_until_endif = !self.has_feature(feature);
            } else if line.trim() == "{{/if}}" {
                skip_until_endif = false;
            } else if !skip_until_endif {
                result.push_str(line);
                result.push('\n');
            }
        }
        
        Ok(result.trim_end().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_conditional_rendering_with_database_feature() {
        let mut renderer = ConditionalTemplateRenderer::new(vec!["database".to_string()]);
        
        renderer.add_template("main.rs", r#"use std::env;

{{#if database}}
use sqlx::PgPool;
{{/if}}

fn main() {
    println!("Hello, world!");
    {{#if database}}
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    {{/if}}
}"#);

        let result = renderer.render("main.rs").unwrap();
        
        // Should include database imports and code
        assert!(result.contains("use sqlx::PgPool;"));
        assert!(result.contains("DATABASE_URL"));
        assert!(!result.contains("{{#if"));
        assert!(!result.contains("{{/if}}"));
    }
    
    #[test]
    fn test_conditional_rendering_without_database_feature() {
        let mut renderer = ConditionalTemplateRenderer::new(vec![]);
        
        renderer.add_template("main.rs", r#"use std::env;

{{#if database}}
use sqlx::PgPool;
{{/if}}

fn main() {
    println!("Hello, world!");
    {{#if database}}
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    {{/if}}
}"#);

        let result = renderer.render("main.rs").unwrap();
        
        // Should NOT include database imports and code
        assert!(!result.contains("use sqlx::PgPool;"));
        assert!(!result.contains("DATABASE_URL"));
        assert!(!result.contains("{{#if"));
        assert!(!result.contains("{{/if}}"));
    }
    
    #[test]
    fn test_conditional_rendering_with_auth_feature() {
        let mut renderer = ConditionalTemplateRenderer::new(vec!["auth".to_string()]);
        
        renderer.add_template("lib.rs", r#"pub mod routes;
{{#if auth}}
pub mod auth;
pub mod jwt;
{{/if}}
{{#if database}}
pub mod db;
{{/if}}

pub fn init() {
    {{#if auth}}
    auth::init_auth_system();
    {{/if}}
}"#);

        let result = renderer.render("lib.rs").unwrap();
        
        // Should include auth modules but not database
        assert!(result.contains("pub mod auth;"));
        assert!(result.contains("pub mod jwt;"));
        assert!(result.contains("auth::init_auth_system()"));
        assert!(!result.contains("pub mod db;"));
    }
    
    #[test]
    fn test_conditional_rendering_multiple_features() {
        let mut renderer = ConditionalTemplateRenderer::new(vec![
            "database".to_string(),
            "auth".to_string(),
            "docker".to_string()
        ]);
        
        renderer.add_template("Cargo.toml", r#"[package]
name = "my-app"
version = "0.1.0"

[dependencies]
tokio = "1.0"
{{#if database}}
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls"] }
{{/if}}
{{#if auth}}
jsonwebtoken = "9.0"
bcrypt = "0.14"
{{/if}}

[dev-dependencies]
{{#if docker}}
testcontainers = "0.15"
{{/if}}"#);

        let result = renderer.render("Cargo.toml").unwrap();
        
        // Should include all feature dependencies
        assert!(result.contains("sqlx ="));
        assert!(result.contains("jsonwebtoken ="));
        assert!(result.contains("bcrypt ="));
        assert!(result.contains("testcontainers ="));
    }
    
    #[test]
    fn test_nested_conditional_rendering() {
        let mut renderer = ConditionalTemplateRenderer::new(vec![
            "api".to_string(),
            "database".to_string()
        ]);
        
        renderer.add_template("main.rs", r#"{{#if api}}
use axum::{Router, routing::get};
{{#if database}}
use sqlx::PgPool;
{{/if}}

async fn main() {
    {{#if database}}
    let pool = PgPool::connect(&database_url).await?;
    {{/if}}
    
    let app = Router::new()
        .route("/", get(handler));
}
{{/if}}"#);

        let result = renderer.render("main.rs").unwrap();
        
        // Should include both API and database code
        assert!(result.contains("use axum::{Router, routing::get};"));
        assert!(result.contains("use sqlx::PgPool;"));
        assert!(result.contains("PgPool::connect"));
    }
    
    #[test]
    fn test_feature_combination_api_plus_database() {
        let mut renderer = ConditionalTemplateRenderer::new(vec![
            "api".to_string(),
            "database".to_string()
        ]);
        
        renderer.add_template("dependencies", r#"axum = "0.7"
tokio = { version = "1", features = ["full"] }
{{#if database}}
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls"] }
diesel = { version = "2.1", features = ["postgres"] }
{{/if}}
{{#if auth}}
jsonwebtoken = "9.0"
{{/if}}"#);

        let result = renderer.render("dependencies").unwrap();
        
        // API + Database should include database dependencies
        assert!(result.contains("sqlx ="));
        assert!(result.contains("diesel ="));
        // But not auth dependencies
        assert!(!result.contains("jsonwebtoken"));
    }
    
    #[test]
    fn test_feature_combination_api_plus_docker() {
        let mut renderer = ConditionalTemplateRenderer::new(vec![
            "api".to_string(),
            "docker".to_string()
        ]);
        
        renderer.add_template("docker-compose.yml", r#"version: '3.8'

services:
  app:
    build: .
    ports:
      - "3000:3000"
{{#if database}}
  postgres:
    image: postgres:15
    environment:
      POSTGRES_PASSWORD: password
{{/if}}
{{#if auth}}
  redis:
    image: redis:7
    ports:
      - "6379:6379"
{{/if}}"#);

        let result = renderer.render("docker-compose.yml").unwrap();
        
        // Should have app service but not database or redis
        assert!(result.contains("app:"));
        assert!(result.contains("3000:3000"));
        assert!(!result.contains("postgres:"));
        assert!(!result.contains("redis:"));
    }
    
    #[test] 
    fn test_template_file_selection_based_on_features() {
        // Test that different template files are selected based on features
        let features = vec!["database".to_string()];
        
        let template_files = get_template_files_for_features(&features);
        
        assert!(template_files.contains(&"templates/features/database/db.rs"));
        assert!(template_files.contains(&"templates/features/database/migrations/001_initial.sql"));
        assert!(!template_files.contains(&"templates/features/auth/jwt.rs"));
    }
    
    #[test]
    fn test_cargo_toml_dependency_merging() {
        // Test that dependencies from multiple features are properly merged
        let mut base_deps = HashMap::new();
        base_deps.insert("tokio".to_string(), "1.0".to_string());
        
        let mut db_deps = HashMap::new();
        db_deps.insert("sqlx".to_string(), "0.7".to_string());
        
        let mut auth_deps = HashMap::new();
        auth_deps.insert("jsonwebtoken".to_string(), "9.0".to_string());
        
        let merged = merge_dependencies(vec![base_deps, db_deps, auth_deps]);
        
        assert_eq!(merged.get("tokio").unwrap(), "1.0");
        assert_eq!(merged.get("sqlx").unwrap(), "0.7");
        assert_eq!(merged.get("jsonwebtoken").unwrap(), "9.0");
    }
}

// Helper functions that would be implemented in the actual code
fn get_template_files_for_features(features: &[String]) -> Vec<&'static str> {
    let mut files = vec![];
    
    if features.contains(&"database".to_string()) {
        files.push("templates/features/database/db.rs");
        files.push("templates/features/database/migrations/001_initial.sql");
    }
    
    if features.contains(&"auth".to_string()) {
        files.push("templates/features/auth/jwt.rs");
        files.push("templates/features/auth/oauth.rs");
    }
    
    files
}

fn merge_dependencies(deps_list: Vec<HashMap<String, String>>) -> HashMap<String, String> {
    let mut merged = HashMap::new();
    
    for deps in deps_list {
        for (key, value) in deps {
            merged.insert(key, value);
        }
    }
    
    merged
}