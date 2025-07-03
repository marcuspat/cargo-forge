use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Create completions directory
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let completions_dir = project_root.join("completions");
    
    if !completions_dir.exists() {
        fs::create_dir_all(&completions_dir).expect("Failed to create completions directory");
    }

    // Generate completion scripts instructions
    let bash_completion = r#"# Add this to your ~/.bashrc or ~/.bash_profile:
if command -v cargo-forge &> /dev/null; then
    eval "$(cargo-forge completions bash)"
fi
"#;

    let zsh_completion = r#"# Add this to your ~/.zshrc:
if command -v cargo-forge &> /dev/null; then
    eval "$(cargo-forge completions zsh)"
fi
"#;

    let fish_completion = r#"# Add this to your ~/.config/fish/config.fish:
if command -v cargo-forge > /dev/null
    cargo-forge completions fish | source
end
"#;

    // Write completion instructions
    fs::write(completions_dir.join("bash_setup.txt"), bash_completion)
        .expect("Failed to write bash completion instructions");
    fs::write(completions_dir.join("zsh_setup.txt"), zsh_completion)
        .expect("Failed to write zsh completion instructions");
    fs::write(completions_dir.join("fish_setup.txt"), fish_completion)
        .expect("Failed to write fish completion instructions");

    println!("cargo:rerun-if-changed=src/cli.rs");
    println!("cargo:rerun-if-changed=src/main.rs");
}