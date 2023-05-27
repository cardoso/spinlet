use wasmtime::Result;

use std::path::Path;

use wasmtime_wasi::preview2::wasi::command::Command;

use crate::context::Context;

use crate::executor::Executor;

pub struct Spinlet {
    pub(crate) executor: Executor<Context>,
    pub(crate) command: Command
}

impl Spinlet {
    pub async fn load(mut executor: Executor<Context>, path: &Path) -> Result<Self> {
        let command = executor.load(&path).await?;
    
        Ok(Spinlet { executor, command })
    }

    pub async fn run(&mut self) -> Result<Result<(), ()>> {
        self.executor.run(&mut self.command).await
    }
}
