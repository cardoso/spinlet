use anyhow::Result;
use lazy_static::lazy_static;

use std::path::Path;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store, WasmBacktraceDetails,
};
use wasmtime_wasi::preview2::{wasi, wasi::command::Command, Table, WasiCtx, WasiView, WasiWallClock, WasiMonotonicClock, WasiCtxBuilder};

lazy_static! {
    static ref ENGINE: Engine = {
        let mut config = Config::new();
        if cfg!(debug_assertions) {
            config.debug_info(true);
            config.wasm_backtrace(true);
            config.wasm_backtrace_details(WasmBacktraceDetails::Enable);
        }

        config.wasm_component_model(true);
        config.async_support(true);
        Engine::new(&config).unwrap()
    };
}

pub struct SpinletCtx {
    table: Table,
    wasi: WasiCtx,
}

impl SpinletCtx {
    pub fn new(args: &[String], envs: &[(impl AsRef<str>, impl AsRef<str>)]) -> Result<Self> {
        let mut table = Table::new();
        let wasi = WasiCtxBuilder::new()
            .set_env(&envs.as_ref())
            .set_args(&args)
            .inherit_stdio()
            .build(&mut table)?;

        Ok(Self {
            wasi,
            table
        })
    }
}

impl WasiView for SpinletCtx {
    fn table(&self) -> &Table {
        &self.table
    }
    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
    fn ctx(&self) -> &WasiCtx {
        &self.wasi
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

pub struct Spinlet {
    store: Store<SpinletCtx>,
    command: Command,
}

impl Spinlet {
    pub fn new(store: Store<SpinletCtx>, command: Command) -> Self {
        Spinlet {
            store,
            command,
        }
    }

    pub async fn load(path: impl AsRef<Path>, ctx: SpinletCtx) -> Result<Self> {
        let mut store = Store::new(&ENGINE, ctx);
        let mut linker = Linker::new(&ENGINE);

        wasi::command::add_to_linker(&mut linker)?;

        let component = Component::from_file(&ENGINE, &path)?;

        let (command, _) =
            Command::instantiate_async(&mut store, &component, &linker).await?;

        Ok(Spinlet::new(store, command))
    }

    pub async fn run(&mut self) -> Result<()> {
        match self.command.call_run(&mut self.store).await? {
            Ok(()) => Ok(()),
            Err(()) => Ok(()),
        }
    }
}
