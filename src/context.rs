use std::path::Path;

use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::{AsyncWriteExt, AsyncReadExt}};
use anyhow::Result;
use wasmtime_wasi::preview2::{Table, WasiCtx, WasiCtxBuilder};

use crate::runtime::Access;

pub use wasmtime_wasi::preview2::WasiView;


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Capabilities {
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    access: Access,
}

impl Capabilities {
    pub async fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().with_extension("toml");
        if !path.exists() {
            let capabilities = Capabilities::default();
            let mut file = File::create(path).await?;
            let contents = toml::to_string(&capabilities)?;
            file.write_all(contents.as_bytes()).await?;
            Ok(capabilities)
        } else {
            let mut file = File::open(path).await?;
            let mut contents = String::new();
            file.read_to_string(&mut contents).await?;
            Ok(toml::from_str(&contents)?)
        }
    }

    pub fn push(&self, ctx: WasiCtxBuilder) -> WasiCtxBuilder {
        self.access.push(ctx)
    }
}


pub struct Context {
    table: Table,
    wasi: WasiCtx,
}

impl Context {
    pub fn new(capabilities: Capabilities) -> Result<Self> {
        let mut table = Table::new();
        let ctx = WasiCtxBuilder::new();
        let ctx = capabilities.push(ctx);
        let wasi = ctx.build(&mut table)?;
        Ok(Context { table, wasi })
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