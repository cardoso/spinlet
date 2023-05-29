mod context;

use wasmtime::{Result, Engine, component::{Linker, Component}, Store, Config};
use wasmtime_wasi::preview2::wasi::{command, command::Command};

pub use context::Context;


pub struct Loader {
    engine: Engine,
    linker: Linker<Context>,
    store: Store<Context>,
}

impl Loader {
    pub fn new(context: Context) -> Self {
        let mut config = Config::new();
        #[cfg(debug_assertions)]
        config.debug_info(true);
        config.wasm_component_model(true);
        config.async_support(true);
        let engine = Engine::new(&config).unwrap_or_else(|error| panic!("Failed to create Wasmtime engine: {error}"));
        let store = Store::new(&engine, context);
        let mut linker = Linker::new(&engine);
        command::add_to_linker(&mut linker).unwrap_or_else(|error| panic!("Failed to add WASI to Wasmtime linker: {error}"));

        Self { engine, linker, store }
    }

    pub async fn load(&mut self, binary: &[u8]) -> Result<Command, anyhow::Error> {
        let component = Component::from_binary(&self.engine, &binary)?;
        let (command, _instance) = Command::instantiate_async(&mut self.store, &component, &self.linker).await?;
        Ok(command)
    }

    pub async fn run(&mut self, command: Command) -> Result<bool, anyhow::Error> {
        match command.call_run(&mut self.store).await? {
            Ok(()) => Ok(true),
            Err(()) => Ok(false),
        }
    }
}