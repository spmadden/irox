


## 0.6.0 (2026-11-04)

### New Features

 - <csr-id-921a2e3274d10bbe85854ae1cb37a97278ca91d9/> add offset tracking
 - <csr-id-2aba85eb45f6879a322a77cf60517b34a8aadfdf/> impl Struct for Vec<T>
 - <csr-id-90ba1d8ff752bab1a349e967921525dabb7c97c0/> impl Struct for Option
 - <csr-id-6a38a798d007e0a7f06720b750e214e97c792054/> new ability to derive EnumTryFromRepr (try_from(&Repr)) for [Repr(u8)] enums

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 927 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-enums_derive v0.3.0, safety bump 29 crates ([`b5c9e4a`](https://github.com/spmadden/irox/commit/b5c9e4aca7410c9d0c2cb670f49c9dd9dc711bf3))
    - Add offset tracking ([`921a2e3`](https://github.com/spmadden/irox/commit/921a2e3274d10bbe85854ae1cb37a97278ca91d9))
    - Impl Struct for Vec<T> ([`2aba85e`](https://github.com/spmadden/irox/commit/2aba85eb45f6879a322a77cf60517b34a8aadfdf))
    - Impl Struct for Option ([`90ba1d8`](https://github.com/spmadden/irox/commit/90ba1d8ff752bab1a349e967921525dabb7c97c0))
    - New ability to derive EnumTryFromRepr (try_from(&Repr)) for [Repr(u8)] enums ([`6a38a79`](https://github.com/spmadden/irox/commit/6a38a798d007e0a7f06720b750e214e97c792054))
</details>

## 0.5.1 (2024-04-21)

<csr-id-206ec2a2390f69081af79ca6bf069db1c13a647e/>

### Documentation

 - <csr-id-b15112657cf1ac9f77f12b2a4ffd70a777331910/> fix Cargo.toml to include local readme

### New Features

 - <csr-id-69e1a20649e8f78cebe54ae516c47697224eee41/> put 'as_bytes' method behind new 'alloc' feature

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 1 calendar day.
 - 20 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-structs v0.5.1 ([`e87bbb1`](https://github.com/spmadden/irox/commit/e87bbb14389ebc9e41e36ac34a020cf19a584805))
    - Fix Cargo.toml to include local readme ([`b151126`](https://github.com/spmadden/irox/commit/b15112657cf1ac9f77f12b2a4ffd70a777331910))
    - Put 'as_bytes' method behind new 'alloc' feature ([`69e1a20`](https://github.com/spmadden/irox/commit/69e1a20649e8f78cebe54ae516c47697224eee41))
    - Release irox-structs v0.5.1 ([`15bf0fd`](https://github.com/spmadden/irox/commit/15bf0fd3b6dd16c4433782983c76fb76d9bbb619))
    - Fix structs to use new bits module ([`206ec2a`](https://github.com/spmadden/irox/commit/206ec2a2390f69081af79ca6bf069db1c13a647e))
</details>

## 0.5.0 (2024-04-01)

<csr-id-9427c1392d00725f7ee3cc73ffb23e25a9dd72a1/>

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 79 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-structs v0.5.0 ([`0b03542`](https://github.com/spmadden/irox/commit/0b035422991c38db37653dc9f8e27b3b12743d6b))
    - Pretty much a full rewrite of the derive code to use less proc_macro2 ([`9427c13`](https://github.com/spmadden/irox/commit/9427c1392d00725f7ee3cc73ffb23e25a9dd72a1))
</details>

## 0.4.0 (2024-01-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 45 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.5.0, safety bump 17 crates ([`a46e9e2`](https://github.com/spmadden/irox/commit/a46e9e2da699f6ccd3a85b660014f0e15e59c0d0))
</details>

## 0.3.1 (2023-11-29)

<csr-id-88ebfb5deea5508ca54f4aaab62f6fd5a36f531c/>

### New Features

 - <csr-id-097319c31680f79ab5c5ef3850822764adf3d4ae/> expose Bits,MutBits to consumers

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 9 calendar days.
 - 31 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-structs v0.3.1 ([`934c196`](https://github.com/spmadden/irox/commit/934c196d87c63b830d1311d57b9606390acb5b1f))
    - Pivot to using Cargo.toml workspace lints ([`88ebfb5`](https://github.com/spmadden/irox/commit/88ebfb5deea5508ca54f4aaab62f6fd5a36f531c))
    - Expose Bits,MutBits to consumers ([`097319c`](https://github.com/spmadden/irox/commit/097319c31680f79ab5c5ef3850822764adf3d4ae))
</details>

## 0.3.0 (2023-10-29)

<csr-id-9c88391ce24d34786e9526378b4f3830f44da1fa/>

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 42 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-structs v0.3.0 ([`f5789b0`](https://github.com/spmadden/irox/commit/f5789b0aba04391dc38956e3538524bfe9a0b65c))
    - Add changelog ([`9c88391`](https://github.com/spmadden/irox/commit/9c88391ce24d34786e9526378b4f3830f44da1fa))
    - Release irox-tools v0.3.0, safety bump 12 crates ([`eb83b27`](https://github.com/spmadden/irox/commit/eb83b27b20c23e51e5b0fc3b7b3704e2c03af46c))
</details>

## v0.2.0 (2023-09-17)

<csr-id-80d2b88bdcb553faaeafc09673c31d7ebedafd19/>
<csr-id-1a365333397b02a5f911d0897c3bf0c80f6c2b80/>
<csr-id-dde7377b87baf75bfe664155378ae1a0b9639fcd/>

### New Features

 - <csr-id-1d102501e7a44c37461e77184031897b0ab07bc0/> update README.md and Cargo.toml
 - <csr-id-c48f8ae52f898bde42818ffaf42f140b1cc68bd8/> new struct serialization modules

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 21 calendar days.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-enums_derive v0.2.0, irox-enums v0.2.0, irox-tools v0.2.0, irox-units v0.2.0, irox-carto v0.2.0, irox-csv v0.2.0, irox-egui-extras v0.2.0, irox-networking v0.2.0, irox-types v0.2.0, irox-influxdb_v1 v0.2.0, irox-structs_derive v0.2.0, irox-structs v0.2.0, irox-nmea0183 v0.1.0, irox-sirf v0.2.0, irox-stats v0.2.0, irox-winlocation-api v0.1.0, irox v0.2.0, safety bump 10 crates ([`6a72204`](https://github.com/spmadden/irox/commit/6a722046661ceef02a66c2067e2c5c15ce102e04))
    - Update cargo.tomls to add repository ([`80d2b88`](https://github.com/spmadden/irox/commit/80d2b88bdcb553faaeafc09673c31d7ebedafd19))
    - Setting up blank changelogs for the modules ([`1a36533`](https://github.com/spmadden/irox/commit/1a365333397b02a5f911d0897c3bf0c80f6c2b80))
    - Update README.md and Cargo.toml ([`1d10250`](https://github.com/spmadden/irox/commit/1d102501e7a44c37461e77184031897b0ab07bc0))
    - There's no reason to have a separate error type, so it just returns std::io::Error now ([`dde7377`](https://github.com/spmadden/irox/commit/dde7377b87baf75bfe664155378ae1a0b9639fcd))
    - New struct serialization modules ([`c48f8ae`](https://github.com/spmadden/irox/commit/c48f8ae52f898bde42818ffaf42f140b1cc68bd8))
</details>

