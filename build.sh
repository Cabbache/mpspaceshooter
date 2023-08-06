#!/bin/bash
cargo build --release
cd utils
wasm-pack build
