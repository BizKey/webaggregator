use anyhow::{Context, Result};
use std::env;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
}

#[derive(Debug, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        Ok(AppConfig {
            server: ServerConfig::from_env()?,
            database: DatabaseConfig::from_env()?,
            logging: LoggingConfig::from_env()?,
        })
    }

    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}

impl ServerConfig {
    pub fn from_env() -> Result<Self> {
        Ok(ServerConfig {
            host: get_env_with_default("SERVER_HOST", "0.0.0.0")?,
            port: get_env_with_default("SERVER_PORT", "8080")?
                .parse()
                .context("Invalid SERVER_PORT value")?,
            workers: get_env_with_default("SERVER_WORKERS", "4")?
                .parse()
                .context("Invalid SERVER_WORKERS value")?,
        })
    }
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self> {
        let acquire_timeout_secs: u64 = get_env_with_default("DB_ACQUIRE_TIMEOUT_SECS", "10")?
            .parse()
            .context("Invalid DB_ACQUIRE_TIMEOUT_SECS value")?;

        let idle_timeout_secs: u64 = get_env_with_default("DB_IDLE_TIMEOUT_SECS", "600")?
            .parse()
            .context("Invalid DB_IDLE_TIMEOUT_SECS value")?;

        let max_lifetime_secs: u64 = get_env_with_default("DB_MAX_LIFETIME_SECS", "1800")?
            .parse()
            .context("Invalid DB_MAX_LIFETIME_SECS value")?;

        Ok(DatabaseConfig {
            url: get_env("DATABASE_URL")?,
            max_connections: get_env_with_default("DB_MAX_CONNECTIONS", "10")?
                .parse()
                .context("Invalid DB_MAX_CONNECTIONS value")?,
            min_connections: get_env_with_default("DB_MIN_CONNECTIONS", "1")?
                .parse()
                .context("Invalid DB_MIN_CONNECTIONS value")?,
            acquire_timeout: Duration::from_secs(acquire_timeout_secs),
            idle_timeout: Duration::from_secs(idle_timeout_secs),
            max_lifetime: Duration::from_secs(max_lifetime_secs),
        })
    }
}

impl LoggingConfig {
    pub fn from_env() -> Result<Self> {
        Ok(LoggingConfig {
            level: get_env_with_default("RUST_LOG", "info")?,
            format: get_env_with_default("LOG_FORMAT", "text")?,
        })
    }
}

fn get_env_with_default(key: &str, default: &str) -> Result<String> {
    match env::var(key) {
        Ok(val) => Ok(val.trim().to_string()),
        Err(env::VarError::NotPresent) => Ok(default.to_string()),
        Err(err) => Err(anyhow::anyhow!("Failed to read env var {}: {}", key, err)),
    }
}

pub fn get_env(key: &str) -> Result<String> {
    Ok(env::var(key)?.trim().to_string())
}
