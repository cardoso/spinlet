use std::path::{Path, PathBuf};

use clap::Parser;
use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Parser, Serialize, Deserialize, Debug)]
pub struct Config {
    #[arg(default_value = ".spinlet")]
    root: String,
    #[arg(default_value = "etc")]
    etc: String,
    #[arg(default_value = "toml")]
    etc_ext: String,
    #[arg(default_value = "bin")]
    bin: String,
    #[arg(default_value = "wasm")]
    bin_ext: String,
    #[arg(default_value = "lib")]
    lib: String,
    #[arg(default_value = "wasm")]
    lib_ext: String,
}

impl Config {
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
        let bin = Path::new(&self.root).join(&self.bin);
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