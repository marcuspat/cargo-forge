pub mod database;
pub mod docker;
pub mod ci;

use std::error::Error;
use std::collections::HashMap;

/// Extended project context for plugins that adds fields needed for file generation
pub struct ProjectContext {
    pub name: String,
    pub dependencies: HashMap<String, String>,
    pub dev_dependencies: HashMap<String, String>,
    pub template_files: HashMap<String, String>,
    pub directories: Vec<String>,
    pub gitignore_entries: Vec<String>,
    pub readme_sections: Vec<String>,
    pub examples: HashMap<String, String>,
}

impl ProjectContext {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            template_files: HashMap::new(),
            directories: Vec::new(),
            gitignore_entries: Vec::new(),
            readme_sections: Vec::new(),
            examples: HashMap::new(),
        }
    }
    
    pub fn add_dependency(&mut self, name: &str, version: &str) {
        self.dependencies.insert(name.to_string(), version.to_string());
    }
    
    pub fn add_dev_dependency(&mut self, name: &str, version: &str) {
        self.dev_dependencies.insert(name.to_string(), version.to_string());
    }
    
    pub fn add_template_file(&mut self, path: &str, content: String) {
        self.template_files.insert(path.to_string(), content);
    }
    
    pub fn create_directory(&mut self, path: &str) {
        self.directories.push(path.to_string());
    }
    
    pub fn add_to_gitignore(&mut self, entry: &str) {
        self.gitignore_entries.push(entry.to_string());
    }
    
    pub fn add_to_readme(&mut self, section: &str) {
        self.readme_sections.push(section.to_string());
    }
    
    pub fn add_example(&mut self, name: &str, code: String) {
        self.examples.insert(name.to_string(), code);
    }
}

pub trait Plugin {
    fn name(&self) -> &str;
    
    fn configure(&self, context: &mut ProjectContext) -> Result<(), Box<dyn Error>>;
    
    fn post_configure(&self, _context: &ProjectContext) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }
    
    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }
    
    pub fn len(&self) -> usize {
        self.plugins.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }
    
    pub fn configure_all(&self, context: &mut ProjectContext) -> Result<(), Box<dyn Error>> {
        for plugin in &self.plugins {
            println!("Configuring plugin: {}", plugin.name());
            plugin.configure(context)?;
        }
        
        for plugin in &self.plugins {
            plugin.post_configure(context)?;
        }
        
        Ok(())
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}