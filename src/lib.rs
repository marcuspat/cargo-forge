// Re-export main types and modules
pub use crate::config::Config;
pub use crate::features::{Plugin, PluginManager, ProjectContext};
pub use crate::forge::Forge;
pub use crate::generator::{Generator, ProjectConfig};
pub use crate::project_types::ProjectType;
pub use crate::templates::TemplateEngine;

// Module declarations
pub mod config;
pub mod external_generators;
pub mod features;
pub mod forge;
pub mod generator;
pub mod project_types;
pub mod templates;
