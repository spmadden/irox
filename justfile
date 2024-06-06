#!/usr/bin/env -S just --justfile

default: build

init:
    cd build && cargo build --release 

build:
    @just init
    ./build/target/release/irox-builder

ci:
    @just init
    ./build/target/release/irox-builder ci

lints:
    @just init
    ./build/target/release/irox-builder lints
