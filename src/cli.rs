use clap::Parser;

pub const BIN_NAME: &str = "spin let";
pub const DEFAULT_SPINLET: &str = "shell";

#[derive(Parser)]
#[command(bin_name = BIN_NAME)]
pub struct Args {
    #[arg(default_value = DEFAULT_SPINLET)]
    pub spinlet: String,
}