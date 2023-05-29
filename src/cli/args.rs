use std::path::{PathBuf, Path};

use clap::Parser;
use serde::{Serialize, Deserialize};

#[derive(Parser, Serialize, Deserialize, Debug)]
#[command(bin_name = "spin let", author, version, about)]
pub struct Args {
    #[arg(default_value = "shell")]
    spinlet: String,
    #[arg(long, short, default_value = ".spinlet")]
    root: String,
    #[arg(last = true)]
    spinlet_args: Vec<String>,
}

impl Args {
    pub fn spinlet(&self) -> &str {
        &self.spinlet
    }

    pub fn root(&self) -> &str {
        &self.root
    }

    pub fn spinlet_args(&self) -> &[String] {
        &self.spinlet_args
    }

    pub fn config(&self) -> PathBuf {
        Path::new(&self.root)
            .join("config.toml")
    }
}