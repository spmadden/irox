

## v0.2.0 (2026-11-04)

### New Features

 - <csr-id-ab5c4c9e8ab0b20c459c4f42555cb1fc7d555701/> improved unwrapping of decoded data
 - <csr-id-7d9255b8bc865a7eef6d10ecfc307c19184096fd/> ability to verify cleartext signed messages

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 270 calendar days.
 - 276 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-enums_derive v0.3.0, safety bump 29 crates ([`b5c9e4a`](https://github.com/spmadden/irox/commit/b5c9e4aca7410c9d0c2cb670f49c9dd9dc711bf3))
    - Improved unwrapping of decoded data ([`ab5c4c9`](https://github.com/spmadden/irox/commit/ab5c4c9e8ab0b20c459c4f42555cb1fc7d555701))
    - Ability to verify cleartext signed messages ([`7d9255b`](https://github.com/spmadden/irox/commit/7d9255b8bc865a7eef6d10ecfc307c19184096fd))
</details>

## v0.1.0 (2026-02-01)

### Chore

 - <csr-id-6420aae3a1d50dbaec8b2f6fa7f8ca04cf7fcd90/> fix lints & bump to 1.90
 - <csr-id-84ff4e9c2fb4577b9e009f5ec6c9b40ec3a26acc/> fix lints & bump to 1.86

### New Features

 - <csr-id-6eb6a2ca9c705f9f160c8380988ec10365d88c3b/> adding blake2 as custom hash impls
 - <csr-id-84f5173419fe167da34e7cb69a55a8963a36f341/> validator returns more info
 - <csr-id-22f353bd78b36d5bb7d99f34eca7d1aeb3bf0029/> SharedPubkey, SignatureSubpacket::Keyblocks, MultiKeybox, and verification of detatched signatures of files
 - <csr-id-93514cb4b9495c9e9d63907f5aef3e7e227e98ef/> use MPI type instead of Vec/Box for keys.
 - <csr-id-9c5c9f1e3f5ab959a698092e2f53429f5c2d54b3/> pubkey packet signature verification
 - <csr-id-a9c8d8da2c0e16145d29e5bd0ce3ae16848862a5/> more tests, keybox, validation progress
 - <csr-id-c8307162d0921641ec409b936aea5da6d64435cd/> basic armor decoding, basic serializations for some packets.
 - <csr-id-a2bf2b8a1d27fce23bcd6dacf51e6c0643891078/> add s2k algorithm
 - <csr-id-2b29e6e0ae2bc439c2a8752a58aa5ccb9f035b22/> super-basic signature validation
 - <csr-id-10ddde7b55ce501fe0338e14ef1e346c676c6e64/> new openpgp module

### Refactor

 - <csr-id-59f35131513a8b43d7e288832276943532e6bfee/> hexdump read -> bits

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release.
 - 13 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-openpgp v0.1.0 ([`6a39df8`](https://github.com/spmadden/irox/commit/6a39df81e09fa20cd5e7427beb3b217218330517))
    - Release irox-bits v0.5.0 ([`9a6d933`](https://github.com/spmadden/irox/commit/9a6d9331ad0ea937fea6635ef972e292881e9cda))
    - Hexdump read -> bits ([`59f3513`](https://github.com/spmadden/irox/commit/59f35131513a8b43d7e288832276943532e6bfee))
    - Fix lints & bump to 1.90 ([`6420aae`](https://github.com/spmadden/irox/commit/6420aae3a1d50dbaec8b2f6fa7f8ca04cf7fcd90))
    - Fix lints & bump to 1.86 ([`84ff4e9`](https://github.com/spmadden/irox/commit/84ff4e9c2fb4577b9e009f5ec6c9b40ec3a26acc))
    - Adding blake2 as custom hash impls ([`6eb6a2c`](https://github.com/spmadden/irox/commit/6eb6a2ca9c705f9f160c8380988ec10365d88c3b))
    - Validator returns more info ([`84f5173`](https://github.com/spmadden/irox/commit/84f5173419fe167da34e7cb69a55a8963a36f341))
    - SharedPubkey, SignatureSubpacket::Keyblocks, MultiKeybox, and verification of detatched signatures of files ([`22f353b`](https://github.com/spmadden/irox/commit/22f353bd78b36d5bb7d99f34eca7d1aeb3bf0029))
    - Use MPI type instead of Vec/Box for keys. ([`93514cb`](https://github.com/spmadden/irox/commit/93514cb4b9495c9e9d63907f5aef3e7e227e98ef))
    - Pubkey packet signature verification ([`9c5c9f1`](https://github.com/spmadden/irox/commit/9c5c9f1e3f5ab959a698092e2f53429f5c2d54b3))
    - More tests, keybox, validation progress ([`a9c8d8d`](https://github.com/spmadden/irox/commit/a9c8d8da2c0e16145d29e5bd0ce3ae16848862a5))
    - Basic armor decoding, basic serializations for some packets. ([`c830716`](https://github.com/spmadden/irox/commit/c8307162d0921641ec409b936aea5da6d64435cd))
    - Add s2k algorithm ([`a2bf2b8`](https://github.com/spmadden/irox/commit/a2bf2b8a1d27fce23bcd6dacf51e6c0643891078))
    - Super-basic signature validation ([`2b29e6e`](https://github.com/spmadden/irox/commit/2b29e6e0ae2bc439c2a8752a58aa5ccb9f035b22))
    - New openpgp module ([`10ddde7`](https://github.com/spmadden/irox/commit/10ddde7b55ce501fe0338e14ef1e346c676c6e64))
</details>

