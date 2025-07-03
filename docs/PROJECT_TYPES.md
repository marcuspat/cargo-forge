# Cargo-Forge Project Types Guide

This guide provides detailed information about each project type supported by Cargo-Forge, including use cases, generated structure, and customization options.

## Table of Contents

- [CLI Tool](#cli-tool)
- [Library](#library)
- [API Server](#api-server)
- [WASM Application](#wasm-application)
- [Game Engine](#game-engine)
- [Embedded System](#embedded-system)
- [Workspace](#workspace)

---

## CLI Tool

Command-line applications with argument parsing, colored output, and progress indicators.

### Use Cases
- System utilities
- Development tools
- Data processing scripts
- Build automation tools

### Generated Structure
```
my-cli/
├── src/
│   ├── main.rs           # Entry point with clap setup
│   ├── commands/         # Subcommand implementations
│   │   ├── mod.rs
│   │   └── init.rs
│   └── utils/            # Utility functions
│       ├── mod.rs
│       └── config.rs
├── tests/
│   └── integration.rs    # CLI integration tests
├── Cargo.toml            # Dependencies: clap, colored, indicatif
├── README.md             # Usage documentation
└── .github/
    └── workflows/
        └── release.yml   # Binary release automation
```

### Key Features
- **Clap** for argument parsing with derive macros
- **Colored** output for better UX
- **Indicatif** for progress bars and spinners
- **Env_logger** for debug output
- Shell completion generation
- Man page generation support

### Example Generated Code
```rust
use clap::{Parser, Subcommand};
use colored::*;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "my-cli")]
#[command(about = "A CLI tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new project
    Init {
        #[arg(short, long)]
        name: String,
    },
    /// Run the application
    Run {
        #[arg(short, long, default_value = "default.conf")]
        config: String,
    },
}
```

### Customization Options
- Subcommands structure
- Configuration file support
- Interactive mode with dialoguer
- Shell completion scripts
- Installation scripts

---

## Library

Reusable Rust library crates with comprehensive documentation and examples.

### Use Cases
- Shared functionality
- Algorithm implementations
- Data structures
- API clients
- Protocol implementations

### Generated Structure
```
my-lib/
├── src/
│   ├── lib.rs            # Library root
│   ├── error.rs          # Error types
│   └── prelude.rs        # Common exports
├── examples/
│   └── basic.rs          # Usage examples
├── benches/
│   └── benchmarks.rs     # Performance benchmarks
├── tests/
│   ├── unit/
│   └── integration.rs
├── Cargo.toml
├── README.md             # API documentation
└── CHANGELOG.md          # Version history
```

### Key Features
- Documentation with examples
- Benchmark setup with Criterion
- Property testing with proptest
- Feature flags for optional functionality
- Workspace-friendly structure

### Example Generated Code
```rust
//! # My Library
//! 
//! This crate provides [...]
//! 
//! ## Examples
//! 
//! ```
//! use my_lib::prelude::*;
//! 
//! let result = my_function()?;
//! ```

#![doc(html_logo_url = "https://example.com/logo.png")]
#![warn(missing_docs)]

pub mod error;
pub mod prelude;

/// The main functionality of this library
pub fn my_function() -> Result<String, error::Error> {
    Ok("Hello from my library!".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        assert_eq!(my_function().unwrap(), "Hello from my library!");
    }
}
```

### Customization Options
- no_std support
- Async runtime selection
- Serialization support (serde)
- FFI bindings
- WASM compatibility

---

## API Server

RESTful API servers using Axum with authentication, database integration, and OpenAPI documentation.

### Use Cases
- Web services
- Microservices
- Backend APIs
- GraphQL servers
- WebSocket servers

### Generated Structure
```
my-api/
├── src/
│   ├── main.rs           # Server entry point
│   ├── routes/           # HTTP route handlers
│   │   ├── mod.rs
│   │   ├── health.rs
│   │   ├── users.rs
│   │   └── auth.rs
│   ├── models/           # Data models
│   │   ├── mod.rs
│   │   └── user.rs
│   ├── middleware/       # Custom middleware
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   └── cors.rs
│   ├── db/               # Database layer
│   │   ├── mod.rs
│   │   └── pool.rs
│   └── utils/
│       ├── mod.rs
│       └── jwt.rs
├── migrations/           # SQL migrations
│   └── 001_users.sql
├── tests/
│   └── api_tests.rs
├── Dockerfile            # Multi-stage build
├── docker-compose.yml    # Local development
├── .env.example          # Environment template
└── openapi.yml          # API specification
```

### Key Features
- **Axum** web framework
- **SQLx** for compile-time checked queries
- **JWT** authentication
- **CORS** configuration
- Request tracing with **tracing**
- Rate limiting
- OpenAPI documentation

### Example Generated Code
```rust
use axum::{
    routing::{get, post},
    Router,
    middleware,
};
use tower_http::cors::CorsLayer;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    // Database connection
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to create pool");

    // Build application routes
    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/auth/login", post(login))
        .layer(middleware::from_fn(auth_middleware))
        .layer(CorsLayer::permissive())
        .with_state(pool);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

### Customization Options
- Database selection (PostgreSQL, MySQL, SQLite)
- Authentication methods (JWT, OAuth, API keys)
- API versioning strategy
- GraphQL integration
- WebSocket support
- gRPC service generation

---

## WASM Application

WebAssembly applications for running Rust in the browser.

### Use Cases
- Web applications
- Browser extensions
- Game development
- Computational libraries
- Blockchain/crypto applications

### Generated Structure
```
my-wasm-app/
├── src/
│   ├── lib.rs            # WASM entry point
│   └── utils.rs          # WASM utilities
├── www/                  # Web assets
│   ├── index.html
│   ├── index.js
│   └── style.css
├── pkg/                  # WASM build output
├── tests/
│   └── web.rs            # WASM tests
├── Cargo.toml            # wasm-bindgen deps
├── package.json          # NPM dependencies
├── webpack.config.js     # Webpack configuration
└── README.md
```

### Key Features
- **wasm-bindgen** for JS interop
- **web-sys** for Web APIs
- **wee_alloc** for smaller binaries
- Webpack dev server setup
- NPM scripts for building
- Testing with wasm-bindgen-test

### Example Generated Code
```rust
use wasm_bindgen::prelude::*;
use web_sys::console;

// Called when the WASM module is instantiated
#[wasm_bindgen(start)]
pub fn main() {
    // Use `wee_alloc` as the global allocator
    #[cfg(feature = "wee_alloc")]
    #[global_allocator]
    static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    
    console::log_1(&"Hello from Rust and WebAssembly!".into());
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[wasm_bindgen]
pub struct App {
    value: i32,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { value: 0 }
    }
    
    pub fn increment(&mut self) {
        self.value += 1;
    }
    
    pub fn get_value(&self) -> i32 {
        self.value
    }
}
```

### Customization Options
- Framework integration (Yew, Seed, Percy)
- Canvas/WebGL support
- Audio API integration
- WebRTC support
- Service worker setup
- Progressive Web App configuration

---

## Game Engine

Game development projects using the Bevy engine.

### Use Cases
- 2D games
- 3D games
- Simulations
- Visualizations
- Interactive applications

### Generated Structure
```
my-game/
├── src/
│   ├── main.rs           # Game entry point
│   ├── systems/          # ECS systems
│   │   ├── mod.rs
│   │   ├── movement.rs
│   │   └── rendering.rs
│   ├── components/       # ECS components
│   │   ├── mod.rs
│   │   └── player.rs
│   ├── resources/        # Game resources
│   │   ├── mod.rs
│   │   └── game_state.rs
│   └── plugins/          # Bevy plugins
│       ├── mod.rs
│       └── menu.rs
├── assets/               # Game assets
│   ├── sprites/
│   ├── models/
│   ├── sounds/
│   └── fonts/
├── Cargo.toml            # Bevy dependencies
└── README.md
```

### Key Features
- **Bevy** ECS game engine
- Asset loading pipeline
- Input handling
- Physics integration (Rapier)
- Audio system
- UI framework

### Example Generated Code
```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(movement_system)
            .add_system(collision_system);
    }
}

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct Velocity(Vec3);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn camera
    commands.spawn(Camera2dBundle::default());
    
    // Spawn player
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            ..default()
        },
        Player { speed: 500.0 },
        Velocity(Vec3::ZERO),
    ));
}
```

### Customization Options
- 2D vs 3D setup
- Physics engine selection
- Networking support
- Platform-specific features
- Debug tools integration
- Performance profiling setup

---

## Embedded System

Embedded Rust projects for microcontrollers and IoT devices.

### Use Cases
- IoT devices
- Robotics
- Sensor systems
- Real-time control
- Firmware development

### Generated Structure
```
my-firmware/
├── src/
│   ├── main.rs           # no_std entry point
│   └── lib.rs            # Hardware abstraction
├── memory.x              # Memory layout
├── .cargo/
│   └── config.toml       # Build configuration
├── Embed.toml            # Debug configuration
├── build.rs              # Build script
├── Cargo.toml            # no_std dependencies
└── README.md
```

### Key Features
- **no_std** environment
- Hardware abstraction layer (HAL)
- Real-time interrupt handling
- Memory-safe peripheral access
- Panic handler implementation
- Debug probe configuration

### Example Generated Code
```rust
#![no_std]
#![no_main]

use panic_halt as _; // Panic handler
use cortex_m_rt::entry; // Runtime entry point
use stm32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    // Take ownership of peripherals
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    
    // Set up the system clock
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();
    
    // Configure GPIO
    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();
    
    // Set up SysTick delay
    let mut delay = cp.SYST.delay(&clocks);
    
    // Main loop
    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}
```

### Customization Options
- Target microcontroller selection
- RTOS integration (RTIC, Embassy)
- Bootloader support
- OTA update capability
- Debug output configuration
- Power management features

---

## Workspace

Multi-crate workspace projects for larger applications.

### Use Cases
- Monorepos
- Microservice collections
- Large applications
- Library collections
- Mixed project types

### Generated Structure
```
my-workspace/
├── crates/
│   ├── core/             # Shared core functionality
│   │   ├── src/
│   │   └── Cargo.toml
│   ├── api/              # API server crate
│   │   ├── src/
│   │   └── Cargo.toml
│   ├── cli/              # CLI tool crate
│   │   ├── src/
│   │   └── Cargo.toml
│   └── common/           # Common utilities
│       ├── src/
│       └── Cargo.toml
├── Cargo.toml            # Workspace manifest
├── Cargo.lock            # Shared lock file
├── README.md             # Workspace documentation
└── .github/
    └── workflows/
        └── ci.yml        # Unified CI pipeline
```

### Key Features
- Shared dependency management
- Unified versioning
- Cross-crate testing
- Optimized builds
- Shared tooling configuration
- Monorepo best practices

### Example Workspace Cargo.toml
```toml
[workspace]
members = [
    "crates/core",
    "crates/api",
    "crates/cli",
    "crates/common",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
authors = ["Your Name"]
edition = "2021"
license = "MIT OR Apache-2.0"

[workspace.dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
anyhow = "1"
tracing = "0.1"

# Optimize release builds
[profile.release]
lto = true
codegen-units = 1
```

### Customization Options
- Crate selection and organization
- Shared vs independent versioning
- Build optimization profiles
- Cross-crate feature flags
- Unified documentation generation
- Deployment strategies

---

## Feature Combinations

Different project types can be combined with various features:

### Authentication Options
- **JWT**: JSON Web Tokens with refresh tokens
- **OAuth**: GitHub, Google, Microsoft providers
- **API Keys**: Simple key-based authentication
- **Session**: Cookie-based sessions

### Database Options
- **PostgreSQL**: With SQLx and migrations
- **MySQL**: Full MySQL/MariaDB support
- **SQLite**: Embedded database
- **MongoDB**: Document database (planned)

### CI/CD Options
- **GitHub Actions**: Complete workflow with caching
- **GitLab CI**: Pipeline with stages
- **Jenkins**: Jenkinsfile generation
- **Custom**: Template for other systems

### Testing Options
- **Unit Tests**: Standard Rust testing
- **Integration Tests**: Full application tests
- **Property Tests**: QuickCheck/proptest
- **Benchmarks**: Criterion benchmarks
- **E2E Tests**: Playwright/Selenium

### Deployment Options
- **Docker**: Multi-stage Dockerfile
- **Kubernetes**: Helm charts
- **Serverless**: Lambda/Cloud Functions
- **SystemD**: Service files

## Choosing the Right Project Type

### Decision Matrix

| Need | Recommended Type |
|------|-----------------|
| Building a tool for developers | CLI Tool |
| Creating reusable functionality | Library |
| Building a web service | API Server |
| Browser-based application | WASM App |
| Making a game | Game Engine |
| Programming hardware | Embedded |
| Large, multi-component project | Workspace |

### Performance Considerations

- **CLI Tool**: Fast startup, minimal dependencies
- **Library**: Zero-cost abstractions, optional features
- **API Server**: Async runtime, connection pooling
- **WASM App**: Small binary size, efficient memory use
- **Game Engine**: Frame timing, asset streaming
- **Embedded**: Deterministic timing, minimal RAM
- **Workspace**: Incremental compilation, shared caches

## Migration Guide

### From Existing Project

To add Cargo-Forge features to an existing project:

```bash
# Initialize in current directory
cargo-forge init --project-type <type>

# This will:
# - Detect existing files
# - Suggest compatible features
# - Merge configurations
# - Preserve your code
```

### Between Project Types

Some project types can be converted:

- **Library → Workspace**: When growing beyond single crate
- **CLI Tool → API Server**: Adding web interface
- **API Server → Workspace**: Microservice extraction
- **WASM App → Library**: Making functionality reusable

## Best Practices

1. **Start Simple**: Use minimal features initially
2. **Iterate**: Add features as needed
3. **Test Early**: Enable testing features from start
4. **Document**: Keep README and docs updated
5. **Version Control**: Commit generated files
6. **CI/CD**: Set up automation immediately
7. **Security**: Use environment variables for secrets

## Further Resources

- [Cargo-Forge Repository](https://github.com/marcuspat/cargo-forge)
- [Documentation](https://docs.rs/cargo-forge)
