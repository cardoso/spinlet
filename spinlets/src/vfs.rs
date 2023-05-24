use std::{path::{Path, PathBuf}, fs::DirEntry};
use anyhow::Result;

#[derive(Debug)]
pub struct Vfs {
    root: PathBuf,
    current: PathBuf,
}

impl Vfs {
    pub fn new() -> Self {
        Vfs {
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

impl Vfs {
    pub fn ls(&self) -> Result<Vec<PathBuf>> {
        Ok(std::fs::read_dir(&self.current())?.flat_map(|entry| {
            entry.map(|entry| entry.path())
        }).collect::<Vec<_>>())
    }

    pub fn cd(&mut self, dir: impl AsRef<Path>) -> Result<()> {
        Ok(self.current = dir.as_ref().to_path_buf())
    }

    pub fn pwd(&self) -> Result<String> {
        Ok(self.current().to_string_lossy().to_string())
    }   
}

impl Default for Vfs {
    fn default() -> Self {
        Self::new()
    }
}

