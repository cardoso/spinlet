use anyhow::Result;
use cap_std::{ambient_authority, fs::Dir};
use lazy_static::lazy_static;
use wasmtime_wasi_http::{WasiHttp, default_outgoing_http};
use std::{path::{PathBuf, Path, self}, io::*, any::Any};
use wasmtime::{
    component::{Component, Linker, Instance},
    Config, Engine, Store, WasmBacktraceDetails,
};
use wasmtime_wasi::{preview2::{wasi, OutputStream, InputStream, stdio, stdio::{Stdout, Stdin}, pipe::{ReadPipe, WritePipe}, wasi::{command::Command, random}, WasiCtxBuilder, Table, WasiCtx, WasiView, DirPerms, FilePerms, }};

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

pub struct CommandCtx {
    table: Table,
    wasi: WasiCtx,
}

impl CommandCtx {
    pub fn spawn_in(path: impl AsRef<Path>, dir: impl AsRef<Path>) -> Result<Self> {
        let dir = Dir::open_ambient_dir(dir, ambient_authority())?;
        let dir_perms = DirPerms::all();
        let file_perms = FilePerms::all();
        let mut table = Table::new();

        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .push_env("WORKSPACE", "/")
            .push_env("VERSION", env!("CARGO_PKG_VERSION"))
            .push_preopened_dir(dir, dir_perms, file_perms, "/")
            .build(&mut table)?;

        Ok(Self {
            table,
            wasi
        })
    }
}

impl WasiView for CommandCtx {
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
    store: Store<CommandCtx>,
    command: Command,
    instance: Instance,
}

impl Spinlet {
    pub fn new(store: Store<CommandCtx>, command: Command, instance: Instance) -> Self {
        Spinlet {
            store,
            command,
            instance,
        }
    }

    pub async fn run(&mut self) -> Result<Result<(), ()>> {
        self.command.call_run(&mut self.store).await
    }
}

impl Spinlet {
    pub async fn load(path: impl AsRef<Path>, dir: impl AsRef<Path>) -> Result<Self> {
        let ctx = CommandCtx::spawn_in(&path, dir)?;
        let mut store = Store::new(&ENGINE, ctx);
        let mut linker = Linker::new(&ENGINE);

        wasi::command::add_to_linker(&mut linker)?;

        let component = Component::from_file(&ENGINE, &path)?;

        let (command, instance) =
            Command::instantiate_async(&mut store, &component, &linker).await?;

        Ok(Spinlet::new(store, command, instance))
    }
}
