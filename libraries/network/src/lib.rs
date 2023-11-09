// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

#![forbid(unsafe_code)]

pub mod address;
pub mod r#async;
pub mod blocking;
pub mod http;
pub mod pool;
pub mod url;
#[cfg(feature = "websockets")]
pub mod websocket;
