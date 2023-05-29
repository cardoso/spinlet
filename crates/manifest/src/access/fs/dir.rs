use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct DirAccess {
    /// The path to the directory relative to the workspace.
    path: String,
    /// Whether the directory can be read.
    #[serde(default)]
    read: bool,
    /// Whether the directory can be mutated.
    #[serde(default)]
    mutate: bool
}

impl DirAccess {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn read(&self) -> bool {
        self.read
    }

    pub fn mutate(&self) -> bool {
        self.mutate
    }

    pub fn read_mutate(&self) -> bool {
        self.read && self.mutate
    }
}