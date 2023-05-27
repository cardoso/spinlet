mod env;
pub use anyhow::Result;
mod console;
pub use env::Workspace;
pub use console::Console;


#[derive(Debug)]
pub struct Spinlet {
    console: Console,
    workspace: env::Workspace,
}

impl Spinlet {
    pub fn get() -> Self {
        Self {
            console: Console::get(),
            workspace: env::Workspace::get(),
        }
    }

    pub fn workspace_mut(&mut self) -> &mut env::Workspace {
        &mut self.workspace
    }

    pub fn workspace(&self) -> &env::Workspace {
        &self.workspace
    }

    pub fn console(&self) -> &Console {
        &self.console
    }
}


