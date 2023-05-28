mod manifest;
mod context;
mod executor;
mod cli;

use std::path::{Path, PathBuf};

pub use cli::Cli;
pub use context::Context;
pub use executor::Executor;
pub use manifest::Manifest;
use wasmtime_wasi::preview2::WasiCtxBuilder;
use wasmtime::Result;

pub struct Spinlet {
    binary: PathBuf,
    executor: Executor<Context>,
}

impl Spinlet {
    pub async fn load(manifest: impl AsRef<Path>, binary: impl AsRef<Path>) -> Result<Self> {
        let path = manifest.as_ref().to_path_buf();
        let binary = binary.as_ref().to_path_buf();
        let manifest = Manifest::load(&path).await?;
        let ctx = WasiCtxBuilder::new();
        let wasi = manifest.provide(ctx)?;
        let context = Context::new(wasi)?;
        let executor = Executor::new(context)?;
        Ok(Self { binary, executor} )
    }

    pub async fn run(&mut self) -> Result<()> {
        self.executor.run(&self.binary).await?;

        Ok(())
    }
}