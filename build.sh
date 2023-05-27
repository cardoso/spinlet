#!/usr/bin/env bash
# -*- coding: utf-8 -*-

cargo build --package spinlet --release
cargo build --package spinlets --release --target wasm32-wasi

# Remove the .spinlets bin directory
rm -rf .spinlet/bin
# Create a new .spinlets bin directory 
mkdir .spinlet/bin
# Copy spinlets to the .spinlets directory
cp target/wasm32-wasi/release/*.wasm .spinlet/bin

# Adapt modules into components
for i in .spinlet/bin/*; do
    echo "Adapting: $(du -h "$i")"
    wasm-tools component new "$i" --output "$i" --adapt wasi_snapshot_preview1=.spinlet/lib/command.wasm
    echo "Finished: $(du -h "$i")"
    echo "------------------------"
done

