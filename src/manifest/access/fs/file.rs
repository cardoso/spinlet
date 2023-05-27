use std::path::PathBuf;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use wasmtime_wasi::{preview2::{WasiCtxBuilder, DirPerms, FilePerms}, Dir, ambient_authority};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FileAccess {
    path: PathBuf,
    #[serde(default)]
    read: bool,
    #[serde(default)]
    write: bool,
}

impl FileAccess {
    pub fn provide(&self, ctx: WasiCtxBuilder) -> std::io::Result<WasiCtxBuilder> {
        let perms = DirPerms::empty();
        let file_perms = match (self.read, self.write) {
            (true, true) => FilePerms::READ | FilePerms::WRITE,
            (true, false) => FilePerms::READ,
            (false, true) => FilePerms::WRITE,
            (false, false) => FilePerms::empty(),
        };

        let ambient_authority = ambient_authority();
        let path = self.path.display().to_string();
        let dir = Dir::open_ambient_dir(&path, ambient_authority)?;

        Ok(ctx.push_preopened_dir(dir, perms, file_perms, path))
    }
}