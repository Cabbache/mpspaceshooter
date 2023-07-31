#!/bin/bash
cargo build --release
cd trajectory
wasm-pack build
