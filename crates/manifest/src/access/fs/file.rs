use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FileAccess {
    /// The path to the file, relative to the workspace.
    path: String,
    /// Whether the file can be read.
    #[serde(default)]
    read: bool,
    /// Whether the file can be written to.
    #[serde(default)]
    write: bool,
}


impl FileAccess {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn read(&self) -> bool {
        self.read
    }

    pub fn write(&self) -> bool {
        self.write
    }
}