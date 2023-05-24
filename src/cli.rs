use std::path::{PathBuf, Path};
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(bin_name = "spin let")]
pub struct Cli {
    /// Found in the .spinlets folder without the .wasm extension)
    #[arg(default_value = "shell")]
    spinlet: String,
    /// Folder to look for spinlets in
    #[arg(short, long, default_value = ".spinlets")]
    dir: PathBuf,
    /// Extension of spinlets
    #[arg(short, long, default_value = "wasm")]
    ext: String,
    /// Workspace to run the spinlet in
    #[arg(short, long, default_value = ".")]
    workspace: PathBuf,
    /// Arguments to pass to the spinlet
    #[arg(last = true)]
    args: Vec<String>,
}

impl Cli {
    pub fn path(&self) -> PathBuf {
        self.dir
            .join(&self.spinlet)
            .with_extension(&self.ext)
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    pub fn spinlet(&self) -> &str {
        &self.spinlet
    }

    pub fn args(&self) -> &[String] {
        &self.args
    }

    pub fn workspace(&self) -> &PathBuf {
        &self.workspace
    }
}
