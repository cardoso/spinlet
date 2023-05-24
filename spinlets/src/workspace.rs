use std::path::{Path, PathBuf};
use anyhow::Result;

#[derive(Debug)]
pub struct Workspace {
    root: PathBuf,
    current: PathBuf,
}

impl Workspace {
    pub fn new() -> Self {
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
}

impl Default for Workspace {
    fn default() -> Self {
        Self::new()
    }
}

