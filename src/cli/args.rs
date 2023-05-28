use clap::Parser;
use serde::{Serialize, Deserialize};

#[derive(Parser, Serialize, Deserialize, Debug)]
#[command(bin_name = "spin let", author, version, about)]
pub struct Args {
    #[arg(default_value = "shell")]
    spinlet: String
}

impl Args {
    pub fn spinlet(&self) -> &str {
        &self.spinlet
    }
}