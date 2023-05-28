use std::path::PathBuf;
use clap::Parser;
use serde::{Deserialize, Serialize};

mod config;
mod args;

use config::Config;
use args::Args;

#[derive(Serialize, Deserialize, Debug)]
pub struct Cli {
    config: Config,
    args: Args,
}

impl Cli {
    pub fn new() -> Self {
        let config = Config::parse();
        let args = Args::parse();
        Self { config, args }
    }

    pub fn manifest(&self) -> PathBuf {
       self.config.manifest(self.args.spinlet())
    }

    pub fn binary(&self) -> PathBuf {
        self.config.binary(self.args.spinlet())
    }

    pub fn run(&self) {
        let spinlet = self.args.spinlet();
        if !self.config.binary(spinlet).exists() {
            println!("Spinlet not found: {spinlet}");
            if let Ok(spinlets) = self.config.list_spinlets() {
                println!("Available spinlets:");
                for spinlet in spinlets {
                    println!(" - {spinlet}");
                }
            }
        }
    }
}
    