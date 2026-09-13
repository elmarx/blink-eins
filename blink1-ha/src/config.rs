use crate::error::Blink1Error;
use crate::utils::Secret;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub mqtt: MqttConfig,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Blink1Error> {
        let contents = fs::read_to_string(path).map_err(Blink1Error::ConfigFileReadError)?;
        let mut config: Config =
            toml::from_str(&contents).map_err(Blink1Error::ConfigParseError)?;

        config.mqtt = config.mqtt.with_env_overrides();

        Ok(config)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MqttConfig {
    pub host: String,
    #[serde(default = "default::mqtt_port")]
    pub port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<Secret<String>>,
    #[serde(default = "default::topic_prefix")]
    pub topic_prefix: String,
    #[serde(default = "default::availability_topic")]
    pub availability_topic: String,
    /// Minimum interval in seconds to publish values, i.e.: do not send intervals more often than this value
    #[serde(default = "default::min_publish_interval")]
    pub min_publish_interval: u64,
    /// Maximum interval in seconds to publish values, i.e.: publish values at least every `max_publish_interval` seconds
    #[serde(default = "default::max_publish_interval")]
    pub max_publish_interval: u64,

    /// homeassistant topic, used for the discovery topic prefix, defaults to homeassistant
    #[serde(default = "default::homeassistant_topic")]
    pub homeassistant_topic: String,

    /// Number of days after which the (retained) discovery message expires at the broker if not
    /// refreshed. Prevents "ghost" entities from sticking around forever should blink1 be
    /// decommissioned. Defaults to 7 days.
    #[serde(default = "default::discovery_expire_days")]
    pub discovery_expire_days: u64,
}

impl MqttConfig {
    /// Override username and password from environment variables if set
    pub fn with_env_overrides(mut self) -> Self {
        if let Ok(username) = std::env::var("MQTT_USERNAME") {
            self.username = Some(username);
        }
        if let Ok(password) = std::env::var("MQTT_PASSWORD") {
            self.password = Some(Secret(password));
        } else if let Ok(password_file) = std::env::var("MQTT_PASSWORD_FILE") {
            // supports e.g. systemd's LoadCredential=/Environment=..%d/.. mechanism,
            // where the secret is provided as a file instead of directly as an
            // environment variable
            if let Ok(password) = fs::read_to_string(password_file) {
                self.password = Some(Secret(password.trim_end().to_string()));
            }
        }

        if let Ok(host) = std::env::var("MQTT_HOST") {
            self.host = host;
        }

        self
    }
}

pub fn get_config_path(config_from_cli: Option<PathBuf>) -> Option<PathBuf> {
    config_from_cli
        .or_else(|| {
            let config_in_cwd = PathBuf::from("config.toml");
            config_in_cwd.exists().then_some(config_in_cwd)
        })
        .or_else(|| {
            let config_in_etc = PathBuf::from("/etc/blink1.toml");
            config_in_etc.exists().then_some(config_in_etc)
        })
}

mod default {
    pub(super) fn mqtt_port() -> u16 {
        1883
    }

    pub(super) fn topic_prefix() -> String {
        "blink1".to_string()
    }

    pub(super) fn availability_topic() -> String {
        "blink1/availability".to_string()
    }

    pub(super) fn homeassistant_topic() -> String {
        "homeassistant".to_string()
    }

    pub(super) fn discovery_expire_days() -> u64 {
        7
    }

    pub(super) fn min_publish_interval() -> u64 {
        10
    }

    pub(super) fn max_publish_interval() -> u64 {
        300
    }
}
