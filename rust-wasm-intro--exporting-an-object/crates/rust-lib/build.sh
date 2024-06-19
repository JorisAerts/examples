#!/bin/bash

OUTDIR="$(cd ../../packages; pwd)/rust-lib"
wasm-pack build --target bundler --out-dir "$OUTDIR"