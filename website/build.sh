#!/usr/bin/env sh

cd site-wasm
wasm-pack build
cd pkg && npm link
cd ../site
npm link site-wasm
npm install
npm run serve