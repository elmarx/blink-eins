use thiserror::Error;

#[derive(Error, Debug)]
pub enum Blink1Error {
    #[error("Failed to read configuration file: {0}")]
    ConfigFileReadError(#[source] std::io::Error),

    #[error("Failed to parse configuration file: {0}")]
    ConfigParseError(#[source] toml::de::Error),

    #[error("Failed to publish MQTT HomeAssistant sensor autodiscovery message: {0}")]
    MqttHaDiscovery(#[source] Box<rumqttc::v5::ClientError>),

    #[error("Failed to publish MQTT availability message: {0}")]
    MqttOnlinePublish(#[source] Box<rumqttc::v5::ClientError>),

    #[error("Failed to subscribe to MQTT topic: {0}")]
    MqttSubscribeError(#[source] Box<rumqttc::v5::ClientError>),

    #[error("Config nof found")]
    ConfigNotFound,
}
