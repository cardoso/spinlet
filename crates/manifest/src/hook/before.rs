use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

/// You can also use this to run a command before an event.
/// 
/// Only built-in commands are supported.
/// 
/// ```toml
/// [hook.build.before]
/// enabled = true
/// ```
#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct BeforeHook {
    /// Enable the before hook.
    #[serde(default)]
    enabled: bool
}

impl BeforeHook {
    pub fn enabled(&self) -> bool {
        self.enabled
    }
}