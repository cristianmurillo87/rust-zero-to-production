use config::{Config, ConfigError, File};
// the ExposeSecret trait  is necessary, so that the expose_secret becommes available
use secrecy::{ExposeSecret, SecretBox};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
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

pub fn get_configuration() -> Result<Settings, ConfigError> {
    let config_file = File::with_name("config/configuration");
    let settings = Config::builder().add_source(config_file).build().unwrap();
    settings.try_deserialize::<Settings>()
}
