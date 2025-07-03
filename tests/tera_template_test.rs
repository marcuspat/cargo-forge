use tera::{Tera, Context};
use std::collections::HashMap;
use serde_json::json;

#[test]
fn test_tera_basic_rendering() {
    let mut tera = Tera::default();
    
    // Add a simple template
    tera.add_raw_template("test", "Hello, {{ name }}!").unwrap();
    
    let mut context = Context::new();
    context.insert("name", "World");
    
    let result = tera.render("test", &context).unwrap();
    assert_eq!(result, "Hello, World!");
}

#[test]
fn test_tera_conditional_rendering() {
    let mut tera = Tera::default();
    
    // Template with conditionals
    let template = r#"
{%- if has_database %}
use sqlx::PgPool;
{%- endif %}
{%- if has_auth %}
use jsonwebtoken::decode;
{%- endif %}

fn main() {
    {%- if has_database %}
    let pool = PgPool::connect("postgres://...").await?;
    {%- endif %}
}
"#;
    
    tera.add_raw_template("conditional", template).unwrap();
    
    // Test with database enabled
    let mut context = Context::new();
    context.insert("has_database", &true);
    context.insert("has_auth", &false);
    
    let result = tera.render("conditional", &context).unwrap();
    assert!(result.contains("use sqlx::PgPool;"));
    assert!(!result.contains("use jsonwebtoken::decode;"));
    assert!(result.contains("let pool = PgPool::connect"));
}

#[test]
fn test_tera_loops_and_arrays() {
    let mut tera = Tera::default();
    
    let template = r#"
[dependencies]
{%- for dep in dependencies %}
{{ dep.name }} = "{{ dep.version }}"
{%- endfor %}
"#;
    
    tera.add_raw_template("deps", template).unwrap();
    
    let mut context = Context::new();
    context.insert("dependencies", &vec![
        json!({"name": "tokio", "version": "1.0"}),
        json!({"name": "serde", "version": "1.0"}),
        json!({"name": "axum", "version": "0.7"}),
    ]);
    
    let result = tera.render("deps", &context).unwrap();
    assert!(result.contains("tokio = \"1.0\""));
    assert!(result.contains("serde = \"1.0\""));
    assert!(result.contains("axum = \"0.7\""));
}

#[test]
fn test_tera_project_type_specific_rendering() {
    let mut tera = Tera::default();
    
    let template = r#"
# {{ project_name }}

{% if project_type == "api-server" -%}
An API server built with Axum.

## Running the server
```bash
cargo run
```

The server will start on http://localhost:3000
{%- elif project_type == "cli-tool" -%}
A command-line tool built with Clap.

## Usage
```bash
cargo run -- --help
```
{%- elif project_type == "game-engine" -%}
A game built with Bevy.

## Running the game
```bash
cargo run
```

## Building for WASM
```bash
cargo build --target wasm32-unknown-unknown
```
{%- endif %}
"#;
    
    tera.add_raw_template("readme", template).unwrap();
    
    // Test API server README
    let mut context = Context::new();
    context.insert("project_name", "My API");
    context.insert("project_type", "api-server");
    
    let result = tera.render("readme", &context).unwrap();
    assert!(result.contains("# My API"));
    assert!(result.contains("An API server built with Axum"));
    assert!(result.contains("http://localhost:3000"));
    
    // Test game engine README
    context.insert("project_name", "My Game");
    context.insert("project_type", "game-engine");
    
    let result = tera.render("readme", &context).unwrap();
    assert!(result.contains("# My Game"));
    assert!(result.contains("A game built with Bevy"));
    assert!(result.contains("Building for WASM"));
}

#[test]
fn test_tera_nested_conditionals() {
    let mut tera = Tera::default();
    
    let template = r#"
{%- if project_type == "api-server" %}
use axum::Router;
{%- if features.database %}
use sqlx::PgPool;
{%- endif %}
{%- if features.auth %}
use tower_http::auth::RequireAuthorizationLayer;
{%- endif %}

async fn main() {
    let app = Router::new()
        {%- if features.auth %}
        .layer(RequireAuthorizationLayer::bearer("secret"))
        {%- endif %}
        ;
}
{%- endif %}
"#;
    
    tera.add_raw_template("nested", template).unwrap();
    
    let mut context = Context::new();
    context.insert("project_type", "api-server");
    
    let mut features = HashMap::new();
    features.insert("database", true);
    features.insert("auth", true);
    context.insert("features", &features);
    
    let result = tera.render("nested", &context).unwrap();
    assert!(result.contains("use axum::Router;"));
    assert!(result.contains("use sqlx::PgPool;"));
    assert!(result.contains("use tower_http::auth::RequireAuthorizationLayer;"));
    assert!(result.contains(".layer(RequireAuthorizationLayer::bearer"));
}

#[test]
fn test_tera_workspace_members_rendering() {
    let mut tera = Tera::default();
    
    let template = r#"
[workspace]
resolver = "2"
members = [
{%- for member in members %}
    "{{ member }}",
{%- endfor %}
]

[workspace.package]
version = "{{ version }}"
authors = ["{{ author }}"]
"#;
    
    tera.add_raw_template("workspace", template).unwrap();
    
    let mut context = Context::new();
    context.insert("members", &vec!["crates/core", "crates/api", "crates/cli"]);
    context.insert("version", "0.1.0");
    context.insert("author", "Test Author");
    
    let result = tera.render("workspace", &context).unwrap();
    assert!(result.contains("\"crates/core\","));
    assert!(result.contains("\"crates/api\","));
    assert!(result.contains("\"crates/cli\","));
    assert!(result.contains("version = \"0.1.0\""));
    assert!(result.contains("authors = [\"Test Author\"]"));
}

#[test]
fn test_tera_embedded_memory_config() {
    let mut tera = Tera::default();
    
    let template = r#"
MEMORY
{
  FLASH : ORIGIN = 0x{{ flash_origin }}, LENGTH = {{ flash_size }}K
  RAM : ORIGIN = 0x{{ ram_origin }}, LENGTH = {{ ram_size }}K
}
"#;
    
    tera.add_raw_template("memory", template).unwrap();
    
    let mut context = Context::new();
    context.insert("flash_origin", "08000000");
    context.insert("flash_size", &256);
    context.insert("ram_origin", "20000000");
    context.insert("ram_size", &64);
    
    let result = tera.render("memory", &context).unwrap();
    assert!(result.contains("FLASH : ORIGIN = 0x08000000, LENGTH = 256K"));
    assert!(result.contains("RAM : ORIGIN = 0x20000000, LENGTH = 64K"));
}

#[test]
fn test_tera_feature_combinations() {
    let mut tera = Tera::default();
    
    let template = r#"
[dependencies]
tokio = { version = "1", features = ["full"] }
{%- if "database" in features %}
sqlx = { version = "0.7", features = ["postgres"] }
{%- endif %}
{%- if "auth" in features %}
jsonwebtoken = "9"
bcrypt = "0.14"
{%- endif %}
{%- if "api" in features %}
axum = "0.7"
tower = "0.4"
{%- endif %}
{%- if "docker" in features %}

[dev-dependencies]
testcontainers = "0.15"
{%- endif %}
"#;
    
    tera.add_raw_template("features", template).unwrap();
    
    let mut context = Context::new();
    context.insert("features", &vec!["database", "api", "docker"]);
    
    let result = tera.render("features", &context).unwrap();
    assert!(result.contains("sqlx = { version = \"0.7\""));
    assert!(result.contains("axum = \"0.7\""));
    assert!(result.contains("testcontainers = \"0.15\""));
    assert!(!result.contains("jsonwebtoken"));
}

#[test]
fn test_tera_error_handling() {
    let mut tera = Tera::default();
    
    // Template with undefined variable
    let template = "Hello, {{ undefined_var }}!";
    tera.add_raw_template("error", template).unwrap();
    
    let context = Context::new();
    let result = tera.render("error", &context);
    
    // Should fail when trying to use undefined variable
    assert!(result.is_err());
}

#[test]
fn test_tera_filters() {
    let mut tera = Tera::default();
    
    let template = r#"
Project: {{ name | upper }}
Snake case: {{ name | replace(from="-", to="_") }}
Title case: {{ name | title }}
"#;
    
    tera.add_raw_template("filters", template).unwrap();
    
    let mut context = Context::new();
    context.insert("name", "my-awesome-project");
    
    let result = tera.render("filters", &context).unwrap();
    assert!(result.contains("Project: MY-AWESOME-PROJECT"));
    assert!(result.contains("Snake case: my_awesome_project"));
    assert!(result.contains("Title case: My-Awesome-Project"));
}