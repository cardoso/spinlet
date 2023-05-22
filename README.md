# Spinlet

A [Spin plugin](https://github.com/fermyon/spin-plugins) and runtime for building and running [wasm command components]() as plugins for [Spin](https://github.com/fermyon/spin-plugins).

## Status

```rust
fn main() {
    for arg in std::env::args() {
        println!("{}", arg);
    }

    // Plugin only has access to environment variables prefixed with `SPIN_`
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

## Usage

```bash
#!/usr/bin/env bash
spin let [spinlet] [spinlet args]
```
