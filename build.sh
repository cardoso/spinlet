#!/usr/bin/env bash
# -*- coding: utf-8 -*-

cargo build --package spinlet --release
cargo build --package spinlets --release --target wasm32-wasi

# Remove the .spinlets directory
rm -rf .spinlets
# Create a new .spinlets directory 
mkdir .spinlets
# Copy spinlets to the .spinlets directory
cp target/wasm32-wasi/release/*.wasm .spinlets

# Adapt modules into components
for i in .spinlets/*; do
    echo "Adapting $i"
    wasm-tools component new "$i" --output "$i" --adapt wasi_snapshot_preview1=./adapters/wasi_preview1_component_adapter.command.wasm -v
    echo "File size: $(du -h "$i")"
done

