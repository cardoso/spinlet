use std::path::Path;

use wasmtime::Result;
use wasmtime_wasi::preview2::wasi::command::Command;

mod context;
mod executor;
mod runtime;
mod cli;

pub use cli::Args;
pub use context::Context;
pub use executor::Executor;
pub use context::Capabilities;

pub struct Spinlet {
    executor: Executor<Context>,
    command: Command
}

impl Spinlet {
    pub async fn load(mut executor: Executor<Context>, path: &Path) -> Result<Self> {
        let command = executor.load(&path).await?;
        
        Ok(Spinlet { executor, command })
    }
    
    pub async fn run(&mut self) -> Result<Result<(), ()>> {
        Ok(self.executor.run(&mut self.command).await?)
    }
}