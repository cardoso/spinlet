use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

pub mod fs;
pub mod env;
pub mod io;

pub use env::EnvAccess;
pub use io::IoAccess;
pub use fs::FsAccess;

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct Access {
    /// Access to file system resources.
    #[serde(default)]
    fs: FsAccess,
    /// Access to environment resourcews.
    #[serde(default)]
    env: EnvAccess,
    /// Access to I/O resources.
    #[serde(default)]
    io: IoAccess
}

impl Access {
    pub fn fs(&self) -> &FsAccess {
        &self.fs
    }

    pub fn env(&self) -> &EnvAccess {
        &self.env
    }

    pub fn io(&self) -> &IoAccess {
        &self.io
    }
}