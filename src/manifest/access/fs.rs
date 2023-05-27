mod dir;
mod file;

use serde::{Serialize, Deserialize};
use wasmtime_wasi::preview2::WasiCtxBuilder;

use dir::DirAccess;
use file::FileAccess;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FsAccess {
    #[serde(default)]
    file: Vec<FileAccess>,
    #[serde(default)]
    dir: Vec<DirAccess>,
}

impl FsAccess {
    pub fn provide(&self, mut ctx: WasiCtxBuilder) -> std::io::Result<WasiCtxBuilder> {
        for dir in &self.dir {
            ctx = dir.provide(ctx)?;
        }

        for file in &self.file {
            ctx = file.provide(ctx)?;
        }

        Ok(ctx)
    }
}