use std::path::Path;
use std::path::PathBuf;

use clap::Parser;
use spinlet::Executor;
use spinlet::Spinlet;
use spinlet::Context;
use spinlet::Args;
use spinlet::Capabilities;

pub const SPINLET_BIN: &'static str = ".spinlet/bin";
pub const SPINLET_BIN_EXT: &'static str = "wasm";

pub const SPINLET_ETC: &'static str = ".spinlet/etc";
pub const SPINLET_ETC_EXT: &'static str = "toml";

pub fn bin(path: impl AsRef<str>) -> PathBuf {
    Path::new(SPINLET_BIN).join(path.as_ref()).with_extension(SPINLET_BIN_EXT)
}

pub fn etc(path: impl AsRef<str>) -> PathBuf {
    Path::new(SPINLET_ETC).join(path.as_ref()).with_extension(SPINLET_ETC_EXT)
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let etc = etc(&args.spinlet);
    let bin = bin(&args.spinlet);
    let capabilities = Capabilities::load(&etc).await.expect("Failed to load capabilities");
    let context = Context::new(capabilities).expect("Failed to create context");
    let executor = Executor::new(context).expect("Failed to create executor");
    let mut spinlet = Spinlet::load(executor, &bin).await.expect("Failed to load spinlet");

    match spinlet.run().await {
        Ok(Ok(())) => tracing::info!("success"),
        Ok(Err(())) => tracing::warn!("failure"),
        Err(error) => tracing::error!("{error}")
    };
}
