use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

/// You can also use this to run a command after an event.
/// 
/// Only built-in commands are supported.
/// 
/// ```toml
/// [hook.build.after]
/// enabled = true
/// ```
#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct AfterHook {
    /// Enable the after hook.
    #[serde(default)]
    enabled: bool
}

impl AfterHook {
    pub fn enabled(&self) -> bool {
        self.enabled
    }
}