mod error;

use wasmtime_wasi::preview2::{Table, WasiCtx, WasiCtxBuilder};
use wasmtime::Result;

use wasmtime_wasi::preview2::WasiView;
use error::ContextError;

pub struct Context {
    table: Table,
    wasi: WasiCtx,
}

impl Context {
    pub fn new(ctx: WasiCtxBuilder) -> Result<Self, ContextError> {
        let mut table = Table::new();
        let wasi = ctx.build(&mut table)?;
        Ok(Self { table, wasi })
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