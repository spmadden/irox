IROX-TOOLS
===========

*Stuff that should have been in the Rust STL, but are not.*

### No-STD support:

* Include with `--no-default-features`

### Features:

* `std` (default) - Enables additional modules that make use of elements in `std` (annotated below)

### Modules:

| Module                                         | `[no_std]`? |                                                                                                                   |
|------------------------------------------------|-------------|-------------------------------------------------------------------------------------------------------------------|
| [`ansi_colors`](./src/ansi_colors.rs)          | ![no_std]   | List of some basic ANSI Console colors                                                                            |
| [`arrays`](./src/arrays.rs)                    | ![no_std]   | Functions and tools for manipulating arrays of items.                                                             |
| [`assert`](./src/assert.rs)                    | ![no_std]   | Additional assert macros for tests                                                                                |
| [`codec`](./src/codec)                         | ![no_std]   | A trait that provides `encode` and `decode` to convert to/from different byte encoding formats                    |
| [`errors`](./src/errors.rs)                    | ![no_std]   | Macros to aid in the creation of crate-level error structs                                                        |
| [`fs`](./src/fs)                               | ![no_std]   | Filesystem utilities                                                                                              |
| [`fmt`](./src/fmt.rs)                          | ![no_std]   | Formatting structs and traits                                                                                     |                                                              
| [`hex`](./src/hex.rs)                          | ![no_std]   | Hexdump & Hex manipulation                                                                                        |                                                                  
| [`iterators`](./src/iterators)                 | ![no_std]   | Iterators adds the `Itertools` Trait, which adds a number of additional helper methods to the `Iterator` Trait.   |
| [`options`](./src/options.rs)                  | ![no_std]   | `MaybeInto`, `MaybeFrom`, and `MaybeMap` - traits to improve chaining of optional functions and erasing `Result`s |
| [`packetio`](./src/packetio.rs)                | ![no_std]   | Traits for packetization of data and movement of packets of data                                                  |
| [`random`](./src/random.rs)                    | ![no_std]   | Pseudo-Random Number Generators (PRNGs), implementation                                                           |
| [`read`](./src/read)                           | ![std]      | Helper functions around `std::io::Read`                                                                           |
| [`sync`](./src/sync)                           | ![std]      | More complex synchronization primitives than in the STD.                                                          |
| primitives: [`f32`](./src/primitives/f32.rs)   | ![no_std]   | A collection of utilities for the f32 built-in                                                                    |                                  
| primitives: [`f64`](./src/primitives/f64.rs)   | ![no_std]   | A collection of utilities for the f64 built-in                                                                    |                                  
| hash: [`murmur3`](./src/hash/murmur3.rs)       | ![no_std]   | Implementation of Murmurhash3                                                                                     |
| hash: [`md5`](./src/hash/md5.rs)               | ![no_std]   | Implementation of MD5 / [RFC-1321](https://datatracker.ietf.org/doc/html/rfc1321)                                 |
| hash: [`sha1`](./src/hash/sha1.rs)             | ![no_std]   | Implementation of SHA1 / [RFC-3174](https://datatracker.ietf.org/doc/html/rfc3174)                                |
| util: [`base64`](./src/util/base64.rs)         | ![no_std]   | RFC-4648 compliant Base64, Base32, Base16 codecs                                                                  |
| util: [`bits`](./src/util/bits.rs)             | ![no_std]   | Powerful Bit Buffer interfaces, giving `std::io::Read` more power                                                 |
| util: [`identifier`](./src/util/identifier.rs) | ![no_std]   | An Identifier represents a way to uniquely identify an item, whether as a `String`, `u64`, or `UUID`.             |
| util: [`scanner`](./src/util/scanner.rs)       | ![std]      | A utility to scan for tokens in a byte stream                                                                     |
| util: [`uuid`](./src/util/uuid.rs)             | ![no_std]   | A basic implementation of a UUID                                                                                  |

[no_std]: https://img.shields.io/badge/no__std-yes-green

[std]: https://img.shields.io/badge/feature-std-lightgrey
