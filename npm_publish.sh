#!/usr/bin/env bash
cd ./crates/bpr_web/
rm -rf ./pkg
wasm-pack build --scope katand
cd ./pkg
npm publish --access=public