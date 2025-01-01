// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//! # [Bits & Bobs](https://www.wordnik.com/words/bits%20and%20bobs).  No-std/No-alloc bit/byte manipulation of streams.
//!
//! The main traits provided by this crate are [`Bits`] (analogous to [`std::io::Read`]),
//! and [`MutBits`] (analogous to [`std::io::Write`]).
//!
//! Also included are multi-OS wrappers [`SeekRead`] and [`SeekWrite`], and a
//! more organic UTF-8 encoding/decoding setup for chars.
//!
//! This is an Irox foundational crate, it has no external dependencies, is `no-std`/`no-alloc` by
//! default (but can be augmented by using the `std` and `alloc` features), and many other irox
//! crates extend and augment the functionality contained herein.  [`irox_structs`] is notable as
//! it provides a derivable way to encode/decode structs as a sequence of bytes.
//!
//! ## Features ##
//! * `alloc`:
//!   * Enables the implementation of [`Bits`] and [`MutBits`] on the types from the [`alloc`] crate:
//!       * [`alloc::string::String`]
//!       * [`alloc::vec::Vec`]
//!       * [`alloc::collections::VecDeque`]
//!   * Enables the following additional methods:
//!     * [`Bits::read_u8_blob()`]  -> [`Vec<u8>`]
//!     * [`Bits::read_be_u16_blob()`]/[`Bits::read_le_u16_blob()`] -> [`Vec<u16>`]
//!     * [`Bits::read_be_u32_blob()`]/[`Bits::read_le_u32_blob()`] -> [`Vec<u32>`]
//!     * [`Bits::read_be_u64_blob()`]/[`Bits::read_le_u64_blob()`] -> [`Vec<u64>`]
//!     * [`Bits::read_all_vec()`] -> [`Vec<u8>`]
//!     * [`Bits::read_exact_vec()`] -> [`Vec<u8>`]
//!     * [`Bits::read_all_str_lossy()`] -> [`alloc::string::String`]
//!     * [`Bits::read_str_sized_lossy()`] -> [`alloc::string::String`]
//!     * [`Bits::read_str_u32_blob()`] -> [`alloc::string::String`]
//!     * [`Bits::read_until()`] -> [`Vec<u8>`]
//!     * [`Bits::consume_until()`] -> `()`
//! * `std`:
//!     * Enables the implementation of [`Bits`] and [`MutBits`] on the types from the `std` crate:
//!         * [`std::fs::File`]
//!     * Also enables the [`SeekRead`] and [`SeekWrite`] traits, which wrap and normalize:
//!         * [`std::os::windows::fs::FileExt::seek_read`] and [`std::os::windows::fs::FileExt::seek_write`]
//!         * [`std::os::unix::fs::FileExt:read_at`] and [`std::os::unix::fs::FileExt::write_at`]
//!

#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;

#[macro_use]
mod macros {
    /// Enables feature-specific code.
    /// Use this macro instead of `cfg(feature = "alloc")` to generate docs properly.
    macro_rules! cfg_feature_alloc {
        ($($item:item)*) => {
            $(
                #[cfg(any(all(doc, docsrs), feature = "alloc"))]
                #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
                $item
            )*
        }
    }

    /// Enables feature-specific code.
    /// Use this macro instead of `cfg(feature = "std")` to generate docs properly.
    macro_rules! cfg_feature_std {
        ($($item:item)*) => {
            $(
                #[cfg(any(all(doc, docsrs), feature = "std"))]
                #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
                $item
            )*
        }
    }
}

pub use bits::*;
pub use bitstream::*;
pub use buf::*;
pub use codec::*;
pub use error::*;
pub use mutbits::*;
pub use seek::*;
pub use stdwrappers::*;

cfg_feature_alloc! {
    mod allocimpls;
    pub use allocimpls::*;
}
mod bits;
mod codec;
mod error;
mod mutbits;
mod seek;

cfg_feature_std! {
    mod stdimpls;
}
mod bitstream;
mod buf;
mod stdwrappers;
pub mod utf;
