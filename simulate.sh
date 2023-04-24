#!/bin/sh

cargo build --target wasm32-unknown-unknown --release

octez-smart-rollup-wasm-debugger target/wasm32-unknown-unknown/release/preimage.wasm --inputs inputs.json