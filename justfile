#!/usr/bin/env -S just --justfile

default +FLAGS='': updates (build FLAGS) (test FLAGS) (format FLAGS) (lints FLAGS) (upgrade FLAGS)

ci +FLAGS='': updates deny (build FLAGS) (test FLAGS) format_check (lints FLAGS) about doc upgrade package

updates:
    rustup update
    cargo update

check_install prereq:
    cargo install --root target --quiet {{prereq}}

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
    ./target/bin/cargo-about generate -c dev/about.toml dev/about.hbs > target/licenses.html
    ./target/bin/cargo-about generate -c dev/about.toml dev/about-md.hbs > target/LICENSES.md

upgrade +FLAGS='':
    just check_install cargo-edit
    ./target/bin/cargo-upgrade --dry-run --pinned -i {{FLAGS}}

doc:
    rustup toolchain install nightly 2>&1 > /dev/null
    RUSTDOCFLAGS=$(xargs -aRustdoc.lints) cargo +nightly doc

unused:
    cargo clippy --bins --lib --all-features -- -Wunused_crate_dependencies

new DEST: 
   just check_install cargo-generate
   mkdir -p {{DEST}}
   ./target/bin/cargo-generate --destination `pwd`/{{DEST}} --path `pwd`/dev/mod_template --init

site:
   just check_install oranda
   ./target/bin/oranda build

book: about
   cargo install --quiet --git https://github.com/spmadden/mdbook mdbook --root target
   ./target/bin/mdbook build

servebook: book
   ./target/bin/mdbook serve

vet:
   just check_install cargo-vet
   ./target/bin/cargo-vet