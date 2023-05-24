use std::env::Args;
pub use anyhow::Result;

mod console;
pub mod vfs;

use console::Console;
use vfs::Vfs;

#[derive(Debug)]
pub struct Spin {
    version: String,
    workspace: String,
    console: Console,
    args: Args,
    vfs: Vfs,
}

impl Spin {
    pub fn get() -> Result<Self> {
        if cfg!(target_os = "wasi") {
            Ok(Self {
                version: std::env::var("VERSION")?,
                workspace: std::env::var("WORKSPACE")?,
                args: std::env::args(),
                console: Console::new(),
                vfs: Vfs::new(),
            })
        } else {
            Ok(Self {
                version: "0.0.0".into(),
                workspace: "/".into(),
                args: std::env::args(),
                console: Console::new(),
                vfs: Vfs::new(),
            })
        }
    }

    pub fn vfs(&self) -> &Vfs {
        &self.vfs
    }

    pub fn vfs_mut(&mut self) -> &mut Vfs {
        &mut self.vfs
    }

    pub fn console(&self) -> &Console {
        &self.console
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn workspace(&self) -> &str {
        &self.workspace
    }

    pub fn args(&self) -> &Args {
        &self.args
    }
}


