#!/usr/bin/env -S just --justfile

default +FLAGS='': (build FLAGS) (test FLAGS) (format FLAGS) (lints FLAGS) (upgrade FLAGS)

ci +FLAGS='': deny build test format_check lints about doc upgrade package

check_install prereq:
    #!/usr/bin/env bash
    set -euxo pipefail
    if ! type "{{prereq}}" > /dev/null; then
      cargo install {{prereq}}
    fi

deny +FLAGS='':
    just check_install cargo-deny
    cargo deny check {{FLAGS}}

build +FLAGS='':
    cargo build {{FLAGS}}

test +FLAGS='':
    cargo test {{FLAGS}}

format +FLAGS='':
    cargo fmt {{FLAGS}}

format_check +FLAGS='':
    cargo fmt --check {{FLAGS}}

lints +FLAGS='':
    xargs -aClippy.lints cargo clippy {{FLAGS}} --

package:
    cargo package --allow-dirty

about:
    just check_install cargo-about
    cargo about generate about.hbs > about.html

upgrade +FLAGS='':
    just check_install cargo-edit
    cargo upgrade --dry-run --pinned -i {{FLAGS}}

doc:
    rustup toolchain install nightly 2>&1 > /dev/null
    RUSTDOCFLAGS=$(xargs -aRustdoc.lints) cargo +nightly doc
