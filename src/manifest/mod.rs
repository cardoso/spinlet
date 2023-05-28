use std::{path::Path, result::Result};

use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};
use wasmtime_wasi::preview2::WasiCtxBuilder;

mod error;
mod access;

use access::Access;

pub use error::ManifestError;

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct Manifest {
    #[serde(default)]
    access: Access,
}

impl Manifest {
    pub fn provide(&self, ctx: WasiCtxBuilder) -> Result<WasiCtxBuilder, ManifestError> {
        Ok(self.access.provide(ctx)?)
    }
}

impl Manifest {

    pub async fn load(path: impl AsRef<Path>) -> Result<Self, ManifestError> {
        let path = path.as_ref().with_extension("toml");
        if !path.exists() {
            let capabilities = Manifest::default();
            let mut file = File::create(path).await?;
            let contents = toml::to_string_pretty(&capabilities)?;
            file.write_all(contents.as_bytes()).await?;
            Ok(capabilities)
        } else {
            let mut file = File::open(path).await?;
            let mut contents = String::new();
            file.read_to_string(&mut contents).await?;
            Ok(toml::from_str(&contents)?)
        }
    }
}