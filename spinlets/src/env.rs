pub use std::env::*;
use std::path::{Path, PathBuf};
use anyhow::Result;

pub fn get_current_dir() {
    let current_dir = std::env::current_dir().unwrap();
    println!("The current directory is {}", current_dir.display());
}

#[derive(Debug)]
pub struct Workspace {
    root: PathBuf,
    current: PathBuf,
}

impl Workspace {
    pub fn get() -> Self {
        Workspace {
            root: "/".into(),
            current: "/".into(),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn current(&self) -> &Path {
        &self.current
    }
}

impl Workspace {
    pub fn ls(&self) -> Result<Vec<PathBuf>> {
        Ok(std::fs::read_dir(&self.current())?.flat_map(|entry| {
            entry.map(|entry| entry.path())
        }).collect::<Vec<_>>())
    }

    pub fn cd(&mut self, dir: impl AsRef<Path>) -> Result<String> {
        self.current = dir.as_ref().to_path_buf();
        Ok("".into())
    }

    pub fn pwd(&self) -> Result<String> {
        Ok(self.current().display().to_string())
    }

    pub fn cat(&self, file: impl AsRef<Path>) -> Result<String> {
        let path = self.current().join(file);
        Ok(std::fs::read_to_string(path)?)
    }
}

impl Default for Workspace {
    fn default() -> Self {
        Self::get()
    }
}
