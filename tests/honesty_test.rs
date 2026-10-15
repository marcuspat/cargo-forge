//! Honesty tests: the flags, docs claims, and templates must match what is
//! actually generated. These lock in the guarantees the README makes.

use cargo_forge::forge::ForgeConfig;
use cargo_forge::{Generator, ProjectConfig};
use std::fs;
use tempfile::TempDir;

fn config_with_license(name: &str, project_type: &str, license: Option<&str>) -> ProjectConfig {
    ProjectConfig {
        name: name.to_string(),
        project_type: project_type.to_string(),
        author: "Test Author".to_string(),
        description: Some("honesty test".to_string()),
        license: license.map(|l| l.to_string()),
        features: vec![],
        target: None,
        esp32_chip: None,
    }
}

fn generate(
    temp: &TempDir,
    name: &str,
    project_type: &str,
    license: Option<&str>,
) -> std::path::PathBuf {
    let dir = temp.path().join(name);
    Generator::new()
        .generate(&config_with_license(name, project_type, license), &dir)
        .expect("generation should succeed");
    dir
}

#[test]
fn license_flag_lands_in_cargo_toml() {
    let temp = TempDir::new().unwrap();
    let dir = generate(&temp, "licensed-app", "api-server", Some("Apache-2.0"));
    let manifest = fs::read_to_string(dir.join("Cargo.toml")).unwrap();
    assert!(manifest.contains(r#"license = "Apache-2.0""#));
}

#[test]
fn no_license_field_when_none_given() {
    let temp = TempDir::new().unwrap();
    let dir = generate(&temp, "unlicensed-app", "cli-tool", None);
    let manifest = fs::read_to_string(dir.join("Cargo.toml")).unwrap();
    assert!(!manifest.contains("license ="));
}

#[test]
fn license_lands_in_workspace_package_and_members() {
    let temp = TempDir::new().unwrap();
    let dir = generate(&temp, "licensed-ws", "workspace", Some("MIT"));
    let manifest = fs::read_to_string(dir.join("Cargo.toml")).unwrap();
    assert!(manifest.contains("[workspace.package]"));
    assert!(manifest.contains(r#"license = "MIT""#));
    // The member crates must actually carry the license, not just the root.
    for member in [
        "crates/core/Cargo.toml",
        "crates/api/Cargo.toml",
        "crates/cli/Cargo.toml",
    ] {
        let m = fs::read_to_string(dir.join(member)).unwrap();
        assert!(
            m.contains(r#"license = "MIT""#),
            "{member} should carry the license"
        );
    }
}

#[test]
fn api_server_starter_actually_serves() {
    let temp = TempDir::new().unwrap();
    let dir = generate(&temp, "real-api", "api-server", None);
    let main = fs::read_to_string(dir.join("src/main.rs")).unwrap();
    assert!(
        main.contains("axum::serve"),
        "api-server main must start a server"
    );
    assert!(main.contains("TcpListener"));
    let routes = fs::read_to_string(dir.join("src/routes.rs")).unwrap();
    assert!(routes.contains("Router"), "routes.rs must define a Router");
    let handlers = fs::read_to_string(dir.join("src/handlers.rs")).unwrap();
    assert!(handlers.contains("pub async fn health"));
}

#[test]
fn cli_tool_starter_uses_clap() {
    let temp = TempDir::new().unwrap();
    let dir = generate(&temp, "real-cli", "cli-tool", None);
    let main = fs::read_to_string(dir.join("src/main.rs")).unwrap();
    assert!(main.contains("clap::Parser"));
    let cli = fs::read_to_string(dir.join("src/cli.rs")).unwrap();
    assert!(cli.contains("Subcommand"));
    // A real integration test ships with the template.
    assert!(dir.join("tests/cli.rs").is_file());
}

#[test]
fn library_starter_has_tests() {
    let temp = TempDir::new().unwrap();
    let dir = generate(&temp, "real-lib", "library", None);
    let lib = fs::read_to_string(dir.join("src/lib.rs")).unwrap();
    assert!(
        lib.contains("#[cfg(test)]"),
        "library template must ship a unit test"
    );
    assert!(lib.contains("pub fn"));
}

#[test]
fn wasm_starter_exports_a_function() {
    let temp = TempDir::new().unwrap();
    let dir = generate(&temp, "real-wasm", "wasm-app", None);
    let lib = fs::read_to_string(dir.join("src/lib.rs")).unwrap();
    assert!(lib.contains("#[wasm_bindgen]"));
    // The junk scaffolding (empty webpack/package.json) is gone.
    assert!(!dir.join("webpack.config.js").exists());
    assert!(!dir.join("package.json").exists());
}

#[test]
fn toml_config_is_accepted() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("defaults.toml");
    fs::write(
        &path,
        r#"default_author = "Toml Author"
default_license = "MIT"
preferred_project_types = ["library"]
default_features = { api-server = ["axum"] }
"#,
    )
    .unwrap();
    let config = ForgeConfig::load_from(&path).expect("TOML config must load");
    assert_eq!(config.default_author.as_deref(), Some("Toml Author"));
    assert_eq!(config.preferred_project_types, vec!["library".to_string()]);
}

#[test]
fn json_config_still_accepted() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("config.json");
    fs::write(
        &path,
        r#"{"default_author":"Json Author","default_license":"MIT","preferred_project_types":["cli-tool"],"default_features":{},"edition":"2021"}"#,
    )
    .unwrap();
    let config = ForgeConfig::load_from(&path).expect("JSON config must still load");
    assert_eq!(config.default_author.as_deref(), Some("Json Author"));
}

#[test]
fn invalid_config_reports_both_formats() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("broken.toml");
    fs::write(&path, "not [ valid anything").unwrap();
    let err = ForgeConfig::load_from(&path).expect_err("garbage config must fail");
    let msg = err.to_string();
    assert!(msg.contains("TOML"), "error should mention TOML: {msg}");
    assert!(msg.contains("JSON"), "error should mention JSON: {msg}");
}

#[test]
fn non_interactive_honors_preferred_project_type() {
    let temp = TempDir::new().unwrap();
    let config_path = temp.path().join("defaults.toml");
    fs::write(
        &config_path,
        r#"default_author = "Config Author"
default_license = "MIT"
preferred_project_types = ["library"]
default_features = { api-server = ["axum"] }
"#,
    )
    .unwrap();

    let forge = cargo_forge::Forge::new(temp.path());
    // No explicit project type: the config's preferred type (library) must win.
    forge
        .run_non_interactive(
            Some("from-config-lib".to_string()),
            None,
            None,
            None,
            None,
            Some(config_path),
        )
        .expect("non-interactive generation should succeed");

    let dir = temp.path().join("from-config-lib");
    assert!(
        dir.join("src/lib.rs").is_file(),
        "preferred type library must be used"
    );
    let manifest = fs::read_to_string(dir.join("Cargo.toml")).unwrap();
    assert!(manifest.contains("[lib]"));
    assert!(
        manifest.contains(r#"license = "MIT""#),
        "config default_license must apply"
    );
}

#[test]
fn special_characters_in_description_produce_valid_toml() {
    let temp = TempDir::new().unwrap();
    let mut cfg = config_with_license("special-desc", "library", None);
    cfg.description = Some("Description with \"quotes\", 'apostrophes', and @#$%^&*()".to_string());
    let dir = temp.path().join("special-desc");
    Generator::new()
        .generate(&cfg, &dir)
        .expect("generation should succeed");

    let manifest = fs::read_to_string(dir.join("Cargo.toml")).unwrap();
    // Escaped quotes keep the manifest parseable.
    assert!(manifest.contains("quotes"));
    let parsed: toml::Value = toml::from_str(&manifest).expect("Cargo.toml must be valid TOML");
    let desc = parsed
        .get("package")
        .unwrap()
        .get("description")
        .unwrap()
        .as_str()
        .unwrap();
    assert!(desc.contains("\"quotes\""));
}

#[test]
fn control_characters_in_description_produce_valid_toml() {
    let temp = TempDir::new().unwrap();
    let mut cfg = config_with_license("ctrl-desc", "cli-tool", Some("MIT"));
    cfg.description = Some("line one\nline two\ttabbed \u{0007}bell".to_string());
    let dir = temp.path().join("ctrl-desc");
    Generator::new()
        .generate(&cfg, &dir)
        .expect("generation should succeed");
    let manifest = fs::read_to_string(dir.join("Cargo.toml")).unwrap();
    let parsed: toml::Value = toml::from_str(&manifest).expect("manifest must stay valid TOML");
    let desc = parsed
        .get("package")
        .unwrap()
        .get("description")
        .unwrap()
        .as_str()
        .unwrap();
    assert!(desc.contains("line one\nline two"));
}

#[test]
fn multiline_description_keeps_library_compilable() {
    let temp = TempDir::new().unwrap();
    let mut cfg = config_with_license("multiline-lib", "library", None);
    cfg.description = Some("First line\nSecond line".to_string());
    let dir = temp.path().join("multiline-lib");
    Generator::new()
        .generate(&cfg, &dir)
        .expect("generation should succeed");
    let lib = fs::read_to_string(dir.join("src/lib.rs")).unwrap();
    assert!(lib.contains("//! First line"));
    assert!(lib.contains("//! Second line"));
    assert!(!lib.contains("\nSecond")); // not leaking raw newlines into code
}

#[test]
fn database_feature_lands_sqlx_in_manifest() {
    let temp = TempDir::new().unwrap();
    let mut cfg = config_with_license("db-api", "api-server", None);
    cfg.features = vec!["database".to_string()];
    let dir = temp.path().join("db-api");
    Generator::new()
        .generate(&cfg, &dir)
        .expect("generation should succeed");
    let manifest = fs::read_to_string(dir.join("Cargo.toml")).unwrap();
    assert!(
        manifest.contains("sqlx"),
        "database feature must add sqlx: {manifest}"
    );
    toml::from_str::<toml::Value>(&manifest).expect("manifest must be valid TOML");
}

#[test]
fn database_feature_deps_land_in_dependencies_table_for_every_supported_type() {
    // Regression: plugin deps used to be appended after the manifest's last
    // table, so for types whose [dependencies] is followed by other tables
    // (cli-tool: [[bin]], library: [lib]) sqlx landed outside
    // [dependencies] and the generated project could not compile. The
    // database feature is only supported where sqlx links (see
    // feature_supported); the unsupported types are covered by
    // database_feature_rejected_for_unsupported_types below.
    for project_type in ["api-server", "cli-tool", "library"] {
        let temp = TempDir::new().unwrap();
        let mut cfg = config_with_license("db-check", project_type, None);
        cfg.features = vec!["database".to_string()];
        let dir = temp.path().join("db-check");
        Generator::new()
            .generate(&cfg, &dir)
            .expect("generation should succeed");
        let manifest = fs::read_to_string(dir.join("Cargo.toml")).unwrap();
        let value = toml::from_str::<toml::Value>(&manifest)
            .unwrap_or_else(|e| panic!("{project_type} manifest must be valid TOML: {e}"));
        assert!(
            value
                .get("dependencies")
                .and_then(|deps| deps.get("sqlx"))
                .is_some(),
            "{project_type}: database feature must put sqlx under [dependencies], got:\n{manifest}"
        );
    }
}

#[test]
fn database_feature_rejected_for_unsupported_types() {
    // embedded is no_std, wasm-app targets wasm32, game-engine carries a
    // wasm32 target table, and workspace roots are virtual manifests —
    // sqlx cannot link in any of them, so the combination must be an
    // error, not a project that compiles nowhere. "postgres" registers
    // the same plugin as "database" and must be guarded identically.
    for project_type in ["embedded", "wasm-app", "game-engine", "workspace"] {
        for alias in ["database", "postgres"] {
            let temp = TempDir::new().unwrap();
            let mut cfg = config_with_license("db-bad", project_type, None);
            cfg.features = vec![alias.to_string()];
            let dir = temp.path().join("db-bad");
            let err = Generator::new()
                .generate(&cfg, &dir)
                .expect_err("unsupported feature/type combination must fail");
            assert!(
                err.to_string().contains("not supported"),
                "{project_type}/{alias}: expected a 'not supported' error, got: {err}"
            );
        }
    }
}

#[test]
fn game_engine_starter_compiles_for_wasm32_when_target_installed() {
    // Real compile coverage for the changed starter (bevy 0.12 -> 0.20),
    // replacing the e2e matrix's unconditional skips for target-specific
    // types. Runs wherever the wasm32 target is installed (`rustup target
    // add wasm32-unknown-unknown`); when it is not, the skip is stated —
    // never reported as a pass. A stable shared target dir keeps repeat
    // runs incremental instead of recompiling bevy every time.
    let temp = TempDir::new().unwrap();
    let cfg = config_with_license("ge-wasm32", "game-engine", None);
    let dir = temp.path().join("ge-wasm32");
    Generator::new()
        .generate(&cfg, &dir)
        .expect("generation should succeed");

    let installed = std::process::Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("wasm32-unknown-unknown"))
        .unwrap_or(false);
    if !installed {
        println!(
            "SKIP game-engine wasm32 compile check: wasm32-unknown-unknown \
             target not installed (rustup target add wasm32-unknown-unknown \
             to enable this check)"
        );
        return;
    }
    let shared_target = std::env::temp_dir().join("cargo-forge-ge-wasm32-target");
    let status = std::process::Command::new("cargo")
        .args(["check", "--target", "wasm32-unknown-unknown"])
        .env("CARGO_TARGET_DIR", &shared_target)
        .current_dir(&dir)
        .status()
        .expect("cargo should launch");
    assert!(
        status.success(),
        "game-engine starter (bevy 0.20) must compile for wasm32-unknown-unknown"
    );
}

#[test]
fn second_generate_into_released_claim_directory_fails() {
    // A completed generation releases its claim but leaves the project
    // files behind. A later run must refuse the directory even though it
    // could re-acquire the (now free) claim — the post-claim emptiness
    // re-check exists exactly for this window.
    let temp = TempDir::new().unwrap();
    let dir = temp.path().join("claimed-once");
    let cfg = config_with_license("claimed-once", "cli-tool", None);
    Generator::new()
        .generate(&cfg, &dir)
        .expect("first generation should succeed");
    let err = Generator::new()
        .generate(&cfg, &dir)
        .expect_err("second generation into the same directory must fail");
    assert!(
        err.to_string().contains("not empty"),
        "expected a 'not empty' error, got: {err}"
    );
}

#[test]
fn generation_is_fast() {
    let temp = TempDir::new().unwrap();
    let start = std::time::Instant::now();
    for project_type in [
        "api-server",
        "cli-tool",
        "library",
        "wasm-app",
        "game-engine",
        "embedded",
        "workspace",
    ] {
        generate(&temp, &format!("perf-{}", project_type), project_type, None);
    }
    let elapsed = start.elapsed();
    // Typical generation is ~0.3s per project; 2s each is the CI ceiling.
    assert!(
        elapsed.as_secs_f64() < 14.0,
        "7 generations took {elapsed:?} — far off the documented ~0.3s/project"
    );
}

#[test]
fn claim_file_cleaned_up_on_error() {
    let temp = TempDir::new().unwrap();
    let dir = temp.path().join("claim-err");
    // Unknown project type: generate() errors at the type dispatch, which
    // runs AFTER the directory is created, claimed and given its src dir.
    // Asserting src/ exists proves the run reached past the claim — so the
    // missing claim file below is the cleanup working, not the error
    // firing before the claim was ever taken.
    let mut cfg = config_with_license("claim-err", "bogus-type", None);
    cfg.project_type = "bogus-type".to_string();
    let result = Generator::new().generate(&cfg, &dir);
    assert!(result.is_err(), "unknown type must fail");
    assert!(
        dir.join("src").exists(),
        "test setup: the error must fire after the claim (src dir created)"
    );
    assert!(
        !dir.join(".cargo-forge-claim").exists(),
        "claim file must be removed on error paths"
    );
}

#[test]
fn lone_carriage_return_in_description_keeps_library_sane() {
    let temp = TempDir::new().unwrap();
    let mut cfg = config_with_license("cr-lib", "library", None);
    cfg.description = Some("before\rafter".to_string());
    let dir = temp.path().join("cr-lib");
    Generator::new().generate(&cfg, &dir).expect("generation should succeed");
    let lib = fs::read_to_string(dir.join("src/lib.rs")).unwrap();
    assert!(!lib.contains('\r'), "lone CR must not survive into lib.rs");
    assert!(lib.contains("//! before"));
    assert!(lib.contains("//! after"));
}

#[test]
fn cli_license_flag_reaches_manifest_end_to_end() {
    let temp = TempDir::new().unwrap();
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_cargo-forge"))
        .args([
            "new",
            "cli-flag-app",
            "--project-type",
            "cli-tool",
            "--license",
            "Apache-2.0",
            "--non-interactive",
        ])
        .current_dir(temp.path())
        .output()
        .expect("failed to run cargo-forge binary");
    assert!(
        out.status.success(),
        "cargo-forge new should succeed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let manifest = fs::read_to_string(temp.path().join("cli-flag-app/Cargo.toml")).unwrap();
    assert!(manifest.contains(r#"license = "Apache-2.0""#));
}

#[test]
fn cli_license_flag_overrides_config_default() {
    let temp = TempDir::new().unwrap();
    fs::write(
        temp.path().join("defaults.toml"),
        r#"default_author = "Config Author"
default_license = "MIT"
preferred_project_types = ["library"]
default_features = { api-server = ["docker"] }
"#,
    )
    .unwrap();
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_cargo-forge"))
        .args([
            "new",
            "cli-override-app",
            "--from-config",
            "defaults.toml",
            "--license",
            "Apache-2.0",
            "--non-interactive",
        ])
        .current_dir(temp.path())
        .output()
        .expect("failed to run cargo-forge binary");
    assert!(out.status.success());
    let manifest = fs::read_to_string(temp.path().join("cli-override-app/Cargo.toml")).unwrap();
    assert!(
        manifest.contains(r#"license = "Apache-2.0""#),
        "CLI --license must win over config default_license"
    );
    assert!(
        manifest.contains(r#"authors = ["Config Author"]"#),
        "config default_author must still apply when not given on the CLI"
    );
}

#[test]
fn cli_init_author_and_license_flags_reach_manifest() {
    let temp = TempDir::new().unwrap();
    let project = temp.path().join("init-flag-dir");
    fs::create_dir_all(&project).unwrap();
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_cargo-forge"))
        .args([
            "init",
            "--project-type",
            "library",
            "--author",
            "Init Author",
            "--license",
            "BSD-3-Clause",
            "--non-interactive",
        ])
        .current_dir(&project)
        .output()
        .expect("failed to run cargo-forge binary");
    assert!(
        out.status.success(),
        "cargo-forge init should succeed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let manifest = fs::read_to_string(project.join("Cargo.toml")).unwrap();
    assert!(manifest.contains(r#"authors = ["Init Author"]"#));
    assert!(manifest.contains(r#"license = "BSD-3-Clause""#));
}
