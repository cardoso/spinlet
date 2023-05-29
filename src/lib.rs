mod config;
mod loader;
mod cli;


pub use cli::Args;
pub use config::Config;
pub use loader::Context;
pub use loader::Loader;

pub use spinlet_manifest::{Manifest, access::fs::{DirAccess, FileAccess}};
pub use cap_primitives::fs::OpenOptions;
pub use wasmtime_wasi::preview2::DirPerms;
pub use wasmtime_wasi::preview2::FilePerms;
pub use wasmtime_wasi::preview2::stdio;
pub use wasmtime_wasi::{preview2::{WasiCtxBuilder, Table}, Dir, ambient_authority};
pub use wasmtime::Result;

pub struct Spinlet {
    loader: Loader,
}

impl Spinlet {
    pub fn new(loader: Loader) -> Self {
        Self { loader }
    }

    pub async fn run(mut self, binary: &[u8]) -> Result<bool> {
        let command = self.loader.load(binary).await?;
        self.loader.run(command).await
    }
}

// impl ContextBuilder {
//     // fn build() -> Context {

//         // let builder = ContextBuilder { manifest, config };

        

//         //  for file in value.access().fs().file() {
//         //     let perms = file.dir_perms();
//         //     let file_perms = file.file_perms();
//         //     let path = file.path();
//         //     let options = file.options();

//         //     let Ok(file) = root.open_with(path, &options) else {
//         //         continue;
//         //     };

//         //     let dir = Dir::from_std_file(file.into_std());
            
//         //     builder = builder.push_preopened_dir(dir, perms, file_perms, path); 
//         // }

//         // let mut table = Table::new();
//         // let wasi = builder.build(&mut table)?;

//         // Ok(Self::new(table, wasi))
//     }
// }

pub struct ContextBuilder {
    manifest: Manifest,
}

impl ContextBuilder {
    pub fn new(manifest: Manifest) -> Self {
        Self { manifest }
    }

    pub fn build(&self, root: Dir) -> Result<Context>  {
        let mut builder = WasiCtxBuilder::new();

        for key in self.manifest.access().env().var().keys() {
            let Ok(value) = std::env::var(key) else { continue };
            builder = builder.push_env(key, value);
        }

        if self.manifest.access().env().args().enabled() {
            for arg in std::env::args() {
                builder = builder.push_arg(arg);
            }
        }

        if self.manifest.access().io().stdin().enabled() {
            builder = builder.set_stdin(stdio::stdin());
        }

        if self.manifest.access().io().stdout().enabled() {
            builder = builder.set_stdout(stdio::stdout());
        }

        if self.manifest.access().io().stderr().enabled() {
            builder = builder.set_stderr(stdio::stderr());
        }

        for dir in self.manifest.access().fs().dir() {
            let perms = match (dir.read(), dir.mutate()) {
                (true, true) => DirPerms::READ | DirPerms::MUTATE,
                (true, false) => DirPerms::READ,
                (false, true) => DirPerms::MUTATE,
                (false, false) => DirPerms::empty(),
            };
            let file_perms = FilePerms::empty();
            let path = dir.path();
            let Ok(dir) = root.open_dir(path) else {
                continue;
            };

            builder = builder.push_preopened_dir(dir, perms, file_perms, path); 
        }

        for file in self.manifest.access().fs().file() {
            let perms = DirPerms::empty();
            let file_perms = match (file.read(), file.write()) {
                (true, true) => FilePerms::READ | FilePerms::WRITE,
                (true, false) => FilePerms::READ,
                (false, true) => FilePerms::WRITE,
                (false, false) => FilePerms::empty(),
            };
            
            let path = file.path();

            let mut options = OpenOptions::new();
            let options = options.read(file.read()).write(file.write());
            let Ok(file) = root.open_with(path, &options) else {
                continue;
            };

            let dir = Dir::from_std_file(file.into_std());
            
            builder = builder.push_preopened_dir(dir, perms, file_perms, path);
        }

        Ok(Context::new(builder))
    }
}