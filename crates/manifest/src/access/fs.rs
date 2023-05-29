pub mod dir;
pub mod file;

use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

pub use dir::DirAccess;
pub use file::FileAccess;

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct FsAccess {
    #[serde(default)]
    /// A list of files that the program is allowed to access.
    file: Vec<FileAccess>,
    #[serde(default)]
    /// A list of directories that the program is allowed to access.
    dir: Vec<DirAccess>,
}

impl FsAccess {
    pub fn file(&self) -> &[FileAccess] {
        &self.file
    }

    pub fn dir(&self) -> &[DirAccess] {
        &self.dir
    }
}