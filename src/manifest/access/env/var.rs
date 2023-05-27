use std::env::VarError;

use serde::{Serialize, Deserialize};
use wasmtime_wasi::preview2::WasiCtxBuilder;


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VarAccess {
    key: String
}


impl VarAccess {
    pub fn provide(&self, ctx: WasiCtxBuilder) -> Result<WasiCtxBuilder, VarError> {
        let value = std::env::var(&self.key)?;
        Ok(ctx.push_env(&self.key, value))
    }
}