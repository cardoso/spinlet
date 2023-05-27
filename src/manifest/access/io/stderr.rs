use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use wasmtime_wasi::preview2::{WasiCtxBuilder, pipe::WritePipe};

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct StderrAccess {
    #[serde(default)]
    enabled: bool,
}

impl StderrAccess {
    pub fn provide(&self, ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        if self.enabled {
            ctx.set_stderr(WritePipe::new(std::io::stderr()))
        } else {
            ctx.set_stderr(WritePipe::new(std::io::sink()))
        }
    }
}