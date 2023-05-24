pub use std::env::Args;
use std::env::Vars;
pub use anyhow::Result;

mod console;
mod workspace;

pub use console::Console;
pub use workspace::Workspace;

#[derive(Debug)]
pub struct Spin {
    vars: Vars,
    args: Args,
    console: Console,
    workspace: Workspace,
}

impl Spin {
    pub fn get() -> Self {
        Self {
            vars: std::env::vars(),
            args: std::env::args(),
            console: Console::new(),
            workspace: Workspace::new(),
        }
    }

    pub fn vfs(&self) -> &Workspace {
        &self.workspace
    }

    pub fn vfs_mut(&mut self) -> &mut Workspace {
        &mut self.workspace
    }

    pub fn console(&self) -> &Console {
        &self.console
    }

    pub fn workspace(&self) -> &Workspace {
        &self.workspace
    }

    pub fn args(&self) -> &Args {
        &self.args
    }

    pub fn vars(&self) -> &Vars {
        &self.vars
    }
}


