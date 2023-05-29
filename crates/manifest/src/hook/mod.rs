pub mod before;
pub mod after;

use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use self::{before::BeforeHook, after::AfterHook};

/// You can also use this to run a command before and after the build.
/// 
/// ```toml
/// [hook.build.before]
/// enabled = true
/// [hook.build.after]
/// enabled = true
/// ```
#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct Hook {
    /// Run your spinlet before the build.
    #[serde(default)]
    before: BeforeHook,
    /// Run your spinlet after the build.
    #[serde(default)]
    after: AfterHook,
}

impl Hook {
    pub fn before(&self) -> &BeforeHook {
        &self.before
    }

    pub fn after(&self) -> &AfterHook {
        &self.after
    }

    pub fn enabled(&self) -> bool {
        self.before.enabled() || self.after.enabled()
    }
}