use wasmtime::{Engine, component::{Linker, Component}, Store, Config};
use anyhow::Result;
use wasmtime_wasi::preview2::{wasi::command::{self, Command}, WasiView};

pub struct Executor<T: WasiView> {
    engine: Engine,
    linker: Linker<T>,
    store: Store<T>,
}

impl<T: WasiView> Executor<T> {
    pub fn new(context: T) -> Result<Self> {
        let mut config = Config::new();
        #[cfg(debug_assertions)]
        config.debug_info(true);
        config.wasm_component_model(true);
        config.async_support(true);

        let engine = Engine::new(&config)?;
        let mut linker = Linker::<T>::new(&engine);

        command::add_to_linker(&mut linker)?;
        
        let store = Store::new(&engine, context);

        Ok(Self { engine, linker, store })
    }

    pub async fn load(&mut self, component: &Component) -> Result<Command> {
        let (command, _instance) = Command::instantiate_async(&mut self.store, component, &mut self.linker).await?;
        Ok(command)
    }

    pub async fn run(&mut self, command: &Command) -> Result<Result<(), ()>> {
        command.call_run(&mut self.store).await
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }
}