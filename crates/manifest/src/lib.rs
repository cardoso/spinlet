use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

pub mod access;
pub use access::Access;

pub mod hook;
pub use hook::Hook;

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct Manifest {
    /// Hook configuration for the application.
    /// 
    /// If set your spinlet will not be available as a command, it will only run as a hook.
    #[serde(default)]
    hook: HashMap<String, Hook>,
    /// The access configuration for the application.
    /// 
    /// You should specify the minimum access level required to run the application.
    #[serde(default)]
    access: Access,
}

impl Manifest {
    pub fn access(&self) -> &Access {
        &self.access
    }
    
    pub fn hook(&self, name: &str) -> Option<&Hook> {
        self.hook.get(name)
    }
}