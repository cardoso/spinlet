use std::path::PathBuf;

use clap::Parser;
use spinlet::runtime::Capabilities;
use tokio::fs::read_to_string;
use toml::from_str;
use spinlet::runtime::{Spinlet, Context};

pub const BIN_NAME: &str = "spin let";
pub const DEFAULT_SPINLET: &str = "shell";

#[derive(Parser)]
#[command(bin_name = BIN_NAME)]
pub struct Args {
    #[arg(default_value = DEFAULT_SPINLET)]
    pub spinlet: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let path = PathBuf::new().join(".spinlet/bin").join(args.spinlet);
    let spinlet = Spinlet::load(path).await.expect("Failed to create context");
    spinlet.run().await.expect("Spinlet failed");
}
