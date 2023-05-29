use wasmtime_wasi::preview2::wasi::command::Command;
use wasmtime::Store;
use anyhow::Result;

pub struct Runner<T: Send> {
    command: Command,
    store: Store<T>
}

impl<T: Send> Runner<T> {
    pub fn new(command: Command, store: Store<T>) -> Self {
        Self { command, store }
    }

    pub async fn run(&mut self) -> Result<bool> {
        match self.command.call_run(&mut self.store).await? {
            Ok(()) => Ok(true),
            Err(()) => Ok(false)
        }
    }
}