use std::result::Result;

use serde::{Serialize, Deserialize};
use wasmtime_wasi::preview2::WasiCtxBuilder;

pub mod fs;
pub mod env;
pub mod io;
pub mod error;

use env::EnvAccess;
use io::IoAccess;
use fs::FsAccess;

pub use error::AccessError;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Access {
    #[serde(default)]
    fs: FsAccess,
    #[serde(default)]
    env: EnvAccess,
    #[serde(default)]
    io: IoAccess
}

impl Access {
    pub fn provide(&self, mut ctx: WasiCtxBuilder) -> Result<WasiCtxBuilder, AccessError> {
        ctx = self.io.provide(ctx);
        ctx = self.env.provide(ctx)?;
        ctx = self.fs.provide(ctx)?;

        Ok(ctx)
    }
}