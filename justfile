#!/usr/bin/env -S just --justfile

default +FLAGS='': updates (build FLAGS) (test FLAGS) (format FLAGS) (lints FLAGS) (upgrade FLAGS)

ci +FLAGS='': updates deny (build FLAGS) (test FLAGS) format_check (lints FLAGS) about doc upgrade package

updates:
    rustup update
    cargo update

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
    cargo build --all-targets --all-features {{FLAGS}}

test +FLAGS='':
    cargo test --all-targets --all-features {{FLAGS}}

format +FLAGS='':
    cargo fmt --all {{FLAGS}}

format_check +FLAGS='':
    cargo fmt --check --all {{FLAGS}}

lints +FLAGS='':
    xargs -aClippy.lints cargo clippy --bins --lib --examples --all-features {{FLAGS}} --

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
