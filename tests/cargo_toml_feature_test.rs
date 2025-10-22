use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toml::{Table, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CargoToml {
    package: Package,
    dependencies: Table,
    #[serde(rename = "dev-dependencies", default)]
    dev_dependencies: Table,
    #[serde(default)]
    features: HashMap<String, Vec<String>>,
    #[serde(flatten)]
    other: Table,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Package {
    name: String,
    version: String,
    edition: String,
    authors: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

/// Manages feature-based dependency modifications for Cargo.toml
struct FeatureDependencyManager {
    base_dependencies: Table,
    base_dev_dependencies: Table,
    feature_dependencies: HashMap<String, Table>,
    feature_dev_dependencies: HashMap<String, Table>,
}

impl FeatureDependencyManager {
    fn new() -> Self {
        Self {
            base_dependencies: Table::new(),
            base_dev_dependencies: Table::new(),
            feature_dependencies: HashMap::new(),
            feature_dev_dependencies: HashMap::new(),
        }
    }

    fn add_base_dependency(&mut self, name: &str, version: Value) {
        self.base_dependencies.insert(name.to_string(), version);
    }

    fn add_feature_dependency(&mut self, feature: &str, name: &str, version: Value) {
        self.feature_dependencies
            .entry(feature.to_string())
            .or_insert_with(Table::new)
            .insert(name.to_string(), version);
    }

    fn add_feature_dev_dependency(&mut self, feature: &str, name: &str, version: Value) {
        self.feature_dev_dependencies
            .entry(feature.to_string())
            .or_insert_with(Table::new)
            .insert(name.to_string(), version);
    }

    fn build_dependencies(&self, features: &[String]) -> Table {
        let mut dependencies = self.base_dependencies.clone();

        for feature in features {
            if let Some(feature_deps) = self.feature_dependencies.get(feature) {
                for (name, version) in feature_deps {
                    dependencies.insert(name.clone(), version.clone());
                }
            }
        }

        dependencies
    }

    fn build_dev_dependencies(&self, features: &[String]) -> Table {
        let mut dev_dependencies = self.base_dev_dependencies.clone();

        for feature in features {
            if let Some(feature_deps) = self.feature_dev_dependencies.get(feature) {
                for (name, version) in feature_deps {
                    dev_dependencies.insert(name.clone(), version.clone());
                }
            }
        }

        dev_dependencies
    }
}

/// Initialize feature dependencies
fn init_feature_dependencies() -> FeatureDependencyManager {
    let mut manager = FeatureDependencyManager::new();

    // Base dependencies (always included)
    manager.add_base_dependency(
        "serde",
        toml::toml! {
            version = "1"
            features = ["derive"]
        }
        .into(),
    );
    manager.add_base_dependency(
        "tokio",
        toml::toml! {
            version = "1"
            features = ["full"]
        }
        .into(),
    );

    // Database feature dependencies
    manager.add_feature_dependency(
        "database",
        "sqlx",
        toml::toml! {
            version = "0.7"
            features = ["runtime-tokio-native-tls", "postgres", "chrono", "uuid"]
        }
        .into(),
    );
    manager.add_feature_dependency(
        "database",
        "chrono",
        toml::toml! {
            version = "0.4"
            features = ["serde"]
        }
        .into(),
    );
    manager.add_feature_dependency(
        "database",
        "uuid",
        toml::toml! {
            version = "1"
            features = ["v4", "serde"]
        }
        .into(),
    );

    // Auth feature dependencies
    manager.add_feature_dependency("auth", "jsonwebtoken", Value::String("9".to_string()));
    manager.add_feature_dependency("auth", "argon2", Value::String("0.5".to_string()));
    manager.add_feature_dependency(
        "auth",
        "validator",
        toml::toml! {
            version = "0.16"
            features = ["derive"]
        }
        .into(),
    );
    manager.add_feature_dependency("auth", "once_cell", Value::String("1".to_string()));
    manager.add_feature_dependency("auth", "rand", Value::String("0.8".to_string()));
    manager.add_feature_dependency("auth", "zeroize", Value::String("1".to_string()));

    // OAuth sub-feature of auth
    manager.add_feature_dependency("oauth", "oauth2", Value::String("4".to_string()));
    manager.add_feature_dependency(
        "oauth",
        "reqwest",
        toml::toml! {
            version = "0.11"
            features = ["json"]
        }
        .into(),
    );

    // API feature dependencies
    manager.add_feature_dependency("api", "axum", Value::String("0.7".to_string()));
    manager.add_feature_dependency("api", "tower", Value::String("0.4".to_string()));
    manager.add_feature_dependency(
        "api",
        "tower-http",
        toml::toml! {
            version = "0.5"
            features = ["fs", "trace", "cors"]
        }
        .into(),
    );

    // Docker feature dependencies (mostly dev dependencies)
    manager.add_feature_dev_dependency(
        "docker",
        "testcontainers",
        Value::String("0.15".to_string()),
    );

    // CI feature dependencies
    manager.add_feature_dev_dependency("ci", "cargo-tarpaulin", Value::String("0.27".to_string()));

    manager
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_dependencies_only() {
        let manager = init_feature_dependencies();
        let deps = manager.build_dependencies(&[]);

        assert!(deps.contains_key("serde"));
        assert!(deps.contains_key("tokio"));
        assert!(!deps.contains_key("sqlx"));
        assert!(!deps.contains_key("jsonwebtoken"));
    }

    #[test]
    fn test_database_feature_dependencies() {
        let manager = init_feature_dependencies();
        let deps = manager.build_dependencies(&["database".to_string()]);

        // Should have base dependencies
        assert!(deps.contains_key("serde"));
        assert!(deps.contains_key("tokio"));

        // Should have database dependencies
        assert!(deps.contains_key("sqlx"));
        assert!(deps.contains_key("chrono"));
        assert!(deps.contains_key("uuid"));

        // Should not have auth dependencies
        assert!(!deps.contains_key("jsonwebtoken"));
    }

    #[test]
    fn test_auth_feature_dependencies() {
        let manager = init_feature_dependencies();
        let deps = manager.build_dependencies(&["auth".to_string()]);

        // Should have auth dependencies
        assert!(deps.contains_key("jsonwebtoken"));
        assert!(deps.contains_key("argon2"));
        assert!(deps.contains_key("validator"));
        assert!(deps.contains_key("once_cell"));
        assert!(deps.contains_key("rand"));
        assert!(deps.contains_key("zeroize"));

        // Should not have database dependencies
        assert!(!deps.contains_key("sqlx"));
    }

    #[test]
    fn test_oauth_feature_dependencies() {
        let manager = init_feature_dependencies();
        let deps = manager.build_dependencies(&["auth".to_string(), "oauth".to_string()]);

        // Should have auth dependencies
        assert!(deps.contains_key("jsonwebtoken"));

        // Should have oauth dependencies
        assert!(deps.contains_key("oauth2"));
        assert!(deps.contains_key("reqwest"));
    }

    #[test]
    fn test_multiple_features_combined() {
        let manager = init_feature_dependencies();
        let deps = manager.build_dependencies(&[
            "database".to_string(),
            "auth".to_string(),
            "api".to_string(),
        ]);

        // Should have all feature dependencies
        assert!(deps.contains_key("sqlx"));
        assert!(deps.contains_key("jsonwebtoken"));
        assert!(deps.contains_key("axum"));
        assert!(deps.contains_key("tower"));
        assert!(deps.contains_key("tower-http"));
    }

    #[test]
    fn test_dev_dependencies_with_features() {
        let manager = init_feature_dependencies();
        let dev_deps = manager.build_dev_dependencies(&["docker".to_string(), "ci".to_string()]);

        assert!(dev_deps.contains_key("testcontainers"));
        assert!(dev_deps.contains_key("cargo-tarpaulin"));
    }

    #[test]
    fn test_dependency_version_formats() {
        let manager = init_feature_dependencies();
        let deps = manager.build_dependencies(&["database".to_string()]);

        // Test that sqlx has proper feature configuration
        let sqlx_dep = deps.get("sqlx").unwrap();
        if let Value::Table(table) = sqlx_dep {
            assert_eq!(table.get("version").unwrap().as_str().unwrap(), "0.7");

            let features = table.get("features").unwrap().as_array().unwrap();
            assert!(features.iter().any(|f| f.as_str().unwrap() == "postgres"));
            assert!(features
                .iter()
                .any(|f| f.as_str().unwrap() == "runtime-tokio-native-tls"));
        } else {
            panic!("sqlx dependency should be a table");
        }
    }

    #[test]
    fn test_cargo_toml_generation() {
        let manager = init_feature_dependencies();
        let features = vec!["database".to_string(), "auth".to_string()];

        let cargo_toml = CargoToml {
            package: Package {
                name: "test-app".to_string(),
                version: "0.1.0".to_string(),
                edition: "2021".to_string(),
                authors: vec!["Test Author".to_string()],
                description: Some("Test application".to_string()),
            },
            dependencies: manager.build_dependencies(&features),
            dev_dependencies: manager.build_dev_dependencies(&features),
            features: HashMap::new(),
            other: Table::new(),
        };

        // Serialize to TOML string
        let toml_string = toml::to_string_pretty(&cargo_toml).unwrap();

        // Verify the output contains expected content
        assert!(toml_string.contains("[package]"));
        assert!(toml_string.contains("name = \"test-app\""));
        assert!(toml_string.contains("[dependencies]"));
        assert!(toml_string.contains("sqlx"));
        assert!(toml_string.contains("jsonwebtoken"));
    }

    #[test]
    fn test_conditional_dependency_inclusion() {
        let manager = init_feature_dependencies();

        // Test that oauth dependencies are only included when oauth feature is enabled
        let deps_without_oauth = manager.build_dependencies(&["auth".to_string()]);
        assert!(!deps_without_oauth.contains_key("oauth2"));

        let deps_with_oauth =
            manager.build_dependencies(&["auth".to_string(), "oauth".to_string()]);
        assert!(deps_with_oauth.contains_key("oauth2"));
    }

    #[test]
    fn test_no_duplicate_dependencies() {
        let mut manager = FeatureDependencyManager::new();

        // Add same dependency in base and feature
        manager.add_base_dependency("common", Value::String("1.0".to_string()));
        manager.add_feature_dependency("feature1", "common", Value::String("2.0".to_string()));

        let deps = manager.build_dependencies(&["feature1".to_string()]);

        // Feature version should override base version
        assert_eq!(deps.get("common").unwrap().as_str().unwrap(), "2.0");
    }
}
