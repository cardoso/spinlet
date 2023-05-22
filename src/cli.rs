use clap::Parser;

#[derive(Parser)]
#[command(bin_name = "spin let")]
pub struct Cli {
    /// Spinlet to run
    spinlet: String,
    /// Arguments to pass to the spinlet
    #[arg(last = true)]
    args: Vec<String>,
}

impl Cli {
    pub fn path(&self) -> String {
        let Self {
            spinlet,
            ..
        } = self;
        format!(".spinlets/{spinlet}.wasm")
    }
}
