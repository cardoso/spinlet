pub use spinlet_config as config;
pub use spinlet_manifest as manifest;

mod loader;
mod cli;
mod runner;
mod context;


pub use cli::Args;
pub use config::Config;
pub use context::Context;
pub use loader::Loader;
pub use runner::Runner;
pub use manifest::{Manifest, access::fs::{DirAccess, FileAccess}};
pub use cap_primitives::fs::OpenOptions;
pub use wasmtime_wasi::preview2::DirPerms;
pub use wasmtime_wasi::preview2::FilePerms;
pub use wasmtime_wasi::preview2::stdio;
pub use wasmtime_wasi::{preview2::{WasiCtxBuilder, Table}, Dir, ambient_authority};
pub use wasmtime::Result;

pub struct Spinlet {
    loader: Loader<Context>
}

impl Spinlet {
    pub fn new(loader: Loader<Context>) -> Self {
        Self { loader }
    }

    pub async fn run(self, binary: &[u8]) -> Result<bool> {
        let (command, store) = self.loader.load(binary).await?;
        let mut runner = Runner::new(command, store);
        runner.run().await
    }
}