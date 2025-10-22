use crate::{Plugin, ProjectContext};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseType {
    PostgreSQL,
    SQLite,
    MySQL,
}

impl fmt::Display for DatabaseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseType::PostgreSQL => write!(f, "PostgreSQL"),
            DatabaseType::SQLite => write!(f, "SQLite"),
            DatabaseType::MySQL => write!(f, "MySQL"),
        }
    }
}

pub struct DatabasePlugin {
    db_type: DatabaseType,
    with_migrations: bool,
}

impl DatabasePlugin {
    pub fn new(db_type: DatabaseType) -> Self {
        Self {
            db_type,
            with_migrations: true,
        }
    }

    pub fn with_migrations(mut self, enabled: bool) -> Self {
        self.with_migrations = enabled;
        self
    }

    fn get_sqlx_features(&self) -> Vec<&'static str> {
        match self.db_type {
            DatabaseType::PostgreSQL => vec!["runtime-tokio-rustls", "postgres"],
            DatabaseType::SQLite => vec!["runtime-tokio-rustls", "sqlite"],
            DatabaseType::MySQL => vec!["runtime-tokio-rustls", "mysql"],
        }
    }

    fn get_database_url_example(&self) -> &'static str {
        match self.db_type {
            DatabaseType::PostgreSQL => "postgresql://username:password@localhost/database",
            DatabaseType::SQLite => "sqlite://database.db",
            DatabaseType::MySQL => "mysql://username:password@localhost/database",
        }
    }

    fn generate_database_module(&self) -> String {
        format!(
            r#"use sqlx::{{pool, prelude::*}};

#[derive(Clone)]
pub struct Database {{
    pool: sqlx::Pool<sqlx::{}>,
}}

impl Database {{
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {{
        let pool = sqlx::{}::PoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        
        Ok(Self {{ pool }})
    }}
    
    pub async fn run_migrations(&self) -> Result<(), sqlx::migrate::MigrateError> {{
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
    }}
    
    pub fn pool(&self) -> &sqlx::Pool<sqlx::{}> {{
        &self.pool
    }}
}}
"#,
            match self.db_type {
                DatabaseType::PostgreSQL => "Postgres",
                DatabaseType::SQLite => "Sqlite",
                DatabaseType::MySQL => "MySql",
            },
            match self.db_type {
                DatabaseType::PostgreSQL => "postgres",
                DatabaseType::SQLite => "sqlite",
                DatabaseType::MySQL => "mysql",
            },
            match self.db_type {
                DatabaseType::PostgreSQL => "Postgres",
                DatabaseType::SQLite => "Sqlite",
                DatabaseType::MySQL => "MySql",
            }
        )
    }

    fn generate_example_migration(&self) -> String {
        match self.db_type {
            DatabaseType::PostgreSQL => r#"-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create update trigger
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at BEFORE UPDATE
    ON users FOR EACH ROW EXECUTE PROCEDURE
    update_updated_at_column();"#
                .to_string(),

            DatabaseType::SQLite => r#"-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create update trigger
CREATE TRIGGER update_users_updated_at
    AFTER UPDATE ON users
BEGIN
    UPDATE users SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;"#
                .to_string(),

            DatabaseType::MySQL => r#"-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;"#
                .to_string(),
        }
    }
}

impl Plugin for DatabasePlugin {
    fn name(&self) -> &str {
        "Database"
    }

    fn configure(&self, context: &mut ProjectContext) -> Result<(), Box<dyn Error>> {
        let features_str = self
            .get_sqlx_features()
            .iter()
            .map(|f| format!(r#""{}""#, f))
            .collect::<Vec<_>>()
            .join(", ");
        context.add_dependency(
            "sqlx",
            &format!(r#"{{ version = "0.7", features = [{}] }}"#, features_str),
        );

        context.add_dependency("tokio", r#"{ version = "1", features = ["full"] }"#);
        context.add_dependency("dotenv", r#""0.15""#);

        context.add_to_gitignore(".env");
        context.add_to_gitignore("*.db");
        context.add_to_gitignore("*.db-shm");
        context.add_to_gitignore("*.db-wal");

        let env_content = format!("DATABASE_URL={}\n", self.get_database_url_example());
        context.add_template_file(".env.example", env_content);

        context.add_template_file("src/database.rs", self.generate_database_module());

        if self.with_migrations {
            context.create_directory("migrations");

            let migration_name = "001_create_users_table.sql";
            context.add_template_file(
                &format!("migrations/{}", migration_name),
                self.generate_example_migration(),
            );

            context.add_template_file("migrations/.gitkeep", "".to_string());
        }

        let example_code = format!(
            r#"use dotenv::dotenv;
use std::env;

mod database;
use database::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {{
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let db = Database::new(&database_url).await?;
    
    // Run migrations if enabled
    #[cfg(feature = "migrations")]
    db.run_migrations().await?;
    
    println!("Connected to {{}} database!", "{}");
    
    Ok(())
}}"#,
            self.db_type
        );

        context.add_example("database_connection", example_code);

        Ok(())
    }
}
