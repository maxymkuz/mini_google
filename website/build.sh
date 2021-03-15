#!/usr/bin/env sh

cd site-wasm
wasm-pack build --target web --out-name wasm --out-dir ./static
#miniserve ./static --index index.html
cargo build
cargo run
