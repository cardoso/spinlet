use std::path::Path;

use serde::{Serialize, Deserialize};
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};
use wasmtime_wasi::preview2::WasiCtxBuilder;

pub mod error;
pub mod access;

use access::Access;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(default)]
    access: Access,
}

impl Manifest {
    pub fn provide(&self, mut ctx: WasiCtxBuilder) -> std::result::Result<WasiCtxBuilder, error::ManifestError> {
        ctx = self.access.provide(ctx)?;
        Ok(ctx)
    }
}

impl Manifest {

    pub async fn load(path: impl AsRef<Path>) -> std::result::Result<Self, error::ManifestError> {
        let path = path.as_ref().with_extension("toml");
        if !path.exists() {
            let capabilities = Manifest::default();
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
}