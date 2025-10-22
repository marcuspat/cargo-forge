use cargo_forge::{features::*, ProjectContext};

#[test]
fn test_plugin_manager_new() {
    let manager = PluginManager::new();
    assert_eq!(manager.len(), 0);
}

#[test]
fn test_plugin_manager_register() {
    let mut manager = PluginManager::new();
    let database_plugin = Box::new(database::DatabasePlugin::new(
        database::DatabaseType::PostgreSQL,
    ));
    manager.register(database_plugin);
    assert_eq!(manager.len(), 1);
}

#[test]
fn test_database_plugin_postgresql() {
    let plugin = database::DatabasePlugin::new(database::DatabaseType::PostgreSQL);
    assert_eq!(plugin.name(), "Database");

    let mut context = ProjectContext::new("test_project");
    plugin.configure(&mut context).unwrap();

    assert!(context.dependencies.contains_key("sqlx"));
    assert!(context.dependencies.contains_key("tokio"));
    assert!(context.dependencies.contains_key("dotenv"));
    assert!(context.template_files.contains_key(".env.example"));
    assert!(context.template_files.contains_key("src/database.rs"));
}

#[test]
fn test_database_plugin_sqlite() {
    let plugin = database::DatabasePlugin::new(database::DatabaseType::SQLite);
    let mut context = ProjectContext::new("test_project");
    plugin.configure(&mut context).unwrap();

    let sqlx_dep = context.dependencies.get("sqlx").unwrap();
    assert!(sqlx_dep.contains("sqlite"));
    assert!(!sqlx_dep.contains("postgres"));
    assert!(!sqlx_dep.contains("mysql"));
}

#[test]
fn test_database_plugin_mysql() {
    let plugin = database::DatabasePlugin::new(database::DatabaseType::MySQL);
    let mut context = ProjectContext::new("test_project");
    plugin.configure(&mut context).unwrap();

    let sqlx_dep = context.dependencies.get("sqlx").unwrap();
    assert!(sqlx_dep.contains("mysql"));
    assert!(!sqlx_dep.contains("postgres"));
    assert!(!sqlx_dep.contains("sqlite"));
}

#[test]
fn test_database_plugin_with_migrations() {
    let plugin =
        database::DatabasePlugin::new(database::DatabaseType::PostgreSQL).with_migrations(true);
    let mut context = ProjectContext::new("test_project");
    plugin.configure(&mut context).unwrap();

    assert!(context.directories.contains(&"migrations".to_string()));
    assert!(context
        .template_files
        .contains_key("migrations/001_create_users_table.sql"));
    assert!(context.template_files.contains_key("migrations/.gitkeep"));
}

#[test]
fn test_database_plugin_without_migrations() {
    let plugin =
        database::DatabasePlugin::new(database::DatabaseType::PostgreSQL).with_migrations(false);
    let mut context = ProjectContext::new("test_project");
    plugin.configure(&mut context).unwrap();

    assert!(!context.directories.contains(&"migrations".to_string()));
}

#[test]
fn test_docker_plugin_simple() {
    let plugin = docker::DockerPlugin::new().with_build_stage(docker::DockerBuildStage::Simple);
    let mut context = ProjectContext::new("test_project");
    plugin.configure(&mut context).unwrap();

    assert!(context.template_files.contains_key("Dockerfile"));
    assert!(context.template_files.contains_key(".dockerignore"));
    assert!(context
        .template_files
        .contains_key("scripts/docker-build.sh"));

    let dockerfile = context.template_files.get("Dockerfile").unwrap();
    assert!(dockerfile.contains("FROM rust:1.75-slim"));
    assert!(!dockerfile.contains("FROM rust:1.75 AS builder"));
}

#[test]
fn test_docker_plugin_multistage() {
    let plugin = docker::DockerPlugin::new().with_build_stage(docker::DockerBuildStage::MultiStage);
    let mut context = ProjectContext::new("test_project");
    plugin.configure(&mut context).unwrap();

    let dockerfile = context.template_files.get("Dockerfile").unwrap();
    assert!(dockerfile.contains("FROM rust:1.75 AS builder"));
    assert!(dockerfile.contains("FROM debian:bookworm-slim"));
}

#[test]
fn test_docker_plugin_with_cache() {
    let plugin =
        docker::DockerPlugin::new().with_build_stage(docker::DockerBuildStage::MultiStageWithCache);
    let mut context = ProjectContext::new("test_project");
    plugin.configure(&mut context).unwrap();

    let dockerfile = context.template_files.get("Dockerfile").unwrap();
    assert!(dockerfile.contains("cargo-chef"));
    assert!(dockerfile.contains("recipe.json"));
}

#[test]
fn test_docker_plugin_with_compose() {
    let plugin = docker::DockerPlugin::new()
        .with_compose(true)
        .expose_port(3000);
    let mut context = ProjectContext::new("test_project");
    plugin.configure(&mut context).unwrap();

    assert!(context.template_files.contains_key("docker-compose.yml"));
    assert!(context
        .template_files
        .contains_key("scripts/docker-compose-start.sh"));

    let compose = context.template_files.get("docker-compose.yml").unwrap();
    assert!(compose.contains("version: '3.8'"));
    assert!(compose.contains("3000:3000"));
}

#[test]
fn test_ci_plugin_github_actions() {
    let plugin = ci::CIPlugin::new(ci::CIPlatform::GitHubActions);
    let mut context = ProjectContext::new("test_project");
    plugin.configure(&mut context).unwrap();

    assert!(context
        .directories
        .contains(&".github/workflows".to_string()));
    assert!(context
        .template_files
        .contains_key(".github/workflows/ci.yml"));
    assert!(!context.template_files.contains_key(".gitlab-ci.yml"));

    let workflow = context
        .template_files
        .get(".github/workflows/ci.yml")
        .unwrap();
    assert!(workflow.contains("name: CI"));
    assert!(workflow.contains("uses: actions/checkout"));
}

#[test]
fn test_ci_plugin_gitlab() {
    let plugin = ci::CIPlugin::new(ci::CIPlatform::GitLabCI);
    let mut context = ProjectContext::new("test_project");
    plugin.configure(&mut context).unwrap();

    assert!(context.template_files.contains_key(".gitlab-ci.yml"));
    assert!(!context
        .template_files
        .contains_key(".github/workflows/ci.yml"));

    let ci_config = context.template_files.get(".gitlab-ci.yml").unwrap();
    assert!(ci_config.contains("stages:"));
    assert!(ci_config.contains("test:cargo:"));
}

#[test]
fn test_ci_plugin_both_platforms() {
    let plugin = ci::CIPlugin::new(ci::CIPlatform::Both);
    let mut context = ProjectContext::new("test_project");
    plugin.configure(&mut context).unwrap();

    assert!(context
        .template_files
        .contains_key(".github/workflows/ci.yml"));
    assert!(context.template_files.contains_key(".gitlab-ci.yml"));
}

#[test]
fn test_ci_plugin_with_features() {
    let plugin = ci::CIPlugin::new(ci::CIPlatform::GitHubActions)
        .with_coverage(true)
        .with_security_audit(true)
        .with_release(true);
    let mut context = ProjectContext::new("test_project");
    plugin.configure(&mut context).unwrap();

    let workflow = context
        .template_files
        .get(".github/workflows/ci.yml")
        .unwrap();
    assert!(workflow.contains("coverage:"));
    assert!(workflow.contains("security_audit:"));
    assert!(workflow.contains("release:"));
    assert!(workflow.contains("cargo-tarpaulin"));
    assert!(workflow.contains("audit-check"));
}

#[test]
fn test_ci_plugin_without_features() {
    let plugin = ci::CIPlugin::new(ci::CIPlatform::GitHubActions)
        .with_coverage(false)
        .with_security_audit(false)
        .with_release(false);
    let mut context = ProjectContext::new("test_project");
    plugin.configure(&mut context).unwrap();

    let workflow = context
        .template_files
        .get(".github/workflows/ci.yml")
        .unwrap();
    assert!(!workflow.contains("coverage:"));
    assert!(!workflow.contains("security_audit:"));
    assert!(!workflow.contains("release:"));
}

#[test]
fn test_multiple_plugins_integration() {
    let mut manager = PluginManager::new();

    manager.register(Box::new(database::DatabasePlugin::new(
        database::DatabaseType::PostgreSQL,
    )));
    manager.register(Box::new(
        docker::DockerPlugin::new()
            .with_compose(true)
            .expose_port(8080),
    ));
    manager.register(Box::new(ci::CIPlugin::new(ci::CIPlatform::Both)));

    let mut context = ProjectContext::new("test_integration");
    manager.configure_all(&mut context).unwrap();

    assert!(context.dependencies.contains_key("sqlx"));
    assert!(context.template_files.contains_key("Dockerfile"));
    assert!(context.template_files.contains_key("docker-compose.yml"));
    assert!(context
        .template_files
        .contains_key(".github/workflows/ci.yml"));
    assert!(context.template_files.contains_key(".gitlab-ci.yml"));
    assert!(context.template_files.contains_key("src/database.rs"));
}

#[test]
fn test_plugin_readme_integration() {
    let mut context = ProjectContext::new("test_readme");

    let db_plugin = database::DatabasePlugin::new(database::DatabaseType::SQLite);
    let docker_plugin = docker::DockerPlugin::new();
    let ci_plugin = ci::CIPlugin::new(ci::CIPlatform::GitHubActions);

    db_plugin.configure(&mut context).unwrap();
    docker_plugin.configure(&mut context).unwrap();
    ci_plugin.configure(&mut context).unwrap();

    let readme = &context.readme_sections.join("\n");
    assert!(readme.contains("Docker Support"));
    assert!(readme.contains("CI/CD"));
}

#[test]
fn test_plugin_gitignore_entries() {
    let mut context = ProjectContext::new("test_gitignore");

    let db_plugin = database::DatabasePlugin::new(database::DatabaseType::SQLite);
    db_plugin.configure(&mut context).unwrap();

    assert!(context.gitignore_entries.contains(&".env".to_string()));
    assert!(context.gitignore_entries.contains(&"*.db".to_string()));
    assert!(context.gitignore_entries.contains(&"*.db-shm".to_string()));
    assert!(context.gitignore_entries.contains(&"*.db-wal".to_string()));
}
