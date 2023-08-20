#!/bin/bash

set -eu
trap "echo The script is terminated; exit" SIGINT

cargo deny check
cargo build
cargo test
cargo fmt --check
cargo clippy -- -Dwarnings
