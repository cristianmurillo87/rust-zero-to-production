use config::{Config, ConfigError};
// the ExposeSecret trait  is necessary, so that the expose_secret becommes available
use secrecy::{ExposeSecret, SecretBox};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretBox<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> SecretBox<String> {
        let connection_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        );

        SecretBox::new(Box::new(connection_string))
    }

    pub fn connection_string_without_db(&self) -> SecretBox<String> {
        let connection_string = format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        );

        SecretBox::new(Box::new(connection_string))
    }
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
                "{} is not a supported environment. Use either 'local' or 'production'",
                other
            )),
        }
    }
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("config");

    let base_config_file_path = configuration_directory.join("base");
    let base_config_file = config::File::from(base_config_file_path).required(true);

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let config_file_path = configuration_directory.join(environment.as_str());
    let config_file = config::File::from(config_file_path).required(true);

    let settings = Config::builder()
        .add_source(base_config_file)
        .add_source(config_file)
        // add settings from environment variables
        // in this case the prefix for such variables should have the prefix APP_ and the value to be set
        // in the configuration file should be prefixed by double underscores (__)
        // e.g. APP_APPLICATION__PORT=8000 would set the Settings.application.port configuration
        .add_source(config::Environment::with_prefix("app").separator("__"))
        .build()
        .unwrap();

    settings.try_deserialize::<Settings>()
}
