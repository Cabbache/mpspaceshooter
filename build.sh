#!/bin/bash
cargo build --release
cd trajectory
mv Cargo.toml original
mv Cargo.wasm.toml Cargo.toml
cargo build --release
mv Cargo.toml Cargo.wasm.toml
mv original Cargo.toml
