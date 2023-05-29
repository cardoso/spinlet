use cap_primitives::ambient_authority;
use clap::Parser;
use spinlet::Args;
use spinlet::Config;
use spinlet::Context;
use spinlet::Loader;
use spinlet::Spinlet;
use spinlet_manifest::Manifest;
use wasmtime_wasi::Dir;

fn main() {
    human_panic::setup_panic!();

    let root = Dir::open_ambient_dir(".", ambient_authority()).unwrap_or_else(|error| panic!("Failed to open root directory: {error}"));
    
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap_or_else(|error| panic!("Failed to build runtime: {error}"))
        .block_on(async {
            let args = Args::parse();
            
            let config = tokio::fs::read_to_string(args.config()).await.unwrap_or_else(|error| panic!("Failed to read config: {error}"));
            let config: Config = toml::from_str(&config).unwrap_or_else(|error| panic!("Failed to parse config: {error}"));

            let manifest = config.manifest(&args.spinlet());
            let manifest = tokio::fs::read_to_string(manifest).await.unwrap_or_else(|error| panic!("Failed to read manifest: {error}"));
            let manifest: Manifest = toml::from_str(&manifest).unwrap_or_else(|error| panic!("Failed to parse manifest: {error}"));

            let context = Context::new(manifest, root);
            let loader = Loader::new(context);
            let spinlet = Spinlet::new(loader);

            let binary = config.binary(args.spinlet());
            let binary = tokio::fs::read(&binary).await.unwrap_or_else(|error| panic!("Failed to read spinlet: {error}"));

            let success = spinlet.run(&binary).await.unwrap_or_else(|error| panic!("Failed to run spinlet: {error}"));
            if success {
                eprintln!("Spinlet ran successfully");
            } else {
                eprintln!("Spinlet failed to run");
            }
        });
}