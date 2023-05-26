use std::path::Path;

use runtime::SpinletHook;
use wasmtime::{Engine, WasmBacktraceDetails, Store, Result, component::{Linker, Component}, Config};
use wasmtime_wasi::preview2::wasi::command::{self, Command};

mod runtime;
mod cli;

pub use cli::Args;
pub use runtime::Capabilities;
pub use runtime::Context;

pub struct Spinlet {
    store: Store<Context>,
    command: Command
}

impl std::fmt::Debug for Spinlet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Spinlet")
            .field("store", &self.store)
            .finish_non_exhaustive()
    }
}

impl Spinlet {
    #[tracing::instrument]
    pub async fn load(path: &Path, context: Context) -> Result<Self> {
        let mut config = Config::new();

        #[cfg(debug_assertions)]
        {
            config.debug_info(true);
            config.wasm_backtrace(true);
            config.wasm_backtrace_details(WasmBacktraceDetails::Enable);
        }

        config.wasm_component_model(true);
        config.async_support(true);

        let engine = Engine::new(&config)?;

        let mut store = Store::new(&engine, context);

        let mut linker = Linker::new(&engine);

        let hook = SpinletHook;

        store.call_hook_async(hook);

        command::add_to_linker(&mut linker)?;
        
        
        let path = path.with_extension("wasm");
        let component = Component::from_file(&engine, path)?;
        let (command, _instance) = Command::instantiate_async(&mut store, &component, &linker).await?;
        Ok(Spinlet { store, command })
    }

    pub fn store(&self) -> &Store<Context> {
        &self.store
    }

    #[tracing::instrument]
    pub async fn run(mut self) -> Result<Result<Self, Self>> {
        match self.command.call_run(&mut self.store).await {
            Ok(result) => match result {
                Ok(()) => {
                    tracing::info!("success");
                    Ok(Ok(self))
                },
                Err(()) => {
                    tracing::warn!("failed");
                    Ok(Err(self))
                }
            },
            Err(error) => {
                tracing::error!("{error}");
                Err(error)
            }
        }
        
    }
}
