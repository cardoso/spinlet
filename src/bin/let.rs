use std::collections::HashMap;
use std::path::{Path, PathBuf};

use cap_primitives::ambient_authority;
use spinlet::Config;
use spinlet::Context;
use spinlet::Loader;
use spinlet::Spinlet;
use spinlet::spin;
use spinlet_manifest::Manifest;
use tokio::process::Command;
use tokio::task::JoinHandle;
use wasmtime_wasi::Dir;
use anyhow::Result;

fn main() {
    human_panic::setup_panic!();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap_or_else(|error| panic!("Failed to build runtime: {error}"))
        .block_on(async {
            let cmd = match Cli::load().await {
                Ok(cmd) => cmd,
                Err(error) => panic!("Failed to load CLI: {error}"),
            };

            match cmd.run().await.expect("Failed to run spinlet") {
                true => std::process::exit(0),
                false => std::process::exit(1),
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
        let config: Config = toml::from_str(&tokio::fs::read_to_string(Path::new(".spinlet").join("config.toml")).await.unwrap_or_else(|error| panic!("Failed to read config: {error}"))).unwrap_or_else(|error| panic!("Failed to parse config: {error}"));
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
        println!("Running command {self:#?}");
        let manifest = self.config.manifest(&self.name);
        let manifest = tokio::fs::read_to_string(manifest).await?;
        let manifest: Manifest = toml::from_str(&manifest)?;

        let context = Context::new(&manifest, self.cwd());
        let loader = Loader::new(context);
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
        let result = command.wait().await.unwrap_or_else(|error| panic!("Failed to run alias: {error}"));

        for task in after_tasks {
            task.await?;
        }

        Ok(result.success())
    }

    pub fn cwd(&self) -> Dir {
        match Dir::open_ambient_dir(".", ambient_authority()) {
                Ok(cwd) => cwd,
                Err(error) => panic!("Failed to open current working directory: {error}"),
            }
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

pub struct TaskLoader {
    manifests: HashMap<String, Manifest>,
    before_hooks: Vec<String>,
    after_hooks: Vec<String>,
}

impl TaskLoader {
    pub fn new(main: &str, manifests: HashMap<String, Manifest>) -> Self {
        let mut before_hooks = Vec::<String>::new();
        let mut after_hooks = Vec::<String>::new();
        for (name, manifest) in &manifests {
            let Some(hook) = manifest.hook(main) else { continue };

            if hook.before().enabled() {
                before_hooks.push(name.to_string());
            }

            if hook.after().enabled() {
                after_hooks.push(name.to_string());
            }
        }

        Self {
            manifests,
            before_hooks,
            after_hooks,
        }
    }

    pub fn before_hooks(&self, config: &Config) -> Result<Vec<JoinHandle<()>>> {
        Ok(self.tasks_for(&self.before_hooks, config)?)
    }

    pub fn after_hooks(&self, config: &Config) -> Result<Vec<JoinHandle<()>>> {
        Ok(self.tasks_for(&self.after_hooks, config)?)
    }

    fn tasks_for(&self, hooks: &Vec<String>, config: &Config) -> Result<Vec<JoinHandle<()>>> {
        let mut tasks = Vec::new();
        for hook in hooks {
            let Some(manifest) = self.manifests.get(hook) else { continue };
            let binary = config.binary(&hook);
            let context = Context::new(&manifest, Dir::open_ambient_dir(".", ambient_authority())?);
            tasks.push(tokio::task::spawn(async move {
                let loader = Loader::new(context);
                let spinlet = Spinlet::new(loader);
                let Ok(bytes) = tokio::fs::read(&binary).await else { return };
                let Ok(_) = spinlet.run(&bytes).await else { return };
                
            }));
        }

        Ok(tasks)
    }
}