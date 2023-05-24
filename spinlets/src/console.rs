use std::io::{Write, BufRead, Stderr, Stdout, Stdin, IsTerminal};

use anyhow::Result;

#[derive(Debug)]
pub struct Console {
    stdin: Stdin,
    stdout: Stdout,
    stderr: Stderr,
}

impl Console {
    pub fn new() -> Self {
        Self {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
            stderr: std::io::stderr(),
        }
    }

    pub fn print(&self, s: impl AsRef<str>) -> Result<()> {
        let mut stderr = self.stderr.lock();
        let bytes = s.as_ref().as_bytes();
        stderr.write_all(bytes)?;
        stderr.flush()?;
        Ok(())
    }

    pub fn print_line(&self, s: impl AsRef<str>) -> Result<()> {
        let mut stderr = self.stderr.lock();
        let bytes = s.as_ref().as_bytes();
        stderr.write_all(bytes)?;
        stderr.write_all(b"\n")?;
        Ok(())
    }
    
    pub fn read_line(&self) -> Result<String> {
        let mut stdin = self.stdin.lock();
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;
        Ok(buffer)
    }

    pub fn is_terminal(&self) -> bool {
        self.stdout.is_terminal()
    }
}