

use std::{path::{Path, PathBuf}, collections::HashMap};
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;


#[derive(Serialize, Deserialize, JsonSchema, Debug)]
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
}
