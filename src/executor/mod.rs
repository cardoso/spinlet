use std::path::Path;
use wasmtime::{Result, Engine, component::{Linker, Component}, Store, Config};
use wasmtime_wasi::preview2::{wasi::{command, command::Command}, WasiView};


pub struct Executor<T> where T: WasiView {
    engine: Engine,
    linker: Linker<T>,
    store: Store<T>,
}

impl<T> Executor<T> where T: WasiView {
    pub fn new(context: T) -> Result<Self> {
        let mut config = Config::new();
        #[cfg(debug_assertions)]
        config.debug_info(true);
        config.wasm_component_model(true);
        config.async_support(true);

        let engine = Engine::new(&config)?;
        let store = Store::<T>::new(&engine, context);
        let mut linker = Linker::<T>::new(&engine);

        command::add_to_linker(&mut linker)?;

        Ok(Self { engine, linker, store })
    }

    pub async fn run(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let component = Component::from_file(&self.engine, path)?;
        let (command, _instance) = Command::instantiate_async(&mut self.store, &component, &self.linker).await?;
        let result = command.call_run(&mut self.store).await?;

        match result {
            Ok(()) => return Ok(()),
            Err(()) => return Ok(()),
        }
    }
}