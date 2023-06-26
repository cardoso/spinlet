pub use std::env;
use std::path::{Path, PathBuf};
use anyhow::Result;
use toml_edit::Document;

#[derive(Debug)]
pub struct Workspace {
    root: PathBuf,
    current: PathBuf,
}

impl Workspace {
    pub fn get() -> Self {
        Workspace {
            root: "/".into(),
            current: "".into(),
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
        Ok(std::fs::read_dir(self.current())?.flat_map(|entry| {
            entry.map(|entry| entry.path())
        }).collect::<Vec<_>>())
    }

    pub fn cd(&mut self, dir: impl AsRef<Path>) -> String {
        for component in dir.as_ref().components() {
            use std::path::Component;
            match component {
                Component::Normal(name) => {
                    self.current.push(name);
                },
                Component::RootDir => {
                    self.current = self.root.clone();
                },
                Component::CurDir => {
                    
                },
                Component::ParentDir => {
                    self.current.pop();
                },
                Component::Prefix(_) => {},
            }
        }
        self.current.display().to_string()
    }

    pub fn pwd(&self) -> Result<String> {
        Ok(self.current().display().to_string())
    }

    pub fn cat(&self, file: impl AsRef<Path>) -> Result<String> {
        let path = self.current().join(file);
        Ok(std::fs::read_to_string(path)?)
    }

    pub fn toml(&self, file: impl AsRef<Path>) -> Result<Document> {
        let path = self.current().join(file);
        let content = std::fs::read_to_string(path)?;
        Ok(content.parse::<Document>()?)
    }
}

impl Default for Workspace {
    fn default() -> Self {
        Self::get()
    }
}

