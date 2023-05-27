use std::path::{PathBuf, Path};
use serde::{Serialize, Deserialize};
use wasmtime_wasi::{preview2::{WasiCtxBuilder, DirPerms, FilePerms}, Dir, ambient_authority};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DirAccess {
    path: PathBuf,
    #[serde(default)]
    read: bool,
    #[serde(default)]
    mutate: bool
}

impl DirAccess {
    pub fn provide(&self, ctx: WasiCtxBuilder) -> std::io::Result<WasiCtxBuilder> {
        let file_perms = FilePerms::empty();
        let perms = match (self.read, self.mutate) {
            (true, true) => DirPerms::READ | DirPerms::MUTATE,
            (true, false) => DirPerms::READ,
            (false, true) => DirPerms::MUTATE,
            (false, false) => DirPerms::empty(),
        };

        let ambient_authority = ambient_authority();
        let dir = Dir::open_ambient_dir(&self.path, ambient_authority)?;
        let path = Path::new("/").join(&self.path);
        Ok(ctx.push_preopened_dir(dir, perms, file_perms, path.display().to_string()))
    }
}