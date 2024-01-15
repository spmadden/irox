// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//! More complex synchronization primitives than in the STD.

pub use once::*;
pub use optional::*;
mod eventual;
mod once;
mod optional;
