use clap::Parser;

#[tokio::main]
async fn main() {
    let result = spinlet::Cli::parse().run().await;
    result.expect("Spinlet failed")
}
