#!/usr/bin/env bash

# SPDX-License-Identifier: MIT
# Copyright 2025 IROX Contributors
#

set -eux

rustup update
rustup component add rustfmt clippy
TMPDIR=$(mktemp -d)
cargo install --target-dir=$TMPDIR cargo-about cargo-deny cargo-edit just cargo-cache cargo-tarpaulin cargo-describe
cargo cache -e -k0 -rall
rm -rf $TMPDIR
