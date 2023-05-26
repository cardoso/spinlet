use std::path::PathBuf;

use clap::Parser;
use spinlet::Executor;
use spinlet::Spinlet;
use spinlet::Context;
use spinlet::Args;
use spinlet::Capabilities;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let path = PathBuf::new().join(".spinlet/bin").join(args.spinlet);
    let capabilities = Capabilities::load(&path.with_extension("toml")).await.expect("Failed to load capabilities");
    let context = Context::new(capabilities).expect("Failed to create context");
    let executor = Executor::new(context).expect("Failed to create executor");
    let mut spinlet = Spinlet::load(&path.with_extension("wasm"), executor).await.expect("Failed to load spinlet");

    match spinlet.run().await {
        Ok(Ok(())) => tracing::info!("success"),
        Ok(Err(())) => tracing::warn!("failure"),
        Err(error) => tracing::error!("{error}")
    };
}
