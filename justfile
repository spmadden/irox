#!/usr/bin/env -S just --justfile

run *PARAMS='':
    cd build && cargo build --release && cd ..
    ./build/target/release/irox-builder {{PARAMS}}
