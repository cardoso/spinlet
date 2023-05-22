use anyhow::Result;
use cap_std::{ambient_authority, fs::Dir};
use std::{path::Path};
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store, WasmBacktraceDetails,
};
use wasmtime_wasi::preview2::{
    wasi,
    wasi::command::Command,
    Table, WasiCtx, WasiCtxBuilder, WasiView, DirPerms, FilePerms,
};

lazy_static::lazy_static! {
    static ref ENGINE: Engine = {
        let mut config = Config::new();
        config.wasm_backtrace_details(WasmBacktraceDetails::Enable);
        config.wasm_component_model(true);
        config.async_support(true);
        Engine::new(&config).unwrap()
    };
}

pub struct CommandCtx {
    table: Table,
    wasi: WasiCtx,
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
}

impl Spinlet {
    pub fn new(store: Store<CommandCtx>, command: Command) -> Self {
        Spinlet {
            store,
            command,
        }
    }

    pub async fn run(&mut self) -> Result<Result<(), ()>> {
        self.command.call_run(&mut self.store).await
    }
}

impl Spinlet {
    pub async fn load(file: impl AsRef<Path>, workspace: impl AsRef<Path>) -> Result<Self> {
        let mut table = Table::new();
        let args = std::env::args().collect::<Vec<_>>();
        let env = std::env::vars().filter(|(k, _)| k.starts_with("SPIN")).collect::<Vec<_>>();

        let (dir, dir_perms, file_perms) = open_dir(workspace)?;

        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .set_env(&env)
            .set_args(&args)
            .push_preopened_dir(dir, dir_perms, file_perms, "/workspace")
            .build(&mut table)?;

        let mut store = Store::new(&ENGINE, CommandCtx { table, wasi });
        let mut linker = Linker::new(&ENGINE);

        wasi::command::add_to_linker(&mut linker)?;

        let component = Component::from_file(&ENGINE, file)?;

        let (command, _instance) =
            Command::instantiate_async(&mut store, &component, &linker).await?;
        


        Ok(Spinlet::new(store, command))
    }
}

fn open_dir(path: impl AsRef<Path>) -> Result<(Dir, DirPerms, FilePerms), anyhow::Error> {
    let dir = cap_std::fs::Dir::open_ambient_dir(path, ambient_authority())?;
    let dir_perms = DirPerms::all();
    let file_perms = FilePerms::all();
    let dir = dir.open_dir(".")?;
    Ok((dir, dir_perms, file_perms))
}
