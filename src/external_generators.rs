use anyhow::{Context, Result};
use std::process::Command;

pub fn generate_esp32_project(
    project_name: &str,
    chip: &str,
    output_dir: &std::path::Path,
) -> Result<()> {
    println!("🔍 Debug: Checking esp-generate installation...");

    // First, check if esp-generate exists
    let help_output = Command::new("esp-generate").arg("--help").output();

    match help_output {
        Ok(_) => {
            println!("✅ esp-generate found");
        }
        Err(e) => {
            println!("❌ esp-generate not found: {}", e);
            println!("📦 Installing esp-generate...");

            let install_status = Command::new("cargo")
                .args(["install", "esp-generate", "--locked"])
                .status()
                .context("Failed to install esp-generate")?;

            if !install_status.success() {
                return Err(anyhow::anyhow!("Failed to install esp-generate"));
            }
            println!("✅ esp-generate installed successfully");
        }
    }

    println!("🚀 Running esp-generate TUI for:");
    println!("  Chip: {}", chip);
    println!("  Project name: {}", project_name);
    println!("  Output directory: {}", output_dir.display());
    println!("📋 esp-generate will now open its interactive interface...\n");

    // Use .status() instead of .output() to allow TUI interaction
    let status = Command::new("esp-generate")
        .args([
            "--chip",
            chip,
            "--output-path",
            output_dir.to_str().unwrap(),
            project_name,
        ])
        .status() // This allows the TUI to interact with the terminal
        .context("Failed to run esp-generate command")?;

    if !status.success() {
        return Err(anyhow::anyhow!(
            "esp-generate failed with exit code: {:?}",
            status.code()
        ));
    }

    println!("\n✅ ESP32 project generated successfully!");
    println!(
        "💡 Run 'cd {}/{} && cargo build' to build your project",
        output_dir.display(),
        project_name
    );

    Ok(())
}

pub fn esp32_chip_options() -> Vec<(&'static str, &'static str)> {
    vec![
        ("esp32", "ESP32 (Xtensa, dual-core)"),
        ("esp32s2", "ESP32-S2 (Xtensa, single-core, USB)"),
        ("esp32s3", "ESP32-S3 (Xtensa, dual-core, USB)"),
        ("esp32c3", "ESP32-C3 (RISC-V, single-core, WiFi/BLE)"),
        ("esp32c6", "ESP32-C6 (RISC-V, single-core, WiFi 6)"),
        ("esp32h2", "ESP32-H2 (RISC-V, single-core, Thread/Zigbee)"),
    ]
}

pub fn interactive_esp32_chip_selection() -> Result<String> {
    use inquire::Select;

    let chip_data = esp32_chip_options();
    let display_options: Vec<String> = chip_data
        .iter()
        .map(|(chip, description)| format!("{} - {}", chip.to_uppercase(), description))
        .collect();

    let selection = Select::new("Select ESP32 chip type:", display_options)
        .with_help_message("Choose the ESP32 variant for your project")
        .prompt()?;

    // Find the matching chip from the original data
    let selected_chip = chip_data
        .iter()
        .find(|(chip, description)| {
            format!("{} - {}", chip.to_uppercase(), description) == selection
        })
        .map(|(chip, _)| chip.to_string())
        .unwrap_or_else(|| "esp32c6".to_string());

    Ok(selected_chip)
}
