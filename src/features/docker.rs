use crate::{Plugin, ProjectContext};
use std::error::Error;

#[derive(Debug, Clone, Copy)]
pub enum DockerBuildStage {
    Simple,
    MultiStage,
    MultiStageWithCache,
}

pub struct DockerPlugin {
    build_stage: DockerBuildStage,
    with_compose: bool,
    expose_port: Option<u16>,
}

impl DockerPlugin {
    pub fn new() -> Self {
        Self {
            build_stage: DockerBuildStage::MultiStage,
            with_compose: false,
            expose_port: None,
        }
    }

    pub fn with_build_stage(mut self, stage: DockerBuildStage) -> Self {
        self.build_stage = stage;
        self
    }

    pub fn with_compose(mut self, enabled: bool) -> Self {
        self.with_compose = enabled;
        self
    }

    pub fn expose_port(mut self, port: u16) -> Self {
        self.expose_port = Some(port);
        self
    }

    fn generate_dockerfile(&self, project_name: &str) -> String {
        match self.build_stage {
            DockerBuildStage::Simple => self.generate_simple_dockerfile(project_name),
            DockerBuildStage::MultiStage => self.generate_multistage_dockerfile(project_name),
            DockerBuildStage::MultiStageWithCache => self.generate_cached_dockerfile(project_name),
        }
    }

    fn generate_simple_dockerfile(&self, project_name: &str) -> String {
        let mut dockerfile = format!(
            r#"FROM rust:1.75-slim

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

"#
        );

        if let Some(port) = self.expose_port {
            dockerfile.push_str(&format!("EXPOSE {}\n\n", port));
        }

        dockerfile.push_str(&format!(r#"CMD ["./target/release/{}"]"#, project_name));
        dockerfile
    }

    fn generate_multistage_dockerfile(&self, project_name: &str) -> String {
        let mut dockerfile = format!(
            r#"# Build stage
FROM rust:1.75 AS builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Build dependencies (this is cached as long as Cargo.toml/lock don't change)
RUN mkdir src && echo "fn main() {{}}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy source code
COPY src ./src

# Build application
RUN touch src/main.rs
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/{} /app/{}

"#,
            project_name, project_name
        );

        if let Some(port) = self.expose_port {
            dockerfile.push_str(&format!("EXPOSE {}\n\n", port));
        }

        dockerfile.push_str(&format!(r#"CMD ["./{}"]"#, project_name));
        dockerfile
    }

    fn generate_cached_dockerfile(&self, project_name: &str) -> String {
        let mut dockerfile = format!(
            r#"# syntax=docker/dockerfile:1.4

# Build stage with cargo-chef for dependency caching
FROM rust:1.75 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/{} /app/{}

"#,
            project_name, project_name
        );

        if let Some(port) = self.expose_port {
            dockerfile.push_str(&format!("EXPOSE {}\n\n", port));
        }

        dockerfile.push_str(&format!(r#"ENTRYPOINT ["./{}"]"#, project_name));
        dockerfile
    }

    fn generate_dockerignore(&self) -> String {
        r#"# Rust build artifacts
target/
Cargo.lock
**/*.rs.bk

# IDE and editor files
.idea/
.vscode/
*.swp
*.swo
*~

# OS files
.DS_Store
Thumbs.db

# Git
.git/
.gitignore

# Documentation
*.md
docs/

# Testing
tests/
benches/

# CI/CD
.github/
.gitlab-ci.yml
.travis.yml

# Environment files
.env
.env.*

# Docker files (avoid recursion)
Dockerfile*
docker-compose*
.dockerignore"#
            .to_string()
    }

    fn generate_docker_compose(&self, project_name: &str) -> String {
        let mut compose = format!(
            r#"version: '3.8'

services:
  app:
    build:
      context: .
      dockerfile: Dockerfile
    image: {}:latest
    container_name: {}"#,
            project_name, project_name
        );

        if let Some(port) = self.expose_port {
            compose.push_str(&format!(
                r#"
    ports:
      - "{}:{}""#,
                port, port
            ));
        }

        compose.push_str(
            r#"
    environment:
      - RUST_LOG=info
    restart: unless-stopped

  # Example database service (uncomment if needed)
  # postgres:
  #   image: postgres:15-alpine
  #   container_name: {}_db
  #   environment:
  #     POSTGRES_USER: myuser
  #     POSTGRES_PASSWORD: mypassword
  #     POSTGRES_DB: mydb
  #   volumes:
  #     - postgres_data:/var/lib/postgresql/data
  #   ports:
  #     - "5432:5432"

# volumes:
#   postgres_data:"#,
        );

        compose
    }
}

impl Default for DockerPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for DockerPlugin {
    fn name(&self) -> &str {
        "Docker"
    }

    fn configure(&self, context: &mut ProjectContext) -> Result<(), Box<dyn Error>> {
        let project_name = context.name.clone();

        context.add_template_file("Dockerfile", self.generate_dockerfile(&project_name));
        context.add_template_file(".dockerignore", self.generate_dockerignore());

        if self.with_compose {
            context.add_template_file(
                "docker-compose.yml",
                self.generate_docker_compose(&project_name),
            );
        }

        let build_script = format!(
            r#"#!/bin/bash
# Build Docker image
docker build -t {} .

# Run the container
docker run --rm {}"#,
            &project_name, &project_name
        );

        context.add_template_file("scripts/docker-build.sh", build_script);

        if self.with_compose {
            let compose_script = r#"#!/bin/bash
# Start services with docker-compose
docker-compose up -d

# View logs
docker-compose logs -f"#
                .to_string();

            context.add_template_file("scripts/docker-compose-start.sh", compose_script);
        }

        let readme_section = format!(
            r#"
## Docker Support

This project includes Docker support for easy deployment.

### Building the Docker image

```bash
docker build -t {} .
```

### Running the container

```bash
docker run --rm {}
```
"#,
            &project_name, &project_name
        );

        if self.with_compose {
            let compose_section = r#"
### Using Docker Compose

Start all services:
```bash
docker-compose up -d
```

View logs:
```bash
docker-compose logs -f
```

Stop all services:
```bash
docker-compose down
```"#;
            context.add_to_readme(&(readme_section + compose_section));
        } else {
            context.add_to_readme(&readme_section);
        }

        Ok(())
    }
}
