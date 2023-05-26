use anyhow::Result;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use tokio::{fs::File, io::AsyncWriteExt};
use tokio::io::AsyncReadExt;
use wasmtime::CallHook;

pub use wasmtime::{
    Config,
    Engine,
    Store,
    WasmBacktraceDetails,
    AsContext,
    AsContextMut,
    CacheStore,
    CallHookHandler,
    HostAbi,
    ResourceLimiter,
    ResourceLimiterAsync,
    WasmParams, WasmResults,
    WasmRet,
    WasmTy,
    IntoFunc,
    LinearMemory,
    MemoryCreator,
    component::{
        Component,
        Linker,
        LinkerInstance,
        ComponentNamedList,
        ComponentType,
        Lift,
        Lower
    }
};
pub use wasmtime_wasi::preview2::{
    pipe::ReadPipe,
    pipe::WritePipe,
    Table,
    WasiCtx,
    WasiView,
    WasiCtxBuilder,
    DirPerms,
    FilePerms,
    stdio,
    clocks::{WasiMonotonicClock, WasiWallClock},
    stream::{InputStream, OutputStream, TableStreamExt},
    wasi::{
        command::Command,
        filesystem::Host,
        streams::Host as _,
        random::Host as _,
        poll::Host as _,
        wall_clock::Host as _,
        monotonic_clock::Host as _,
        timezone::Host as _,
        environment::Host as _,
        preopens::Host as _,
        exit::Host as _,
        stderr::Host as _,
        stdin::Host as _,
        stdout::Host as _,
        insecure_random_seed::Host as _,
        command::{
            insecure_random::Host as _,
            insecure_random_seed::Host as _,
            instance_network::Host as _,
            ip_name_lookup::Host as _,
            network::Host as _,
            tcp::Host as _,
            udp::Host as _,
            udp_create_socket::Host as _,
            tcp_create_socket::Host as _
    }
}};
use std::path::{Path, PathBuf};


#[derive(Debug, Serialize, Deserialize)]
pub struct FileAccess {
    path: PathBuf,
    #[serde(default)]
    read: bool,
    #[serde(default)]
    write: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DirAccess {
    path: PathBuf,
    #[serde(default)]
    read: bool,
    #[serde(default)]
    mutate: bool
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Access {
    #[serde(default)]
    file: Vec<FileAccess>,
    #[serde(default)]
    dir: Vec<DirAccess>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Capabilities {
    #[serde(default)]
    stdin: bool,
    #[serde(default)]
    stdout: bool,
    #[serde(default)]
    stderr: bool,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    env: Vec<String>,
    #[serde(default)]
    access: Access,
}

impl Capabilities {
    pub async fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().with_extension("toml");
        if !path.exists() {
            let capabilities = Capabilities::default();
            let mut file = File::create(path).await?;
            let contents = toml::to_string(&capabilities)?;
            file.write_all(contents.as_bytes()).await?;
            Ok(capabilities)
        } else {
            let mut file = File::open(path).await?;
            let mut contents = String::new();
            file.read_to_string(&mut contents).await?;
            Ok(toml::from_str(&contents)?)
        }
    }

    pub fn push_all(&self, mut ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        ctx = self.push_stdin(ctx);
        ctx = self.push_stdout(ctx);
        ctx = self.push_stderr(ctx);
        ctx = self.push_env(ctx);
        ctx = self.push_access(ctx);
        ctx
    }

    fn push_access(&self, mut ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        let authority = cap_std::ambient_authority();
        let workspace = match cap_std::fs::Dir::open_ambient_dir(".", authority) {
            Ok(dir) => dir,
            Err(p) => {
                tracing::error!("failed to open workspace directory: {}", p);
                return ctx;
            }
        };
        
            for access in &self.access.dir {
                let path = &access.path.display();
                match workspace.open_dir(&access.path) {
                    Ok(dir) => {
                        tracing::info!("dir: {path}");
                        let dir_perms = if access.read {
                            DirPerms::READ
                        } else {
                            DirPerms::empty()
                        } | if access.mutate {
                            DirPerms::MUTATE
                        } else {
                            DirPerms::empty()
                        };

                        let file_perms = if access.read {
                            FilePerms::READ
                        } else {
                            FilePerms::empty()
                        } | if access.mutate {
                            FilePerms::WRITE
                        } else {
                            FilePerms::empty()
                        };

                        ctx = ctx.push_preopened_dir(dir, dir_perms, file_perms, access.path.display().to_string())
                    },
                    Err(error) => {
                        tracing::warn!("dir: {path} opened: {error}");
                    }
                }
            }

            for access in &self.access.file {
                let path = &access.path.display();
                match workspace.open_dir(&access.path) {
                    Ok(dir) => {
                        tracing::info!("file: {path}");
                        let dir_perms = if access.read {
                            DirPerms::READ
                        } else {
                            DirPerms::empty()
                        } | if access.write {
                            DirPerms::MUTATE
                        } else {
                            DirPerms::empty()
                        };

                        let file_perms = if access.read {
                            FilePerms::READ
                        } else {
                            FilePerms::empty()
                        } | if access.write {
                            FilePerms::WRITE
                        } else {
                            FilePerms::empty()
                        };

                        ctx = ctx.push_preopened_dir(dir, dir_perms, file_perms, access.path.display().to_string())
                    },
                    Err(error) => {
                        tracing::warn!("file: {path} opened: {error}");
                    }
                }
        }

        ctx
    }

    fn push_env(&self, mut ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        for key in &self.env {
            tracing::info!("env: {}", key);
            if let Ok(value) = std::env::var(&key) {
                ctx = ctx.push_env(key, value);
            } else {
                tracing::warn!("env: {} not found", key);
            }
        }

        ctx
    }

    fn push_stderr(&self, mut ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        if self.stderr {
            tracing::info!("stderr: inherited");
            ctx = ctx.set_stderr(stdio::stderr());
        } else {
            tracing::info!("stderr: sink");
            ctx = ctx.set_stderr(WritePipe::new(std::io::sink()));
        }
        
        ctx
    }

    fn push_stdout(&self, mut ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        if self.stdout {
            tracing::info!("stdout: inherited");
            ctx = ctx.set_stdout(stdio::stdout());
        } else {
            tracing::info!("stdout: sink");
            ctx = ctx.set_stdout(WritePipe::new(std::io::sink()));
        }

        ctx
    }

    fn push_stdin(&self, mut ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        if self.stdin {
            tracing::info!("stdin: inherited");
            ctx = ctx.set_stdin(stdio::stdin());
        } else {
            tracing::info!("stdin: empty");
            ctx = ctx.set_stdin(ReadPipe::new(std::io::empty()));
        }

        ctx
    }
}

pub struct Context {
    table: Table,
    wasi: WasiCtx,
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("table", &self.table)
            .finish_non_exhaustive()
    }
}

impl Context {
    pub async fn load(path: impl AsRef<Path>) -> Result<Self> {
        let capabilities = Capabilities::load(&path).await?;
        Self::from_file(capabilities)
    }

    #[tracing::instrument]
    pub fn from_file(capabilities: Capabilities) -> Result<Self> {
        let mut table = Table::new();
        let ctx = WasiCtxBuilder::new();
        let wasi = capabilities.push_all(ctx);
        let wasi = wasi.build(&mut table)?;
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

#[derive(Debug)]
pub struct SpinletHook;

#[async_trait]
impl CallHookHandler<Context> for SpinletHook {
    #[tracing::instrument]
    async fn handle_call_event(&self, t: &mut Context, ch: CallHook) -> Result<()> {
        if ch.entering_host() {
            tracing::info!("entering host");
        }

        if ch.exiting_host() {
            tracing::info!("exiting host");
        }

        match ch {
            CallHook::CallingHost => {
                tracing::info!("calling host");
            },
            CallHook::CallingWasm => {
                tracing::info!("calling wasm");
            },
            CallHook::ReturningFromHost => {
                tracing::info!("returning from host");
            },
            CallHook::ReturningFromWasm => {
                tracing::info!("returning from wasm");
            }
        }
        Ok(())
    }

}

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
    pub async fn load(path: impl AsRef<Path>, context: Context) -> Result<Self> {
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

        wasmtime_wasi::preview2::wasi::command::add_to_linker(&mut linker)?;
        
        let path = path.as_ref().with_extension("wasm");
        let component = Component::from_file(&engine, path)?;
        let (command, _instance) = Command::instantiate_async(&mut store, &component, &linker).await?;
        
        Ok(Spinlet { store, command })
    }

    pub fn store(&self) -> &Store<Context> {
        &self.store
    }

    pub async fn run(mut self) -> Result<Result<Self, Self>> {
        match self.command.call_run(&mut self.store).await {
            Ok(result) => match result {
                Ok(()) => {
                    tracing::info!("success");
                    Ok(Ok(self))
                },
                Err(()) => {
                    tracing::info!("failed");
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
