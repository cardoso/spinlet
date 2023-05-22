use clap::Parser;
use spinlet::command::Spinlet;

#[derive(Parser)]
#[command(bin_name = "spin let")]
pub struct Cli {
    /// Spinlet to run
    spinlet: String,
}

impl Cli {
    pub fn path(&self) -> String {
        format!(".spinlets/{}.wasm", self.spinlet)
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match Spinlet::load(cli.path()).await {
        Ok(mut spinlet) => match spinlet.run().await {
            Ok(result) => match result {
                Ok(()) => (),
                Err(()) => eprintln!("Spinlet exited with an error"),
            },
            Err(error) => eprintln!("{error}"),
        },
        Err(error) => eprintln!("{error:?}"),
    }
}
