#!/usr/bin/env -S just --justfile

default: build test format lints upgrade package

ci: deny build test format_check lints about upgrade package

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

format:
    cargo fmt

format_check: prereqs
    cargo fmt --check

lints +FLAGS='': prereqs
    xargs -aClippy.lints cargo clippy {{FLAGS}} --

package:
    cargo package --allow-dirty

about: prereqs
    just check_install cargo-about
    cargo about generate about.hbs > about.html

upgrade: prereqs
    just check_install cargo-edit
    cargo upgrade --dry-run --pinned -i
