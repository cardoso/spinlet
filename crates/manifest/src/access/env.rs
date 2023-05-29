use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

pub mod var;
pub mod args;

pub use var::VarAccess;
pub use args::ArgsAccess;

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct EnvAccess {
    /// Whether or not to allow access to environment variables.
    #[serde(default)]
    var: VarAccess,
    /// Whether or not to allow access to the command line arguments.
    #[serde(default)]
    args: ArgsAccess,
}

impl EnvAccess {
    pub fn var(&self) -> &VarAccess {
        &self.var
    }

    pub fn args(&self) -> &ArgsAccess {
        &self.args
    }
}
