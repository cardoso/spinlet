use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use wasmtime_wasi::preview2::WasiCtxBuilder;
mod var;
mod args;

use var::VarAccess;

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct EnvAccess {
    #[serde(default)]
    var: Vec<VarAccess>,
    #[serde(default)]
    args: args::ArgsAccess,
}

impl EnvAccess {
    pub fn provide(&self, mut ctx: WasiCtxBuilder) -> Result<WasiCtxBuilder, std::env::VarError> {
        ctx = self.args.provide(ctx);
        for var in &self.var {
            ctx = var.provide(ctx)?;
        }
        Ok(ctx)
    }
}
