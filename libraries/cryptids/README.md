IROX-Cryptids
===========

*IROX Cryptographic Primitives - probably very hazardous*

:warning::construction: The primitives implemented here have not undergone any security analysis and likely never will.
These were built for my own educational purposes and shouldn't be used in production environments without exhaustive
testing and review. :construction::warning:

### No-STD support:

* Fully `no-std`/`no-alloc`

### Features:

* None

### Dependencies:

* [irox-bits](../bits)
* [irox-tools](../tools)

### Modules:

| Module                          | `[no_std]`? | `[no_alloc]`? |                   |
|---------------------------------|-------------|---------------|-------------------|
| [`aead`](./src/aead.rs)         | ![no_std]   | ![no_alloc]   | ![tested_rfc8439] | 
| [`chacha20`](./src/chacha20.rs) | ![no_std]   | ![no_alloc]   | ![tested_rfc8439] | 
| [`poly1305`](./src/poly1305.rs) | ![no_std]   | ![no_alloc]   | ![tested_rfc8439] | 
| [`pbkdf2`](./src/pbkdf2.rs)     | ![no_std]   | ![no_alloc]   | ![tested_rfc6070] |
| [`sha2`](./lib.rs)              | ![no_std]   | ![no_alloc]   | ![tested_nist]    |
| [`ed25519`](./src/ed25519.rs)   | ![no_std]   | ![no_alloc]   | ![tested_rfc8032] |
| [`x25519`](./src/x25519.rs)     | ![no_std]   | ![no_alloc]   | ![tested_rfc7748] |

[no_std]: https://img.shields.io/badge/no__std-yes-green

[no_alloc]: https://img.shields.io/badge/no__alloc-yes-green

[std]: https://img.shields.io/badge/feature-std-lightgrey

[untested]: https://img.shields.io/badge/passes%20tests-no-red

[tested_rfc6070]: https://img.shields.io/badge/passes%20tests-rfc6070-green
[tested_rfc7748]: https://img.shields.io/badge/passes%20tests-rfc7748-green
[tested_rfc8032]: https://img.shields.io/badge/passes%20tests-rfc8032-green
[tested_rfc8439]: https://img.shields.io/badge/passes%20tests-rfc8439-green
[tested_nist]: https://img.shields.io/badge/passes%20test-nist%20cavp-green

