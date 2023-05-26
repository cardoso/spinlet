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
    WasmParams,
    WasmResults,
    WasmRet,
    WasmTy,
    IntoFunc,
    LinearMemory,
    MemoryCreator,
    Caller,
    component::{
        Component,
        Linker,
        LinkerInstance,
        ComponentNamedList,
        ComponentType,
        Lift,
        Lower,
    }
};

use wasmtime_wasi::Dir;
pub use wasmtime_wasi::preview2::{
    RngCore,
    InputStream,
    OutputStream,
    WasiMonotonicClock,
    WasiWallClock,
    Table,
    WasiCtx,
    WasiView,
    WasiCtxBuilder,
    DirPerms,
    FilePerms,
    pipe::{ReadPipe, WritePipe},
    stdio::{Stderr, Stdin, Stdout},
    stream::TableStreamExt,
    wasi::{
        filesystem::Host as FilesystemHost,
        streams::Host as StreamsHost,
        random::Host as RandomHost,
        poll::Host as PollHost,
        wall_clock::Host as WallClockHost,
        monotonic_clock::Host as MonotonicClockHost,
        timezone::Host as TimezoneHOst,
        environment::Host as EnvironmentHost,
        preopens::Host as PreopensHost,
        exit::Host as ExitHost,
        stderr::Host as StderrHost,
        stdin::Host as StdinHost,
        stdout::Host as StdoutHost,
        insecure_random::Host as InsecureRandomHost,
        insecure_random_seed::Host as InsecureRandomSeedHost,
        command::{
            Command,
            instance_network::Host as InstanceNetworkHost,
            ip_name_lookup::Host as IpNameLookupHost,
            network::Host as NetworkHost,
            tcp::Host as TcpHost,
            udp::Host as UdpHost,
            udp_create_socket::Host as UdpCreateSocketHost,
            tcp_create_socket::Host as TcpCreateSocketHost,
        }
    }
};

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
pub struct EnvAccess {
    key: String,
}

impl EnvAccess {
    pub fn push(&self, mut ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        match std::env::var(&self.key) {
            Ok(value) => {
                tracing::info!("env: {key}={value}", key = self.key, value = value);
                ctx = ctx.push_env(&self.key, value);
            },
            Err(error) => {
                tracing::warn!("env: {key}={error}", key = self.key, error = error);
            }
        }

        ctx
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StdioAccess {
    #[serde(default)]
    stdin: bool,
    #[serde(default)]
    stdout: bool,
    #[serde(default)]
    stderr: bool,
}

impl StdioAccess {
    pub fn push(&self, mut ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        ctx = self.push_stdin(ctx);
        ctx = self.push_stdout(ctx);
        ctx = self.push_stderr(ctx);
        ctx
    }

    fn push_stderr(&self, mut ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        if self.stderr {
            tracing::info!("stderr: inherited");
            ctx = ctx.set_stderr(WritePipe::new(std::io::stderr()));
        } else {
            tracing::info!("stderr: sink");
            ctx = ctx.set_stderr(WritePipe::new(std::io::sink()));
        }
        
        ctx
    }

    fn push_stdout(&self, mut ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        if self.stdout {
            tracing::info!("stdout: inherited");
            ctx = ctx.set_stdout(WritePipe::new(std::io::stdout()));
        } else {
            tracing::info!("stdout: sink");
            ctx = ctx.set_stdout(WritePipe::new(std::io::sink()));
        }

        ctx
    }

    fn push_stdin(&self, mut ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        if self.stdin {
            tracing::info!("stdin: inherited");
            ctx = ctx.set_stdin(ReadPipe::new(std::io::stdin()));
        } else {
            tracing::info!("stdin: empty");
            ctx = ctx.set_stdin(ReadPipe::new(std::io::empty()));
        }

        ctx
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Access {
    #[serde(default)]
    file: Vec<FileAccess>,
    #[serde(default)]
    dir: Vec<DirAccess>,
    #[serde(default)]
    env: Vec<EnvAccess>,
    #[serde(default)]
    stdio: StdioAccess
}

impl Access {
    pub fn push(&self, mut ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        let authority = cap_std::ambient_authority();
        let workspace = match cap_std::fs::Dir::open_ambient_dir(".", authority) {
            Ok(dir) => dir,
            Err(p) => {
                tracing::error!("failed to open workspace directory: {}", p);
                return ctx;
            }
        };

        ctx = self.stdio.push(ctx);
        for env in &self.env {
            ctx = env.push(ctx);
        }
        ctx = self.push_dirs(&workspace, ctx);
        ctx = self.push_files(&workspace, ctx);
        ctx
    }

    fn push_dirs(&self, root: &Dir, mut ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        for access in &self.dir {
            let path = &access.path.display();
            match root.open_dir(&access.path) {
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

        ctx
    }
    fn push_files(&self, workspace: &Dir, mut ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        for access in &self.file {
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
        };

        ctx
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Capabilities {
    #[serde(default)]
    args: Vec<String>,
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

    pub fn push(&self, ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        self.access.push(ctx)
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
    pub fn new(capabilities: Capabilities) -> Result<Self> {
        let mut table = Table::new();
        let ctx = WasiCtxBuilder::new();
        let ctx = capabilities.push(ctx);
        let wasi = ctx.build(&mut table)?;
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

