use clap::Parser;
use spinlet::command::Spinlet;
use spinlet::cli::Cli;

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
