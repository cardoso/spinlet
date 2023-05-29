use std::path::{PathBuf, Path};
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

pub mod parent;
pub mod child;
pub mod current;

pub use child::Child;
pub use parent::Parent;
pub use current::Current;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Config {
    #[serde(default = "default::name")]
    name: String,
    #[serde(default = "default::version")]
    version: String,
    #[serde(default = "default::repository")]
    repository: String,
    #[serde(default = "default::root")]
    root: String,
    #[serde(default = "default::etc")]
    etc: String,
    #[serde(default = "default::etc_ext")]
    etc_ext: String,
    #[serde(default = "default::bin")]
    bin: String,
    #[serde(default = "default::bin_ext")]
    bin_ext: String,
    #[serde(default = "default::lib")]
    lib: String,
    #[serde(default = "default::lib_ext")]
    lib_ext: String,
    #[serde(default)]
    parent: Parent,
    #[serde(default)]
    current: Current,
    #[serde(default)]
    child: Child,
}

impl Config {
    pub fn root(&self) -> &Path {
        Path::new(&self.root)
    }

    pub fn manifest(&self, spinlet: &str) -> PathBuf {
        Path::new(&self.root)
            .join(&self.etc)
            .join(spinlet)
            .with_extension(&self.etc_ext)
    }

    pub fn binary(&self, spinlet: &str) -> PathBuf {
        self.bin()
            .join(spinlet)
            .with_extension(&self.bin_ext)
    }

    pub fn bin(&self) -> PathBuf {
        self.root()
            .join(&self.bin)
    }

    pub fn etc(&self) -> PathBuf {
        self.root()
            .join(&self.etc)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn parent(&self) -> &Parent {
        &self.parent
    }

    pub fn child(&self) -> &Child {
        &self.child
    }

    pub fn current(&self) -> &Current {
        &self.current
    }
}

pub mod default {
    pub fn name() -> String {
        "spin".to_string()
    }

    pub fn version() -> String {
        "0.1.0".to_string()
    }

    pub fn repository() -> String {
        "https://github.com/cardoso/spinlet".to_string()
    }

    pub fn root() -> String {
        ".spinlet".to_string()
    }

    pub fn etc() -> String {
        "etc".to_string()
    }

    pub fn etc_ext() -> String {
        "toml".to_string()
    }

    pub fn bin() -> String {
        "bin".to_string()
    }

    pub fn bin_ext() -> String {
        "wasm".to_string()
    }

    pub fn lib() -> String {
        "lib".to_string()
    }

    pub fn lib_ext() -> String {
        "wasm".to_string()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: default::name(),
            version: default::version(),
            repository: default::repository(),
            root: default::root(),
            etc: default::etc(),
            etc_ext: default::etc_ext(),
            bin: default::bin(),
            bin_ext: default::bin_ext(),
            lib: default::lib(),
            lib_ext: default::lib_ext(),
            parent: Parent::default(),
            current: Current::default(),
            child: Child::default(),
        }
    }
}