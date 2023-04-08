#!/bin/sh

cargo build --target wasm32-unknown-unknown --release

octez-smart-rollup-wasm-debugger target/wasm32-unknown-unknown/release/store.wasm --inputs inputs.json