use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct StdinAccess {
    /// Whether or not to allow access to the standard input stream.
    #[serde(default)]
    enabled: bool,
}

impl StdinAccess {
    pub fn enabled(&self) -> bool {
        self.enabled
    }
}