#!/bin/bash

cd library_wasm/

cargo update
cargo install wasm-pack
#wasm-pack build --target web
wasm-pack build --target web --out-dir ../../wasm-web/pkg/
