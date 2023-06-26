use std::collections::HashMap;
use std::path::{Path, PathBuf};

use cap_primitives::ambient_authority;
use spinlet::{Config, TaskLoader};
use spinlet::Context;
use spinlet::Loader;
use spinlet::Spinlet;
use spinlet::spin;
use spinlet_manifest::Manifest;
use tokio::process::Command;
use wasmtime_wasi::Dir;
use anyhow::Result;

fn main() {
    human_panic::setup_panic!();

    let rt = match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build() {
            Ok(rt) => rt,
            Err(error) => {
                eprintln!("Failed to build spinlet runtime: {error}");
                return;
            },
        };


        rt.block_on(async {
            let cmd = match Cli::load().await {
                Ok(cmd) => cmd,
                Err(error) => {
                    eprintln!("Failed to load spinlet CLI: {error}");
                    return;
                },
            };

            match cmd.run().await {
                Ok(true) => (),
                Ok(false) => {
                    eprintln!("Spinlet exited with non-zero exit code");
                },
                Err(error) => {
                    eprintln!("Failed to run spinlet: {error}");
                },
            }
        });
}

#[derive(Debug)]
pub struct Cli {
    name: String,
    args: Vec<String>,
    config: Config,
}

impl Cli {
    pub async fn load() -> Result<Self> {
        let mut args = std::env::args().skip(1);

        let path = Path::new(".spinlet");

        if !path.exists() {
            tokio::fs::create_dir(path).await?;
        }

        let path = path.join("config.toml");

        if !path.exists() {
            let cfg = Config::default();
            let txt = toml::to_string(&cfg)?;
            tokio::fs::write(&path, txt).await?;
        }

        let config = tokio::fs::read_to_string(path).await?;
        let config: Config = toml::from_str(&config)?;
        let name = args.next().unwrap_or(config.child().name().to_string());

        Ok(Self {
            name,
            args: args.collect(),
            config,
        })
    }

    async fn run(&self) -> Result<bool> {
        match self.is_alias() {
            true => self.run_alias().await,
            false => self.run_command().await,
        }
    }

    async fn run_command(&self) -> Result<bool> {
        let manifest = self.config.manifest(&self.name);
        let manifest = tokio::fs::read_to_string(manifest).await?;
        let manifest: Manifest = toml::from_str(&manifest)?;
        let context = Context::new(&manifest, self.cwd()?);
        let loader = Loader::new(context)?;
        let spinlet = Spinlet::new(loader);
        let binary = tokio::fs::read(self.binary()).await?;
        let success = spinlet.run(&binary).await?;
        Ok(success)
    }

    async fn run_alias(&self) -> Result<bool> {
        let manifests = self.load_manifests().await?;

        let task_loader = TaskLoader::new(&self.name, manifests);
        let (before_tasks, after_tasks) = (task_loader.before_hooks(&self.config)?, task_loader.after_hooks(&self.config)?);

        for task in before_tasks {
            task.await?;
        }

        let spin = match spin::bin_path() {
            Ok(spin) => spin,
            Err(_) => "spin".to_string(),
        };

        let mut command = Command::new(spin).arg(&self.name).args(self.args.clone()).spawn()?;
        let result = command.wait().await?;

        for task in after_tasks {
            task.await?;
        }

        Ok(result.success())
    }

    pub fn cwd(&self) -> std::io::Result<Dir> {
        Dir::open_ambient_dir(".", ambient_authority())
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn is_alias(&self) -> bool {
        self.config.parent().alias(&self.name).is_some()
    }

    pub fn binary(&self) -> PathBuf {
        self.config.binary(&self.name)
    }

    pub async fn load_manifests(&self) -> Result<HashMap<String, Manifest>> {
        let mut manifests = HashMap::<String, Manifest>::new();
        for entry in self.config.etc().read_dir()? {
            let entry = entry?;
            let entry = entry.path();
            let Some(name) = entry.file_stem() else { continue };
            let entry: &Path = &entry;
            let manifest = tokio::fs::read_to_string(&entry).await?;
            let manifest = toml::from_str::<Manifest>(&manifest)?;
            manifests.insert(name.to_string_lossy().to_string(), manifest);
        }

        Ok(manifests)
    }

    
}

