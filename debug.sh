#!/usr/bin/env bash
# -*- coding: utf-8 -*-

cargo build --package spinlet
cargo build --package spinlets --target wasm32-wasi

# Remove the .spinlets directory
rm -rf .spinlet/bin
# Create a new .spinlets directory 
mkdir .spinlet/bin
# Copy spinlets to the .spinlets directory
cp target/wasm32-wasi/debug/*.wasm .spinlet/bin

rm -rf target/.spinlet
mkdir target/.spinlet
mkdir target/.spinlet/bin
mkdir target/.spinlet/lib

for i in .spinlet/lib/*; do
    wasm-tools print "$i" --output "target/$i.wat" -vvv
    wasm-tools objdump "$i" --output "target/$i.objdump.ron" -vvv
    wasm-tools dump "$i" --output "target/$i.dump.ron" -vvv
done

# Adapt modules into components
for i in .spinlet/bin/*; do
    echo "Adapting: $(du -h "$i")"
    wasm-tools demangle "$i" --output "$i" -vvv
    wasm-tools print "$i" --output "target/$i.wat" -vvv
    wasm-tools objdump "$i" --output "target/$i.objdump.ron" -vvv
    wasm-tools dump "$i" --output "target/$i.dump.ron" -vvv
    wasm-tools validate "$i" --features all -vvv
    wasm-tools component new "$i" --output "$i" --adapt wasi_snapshot_preview1=.spinlet/lib/command.wasm -vvv
    wasm-tools validate "$i" --features all -vvv
    wasm-tools print "$i" --output "target/$i.adapted.wat" -vvv
    wasm-tools objdump "$i" --output "target/$i.adapted.objdump.ron" -vvv
    wasm-tools dump "$i" --output "target/$i.adapted.dump.ron" -vvv
    echo "Finished: $(du -h "$i")"
    echo "------------------------"
done

