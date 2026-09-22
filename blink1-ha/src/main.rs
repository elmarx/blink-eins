use crate::cli::Args;
use crate::config::{Config, get_config_path};
use crate::error::Blink1Error;
use clap::Parser;
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::{AsyncClient, MqttOptions};
use std::time::Duration;
use tokio_stream::StreamExt;

mod cli;
mod config;

mod error;
mod mqtt;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Blink1Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let args = Args::parse();
    let config = args.config;
    let config = get_config_path(config).ok_or(Blink1Error::ConfigNotFound)?;

    let config = Config::from_file(&config)?;

    tracing::info!(?config, "Loaded configuration");

    let mut stream = mqtt::init(&config.mqtt).await?;

    while let Some(event) = stream.next().await {
        tracing::info!(?event, "Received MQTT event");
    }

    Ok(())
}
