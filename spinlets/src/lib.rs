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
        if cfg!(not(target_arch = "wasm32")) {
            panic!("Spinlet::get() is only available in WASM for security reasons.");
        }

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

    pub fn print(&self, message: &str) {
        match self.console.print(message) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error printing to console: {}", e);
            }
        }
    }

    pub fn print_line(&self, message: &str) {
        match self.console.print_line(message) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error printing to console: {}", e);
            }
        }
    }

    pub fn read_line(&self) -> String {
        match self.console.read_line() {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error reading from console: {}", e);
                String::new()
            }
        }
    }
}


