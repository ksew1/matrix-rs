#!/bin/bash
set -e
set -o pipefail


export RUSTFLAGS="-D warnings"

cargo fmt -- --check
cargo lint
cargo test