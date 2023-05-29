use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct VarAccess {
    #[serde(default)]
    /// The keys of the environment variables that are allowed to be accessed.
    keys: Vec<String>
}

impl VarAccess {
    pub fn keys(&self) -> &[String] {
        &self.keys
    }
}