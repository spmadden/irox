#!/usr/bin/env -S just --justfile

default: build

init:
    pushd build && cargo build --release || popd

build:
    @just init
    ./build/target/release/irox-build

ci:
    @just init
    ./build/target/release/irox-build ci

lints:
    @just init
    ./build/target/release/irox-build lints
