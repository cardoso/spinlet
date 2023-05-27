use serde::{Serialize, Deserialize};
use wasmtime_wasi::preview2::WasiCtxBuilder;


mod var;

use var::VarAccess;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EnvAccess {
    #[serde(default)]
    var: Vec<VarAccess>,
}

impl EnvAccess {
    pub fn provide(&self, mut ctx: WasiCtxBuilder) -> Result<WasiCtxBuilder, std::env::VarError> {
        for var in &self.var {
            ctx = var.provide(ctx)?;
        }
        Ok(ctx)
    }
}
