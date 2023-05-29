use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct StderrAccess {
    /// Whether or not to allow access to the standard error stream.
    #[serde(default)]
    enabled: bool,
}

impl StderrAccess {
    pub fn enabled(&self) -> bool {
        self.enabled
    }
}