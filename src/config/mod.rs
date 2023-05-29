

use std::{path::{Path, PathBuf}, collections::HashMap};
use serde::{Serialize, Deserialize};
use anyhow::Result;


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    name: String,
    version: String,
    repository: String,
    root: String,
    etc: String,
    etc_ext: String,
    bin: String,
    bin_ext: String,
    lib: String,
    lib_ext: String,
    alias: HashMap<String, String>,
}

impl Config {
    pub fn parse(str: &str) -> Result<Self> {
        Ok(toml::from_str(str)?)
    }

    pub fn root(&self) -> &str {
        &self.root
    }

    pub fn manifest(&self, spinlet: &str) -> PathBuf {
        Path::new(&self.root)
            .join(&self.etc)
            .join(spinlet)
            .with_extension(&self.etc_ext)
    }

    pub fn binary(&self, spinlet: &str) -> PathBuf {
        Path::new(&self.root)
            .join(&self.bin)
            .join(spinlet)
            .with_extension(&self.bin_ext)
    }

    pub fn list_spinlets(&self) -> Result<Vec<String>> {
        let bin = Path::new(&self.name).join(&self.bin);
        let mut spinlets = vec![];
        for entry in bin.read_dir()? {
            let Ok(entry) = entry else { continue };
            let path = entry.path();
            if !path.exists() { continue };
            if !path.is_file() { continue };
            let Some(ext) = path.extension() else { continue };
            if ext.to_string_lossy().eq(self.bin_ext.as_str()) { continue };
            let Some(stem) = path.file_stem() else { continue };
            let stem = stem.to_string_lossy();
            spinlets.push(stem.to_string());
        }
        Ok(spinlets)
    }
}
