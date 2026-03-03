use std::time::Duration;

use config::{Config, Environment, File};
use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

#[derive(Debug, Clone, Deserialize)]
pub struct HttpServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: HttpServerSettings,
    // TODO: secret string
    pub postgres: PostgresConfig,
}

impl AppConfig {
    /// Load application config from config file and environment.
    /// File: `config.toml` (optional). Env: `APP__` prefix (e.g. `APP__POSTGRES__HOST`).
    pub fn load() -> Result<Self, config::ConfigError> {
        let builder = Config::builder()
            .add_source(File::with_name("config.toml").required(false))
            .add_source(Environment::with_prefix("APP").separator("__"));
        let config = builder.build()?;
        config.try_deserialize::<AppConfig>()
    }

    #[cfg(feature = "test-utils")]
    pub fn load_tests() -> Self {
        Self::load().expect("failed to load config")
    }
}

/// PostgreSQL connection options for use with sqlx.
#[derive(Debug, Clone, Deserialize)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,

    /// Maximum number of connections in the pool (default: 10).
    #[serde(default = "defaults::max_connections")]
    pub max_connections: u32,
    /// Connection timeout in seconds (default: 5).
    #[serde(default = "defaults::connect_timeout_secs")]
    pub connect_timeout_secs: u64,
}

/// Create a sqlx PostgreSQL pool from PostgresConfig.
pub async fn create_pg_pool(cfg: &PostgresConfig) -> Result<sqlx::PgPool, sqlx::Error> {
    let connect_opts = PgConnectOptions::new()
        .host(&cfg.host)
        .port(cfg.port)
        .username(&cfg.user)
        .password(&cfg.password)
        .database(&cfg.database);
    PgPoolOptions::new()
        .max_connections(cfg.max_connections)
        .acquire_timeout(Duration::from_secs(cfg.connect_timeout_secs))
        .connect_with(connect_opts)
        .await
}

mod defaults {
    pub(super) fn max_connections() -> u32 {
        10
    }
    pub(super) fn connect_timeout_secs() -> u64 {
        5
    }
}
