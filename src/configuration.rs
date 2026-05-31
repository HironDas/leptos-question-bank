use secrecy::SecretString;
use serde::Deserialize;
use serde_aux::prelude::*;

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

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Environment::Local),
            "production" => Ok(Environment::Production),
            other => Err(format!(
                "{} is not a valid environment. Use either 'local' or 'production'",
                other
            )),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    // pub base_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub database_name: String,
    pub require_ssl: bool,
}

#[cfg(feature = "ssr")]
use sqlx::postgres::PgConnectOptions;

impl DatabaseSettings {
    #[cfg(feature = "ssr")]
    pub fn without_db(&self) -> PgConnectOptions {
        //use leptos::logging::log;
        use secrecy::ExposeSecret;
        use sqlx::postgres::PgSslMode;

        let ssl_mode = match self.require_ssl {
            true => PgSslMode::Require,
            false => PgSslMode::Prefer,
        };
        // log!("Database Name: {}", self.database_name);
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(&self.password.expose_secret())
            .ssl_mode(ssl_mode)
    }

    #[cfg(feature = "ssr")]
    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
        //.log_statements(tracing::log::LevelFilter::Trace)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[cfg(feature = "ssr")]
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("config");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    let environment_filename = format!("{}.yaml", environment.as_str());
    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(&environment_filename),
        ));

    // Add in settings from environment variables (with a prefix of APP and '__' as separator)
    // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
    let settings = settings
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn environment_local_from_string() {
        let env: Environment = "local".to_string().try_into().unwrap();
        assert_eq!(env.as_str(), "local");
    }

    #[test]
    fn environment_production_from_string() {
        let env: Environment = "production".to_string().try_into().unwrap();
        assert_eq!(env.as_str(), "production");
    }

    #[test]
    fn environment_case_insensitive() {
        let env: Environment = "PRODUCTION".to_string().try_into().unwrap();
        assert_eq!(env.as_str(), "production");
    }

    #[test]
    fn environment_invalid_string_is_rejected() {
        let result: Result<Environment, _> = "staging".to_string().try_into();
        assert!(result.is_err());
    }

    #[test]
    fn environment_empty_string_is_rejected() {
        let result: Result<Environment, _> = "".to_string().try_into();
        assert!(result.is_err());
    }

    #[test]
    fn application_settings_deserialization() {
        // Verify the ApplicationSettings struct fields exist and types are correct
        let settings = ApplicationSettings {
            port: 3000,
            host: "127.0.0.1".to_string(),
        };
        assert_eq!(settings.port, 3000);
        assert_eq!(settings.host, "127.0.0.1");
    }
}
