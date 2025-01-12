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
| [`aes`](./src/aes.rs)           | ![no_std]   | ![no_alloc]   | ![untested]       |
| [`chacha20`](./src/chacha20.rs) | ![no_std]   | ![no_alloc]   | ![tested_rfc8439] | 

[no_std]: https://img.shields.io/badge/no__std-yes-green

[no_alloc]: https://img.shields.io/badge/no__alloc-yes-green

[std]: https://img.shields.io/badge/feature-std-lightgrey

[untested]: https://img.shields.io/badge/passes%20tests-no-red

[tested_rfc8439]: https://img.shields.io/badge/passes%20tests-rfc8439-green
