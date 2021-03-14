#!/usr/bin/env sh

cd site-wasm
wasm-pack build --target web
rollup ./main.js --format iife --file ./pkg/bundle.js
miniserve ./static --index index.html
