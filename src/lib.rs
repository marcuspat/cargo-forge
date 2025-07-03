// Re-export main types and modules
pub use crate::forge::Forge;
pub use crate::generator::{Generator, ProjectConfig};
pub use crate::project_types::ProjectType;
pub use crate::templates::TemplateEngine;
pub use crate::features::{Plugin, PluginManager, ProjectContext};
pub use crate::config::Config;

// Module declarations
pub mod forge;
pub mod generator;
pub mod project_types;
pub mod templates;
pub mod features;
pub mod config;