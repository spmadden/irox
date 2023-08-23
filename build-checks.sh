#!/bin/bash

set -eu
trap "echo The script is terminated; exit" SIGINT

if ! type "cargo-binstall" > /dev/null; then
  cargo install cargo-binstall
fi
REQUIREDCMDS=("cargo-deny" "cargo-about")
for CMD in "${REQUIREDCMDS[@]}"; do
  if ! type "$CMD" > /dev/null ; then
    cargo binstall $CMD
  fi
done

cargo deny check
cargo build
cargo test
cargo fmt --check
xargs -aClippy.lints cargo clippy --
cargo about generate about.hbs > about.html
