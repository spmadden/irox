#!/bin/bash
# SPDX-License-Identifier: MIT
# Copyright 2024 IROX Contributors
#

set -euxo pipefail

if [[ ! `command -v rustup` ]]; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  . ~/.cargo/env
  if [[ ! `command -v rustup` ]]; then
    echo "Cannot find rustup, and could not install it.";
    exit -1
  fi
fi

rustup update
dnf install gcc-c++ make perl-FindBin perl-IPC-Cmd perl-File-Compare perl-File-Copy openssl-devel cmake
