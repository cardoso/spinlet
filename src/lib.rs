use std::path::Path;

use wasmtime::{Result, component::Component};
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
   
    pub async fn load(path: &Path, mut executor: Executor<Context>) -> Result<Self> {
        let component = Component::from_file(&executor.engine(), path)?;
        let command = executor.load(&component).await?;
        
        Ok(Spinlet { executor, command })
    }
    
    pub async fn run(&mut self) -> Result<Result<(), ()>> {
        self.executor.run(&mut self.command).await
    }
}