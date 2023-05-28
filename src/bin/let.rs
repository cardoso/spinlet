
use spinlet::Cli;
use spinlet::Spinlet;

fn main() {
    human_panic::setup_panic!();

    let cli = Cli::new();
    cli.run();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap_or_else(|error| panic!("Failed to build runtime: {error}"))
        .block_on(async {
            let mut spinlet = match Spinlet::load(&cli.manifest(), &cli.binary()).await {
                Ok(spinlet) => spinlet,
                Err(error) => {
                    eprintln!("Failed to load spinlet: {error}");
                    return;
                },
            };

            match spinlet.run().await {
                Ok(result) => result,
                Err(error) => {
                    eprintln!("Failed to run spinlet: {error}");
                    return;
                },
            };
        });
}