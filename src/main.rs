mod cli;

use anyhow::Result;
use cargo_forge::Forge;
use clap::{CommandFactory, Parser};
use clap_complete::{generate, Generator};
use colored::*;
use std::io;

use crate::cli::{Cli, Commands};

fn print_completions<G: Generator>(gen: G, cmd: &mut clap::Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn display_logo() {
    let logo = r#"
    ╔═══════════════════════════════════════════════════╗
    ║                                                   ║
    ║   ⚒️  ▄████▄   ▄▄▄       ██▀███    ▄████   ▄▄▄▄  ║
    ║      ▒██▀ ▀█  ▒████▄    ▓██ ▒ ██▒ ██▒ ▀█▒ ▒██▄   ║
    ║      ▒▓█    ▄ ▒██  ▀█▄  ▓██ ░▄█ ▒▒██░▄▄▄░ ▒▓███  ║
    ║      ▒▓▓▄ ▄██▒░██▄▄▄▄██ ▒██▀▀█▄  ░▓█  ██▓ ▒██▒   ║
    ║      ▒ ▓███▀ ░ ▓█   ▓██▒░██▓ ▒██▒░▒▓███▀▒ ▒██░   ║
    ║      ░ ░▒ ▒  ░ ▒▒   ▓▒█░░ ▒▓ ░▒▓░ ░▒   ▒  ░ ▒░   ║
    ║        ░  ▒     ▒   ▒▒ ░  ░▒ ░ ▒░  ░   ░  ░ ░░   ║
    ║      ░          ░   ▒     ░░   ░ ░ ░   ░    ░    ║
    ║      ░ ░            ░  ░   ░           ░    ░    ║
    ║      ░                                           ║
    ║                                                   ║
    ║      🔨 FORGE - Powerful Rust Project Generator   ║
    ║                                                   ║
    ╚═══════════════════════════════════════════════════╝
"#;
    println!("{}", logo.bright_cyan());
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::New {
            name,
            project_type,
            author,
            description,
            license,
            non_interactive,
            from_config,
            dry_run,
        }) => {
            // Display logo unless in non-interactive mode
            if !non_interactive {
                display_logo();
            }

            let forge = Forge::new(".");

            if dry_run {
                println!(
                    "{}",
                    "🔍 DRY RUN MODE - No files will be created"
                        .bright_yellow()
                        .bold()
                );
                forge.run_dry_run(
                    name,
                    project_type,
                    author,
                    description,
                    license,
                    non_interactive,
                    from_config,
                )?;
            } else if non_interactive {
                forge.run_non_interactive(
                    name,
                    project_type,
                    author,
                    description,
                    license,
                    from_config,
                )?;
            } else if let Some(config_path) = from_config {
                forge.run_from_config(
                    config_path,
                    name,
                    project_type,
                    author,
                    description,
                    license,
                )?;
            } else if name.is_some() && project_type.is_some() {
                forge.run_with_args(name, project_type, author, description, license)?;
            } else {
                // Main interactive mode. CLI flags pre-answer their prompts
                // so `--author/--description/--license` are honored here too
                // instead of being discarded.
                forge
                    .with_cli_overrides(author, description, license)
                    .run()?;
            }
        }
        Some(Commands::Init {
            project_type,
            author,
            license,
            non_interactive,
            from_config,
            dry_run,
        }) => {
            if !non_interactive {
                display_logo();
            }

            let forge = Forge::new(".");

            if dry_run {
                println!(
                    "{}",
                    "🔍 DRY RUN MODE - No files will be created"
                        .bright_yellow()
                        .bold()
                );
                forge.run_init_dry_run(
                    project_type,
                    author,
                    license,
                    non_interactive,
                    from_config,
                )?;
            } else if non_interactive {
                forge.run_init_non_interactive(project_type, author, license, from_config)?;
            } else if let Some(config_path) = from_config {
                forge.run_init_from_config(config_path, project_type, author, license)?;
            } else {
                forge.run_init(project_type, author, license)?;
            }
        }
        Some(Commands::Completions { shell }) => {
            let mut cmd = Cli::command();
            print_completions(shell, &mut cmd);
        }
        None => {
            display_logo();
            println!("{}", "🔨 Welcome to Cargo-Forge!".bright_cyan().bold());
            println!(
                "{}",
                "Starting interactive project creation...\n".bright_white()
            );

            let forge = Forge::new(".");
            forge.run()?;
        }
    }

    Ok(())
}
