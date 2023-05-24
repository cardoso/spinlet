mod command;
mod cli;

use clap::Parser;
use command::*;
use cli::*;

pub async fn run() {
    let cli = Cli::parse();

    match Spinlet::load(&cli.path(), &cli.dir()).await {
        Ok(mut spinlet) => match spinlet.run().await {
            Ok(result) => match result {
                Ok(()) => println!("spinlet ran successfully"),
                Err(()) => println!("spinlet failed to run"),
            },
            Err(error) => println!("error loading spinlet: {}", error),
        },
        Err(error) => println!("error loading spinlet: {}", error),
    }
}
