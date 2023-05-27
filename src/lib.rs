mod manifest;
mod context;
mod executor;
mod cli;

use std::path::Path;

pub use cli::Args;
pub use context::Context;
pub use executor::Executor;
pub use manifest::Manifest;
use wasmtime_wasi::preview2::wasi::command::Command;
use wasmtime::Result;

pub struct Spinlet {
    executor: Executor<Context>,
    command: Command
}

impl Spinlet {
    pub fn new(executor: Executor<Context>, command: Command) -> Self {
        Spinlet { executor, command }
    }

    pub async fn run(&mut self) -> Result<Result<(), ()>> {
        self.executor.run(&mut self.command).await
    }
}

pub fn list_spinlets(path: impl AsRef<Path>) {
    let Ok(entries) = path.as_ref().read_dir() else { return };
    eprintln!("Available spinlets:");
    for entry in entries {
        let Ok(entry) = entry else { continue };
        let Some(name) = spinlet_name(entry.path()) else { continue };
        eprintln!("  {name}");
    }
}

fn spinlet_name(path: impl AsRef<Path>) -> Option<String> {
    if not_file(&path) { return None };
    if not_wasm(&path) { return None };
    file_stem(path)
}

fn file_stem(path: impl AsRef<Path>) -> Option<String> {
    let stem = path.as_ref().file_stem()?;
    Some(stem.to_string_lossy().to_string())
}

fn not_file(path: impl AsRef<Path>) -> bool {
    !path.as_ref().is_file()
}

fn not_wasm(path: impl AsRef<Path>) -> bool {
    let Some(ext) = path.as_ref().extension() else { return true };
    ext != "wasm"
}


pub const SPINLET_ETC: &'static str = ".spinlet/etc";
pub const SPINLET_ETC_EXT: &'static str = "toml";


pub async fn load_manifest(spinlet: &str) -> Manifest {
    let path = Path::new(SPINLET_ETC).join(spinlet).with_extension(SPINLET_ETC_EXT);
    match Manifest::load(&path).await {
        Ok(manifest) => manifest,
        Err(error) => {
            eprintln!("error loading manifest: {error}");
            Manifest::default()
        }
    }
}