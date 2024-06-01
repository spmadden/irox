#!/usr/bin/env -S just --justfile

default +FLAGS='': updates (recurse FLAGS) (build FLAGS) (test FLAGS) (format FLAGS) (lints FLAGS) (upgrade FLAGS)

ci +FLAGS='': updates deny (recurse_ci FLAGS) (build FLAGS) (test FLAGS) format_check (lints_deny FLAGS) about doc upgrade

GITHUB_ACTIONS := env_var_or_default('GITHUB_ACTIONS', 'false')

set windows-shell := ["c:\\Program Files\\PowerShell\\7\\pwsh.exe", "-NoLogo", "-noni", "-Command"]

updates:
    @just logstart updates
    rustup update
    cargo update
    @just logend

check_install prereq:
   cargo install {{prereq}}

deny +FLAGS='':
    @just logstart deny
    cargo install --locked cargo-deny 
    cargo deny check {{FLAGS}}
    @just logend

build +FLAGS='':
    @just logstart build
    cargo build --all-targets --all-features {{FLAGS}}
    @just logend

[linux]
check TARGET +FLAGS='':
    @just logstart check-{{TARGET}}
    just check_install cargo-describe
    cargo describe -f name -o plain | sed 's/.*/\-p \0/' | xargs cargo clean
    cargo check --target {{TARGET}} {{FLAGS}}
    @just logend

[windows]
check TARGET +FLAGS='':
    $ErrorActionPreference = "Stop"
    @just logstart check-{{TARGET}}
    just check_install cargo-describe
    &"cargo" clean @(cargo describe -o plain -f name | %{$_ -replace '.+','-p$0'})
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
    cargo install --locked cargo-about
    cargo about generate about.hbs > about.html
    @just logend

upgrade +FLAGS='':
    @just logstart upgrade
    cargo install --locked cargo-edit
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

[linux]
new DEST: 
    just check_install cargo-generate
    mkdir -p {{DEST}}
    cargo generate --destination `pwd`/{{DEST}} --path `pwd`/dev/mod_template --init

release +FLAGS='':
    just check_install cargo-smart-release
    cargo smart-release --no-conservative-pre-release-version-handling --no-isolate-dependencies-from-breaking-changes -u {{FLAGS}}

[linux]
recurse +FLAGS='':
    #!/usr/bin/bash
    set -euo pipefail
    for module in `find -mindepth 2 -name 'justfile' -printf '%h\n'` ; do \
        just logstart module-$module; \
        just $module/default {{FLAGS}}; \
        just logend; \
    done

[windows]
recurse +FLAGS='':
    $ErrorActionPreference = "Stop"
    foreach ($module in @(Get-ChildItem -Path 'libraries' -Recurse -Filter 'justfile' | Resolve-Path -Path {$_.DirectoryName} -Relative)) { \
       Write-Host $module; \
       just logstart module-$module; \
       just $module/default {{FLAGS}}; \
       just logend; \
    }

[windows]
recurse_ci +FLAGS='':
    $ErrorActionPreference = "Stop"
    foreach ($module in @(Get-ChildItem -Path 'libraries' -Recurse -Filter 'justfile' | Resolve-Path -Path {$_.DirectoryName} -Relative)) { \
       Write-Host $module; \
       just logstart module-$module; \
       just $module/ci {{FLAGS}}; \
       just logend; \
    }

[linux]
recurse_ci +FLAGS='':
    @for module in `find -mindepth 2 -name 'justfile' -printf '%h\n'` ; do \
        just logstart module-$module; \
        just $module/ci {{FLAGS}}; \
        just logend; \
    done

[linux]
logstart RECIPE:
    #!/bin/bash
    if [[ "{{GITHUB_ACTIONS}}" == "true" ]] ; then echo "::group::{{RECIPE}}"; fi

[linux]
logend:
    #!/bin/bash
    if [[ "{{GITHUB_ACTIONS}}" == "true" ]] ; then echo "::endgroup::" ; fi

[windows]
logstart RECIPE:
    @if ( "{{GITHUB_ACTIONS}}" -eq "true" ) { Write-Output "::group::{{RECIPE}}" }

[windows]
logend:
    @if ( "{{GITHUB_ACTIONS}}" -eq "true" ) { Write-Output "::endgroup::" }

[linux]
buildperf:
    @cargo clean
    @cargo fetch
    time cargo build --release

[windows]
buildperf:
    @cargo clean
    @cargo fetch
    pwsh.exe -NoLogo -noni -Command Measure-Command{ cargo build --release }
