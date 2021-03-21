use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use std::convert::{TryFrom, TryInto};

use crate::domain::SubscriberEmail;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub email_client: EmailSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub database_name: String,
    pub host: String,
    pub password: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub require_ssl: bool,
    pub username: String,
}

#[derive(serde::Deserialize)]
pub struct EmailSettings {
    pub base_url: String,
    pub sender_email: String,
}

impl EmailSettings {
    pub fn sender(&self) -> Result<SubscriberEmail, String> {
        SubscriberEmail::parse(self.sender_email.clone())
    }
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub host: String,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .password(&self.password)
            .port(self.port)
            .ssl_mode(ssl_mode)
            .username(&self.username)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();

    let base_path = std::env::current_dir().expect("Cannot get current directory");
    let config_dir = base_path.join("configuration");
    settings.merge(config::File::from(config_dir.join("base")).required(true))?;

    let env: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Cannot parse APP_ENVIRONMENT");
    settings.merge(config::File::from(config_dir.join(env.as_str())).required(true))?;

    settings.merge(config::Environment::with_prefix("app").separator("__"))?;

    settings.try_into()
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "Unsupported environment '{}'. Use either 'local' or 'production'",
                other
            )),
        }
    }
}
