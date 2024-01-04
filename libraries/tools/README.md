IROX-TOOLS
===========

*Stuff that should have been in the Rust STL, but are not.*

### No-STD support:
* Include with `--no-default-features`

### Features:
* `std` (default) - Enables additional modules that make use of elements in `std` (annotated below)

### Modules:
* [`ansi_colors`](./src/ansi_colors.rs) - List of some basic ANSI Console colors
* [`arrays`](./src/arrays.rs) - Functions and tools for manipulating arrays of items.
* [`assert`](./src/assert.rs) - Additional assert macros for tests
* [`base64`](./src/base64.rs) - RFC-4648 compliant Base64, Base32, Base16 codecs
* [`bits`](./src/bits.rs) - Powerful Bit Buffer interfaces, giving `std::io::Read` more power
* [`codec`](./src/codec.rs) - A trait that provides `encode` and `decode` to convert to/from different byte encoding formats
* [`f64`](./src/f64.rs) - A collection of utilities for the f64 built-in
* [`format`](./src/fmt) - Formatting structs and traits
* [`hex`](./src/hex.rs) - Hexdump & Hex manipulation
* [`identifier`](./src/identifier.rs) - An Identifier represents a way to uniquely identify an item, whether as a `String`, `u64`, or `UUID`.
* [`murmur3`](./src/murmur3.rs) - Implementation of Murmurhash3
* [`options`](./src/options.rs) - `MaybeInto`, `MaybeFrom`, and `MaybeMap` - traits to improve chaining of optional functions and erasing `Result`s
* [`packetio`](./src/packetio.rs) - Traits for packetization of data and movement of packets of data
* [`random`](./src/random.rs) - Pseudo-Random Number Generators (PRNGs), implementation 