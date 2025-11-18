use anyhow::Result;
use std::collections::HashSet;
use tera::{Context, Tera};

/// Conditional template renderer that supports feature-based conditional rendering
pub struct ConditionalRenderer {
    tera: Tera,
    features: HashSet<String>,
}

impl ConditionalRenderer {
    pub fn new(features: Vec<String>) -> Result<Self> {
        let mut tera = Tera::default();

        // Register custom functions for feature checking
        tera.register_function("has_feature", has_feature_function);
        tera.register_function("has_any_feature", has_any_feature_function);
        tera.register_function("has_all_features", has_all_features_function);

        Ok(Self {
            tera,
            features: features.into_iter().collect(),
        })
    }

    /// Add a template with a given name
    pub fn add_template(&mut self, name: &str, content: &str) -> Result<()> {
        self.tera.add_raw_template(name, content)?;
        Ok(())
    }

    /// Render a template with feature-aware context
    pub fn render(&self, template_name: &str, mut context: Context) -> Result<String> {
        // Add features to the context
        context.insert("features", &self.features);

        // Add feature checking helpers
        for feature in &self.features {
            context.insert(&format!("has_{}", feature), &true);
        }

        let rendered = self.tera.render(template_name, &context)?;
        Ok(rendered)
    }

    /// Check if a specific feature is enabled
    pub fn has_feature(&self, feature: &str) -> bool {
        self.features.contains(feature)
    }

    /// Get all enabled features
    pub fn get_features(&self) -> Vec<String> {
        self.features.iter().cloned().collect()
    }
}

/// Tera function to check if a feature is enabled
fn has_feature_function(
    args: &HashMap<String, serde_json::Value>,
) -> tera::Result<serde_json::Value> {
    let feature = args
        .get("feature")
        .and_then(|v| v.as_str())
        .ok_or_else(|| tera::Error::msg("has_feature requires a 'feature' parameter"))?;

    let features = args
        .get("features")
        .and_then(|v| v.as_array())
        .ok_or_else(|| tera::Error::msg("has_feature requires 'features' in context"))?;

    let feature_strings: Vec<String> = features
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect();

    Ok(serde_json::Value::Bool(
        feature_strings.contains(&feature.to_string()),
    ))
}

/// Tera function to check if any of the specified features are enabled
fn has_any_feature_function(
    args: &HashMap<String, serde_json::Value>,
) -> tera::Result<serde_json::Value> {
    let check_features = args
        .get("check")
        .and_then(|v| v.as_array())
        .ok_or_else(|| tera::Error::msg("has_any_feature requires a 'check' array parameter"))?;

    let features = args
        .get("features")
        .and_then(|v| v.as_array())
        .ok_or_else(|| tera::Error::msg("has_any_feature requires 'features' in context"))?;

    let feature_strings: HashSet<String> = features
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect();

    for check_feature in check_features {
        if let Some(feature_str) = check_feature.as_str() {
            if feature_strings.contains(feature_str) {
                return Ok(serde_json::Value::Bool(true));
            }
        }
    }

    Ok(serde_json::Value::Bool(false))
}

/// Tera function to check if all specified features are enabled
fn has_all_features_function(
    args: &HashMap<String, serde_json::Value>,
) -> tera::Result<serde_json::Value> {
    let check_features = args
        .get("check")
        .and_then(|v| v.as_array())
        .ok_or_else(|| tera::Error::msg("has_all_features requires a 'check' array parameter"))?;

    let features = args
        .get("features")
        .and_then(|v| v.as_array())
        .ok_or_else(|| tera::Error::msg("has_all_features requires 'features' in context"))?;

    let feature_strings: HashSet<String> = features
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect();

    for check_feature in check_features {
        if let Some(feature_str) = check_feature.as_str() {
            if !feature_strings.contains(feature_str) {
                return Ok(serde_json::Value::Bool(false));
            }
        }
    }

    Ok(serde_json::Value::Bool(true))
}

use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conditional_rendering_with_features() {
        let mut renderer =
            ConditionalRenderer::new(vec!["database".to_string(), "auth".to_string()]).unwrap();

        renderer
            .add_template(
                "test",
                r#"
Base content
{% if has_database %}
Database feature is enabled
{% endif %}
{% if has_auth %}
Auth feature is enabled
{% endif %}
{% if has_cache %}
Cache feature is enabled
{% endif %}
"#,
            )
            .unwrap();

        let context = Context::new();
        let result = renderer.render("test", context).unwrap();

        assert!(result.contains("Database feature is enabled"));
        assert!(result.contains("Auth feature is enabled"));
        assert!(!result.contains("Cache feature is enabled"));
    }

    #[test]
    fn test_feature_functions() {
        let mut renderer =
            ConditionalRenderer::new(vec!["api".to_string(), "database".to_string()]).unwrap();

        renderer
            .add_template(
                "test",
                r#"
{% if has_feature(feature="api", features=features) %}
Has API feature
{% endif %}
{% if has_any_feature(check=["cache", "database"], features=features) %}
Has database or cache
{% endif %}
{% if has_all_features(check=["api", "database"], features=features) %}
Has both API and database
{% endif %}
{% if has_all_features(check=["api", "cache"], features=features) %}
Has both API and cache
{% endif %}
"#,
            )
            .unwrap();

        let context = Context::new();
        let result = renderer.render("test", context).unwrap();

        assert!(result.contains("Has API feature"));
        assert!(result.contains("Has database or cache"));
        assert!(result.contains("Has both API and database"));
        assert!(!result.contains("Has both API and cache"));
    }

    #[test]
    fn test_nested_conditionals() {
        let mut renderer =
            ConditionalRenderer::new(vec!["api".to_string(), "database".to_string()]).unwrap();

        renderer
            .add_template(
                "test",
                r#"
{% if has_api %}
API is enabled
{% if has_database %}
Both API and database are enabled
{% endif %}
{% endif %}
"#,
            )
            .unwrap();

        let context = Context::new();
        let result = renderer.render("test", context).unwrap();

        assert!(result.contains("API is enabled"));
        assert!(result.contains("Both API and database are enabled"));
    }
}
