use serde::{Serialize, Deserialize};
use wasmtime_wasi::preview2::WasiCtxBuilder;

mod stdin;
mod stdout;
mod stderr;

use stdin::StdinAccess;
use stdout::StdoutAccess;
use stderr::StderrAccess;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IoAccess {
    #[serde(default)]
    stdin: StdinAccess,
    #[serde(default)]
    stdout: StdoutAccess,
    #[serde(default)]
    stderr: StderrAccess,
}

impl IoAccess {
    pub fn provide(&self, mut ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        ctx = self.stdin.provide(ctx);
        ctx = self.stdout.provide(ctx);
        ctx = self.stderr.provide(ctx);
        ctx
    }
}