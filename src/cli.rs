use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "cargo-forge",
    about = "A powerful Rust project generator",
    version,
    author
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new Rust project interactively
    New {
        /// Project name
        #[arg(help = "Name of the project to create")]
        name: Option<String>,

        /// Project type
        #[arg(
            short,
            long,
            help = "Type of project (api-server, cli-tool, library, wasm-app, game-engine, embedded, workspace)"
        )]
        project_type: Option<String>,

        /// Author name
        #[arg(short, long, help = "Author name for the project")]
        author: Option<String>,

        /// Project description
        #[arg(short, long, help = "Description of the project")]
        description: Option<String>,

        /// License
        #[arg(short, long, help = "License for the project")]
        license: Option<String>,

        /// Non-interactive mode with defaults for CI usage
        #[arg(long, help = "Use defaults without prompting (for CI environments)")]
        non_interactive: bool,

        /// Use configuration from file
        #[arg(long, help = "Use saved preferences from config file")]
        from_config: Option<PathBuf>,

        /// Dry run mode - preview without creating files
        #[arg(long, help = "Preview the project structure without creating files")]
        dry_run: bool,
    },

    /// Initialize a new project in the current directory
    Init {
        /// Project type
        #[arg(
            short,
            long,
            help = "Type of project (api-server, cli-tool, library, wasm-app, game-engine, embedded, workspace)"
        )]
        project_type: Option<String>,

        /// Author name
        #[arg(short, long, help = "Author name for the project")]
        author: Option<String>,

        /// License
        #[arg(short, long, help = "License for the project")]
        license: Option<String>,

        /// Non-interactive mode with defaults for CI usage
        #[arg(long, help = "Use defaults without prompting (for CI environments)")]
        non_interactive: bool,

        /// Use configuration from file
        #[arg(long, help = "Use saved preferences from config file")]
        from_config: Option<PathBuf>,

        /// Dry run mode - preview without creating files
        #[arg(long, help = "Preview the project structure without creating files")]
        dry_run: bool,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum, help = "Shell to generate completions for")]
        shell: clap_complete::Shell,
    },
}
