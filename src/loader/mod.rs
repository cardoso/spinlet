use wasmtime::{Result, Engine, component::{Linker, Component}, Store, Config};
use wasmtime_wasi::preview2::{wasi::{command, command::Command}, WasiView};

pub struct Loader<T: Send + WasiView> {
    engine: Engine,
    context: T
}

impl<T: Send + WasiView> Loader<T> {
    pub fn new(context: T) -> Result<Self> {
        let mut config = Config::new();
        #[cfg(debug_assertions)]
        config.debug_info(true);
        config.wasm_component_model(true);
        config.async_support(true);
        let engine = Engine::new(&config)?;
        Ok(Self { engine, context })
    }

    pub async fn load(self, binary: &[u8]) -> Result<(Command, Store<T>)> {
        let mut store = Store::new(&self.engine, self.context);
        let mut linker = Linker::new(&self.engine);
        command::add_to_linker(&mut linker)?;
        let component = Component::from_binary(&self.engine, binary)?;
        let (command, _instance) = Command::instantiate_async(&mut store, &component, &linker).await?;
        Ok((command, store))
    }
}