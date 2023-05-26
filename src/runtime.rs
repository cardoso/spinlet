use anyhow::Result;
use serde::{Serialize, Deserialize};
use tokio::{fs::File, io::AsyncWriteExt};
use tokio::io::AsyncReadExt;

use std::path::{Path, PathBuf};
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store, WasmBacktraceDetails,
};

use wasmtime_wasi::preview2::{
    pipe::ReadPipe,
    pipe::WritePipe,
    wasi::command::Command,
    Table,
    WasiCtx,
    WasiView,
    WasiCtxBuilder,
    DirPerms,
    FilePerms,
    stdio,
    
};


#[derive(Serialize, Deserialize)]
pub struct FileAccess {
    path: PathBuf,
    read: bool,
    write: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Capabilities {
    stdin: bool,
    stdout: bool,
    stderr: bool,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    env: Vec<String>,
    #[serde(default)]
    access: Vec<FileAccess>,
}

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            stdin: false,
            stdout: false,
            stderr: false,
            args: Vec::new(),
            env: Vec::new(),
            access: Vec::new(),
        }
    }
}

impl Capabilities {
    pub fn build(self, table: &mut Table) -> Result<WasiCtx> {
        let mut ctx = WasiCtxBuilder::new();
        
        ctx = self.stdin(ctx);

        ctx = self.stdout(ctx);

        ctx = self.stderr(ctx);

        ctx = self.env(ctx);

        ctx = self.files(ctx);

        ctx.build(table)
    }

    fn files(self, mut ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        let authority = cap_std::ambient_authority();
        
        for access in self.access {
            let path = &access.path.display();
            match cap_std::fs::Dir::open_ambient_dir(&access.path, authority) {
                Ok(file) => {
                    log::info!("file: {path}");
                    ctx = ctx.push_preopened_dir(file, DirPerms::all(), FilePerms::all(), access.path.display().to_string())
                },
                Err(error) => {
                    log::warn!("file: {path} opened: {error}");
                }
            }
        }

        ctx
    }

    fn env(&self, mut ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        for key in &self.env {
            log::info!("env: {}", key);
            if let Ok(value) = std::env::var(&key) {
                ctx = ctx.push_env(key, value);
            } else {
                log::warn!("env: {} not found", key);
            }
        }

        ctx
    }

    fn stderr(&self, mut ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        if self.stderr {
            log::info!("stderr: inherited");
            ctx = ctx.set_stderr(stdio::stderr());
        } else {
            log::info!("stderr: sink");
            ctx = ctx.set_stderr(WritePipe::new(std::io::sink()));
        }
        
        ctx
    }

    fn stdout(&self, mut ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        if self.stdout {
            log::info!("stdout: inherited");
            ctx = ctx.set_stdout(stdio::stdout());
        } else {
            log::info!("stdout: sink");
            ctx = ctx.set_stdout(WritePipe::new(std::io::sink()));
        }

        ctx
    }

    fn stdin(&self, mut ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        if self.stdin {
            log::info!("stdin: inherited");
            ctx = ctx.set_stdin(stdio::stdin());
        } else {
            log::info!("stdin: empty");
            ctx = ctx.set_stdin(ReadPipe::new(std::io::empty()));
        }

        ctx
    }
}

pub struct Context {
    table: Table,
    wasi: WasiCtx,
}

impl Context {

    pub async fn load(path: impl AsRef<Path>) -> Result<Self> {
        log::info!("loading: {}", path.as_ref().display());
        let path = path.as_ref().with_extension("toml");
        if !path.exists() {
            let capabilities = Capabilities::default();
            let mut file = File::create(&path).await?;
            let buf = toml::to_string_pretty(&capabilities)?;
            file.write_all(buf.as_bytes()).await?;
            log::info!("created: {}", path.display());
        }
        let mut file = File::open(path).await?;
        let mut buf = String::new();
        file.read_to_string(&mut buf).await?;
        let capabilities = toml::from_str::<Capabilities>(&buf).unwrap_or_default();
        let mut table = Table::new();
        let wasi = capabilities.build(&mut table)?;
        Ok(Context { table, wasi })
    }
}

impl WasiView for Context {
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
    store: Store<Context>,
    command: Command,
}

impl Spinlet {
    pub fn new(store: Store<Context>, command: Command) -> Self {
        Spinlet {
            store,
            command,
        }
    }

    pub async fn load(path: impl AsRef<Path>) -> Result<Self> {
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
        let ctx = Context::load(path.as_ref()).await?;
        let mut store = Store::new(&engine, ctx);
        let mut linker = Linker::new(&engine);

        wasmtime_wasi::preview2::wasi::command::add_to_linker(&mut linker)?;

        let component = Component::from_file(&engine, path.as_ref().with_extension("wasm"))?;

        let (command, _) =
            Command::instantiate_async(&mut store, &component, &linker).await?;

        Ok(Spinlet { store, command })
    }

    pub async fn run(self) -> Result<()> {
        match self.command.call_run(self.store).await {
            Ok(result) => match result {
                Ok(()) => Ok(()),
                Err(()) => Err(anyhow::anyhow!("command exited with error")),
            },
            Err(error) => Err(error),
        }
    }
}
