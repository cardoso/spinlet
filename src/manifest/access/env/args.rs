use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use wasmtime_wasi::preview2::WasiCtxBuilder;


#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct ArgsAccess {
    #[serde(default)]
    enabled: bool
}

impl ArgsAccess {
    pub fn provide(&self, mut ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        if !self.enabled { return ctx }
        for arg in std::env::args() {
            ctx = ctx.push_arg(arg);
        }
        ctx
    }
}