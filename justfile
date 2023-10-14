#!/usr/bin/env -S just --justfile

default: build test format lints upgrade 

ci: deny build test format_check lints about doc upgrade package

check_install prereq:
    #!/usr/bin/env bash
    set -euxo pipefail
    if ! type "{{prereq}}" > /dev/null; then
      cargo install {{prereq}}
    fi

deny:
    just check_install cargo-deny
    cargo deny check

build:
    cargo build

test:
    cargo test

format:
    cargo fmt

format_check:
    cargo fmt --check

lints +FLAGS='':
    xargs -aClippy.lints cargo clippy {{FLAGS}} --

package:
    cargo package --allow-dirty

about:
    just check_install cargo-about
    cargo about generate about.hbs > about.html

upgrade:
    just check_install cargo-edit
    cargo upgrade --dry-run --pinned -i

doc:
    rustup toolchain install nightly 2>&1 > /dev/null
    RUSTDOCFLAGS=$(xargs -aRustdoc.lints) cargo +nightly doc
