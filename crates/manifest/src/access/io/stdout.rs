use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct StdoutAccess {
    /// Whether or not to allow access to the standard output stream.
    #[serde(default)]
    enabled: bool,
}

impl StdoutAccess {
    pub fn enabled(&self) -> bool {
        self.enabled
    }
}