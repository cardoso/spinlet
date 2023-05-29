mod error;

use cap_primitives::fs::OpenOptions;
use wasmtime_wasi::{preview2::{Table, WasiView, WasiCtx, WasiCtxBuilder, FilePerms, DirPerms, stdio}, Dir};
use spinlet_manifest::Manifest;

pub struct Context {
    table: Table,
    wasi: WasiCtx,
}

impl Context {
    pub fn new(manifest: Manifest, root: Dir) -> Self {
        let mut builder = WasiCtxBuilder::new();

        for key in manifest.access().env().var().keys() {
            let Ok(value) = std::env::var(key) else { continue };
            builder = builder.push_env(key, value);
        }

        if manifest.access().env().args().enabled() {
            for arg in std::env::args() {
                builder = builder.push_arg(arg);
            }
        }

        if manifest.access().io().stdin().enabled() {
            builder = builder.set_stdin(stdio::stdin());
        }

        if manifest.access().io().stdout().enabled() {
            builder = builder.set_stdout(stdio::stdout());
        }

        if manifest.access().io().stderr().enabled() {
            builder = builder.set_stderr(stdio::stderr());
        }

        for dir in manifest.access().fs().dir() {
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

        for file in manifest.access().fs().file() {
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

        let mut table = Table::new();
        let wasi = builder.build(&mut table).unwrap();

        Self { table, wasi }
    }
}

impl WasiView for Context {
    fn table(&self) -> &Table {
        &self.table
    }
    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
    fn ctx(&self) -> &WasiCtx {
        &self.wasi
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}