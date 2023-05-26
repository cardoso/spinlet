use std::path::PathBuf;

use clap::Parser;
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
    let context = Context::load(&path).await.expect("Failed to create context");
    
    
    let spinlet = Spinlet::load(&path, context).await.expect("Failed to load spinlet");

    match spinlet.run().await {
        Ok(Ok(spinlet)) => tracing::info!("{spinlet:#?}"),
        Ok(Err(spinlet)) => tracing::warn!("{spinlet:#?}"),
        Err(error) => tracing::error!("{error}")
    };


    
}
