#!/bin/bash

set -ex

echo "Building for wasm"
cargo build --release --target wasm32-unknown-unknown

cp target/wasm32-unknown-unknown/release/path_finding.wasm docs/


