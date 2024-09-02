#!/usr/bin/env -S just --justfile

# Set shell for Windows OSs:
set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

run *PARAMS='':
    cd build && cargo build --release && cd ..
    ./build/target/release/irox-builder {{PARAMS}}
