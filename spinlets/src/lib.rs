use anyhow::Context;
pub use anyhow::Result;
pub use console::Term;
pub use dialoguer::*;
pub use console::*;
pub use indicatif::*;
use std::env::Args;
pub use std::io::BufRead;
pub use std::io::BufWriter;
pub use std::io::Read;
use std::io::StderrLock;
use std::io::StdinLock;
use std::io::StdoutLock;
pub use std::io::Write;
pub use is_terminal::*;

#[derive(Debug)]
pub struct Spin<'spin> {
    version: String,
    workspace: String,
    args: Args,
    stdin: StdinLock<'spin>,
    stdout: StdoutLock<'spin>,
    stderr: StderrLock<'spin>,
}

impl<'spin> Spin<'spin> {
    pub fn get() -> anyhow::Result<Self> {
        Ok(Self {
            version: std::env::var("VERSION")?,
            workspace: std::env::var("WORKSPACE")?,
            args: std::env::args(),
            stdin: std::io::stdin().lock(),
            stdout: std::io::stdout().lock(),
            stderr: std::io::stderr().lock(),
        })
    }

    pub fn write(&mut self, s: impl AsRef<str>) -> Result<()> {
        let bytes = s.as_ref().as_bytes();
        self.stderr.write_all(bytes)?;
        self.stderr.flush()?;
        Ok(())
    }

    pub fn writeln(&mut self, s: impl AsRef<str>) -> Result<()> {
        let bytes = s.as_ref().as_bytes();
        self.stderr.write_all(bytes)?;
        self.stderr.write_all(b"\n")?;
        Ok(())
    }

    pub fn read_line(&mut self) -> Result<String> {
        let buffer = &mut String::new();
        self.stdin.read_line(buffer)?;
        Ok(buffer)
    }
}
