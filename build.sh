#!/bin/bash


cargo build  --target wasm32-unknown-unknown &&
mkdir -p pkg &&
wasm-bindgen ~/.cargo/target/wasm32-unknown-unknown/debug/mandelbrot-wasm.wasm --out-dir pkg --target web &&
# cp index.html pkg &&
ln -s ../index.html pkg/index.html
ln -s ../styles.css pkg/styles.css
ln -s ../main.js pkg/main.js

http-server ./pkg/ 
