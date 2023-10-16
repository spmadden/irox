#!/usr/bin/env bash

set -eux

rustup update
rustup component add rustfmt clippy
TMPDIR=$(mktemp -d)
cargo install --target-dir=$TMPDIR cargo-binstall
cargo binstall -y cargo-about cargo-deny cargo-smart-release cargo-edit just cargo-cache
cargo cache -e -k0 -rall
rm -rf $TMPDIR