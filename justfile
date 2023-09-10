#!/usr/bin/env -S just --justfile

default: build_checks


check_install prereq:
    #!/usr/bin/env bash
    set -euxo pipefail
    if ! type "{{prereq}}" > /dev/null; then
      cargo install {{prereq}}
    fi

prereqs:
    just check_install cargo-binstall

deny: prereqs
    just check_install cargo-deny
    cargo deny check

build: prereqs
    cargo build

test: prereqs
    cargo test

format: prereqs
    cargo fmt --check

lints: prereqs
    xargs -aClippy.lints cargo clippy --

about: prereqs
    just check_install cargo-about
    cargo about generate about.hbs > about.html

build_checks: deny build test format lints about