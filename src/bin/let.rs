use std::path::Path;
use std::path::PathBuf;

use clap::Parser;

use human_panic::setup_panic;
use spinlet::Executor;
use spinlet::Spinlet;
use spinlet::Context;
use spinlet::Args;
use spinlet::list_spinlets;
use spinlet::load_manifest;

pub const SPINLET_BIN: &'static str = ".spinlet/bin";
pub const SPINLET_BIN_EXT: &'static str = "wasm";




pub fn bin_path(spinlet: &str) -> PathBuf {
    Path::new(SPINLET_BIN).join(spinlet).with_extension(SPINLET_BIN_EXT)
}

#[tokio::main]
async fn main() {
    setup_panic!();

    let args = Args::parse();
    let path = bin_path(&args.spinlet);

    if !path.exists() {
        list_spinlets(SPINLET_BIN);
        return;
    }

    let manifest = load_manifest(&args.spinlet).await;
    let wasi = manifest.provide().expect("Failed to provide capabilities");
    let context = Context::new(wasi).expect("Failed to create context");
    let mut executor = Executor::new(context).expect("Failed to create executor");

    

    let command = executor.load(&path).await.expect("Failed to load command");
    let mut spinlet = Spinlet::new(executor, command);

    match spinlet.run().await {
        Ok(Ok(())) => tracing::info!("success"),
        Ok(Err(())) => tracing::warn!("failure"),
        Err(error) => tracing::error!("{error}")
    };
}
