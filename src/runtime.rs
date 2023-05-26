use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use wasmtime_wasi::{
    Dir, 
    ambient_authority, 
    preview2::{
        WasiCtxBuilder,
        DirPerms,
        FilePerms,
        pipe::{ReadPipe, WritePipe},
    }
};



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
        let path = ".";
        let workspace = match open_dir(path) {
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

fn open_dir(path: &str) -> std::result::Result<Dir, std::io::Error> {
    Dir::open_ambient_dir(path, ambient_authority())
}
