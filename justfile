#!/usr/bin/env -S just --justfile

default: build

init:
    cd build && cargo build --release 

build:
    @just init
    ./build/target/release/irox-build

ci:
    @just init
    ./build/target/release/irox-build ci

lints:
    @just init
    ./build/target/release/irox-build lints
