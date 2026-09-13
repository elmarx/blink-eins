use crate::cli::Args;
use crate::config::{Config, get_config_path};
use crate::error::Blink1Error;
use clap::Parser;

mod cli;
mod config;

mod error;
mod utils;

fn main() -> Result<(), Blink1Error> {
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

    Ok(())
}
