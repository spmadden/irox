IROX-TOOLS
===========

*Stuff that should have been in the Rust STL, but are not.*

### No-STD support:

* Include with `--no-default-features`

### Features:

* `std` (default) - Enables additional modules that make use of elements in `std` (annotated below)

### Modules:

| Module                                   | `#[no_std]`? |                                                                                                                   |
|------------------------------------------|--------------|-------------------------------------------------------------------------------------------------------------------|
| [`ansi_colors`](./src/ansi_colors.rs)    | ![no_std]    | List of some basic ANSI Console colors                                                                            |
| [`arrays`](./src/arrays.rs)              | ![no_std]    | Functions and tools for manipulating arrays of items.                                                             |
| [`assert`](./src/assert.rs)              | ![no_std]    | Additional assert macros for tests                                                                                |
| [`base64`](./src/util/base64.rs)         | ![no_std]    | RFC-4648 compliant Base64, Base32, Base16 codecs                                                                  |
| [`bits`](./src/util/bits.rs)             | ![no_std]    | Powerful Bit Buffer interfaces, giving `std::io::Read` more power                                                 |
| [`codec`](./src/codec)                   | ![no_std]    | A trait that provides `encode` and `decode` to convert to/from different byte encoding formats                    |
| [`errors`](./src/errors.rs)              | ![no_std]    | Macros to aid in the creation of crate-level error structs                                                        |
| [`fs`](./src/fs)                         | ![no_std]    | Filesystem utilities                                                                                              |
| [`f64`](./src/primitives/f64.rs)         | ![no_std]    | A collection of utilities for the f64 built-in                                                                    |                                  
| [`fmt`](./src/fmt.rs)                    | ![no_std]    | Formatting structs and traits                                                                                     |                                                              
| [`hex`](./src/hex.rs)                    | ![no_std]    | Hexdump & Hex manipulation                                                                                        |                                                                  
| [`identifier`](./src/util/identifier.rs) | ![no_std]    | An Identifier represents a way to uniquely identify an item, whether as a `String`, `u64`, or `UUID`.             |
| [`iterators`](./src/iterators)           | ![no_std]    | Iterators adds the `Itertools` Trait, which adds a number of additional helper methods to the `Iterator` Trait.   |
| [`murmur3`](./src/murmur3.rs)            | ![no_std]    | Implementation of Murmurhash3                                                                                     |
| [`options`](./src/options.rs)            | ![no_std]    | `MaybeInto`, `MaybeFrom`, and `MaybeMap` - traits to improve chaining of optional functions and erasing `Result`s |
| [`packetio`](./src/packetio.rs)          | ![std]       | Traits for packetization of data and movement of packets of data                                                  |
| [`random`](./src/random.rs)              | ![no_std]    | Pseudo-Random Number Generators (PRNGs), implementation                                                           |
| [`read`](./src/read)                     | ![std]       | Helper functions around `std::io::Read`                                                                           |
| [`scanner`](./src/util/scanner.rs)       | ![std]       | A utility to scan for tokens in a byte stream                                                                     |
| [`sync`](./src/sync)                     | ![std]       | More complex synchronization primitives than in the STD.                                                          |
| [`uuid`](./src/util/uuid.rs)             | ![no_std]    | A basic implementation of a UUID                                                                                  |

[no_std]: https://img.shields.io/badge/no__std-yes-green

[std]: https://img.shields.io/badge/feature-std-lightgrey
