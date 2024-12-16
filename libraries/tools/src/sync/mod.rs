// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//! More complex synchronization primitives than in the STD.

use crate::cfg_feature_std;
pub use eventual::*;
pub use flags::*;
pub use optional::*;
mod eventual;
cfg_feature_std! {
    mod once;
    pub use exchange::*;
    mod exchange;
}
mod flags;
mod optional;
