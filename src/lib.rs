pub use spinlet_config as config;
pub use spinlet_manifest as manifest;

pub mod spin;

mod loader;
mod runner;
mod context;

pub use config::Config;
pub use context::Context;
pub use loader::Loader;
pub use runner::Runner;
pub use manifest::{Manifest, access::fs::{DirAccess, FileAccess}};
pub use cap_primitives::fs::OpenOptions;
pub use wasmtime_wasi::preview2::DirPerms;
pub use wasmtime_wasi::preview2::FilePerms;
pub use wasmtime_wasi::preview2::stdio;
pub use wasmtime_wasi::{preview2::{WasiCtxBuilder, Table}, Dir, ambient_authority};
pub use wasmtime::Result;

pub struct Spinlet {
    loader: Loader<Context>
}

impl Spinlet {
    pub fn new(loader: Loader<Context>) -> Self {
        Self { loader }
    }

    pub async fn run(self, binary: &[u8]) -> Result<bool> {
        let (command, store) = self.loader.load(binary).await?;
        let mut runner = Runner::new(command, store);
        runner.run().await
    }
}

use std::collections::HashMap;

use tokio::task::JoinHandle;

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
        self.tasks_for(&self.before_hooks, config)
    }

    pub fn after_hooks(&self, config: &Config) -> Result<Vec<JoinHandle<()>>> {
        self.tasks_for(&self.after_hooks, config)
    }

    pub fn tasks_for(&self, hooks: &Vec<String>, config: &Config) -> Result<Vec<JoinHandle<()>>> {
        let mut tasks = Vec::new();
        for hook in hooks {
            let Some(manifest) = self.manifests.get(hook) else { continue };
            let binary = config.binary(hook);
            let context = Context::new(manifest, Dir::open_ambient_dir(".", ambient_authority())?);
            tasks.push(tokio::task::spawn(async move {
                let Ok(loader) = Loader::new(context) else { return };
                let spinlet = Spinlet::new(loader);
                let Ok(bytes) = tokio::fs::read(&binary).await else { return };
                let Ok(_) = spinlet.run(&bytes).await else { return };
                
            }));
        }

        Ok(tasks)
    }
}