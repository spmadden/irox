#!/usr/bin/env bash

set -eux

rustup update
rustup component add rustfmt clippy
TMPDIR=$(mktmp -d)
cargo install --target-dir=$TMPDIR cargo-about cargo-deny cargo-edit cargo-smart-release just cargo-cache cargo-binstall
cargo cache -e -k0 -rall
rm -rf $TMPDIR