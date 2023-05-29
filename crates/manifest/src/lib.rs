use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

pub mod access;
pub use access::Access;

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct Manifest {
    /// The access configuration for the application.
    #[serde(default)]
    access: Access,
}

impl Manifest {
    pub fn access(&self) -> &Access {
        &self.access
    }
}