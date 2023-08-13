#!/bin/bash

#The -musl in Rust targets refers to the MUSL libc, an alternative C library. When building with a -musl target, Rust produces a statically linked binary, including the C standard library within the binary itself. This leads to more portable and self-contained binaries, especially useful in environments with varying shared library versions.

#cross compliation
#cargo build --target=x86_64-unknown-linux-musl --workspace --release

cargo build --release
cd utils
wasm-pack build --target web
cd ../
cp -r utils/pkg client/
