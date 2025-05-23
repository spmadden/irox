// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! OpenPGP Packet IO
//!

#![forbid(unsafe_code)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![cfg_attr(docsrs, feature(doc_cfg))]
extern crate core;

pub mod armor;
pub mod keybox;
pub mod keygrip;
pub mod packets;
pub mod types;
pub mod validator;
