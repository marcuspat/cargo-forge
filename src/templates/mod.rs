pub mod conditional;

use anyhow::Result;
use tera::{Context, Tera};
use include_dir::{include_dir, Dir};
use std::collections::HashSet;

// Embed all templates at compile time
static TEMPLATES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates");

pub struct TemplateEngine {
    tera: Tera,
    features: HashSet<String>,
}

impl TemplateEngine {
    pub fn new() -> Result<Self> {
        Self::with_features(vec![])
    }
    
    pub fn with_features(features: Vec<String>) -> Result<Self> {
        let mut tera = Tera::default();
        
        // Load all embedded templates
        Self::load_embedded_templates(&mut tera)?;
        
        Ok(Self { 
            tera,
            features: features.into_iter().collect(),
        })
    }

    pub fn render(&self, template_name: &str, context: &Context) -> Result<String> {
        let mut context = context.clone();
        
        // Add features to context for conditional rendering
        context.insert("features", &self.features.iter().cloned().collect::<Vec<_>>());
        
        // Add individual feature flags
        for feature in &self.features {
            context.insert(&format!("has_{}", feature), &true);
        }
        
        let rendered = self.tera.render(template_name, &context)?;
        Ok(rendered)
    }

    pub fn render_with_context(&self, template_name: &str, context: &Context) -> Result<String> {
        self.render(template_name, context)
    }

    /// Load all embedded templates recursively
    fn load_embedded_templates(tera: &mut Tera) -> Result<()> {
        Self::load_directory_templates(tera, &TEMPLATES_DIR, "")?;
        Ok(())
    }

    /// Recursively load templates from an embedded directory
    fn load_directory_templates(tera: &mut Tera, dir: &Dir<'_>, prefix: &str) -> Result<()> {
        // Process all files in the current directory
        for file in dir.files() {
            if let Some(file_name) = file.path().file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    if file_name_str.ends_with(".tera") {
                        // Create template name with directory prefix
                        let template_name = if prefix.is_empty() {
                            file_name_str.to_string()
                        } else {
                            format!("{}/{}", prefix, file_name_str)
                        };
                        
                        // Get file contents as string
                        if let Some(contents) = file.contents_utf8() {
                            tera.add_raw_template(&template_name, contents)?;
                        }
                    }
                }
            }
        }
        
        // Recursively process subdirectories
        for subdir in dir.dirs() {
            if let Some(dir_name) = subdir.path().file_name() {
                if let Some(dir_name_str) = dir_name.to_str() {
                    let new_prefix = if prefix.is_empty() {
                        dir_name_str.to_string()
                    } else {
                        format!("{}/{}", prefix, dir_name_str)
                    };
                    Self::load_directory_templates(tera, subdir, &new_prefix)?;
                }
            }
        }
        
        Ok(())
    }

    /// Get a list of all available templates
    pub fn list_templates(&self) -> Vec<String> {
        let mut templates = Vec::new();
        Self::collect_template_names(&TEMPLATES_DIR, "", &mut templates);
        templates.sort();
        templates
    }

    fn collect_template_names(dir: &Dir<'_>, prefix: &str, templates: &mut Vec<String>) {
        for file in dir.files() {
            if let Some(file_name) = file.path().file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    if file_name_str.ends_with(".tera") {
                        let template_name = if prefix.is_empty() {
                            file_name_str.to_string()
                        } else {
                            format!("{}/{}", prefix, file_name_str)
                        };
                        templates.push(template_name);
                    }
                }
            }
        }
        
        for subdir in dir.dirs() {
            if let Some(dir_name) = subdir.path().file_name() {
                if let Some(dir_name_str) = dir_name.to_str() {
                    let new_prefix = if prefix.is_empty() {
                        dir_name_str.to_string()
                    } else {
                        format!("{}/{}", prefix, dir_name_str)
                    };
                    Self::collect_template_names(subdir, &new_prefix, templates);
                }
            }
        }
    }
    
    /// Get templates for specific features
    pub fn get_feature_templates(&self, features: &[String]) -> Vec<String> {
        let mut templates = Vec::new();
        
        // Get base templates
        templates.extend(self.list_templates());
        
        // Add feature-specific templates
        for feature in features {
            let feature_prefix = format!("features/{}/", feature);
            templates.extend(
                self.list_templates()
                    .into_iter()
                    .filter(|t| t.starts_with(&feature_prefix))
            );
        }
        
        templates.sort();
        templates.dedup();
        templates
    }
    
    /// Check if a feature is enabled
    pub fn has_feature(&self, feature: &str) -> bool {
        self.features.contains(feature)
    }
    
    /// Add a feature
    pub fn add_feature(&mut self, feature: String) {
        self.features.insert(feature);
    }
    
    /// Get all enabled features
    pub fn get_features(&self) -> Vec<String> {
        self.features.iter().cloned().collect()
    }
}