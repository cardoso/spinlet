use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct ArgsAccess {
    /// Whether or not to allow access to the command line arguments.
    #[serde(default)]
    enabled: bool
}

impl ArgsAccess {
    pub fn enabled(&self) -> bool {
        self.enabled
    }
}