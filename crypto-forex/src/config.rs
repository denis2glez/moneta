//! Manage the application configuration hierarchically using the content of `config` directory.

use serde::Deserialize;
use sqlx::postgres::PgConnectOptions;
use sqlx::ConnectOptions;
use std::convert::{TryFrom, TryInto};

/// Possible runtime environment of the application.
pub enum Env {
    Development,
    Production,
}

impl Env {
    pub fn as_str(&self) -> &'static str {
        match self {
            Env::Development => "dev",
            Env::Production => "prod",
        }
    }
}

impl TryFrom<String> for Env {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "dev" => Ok(Self::Development),
            "prod" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `dev` or `prod`.",
                other
            )),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Configuration {
    pub database: DbConfig,
    pub application: AppConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DbConfig {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DbConfig {
    pub fn without_db(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        let mut options = self.without_db().database(&self.database_name);
        options.log_statements(log::LevelFilter::Trace);
        options
    }
}

pub fn load_config() -> Result<Configuration, config::ConfigError> {
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("config");

    // Read the "default" configuration file
    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;

    // Detect the running environment.
    // Default to `dev` if unspecified.
    let environment: Env = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "dev".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    // Layer on the environment-specific values.
    settings.merge(
        config::File::from(configuration_directory.join(environment.as_str())).required(true),
    )?;

    // Add in settings from environment variables (with a prefix of APP and '_' as separator)
    // E.g. `APP_APPLICATION_PORT=5001 would set `Settings.application.port`
    settings.merge(config::Environment::with_prefix("app").separator("_"))?;

    settings.try_into()
}
