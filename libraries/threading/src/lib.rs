// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

//!
//! Multithreading and Asynchronous programming tools and utilities
//!

#![forbid(unsafe_code)]

pub use current::*;
pub use single::*;
pub use task::*;

mod current;
mod single;
mod task;

pub trait Executor {}
