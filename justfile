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
    cargo test --all-features {{FLAGS}}

format +FLAGS='':
    cargo fmt --all {{FLAGS}}

format_check +FLAGS='':
    cargo fmt --check --all {{FLAGS}}

lints +FLAGS='':
   cargo clippy --bins --lib --examples --all-features {{FLAGS}} --

package:
    cargo package --allow-dirty

about:
    just check_install cargo-about
    cargo about generate about.hbs > about.html

upgrade +FLAGS='':
    just check_install cargo-edit
    cargo upgrade --dry-run --pinned -i {{FLAGS}}

doc:
   cargo doc

unused:
    cargo clippy --bins --lib --all-features -- -Wunused_crate_dependencies

new DEST: 
   just check_install cargo-generate
   mkdir -p {{DEST}}
   cargo generate --destination `pwd`/{{DEST}} --path `pwd`/dev/mod_template --init
