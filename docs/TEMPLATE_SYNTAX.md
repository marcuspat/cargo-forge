# Cargo Forge Template Syntax Guide

Cargo Forge uses the [Tera](https://tera.netlify.app/) template engine for generating project files. This guide covers the syntax, custom filters, and best practices for creating and modifying templates.

## Table of Contents

- [Basic Syntax](#basic-syntax)
- [Variables](#variables)
- [Control Structures](#control-structures)
- [Filters](#filters)
- [Custom Filters](#custom-filters)
- [Template Organization](#template-organization)
- [Best Practices](#best-practices)
- [Common Patterns](#common-patterns)
- [Debugging Templates](#debugging-templates)

## Basic Syntax

### Variable Interpolation

Variables are inserted using double curly braces:

```tera
# {{ project_name }}

Created by {{ author }} on {{ date }}
```

### Comments

Comments are not rendered in the output:

```tera
{# This is a comment and won't appear in the generated file #}
```

### Raw Blocks

To output literal `{{` or `{%` sequences:

```tera
{% raw %}
This {{ will }} be rendered as-is
{% endraw %}
```

## Variables

### Standard Variables

All templates have access to these variables:

| Variable | Type | Description | Example |
|----------|------|-------------|---------|
| `name` | String | Project name | `"my-awesome-app"` |
| `author` | String | Project author | `"Jane Doe"` |
| `description` | String | Project description | `"A cool Rust app"` |
| `license` | String | License identifier | `"MIT OR Apache-2.0"` |
| `rust_version` | String | Minimum Rust version | `"1.70.0"` |
| `version` | String | Initial version | `"0.1.0"` |
| `date` | String | Creation date | `"2024-01-15"` |
| `year` | String | Current year | `"2024"` |

### Project Type Variables

Additional variables based on project type:

#### API Server
```tera
host: "localhost"
port: 3000
database: true/false
auth: true/false
```

#### CLI Tool
```tera
bin_name: "my-cli"
has_subcommands: true/false
```

#### Library
```tera
include_benchmarks: true/false
include_examples: true/false
```

#### WASM App
```tera
wasm_pack_target: "web"
include_webpack: true/false
```

#### Game Engine
```tera
game_title: "My Game"
target_fps: 60
```

#### Embedded
```tera
target_chip: "stm32f4"
memory_layout: "memory.x"
```

#### Workspace
```tera
workspace_members: ["crate1", "crate2"]
shared_dependencies: true/false
```

## Control Structures

### Conditionals

```tera
{% if database %}
use sqlx::PgPool;
{% endif %}

{% if auth and database %}
// Auth with database backend
{% elif auth %}
// Auth with in-memory store
{% else %}
// No authentication
{% endif %}
```

### Loops

```tera
{% for dep in dependencies %}
{{ dep.name }} = "{{ dep.version }}"
{% endfor %}

{% for member in workspace_members %}
members = ["{{ member }}"]
{% endfor %}
```

### Matching

```tera
{% match database_type %}
  {% when "postgresql" %}
    use sqlx::postgres::PgPool;
  {% when "mysql" %}
    use sqlx::mysql::MySqlPool;
  {% when "sqlite" %}
    use sqlx::sqlite::SqlitePool;
{% endmatch %}
```

## Filters

### Built-in Filters

```tera
{{ name | upper }}              # MY-PROJECT
{{ name | lower }}              # my-project
{{ name | capitalize }}         # My-project
{{ name | trim }}               # Remove whitespace
{{ name | truncate(10) }}       # Truncate to 10 chars
{{ name | default("unnamed") }} # Fallback value
{{ name | escape }}             # HTML escape
```

### Custom Filters

Cargo Forge provides these custom filters:

#### Case Conversion
```tera
{{ name | snake_case }}     # my_awesome_project
{{ name | pascal_case }}    # MyAwesomeProject
{{ name | camel_case }}     # myAwesomeProject
{{ name | kebab_case }}     # my-awesome-project
{{ name | shouty_case }}    # MY_AWESOME_PROJECT
```

#### Path Handling
```tera
{{ path | normalize_path }} # Convert to OS-specific separators
{{ path | unix_path }}      # Force Unix-style paths
{{ path | windows_path }}   # Force Windows-style paths
```

#### String Manipulation
```tera
{{ text | indent(4) }}      # Indent by 4 spaces
{{ text | quote }}          # Add quotes around text
{{ text | unquote }}        # Remove quotes
{{ name | sanitize }}       # Remove special characters
```

## Template Organization

### File Structure

```
templates/
├── <project_type>/
│   ├── Cargo.toml.tera       # Main manifest
│   ├── README.md.tera        # Project documentation
│   ├── .gitignore.tera       # Git ignore file
│   └── src/
│       ├── main.rs.tera      # Entry point
│       └── lib.rs.tera       # Library root
└── features/
    ├── auth/                 # Authentication templates
    ├── database/             # Database templates
    └── docker/               # Docker templates
```

### Including Templates

```tera
{# Include another template #}
{% include "common/header.tera" %}

{# Include with context #}
{% include "features/auth/jwt.tera" with context %}
```

### Template Inheritance

Base template (`base.tera`):
```tera
// Copyright {{ year }} {{ author }}
// License: {{ license }}

{% block imports %}{% endblock %}

{% block main %}{% endblock %}
```

Child template:
```tera
{% extends "base.tera" %}

{% block imports %}
use clap::Parser;
{% endblock %}

{% block main %}
fn main() {
    println!("Hello, world!");
}
{% endblock %}
```

## Best Practices

### 1. Use Meaningful Variable Names

```tera
{# Good #}
{{ project_name }}
{{ database_url }}

{# Avoid #}
{{ name }}
{{ url }}
```

### 2. Provide Sensible Defaults

```tera
const PORT: u16 = {{ port | default(value=3000) }};
const HOST: &str = "{{ host | default(value="localhost") }}";
```

### 3. Handle Optional Features Gracefully

```tera
{% if features.database is defined and features.database %}
mod database;
{% endif %}
```

### 4. Use Filters for Consistency

```tera
mod {{ module_name | snake_case }};
struct {{ struct_name | pascal_case }};
const {{ const_name | shouty_case }}: &str = "value";
```

### 5. Comment Complex Logic

```tera
{# Generate feature flags based on selected features #}
{% for feature in features %}
  {% if feature.requires_flag %}
#[cfg(feature = "{{ feature.name }}")]
  {% endif %}
{% endfor %}
```

## Common Patterns

### Optional Dependencies

```tera
[dependencies]
{% if database %}
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }
{% endif %}
{% if auth %}
jsonwebtoken = "9"
bcrypt = "0.15"
{% endif %}
```

### Feature-Gated Code

```tera
{% if feature_enabled("async") %}
async fn process() -> Result<()> {
    // Async implementation
}
{% else %}
fn process() -> Result<()> {
    // Sync implementation
}
{% endif %}
```

### Dynamic File Lists

```tera
{% for module in modules %}
mod {{ module.name }};
{% endfor %}

pub use self::{
{% for module in modules %}
    {{ module.name }}::{{ module.export }},
{% endfor %}
};
```

### Cross-Platform Paths

```tera
{# Always use forward slashes in templates #}
include_str!("../templates/{{ template_path }}")

{# The normalize_path filter will convert at generation time #}
const ASSET_PATH: &str = "{{ asset_path | normalize_path }}";
```

### Conditional Compilation

```tera
{% if target_os == "windows" %}
use windows_specific::Module;
{% elif target_os == "linux" %}
use linux_specific::Module;
{% else %}
use generic::Module;
{% endif %}
```

## Debugging Templates

### Error Messages

Template errors include line numbers and context:

```
Template error at line 15: Undefined variable 'unknown_var'
  13 | fn main() {
  14 |     let config = Config {
> 15 |         name: "{{ unknown_var }}",
  16 |         port: {{ port }},
  17 |     };
```

### Debug Output

Use the `debug` filter to inspect variables:

```tera
{# Debug output - remove before committing #}
{{ my_variable | debug }}
```

### Testing Templates

Create unit tests for complex templates:

```rust
#[test]
fn test_cargo_toml_template() {
    let mut context = Context::new();
    context.insert("name", "test-project");
    context.insert("version", "0.1.0");
    context.insert("database", &true);
    
    let template = include_str!("../templates/api_server/Cargo.toml.tera");
    let rendered = Tera::one_off(template, &context, false).unwrap();
    
    assert!(rendered.contains("name = \"test-project\""));
    assert!(rendered.contains("sqlx"));
}
```

## Advanced Features

### Macros

Define reusable template snippets:

```tera
{% macro dependency(name, version, features=None) %}
{{ name }} = {% if features %}{ version = "{{ version }}", features = {{ features }} }{% else %}"{{ version }}"{% endif %}
{% endmacro %}

[dependencies]
{{ self::dependency(name="tokio", version="1", features=["full"]) }}
{{ self::dependency(name="serde", version="1") }}
```

### Functions

Custom functions can be registered in the template engine:

```tera
{# Custom function example #}
{{ generate_uuid() }}
{{ format_date(date, "%Y-%m-%d") }}
```

### Whitespace Control

Control whitespace around tags:

```tera
{%- if condition -%}
    This has no extra whitespace
{%- endif -%}
```

## Template Examples

### Complete Cargo.toml Template

```tera
[package]
name = "{{ name }}"
version = "{{ version | default(value="0.1.0") }}"
edition = "2021"
authors = ["{{ author }}"]
description = "{{ description }}"
license = "{{ license | default(value="MIT OR Apache-2.0") }}"

[dependencies]
{% for dep in dependencies | sort(attribute="name") %}
{{ dep.name }} = "{{ dep.version }}"
{% endfor %}

{% if dev_dependencies %}
[dev-dependencies]
{% for dep in dev_dependencies %}
{{ dep.name }} = "{{ dep.version }}"
{% endfor %}
{% endif %}

{% if features %}
[features]
default = [{% for f in default_features %}"{{ f }}"{% if not loop.last %}, {% endif %}{% endfor %}]
{% for feature in features %}
{{ feature.name }} = [{% for dep in feature.deps %}"{{ dep }}"{% if not loop.last %}, {% endif %}{% endfor %}]
{% endfor %}
{% endif %}
```

### Conditional Feature Implementation

```tera
use std::error::Error;

{% if database %}
use sqlx::{PgPool, postgres::PgPoolOptions};
{% endif %}
{% if auth %}
use jsonwebtoken::{encode, decode, Header, Validation};
{% endif %}

pub struct App {
    {% if database %}
    db: PgPool,
    {% endif %}
    {% if auth %}
    jwt_secret: String,
    {% endif %}
    config: Config,
}

impl App {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        {% if database %}
        let db = PgPoolOptions::new()
            .max_connections(5)
            .connect("{{ database_url }}").await?;
        {% endif %}
        
        Ok(Self {
            {% if database %}
            db,
            {% endif %}
            {% if auth %}
            jwt_secret: std::env::var("JWT_SECRET")?,
            {% endif %}
            config: Config::load()?,
        })
    }
}
```

## Contributing Templates

When contributing new templates:

1. Follow the existing naming conventions
2. Include comprehensive variable documentation
3. Test with various input combinations
4. Ensure cross-platform compatibility
5. Add integration tests

See [CONTRIBUTING.md](../CONTRIBUTING.md) for more details on contributing to Cargo Forge.