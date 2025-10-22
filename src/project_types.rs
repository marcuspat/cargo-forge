use serde::Serialize;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ProjectType {
    ApiServer,
    CliTool,
    Library,
    WasmApp,
    GameEngine,
    Embedded,
    Workspace,
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectType::ApiServer => write!(f, "api-server"),
            ProjectType::CliTool => write!(f, "cli-tool"),
            ProjectType::Library => write!(f, "library"),
            ProjectType::WasmApp => write!(f, "wasm-app"),
            ProjectType::GameEngine => write!(f, "game-engine"),
            ProjectType::Embedded => write!(f, "embedded"),
            ProjectType::Workspace => write!(f, "workspace"),
        }
    }
}

impl FromStr for ProjectType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "api-server" => Ok(ProjectType::ApiServer),
            "cli-tool" => Ok(ProjectType::CliTool),
            "library" => Ok(ProjectType::Library),
            "wasm-app" => Ok(ProjectType::WasmApp),
            "game-engine" => Ok(ProjectType::GameEngine),
            "embedded" => Ok(ProjectType::Embedded),
            "workspace" => Ok(ProjectType::Workspace),
            _ => Err(anyhow::anyhow!("Invalid project type: {}", s)),
        }
    }
}

impl ProjectType {
    pub fn default_features(&self) -> Vec<&'static str> {
        match self {
            ProjectType::ApiServer => vec!["axum", "tokio", "serde", "tower"],
            ProjectType::CliTool => vec!["clap", "anyhow", "env_logger"],
            ProjectType::Library => vec![],
            ProjectType::WasmApp => vec!["wasm-bindgen", "web-sys", "js-sys"],
            ProjectType::GameEngine => vec!["bevy"],
            ProjectType::Embedded => vec!["cortex-m", "cortex-m-rt", "panic-halt"],
            ProjectType::Workspace => vec!["tokio", "serde", "anyhow"],
        }
    }

    pub fn requires_external_generator(&self, target: Option<&str>) -> bool {
        match self {
            ProjectType::Embedded => target == Some("esp32"),
            _ => false,
        }
    }
}
