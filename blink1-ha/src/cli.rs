use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, env = "BLINK1_HA_CONFIG")]
    pub config: Option<PathBuf>,
}
