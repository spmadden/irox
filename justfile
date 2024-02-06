#!/usr/bin/env -S just --justfile

default +FLAGS='': updates (recurse FLAGS) (build FLAGS) (test FLAGS) (format FLAGS) (lints FLAGS) (upgrade FLAGS)

ci +FLAGS='': updates deny (recurse_ci FLAGS) (build FLAGS) (test FLAGS) format_check (lints_deny FLAGS) about doc upgrade

GITHUB_ACTIONS := env_var_or_default('GITHUB_ACTIONS', 'false')

updates:
    @just logstart updates
    rustup update
    cargo update
    @just logend

check_install prereq:
    #!/usr/bin/env bash
    set -euxo pipefail
    if ! type "{{prereq}}" > /dev/null; then
      cargo install {{prereq}}
    fi

deny +FLAGS='':
    @just logstart deny
    just check_install cargo-deny
    cargo deny check {{FLAGS}}
    @just logend

build +FLAGS='':
    @just logstart build
    cargo build --all-targets --all-features {{FLAGS}}
    @just logend

check TARGET +FLAGS='':
   @just logstart check-{{TARGET}}
   just check_install cargo-describe
   cargo describe -f name -o plain | sed 's/.*/\-p \0/' | xargs cargo clean
   cargo check --target {{TARGET}} {{FLAGS}}
   @just logend

check_all +FLAGS='':
    @just check x86_64-pc-windows-msvc {{FLAGS}}
    @just check x86_64-unknown-linux-gnu {{FLAGS}}
    @just check wasm32-unknown-unknown {{FLAGS}}

test +FLAGS='':
    @just logstart test
    cargo test --all-features {{FLAGS}}
    @just logend

format +FLAGS='':
    @just logstart format
    cargo fmt --all {{FLAGS}}
    @just logend

format_check +FLAGS='':
    @just logstart format_check
    cargo fmt --check --all {{FLAGS}}
    @just logend

lints +FLAGS='':
    @just logstart lints
    cargo clippy --bins --lib --examples --all-features {{FLAGS}} --
    @just logend

lints_deny +FLAGS='':
    @just logstart lints
    cargo clippy --bins --lib --examples --all-features {{FLAGS}} -- -Dwarnings
    @just logend

package:
    @just logstart package
    cargo package -p irox --all-features
    @just logend

about:
    @just logstart about
    just check_install cargo-about
    cargo about generate about.hbs > about.html
    @just logend

upgrade +FLAGS='':
    @just logstart upgrade
    just check_install cargo-edit
    cargo upgrade --dry-run --pinned -i {{FLAGS}}
    @just logend

doc:
    @just logstart doc
    cargo doc
    @just logend

unused:
    @just logstart unused
    cargo clippy --bins --lib --all-features -- -Wunused_crate_dependencies
    @just logend

new DEST: 
    just check_install cargo-generate
    mkdir -p {{DEST}}
    cargo generate --destination `pwd`/{{DEST}} --path `pwd`/dev/mod_template --init

release +FLAGS='':
    just check_install cargo-smart-release
    cargo smart-release --no-conservative-pre-release-version-handling --no-isolate-dependencies-from-breaking-changes -u {{FLAGS}}

recurse +FLAGS='':
    @for module in `find -mindepth 2 -name 'justfile' -printf '%h\n'` ; do \
        just logstart module-$module; \
        just $module/default {{FLAGS}}; \
        just logend; \
    done

recurse_ci +FLAGS='':
    @for module in `find -mindepth 2 -name 'justfile' -printf '%h\n'` ; do \
        just logstart module-$module; \
        just $module/ci {{FLAGS}}; \
        just logend; \
    done

logstart RECIPE:
    #!/bin/bash
    if [[ "{{GITHUB_ACTIONS}}" == "true" ]] ; then echo "::group::{{RECIPE}}"; fi

logend:
    #!/bin/bash
    if [[ "{{GITHUB_ACTIONS}}" == "true" ]] ; then echo "::endgroup::" ; fi
