use eyre::{Result, WrapErr};
use secrecy::{ExposeSecret, SecretString};
use sqlx::{
    postgres::{PgConnectOptions, PgSslMode},
    ConnectOptions,
};
use std::{
    convert::{TryFrom, TryInto},
    net::IpAddr,
};

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: IpAddr,
    pub base_url: String,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    #[must_use]
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    #[must_use]
    pub fn with_db(&self) -> PgConnectOptions {
        let mut options = self.without_db().database(&self.database_name);
        options.log_statements(tracing::log::LevelFilter::Trace);
        options
    }
}

pub fn extract(environment: Environment) -> Result<Settings> {
    let base_path =
        std::env::current_dir().wrap_err("failed to determine the current directory")?;
    let configuration_directory = base_path.join("configuration");

    // Initialise our configuration reader
    let settings = config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("base")).required(true))
        .add_source(
            config::File::from(configuration_directory.join(environment.as_str())).required(true),
        )
        // Add in settings from environment variables (with a prefix of APP and '__' as separator)
        // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
        .add_source(config::Environment::with_prefix("app").separator("__"))
        .build()?;

    // Try to convert the configuration values it read into
    // our Settings type
    settings
        .try_deserialize()
        .wrap_err("failed to deserialize config files")
}

/// Detect the running environment.
/// Default to `Environment::Local` if unspecified.
pub fn get_environment() -> Result<Environment> {
    std::env::var("APP_ENVIRONMENT").map_or(Ok(Environment::Local), |s| {
        s.try_into().wrap_err("failed to parse APP_ENVIRONMENT")
    })
}

/// The possible runtime environment for our application.
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = eyre::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(eyre::eyre!(
                "{other} is not a supported environment. Use either `local` or `production`"
            )),
        }
    }
}
