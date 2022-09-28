#!/bin/bash


cargo build  --target wasm32-unknown-unknown
mkdir -p pkg
wasm-bindgen target/wasm32-unknown-unknown/debug/mandelbrot-wasm.wasm --out-dir pkg --target web
cp index.html pkg

http-server ./pkg/ 
