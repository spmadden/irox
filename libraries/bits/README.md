IROX-BITS
===========

*Bits & Bobs. No-std/No-alloc bit/byte manipulation of streams.*

The main traits provided by this crate are [`Bits`] (analogous to [`std::io::Read`]), and [`MutBits`] (analogous
to [`std::io::Write`]).

Also included are multi-OS wrappers [`SeekRead`] and [`SeekWrite`], and a
more organic UTF-8 encoding/decoding setup for chars.

This is an Irox foundational crate, it has no external dependencies, is `no-std`/`no-alloc` by
default (but can be augmented by using the `std` and `alloc` features), and many other irox
crates extend and augment the functionality contained herein.  [`irox_structs`] is notable as
it provides a derivable way to encode/decode structs as a sequence of bytes.

### No-STD support:

* The core traits [`Bits`] and [`MutBits`] and the majority of the functionality of this crate are
  available `no-std`/`no-alloc`.

### Features:

* `alloc`:
    * Enables the implementation of [`Bits`] and [`MutBits`] on the types from the `alloc` crate:
        * [`alloc::string::String`]
        * [`alloc::vec::Vec`]
        * [`alloc::collections::VecDeque`]
    * Enables the following additional methods:
        * [`Bits::read_u8_blob()`] -> `Vec<u8>`
        * [`Bits::read_be_u16_blob()`]/[`Bits::read_le_u16_blob()`] -> `Vec<u16>`
        * [`Bits::read_be_u32_blob()`]/[`Bits::read_le_u32_blob()`] -> `Vec<u32>`
        * [`Bits::read_be_u64_blob()`]/[`Bits::read_le_u64_blob()`] -> `Vec<u64>`
        * [`Bits::read_all_vec()`]  -> `Vec<u8>`
        * [`Bits::read_exact_vec()`]  -> `Vec<u8>`
        * [`Bits::read_all_str_lossy()`] -> [`alloc::string::String`]
        * [`Bits::read_str_sized_lossy()`] -> [`alloc::string::String`]
        * [`Bits::read_str_u32_blob()`] -> [`alloc::string::String`]
        * [`Bits::read_until()`] -> `Vec<u8>`
        * [`Bits::consume_until()`] -> `()`
        * [`MutBits::write_fmt_impl()`]
* `std`:
    * Enables the implementation of [`Bits`] and [`MutBits`] on the types from the `std` crate:
        * [`std::fs::File`]
    * Also enables the [`SeekRead`] and [`SeekWrite`] traits, which wrap and normalize:
        * [`std::os::windows::fs::FileExt::seek_read`] and [`std::os::windows::fs::FileExt::seek_write`]
        * [`std::os::unix::fs::FileExt:read_at`] and [`std::os::unix::fs::FileExt::write_at`]
    * Enables [`std::error::Error`] compatibility with [`BitsError`]
    * Enables (nearly) seamless translation between [`std::io::Error`] / [`BitsError`]
      and [`std::io::ErrorKind`] / [`BitsErrorKind`]