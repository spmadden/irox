// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

//!
//! Helper functions around [`std::io::Read`]

extern crate alloc;

use crate::cfg_feature_std;
#[cfg(all(feature = "std", any(unix, windows)))]
pub use pagefile::*;

pub use buffer::*;
mod buffer;

#[cfg(all(feature = "std", any(unix, windows)))]
mod multi_stream;
#[cfg(all(feature = "std", any(unix, windows)))]
pub use multi_stream::*;

#[cfg(all(feature = "std", any(unix, windows)))]
mod pagefile;

cfg_feature_std! {
    mod conv;
    pub use conv::*;

    mod counting;
    pub use counting::*;

    mod readerator;
    pub use readerator::*;

    mod read_std;
    pub use read_std::*;
}
