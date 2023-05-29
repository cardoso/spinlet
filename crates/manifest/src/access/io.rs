use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

pub mod stdin;
pub mod stdout;
pub mod stderr;

pub use stdin::StdinAccess;
pub use stdout::StdoutAccess;
pub use stderr::StderrAccess;

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct IoAccess {
    /// Whether or not to allow access to the standard input stream.
    #[serde(default)]
    stdin: StdinAccess,
    /// Whether or not to allow access to the standard output stream.
    #[serde(default)]
    stdout: StdoutAccess,
    /// Whether or not to allow access to the standard error stream.
    #[serde(default)]
    stderr: StderrAccess,
}

impl IoAccess {
    pub fn stdin(&self) -> &StdinAccess {
        &self.stdin
    }

    pub fn stdout(&self) -> &StdoutAccess {
        &self.stdout
    }

    pub fn stderr(&self) -> &StderrAccess {
        &self.stderr
    }
}