use std::path::PathBuf;

use clap::Parser;
use spinlet::Capabilities;
use spinlet::Spinlet;
use spinlet::Context;
use spinlet::Args;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let path = PathBuf::new().join(".spinlet/bin").join(args.spinlet);
    let capabilities = Capabilities::load(&path).await.expect("Failed to load capabilities");
    let context = Context::new(capabilities).expect("Failed to create context");
    let spinlet = Spinlet::load(&path, context).await.expect("Failed to load spinlet");

    match spinlet.run().await {
        Ok(Ok(spinlet)) => tracing::info!("{spinlet:#?}"),
        Ok(Err(spinlet)) => tracing::warn!("{spinlet:#?}"),
        Err(error) => tracing::error!("{error}")
    };
}
