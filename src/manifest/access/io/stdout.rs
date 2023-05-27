use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use wasmtime_wasi::preview2::{pipe::WritePipe, WasiCtxBuilder};

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct StdoutAccess {
    #[serde(default)]
    enabled: bool,
}

impl StdoutAccess {
    pub fn provide(&self, ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        if self.enabled {
            ctx.set_stdout(WritePipe::new(std::io::stdout()))
        } else {
            ctx.set_stdout(WritePipe::new(std::io::sink()))
        }
    }
}