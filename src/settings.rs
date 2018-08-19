extern crate serde;
use config::{Config, ConfigError, Environment, File};

#[derive(Debug, Deserialize)]
pub struct Mqtt {
    pub url: String,
    pub tls: bool,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub out_client: Mqtt,
    pub out_topic: String,
    pub heartbeat_template: String,
    pub delay_seconds: u64,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // merging in the "default" configuration file
        s.merge(File::with_name("config/default"))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("app").separator("__"))?;

        s.try_into()
    }
}
