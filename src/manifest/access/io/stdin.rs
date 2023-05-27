use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use wasmtime_wasi::preview2::{WasiCtxBuilder, pipe::ReadPipe};

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct StdinAccess {
    #[serde(default)]
    enabled: bool,
}

impl StdinAccess {
    pub fn provide(&self, ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        if self.enabled {
            ctx.set_stdin(ReadPipe::new(std::io::stdin()))
        } else {
            ctx.set_stdin(ReadPipe::new(std::io::empty()))
        }
    }
}