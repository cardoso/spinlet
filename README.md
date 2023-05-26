# Spinlet

A [Spin plugin](https://github.com/fermyon/spin-plugins) and runtime for building and running [wasm32-wasi cli components](https://github.com/WebAssembly/wasi-cli) as plugins for [Spin](https://github.com/fermyon/spin-plugins).

## Status

### Sandboxed Environment

- [x] access control

```toml
[access.stdio]
stdin = true
stdout = true
stderr = true

[[access.env]]
key = "HOME"

[[access.dir]]
path = "."
read = true

[[access.file]]
path = "Cargo.toml"
read = true

[[access.dir]]
path = "registry"
read = true

[[access.file]]
path = "registry/spin.toml"
read = true
write = true
```

- [x] `std::env::args`

```rust
fn main() {
    for arg in std::env::args() {
        println!("{}", arg);
    }

    // Plugin only has access to environment variables specified in the manifest
    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }
}
```

```bash
➜  spinlet git:(main) ✗ spin let update
You're using a pre-release version of Spin (1.3.0-pre0). This plugin might not be compatible (supported: >=0.7). Continuing anyway.
/Users/cardoso/Library/Application Support/spin/plugins/let/let
update
SPIN_BIN_PATH: /Users/cardoso/.cargo/bin/spin
SPIN_BRANCH: main
SPIN_BUILD_DATE: 2023-05-20
SPIN_COMMIT_DATE: 2023-05-19
SPIN_COMMIT_SHA: d476000
SPIN_DEBUG: false
SPIN_TARGET_TRIPLE: aarch64-apple-darwin
SPIN_VERSION: 1.3.0-pre0
SPIN_VERSION_MAJOR: 1
SPIN_VERSION_MINOR: 3
SPIN_VERSION_PATCH: 0
SPIN_VERSION_PRE: pre0
```

- [x] `std::fs::read_dir`

```rust
pub fn main() {
    /// Plugin only has access to files in the current working directory
    match std::fs::read_dir("/workspace") {
        Ok(dir) => {
            for entry in dir {
                match entry {
                    Ok(entry) => println!("{}", entry.path().display()),
                    Err(error) => println!("error reading entry: {}", error),
                }
            }
        }
        Err(error) => println!("error reading /: {}", error),
    }
}
```

```bash
➜  spinlet git:(main) ✗ pwd
/Users/cardoso/Developer/cardoso/spinlet/spinlet
➜  spinlet git:(main) ✗ spin let workspace
You're using a pre-release version of Spin (1.3.0-pre0). This plugin might not be compatible (supported: >=0.7). Continuing anyway.
/workspace/Cargo.toml
/workspace/.spinlets
/workspace/.DS_Store
/workspace/target
/workspace/install.sh
/workspace/let-0.1.5-macos-aarch64.tar.gz
/workspace/Cargo.lock
/workspace/README.md
/workspace/adapters
/workspace/build.sh
/workspace/.gitignore
/workspace/spinlets
/workspace/.git
/workspace/let.json
/workspace/spin-pluginify.toml
/workspace/src
```

## Usage

```bash
spin let [spinlet] -- [spinlet args]
```

```terminal
Usage: spin let [OPTIONS] <SPINLET> [-- <ARGS>...]

Arguments:
  <SPINLET>  Spinlet to run
  [ARGS]...  Arguments to pass to the
             spinlet

Options:
  -w, --workspace <WORKSPACE>
          Workspace to run the spinlet
          in [default: .]
  -h, --help
          Print help
```
