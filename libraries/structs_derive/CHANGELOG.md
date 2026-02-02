


## v0.5.0 (2026-02-01)

### Chore

 - <csr-id-b4557e31ce7e4e9335ec1e25753733b858293995/> bump version to 0.5.0

### New Features

 - <csr-id-b0386c20f52f1d585e7f74efe159b3e822dd138f/> ability to derive a basic SharedX variant of a struct that wraps the inner struct's fields with an RwLock
 - <csr-id-4e504268ee4b91d397e688542722a66495812895/> add todo!(), Self: Sized, and syn compile error helper

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 415 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version to 0.5.0 ([`b4557e3`](https://github.com/spmadden/irox/commit/b4557e31ce7e4e9335ec1e25753733b858293995))
    - Ability to derive a basic SharedX variant of a struct that wraps the inner struct's fields with an RwLock ([`b0386c2`](https://github.com/spmadden/irox/commit/b0386c20f52f1d585e7f74efe159b3e822dd138f))
    - Add todo!(), Self: Sized, and syn compile error helper ([`4e50426`](https://github.com/spmadden/irox/commit/4e504268ee4b91d397e688542722a66495812895))
</details>

## v0.4.0 (2024-12-13)

### New Features (BREAKING)

 - <csr-id-a9800369d86f46905c1309ca4e790220195807ec/> add new dynamic byte ordering support in Bits

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 13 calendar days.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-structs_derive v0.4.0 ([`c09cf6a`](https://github.com/spmadden/irox/commit/c09cf6a256ad7abdd5abf8d7376dd6ffcf6d0433))
    - Add new dynamic byte ordering support in Bits ([`a980036`](https://github.com/spmadden/irox/commit/a9800369d86f46905c1309ca4e790220195807ec))
</details>

## v0.3.2 (2024-06-28)

### New Features

 - <csr-id-036bba99979a49bf015c8355533b8a69ca09511f/> drop proc_macro2 as an internal dep
 - <csr-id-2099764e9e5811d11ce4280558ec280fb9a9dbcc/> new proc_macro derive-helpers module

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 37 calendar days.
 - 68 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-structs_derive v0.3.2 ([`bfd40ea`](https://github.com/spmadden/irox/commit/bfd40eace98e42707aeab8b9ea1b5ca6955f94cb))
    - Drop proc_macro2 as an internal dep ([`036bba9`](https://github.com/spmadden/irox/commit/036bba99979a49bf015c8355533b8a69ca09511f))
    - New proc_macro derive-helpers module ([`2099764`](https://github.com/spmadden/irox/commit/2099764e9e5811d11ce4280558ec280fb9a9dbcc))
</details>

## v0.3.1 (2024-04-21)

<csr-id-206ec2a2390f69081af79ca6bf069db1c13a647e/>

### Bug Fixes

 - <csr-id-f0e7ae115ce7529c398c0731d8928b12d6d90cae/> change derive to export structs rather than irox_bits directly

### Refactor

 - <csr-id-206ec2a2390f69081af79ca6bf069db1c13a647e/> fix structs to use new bits module

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 20 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-structs_derive v0.3.1 ([`07571ee`](https://github.com/spmadden/irox/commit/07571ee5e3c44888477ae3e832ecc3d8aef939d0))
    - Change derive to export structs rather than irox_bits directly ([`f0e7ae1`](https://github.com/spmadden/irox/commit/f0e7ae115ce7529c398c0731d8928b12d6d90cae))
    - Fix structs to use new bits module ([`206ec2a`](https://github.com/spmadden/irox/commit/206ec2a2390f69081af79ca6bf069db1c13a647e))
</details>

## v0.3.0 (2024-04-01)

<csr-id-56ab68398e06e2df370cf615665988ee62346999/>
<csr-id-9427c1392d00725f7ee3cc73ffb23e25a9dd72a1/>

### Chore

 - <csr-id-56ab68398e06e2df370cf615665988ee62346999/> lints and fmt

### New Features

 - <csr-id-00590ab1f9749f74bf3a2cc54dea8635328555d1/> add little-endian support for derived structs

### Refactor (BREAKING)

 - <csr-id-9427c1392d00725f7ee3cc73ffb23e25a9dd72a1/> pretty much a full rewrite of the derive code to use less proc_macro2

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-structs_derive v0.3.0 ([`c590e04`](https://github.com/spmadden/irox/commit/c590e0406b0f1551d774bf48ba24306597121811))
    - Pretty much a full rewrite of the derive code to use less proc_macro2 ([`9427c13`](https://github.com/spmadden/irox/commit/9427c1392d00725f7ee3cc73ffb23e25a9dd72a1))
    - Lints and fmt ([`56ab683`](https://github.com/spmadden/irox/commit/56ab68398e06e2df370cf615665988ee62346999))
    - Add little-endian support for derived structs ([`00590ab`](https://github.com/spmadden/irox/commit/00590ab1f9749f74bf3a2cc54dea8635328555d1))
</details>

## v0.2.3 (2023-11-29)

<csr-id-88ebfb5deea5508ca54f4aaab62f6fd5a36f531c/>

### Chore

 - <csr-id-88ebfb5deea5508ca54f4aaab62f6fd5a36f531c/> pivot to using Cargo.toml workspace lints

### New Features

 - <csr-id-29987c2492e82336d874a5f0cc2082d565faa5bf/> derive the new Primitive blob types

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 9 calendar days.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-structs_derive v0.2.3 ([`7cf3f82`](https://github.com/spmadden/irox/commit/7cf3f823cdac76b71f6aece013deecab4bd39e5a))
    - Pivot to using Cargo.toml workspace lints ([`88ebfb5`](https://github.com/spmadden/irox/commit/88ebfb5deea5508ca54f4aaab62f6fd5a36f531c))
    - Derive the new Primitive blob types ([`29987c2`](https://github.com/spmadden/irox/commit/29987c2492e82336d874a5f0cc2082d565faa5bf))
</details>

## v0.2.2 (2023-10-29)

### Bug Fixes

 - <csr-id-c608a4c0995d087749df00ff3b3053b25edffcec/> bump version to 0.2.2

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 22 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-structs_derive v0.2.2 ([`34beb37`](https://github.com/spmadden/irox/commit/34beb379d42063a43f36ccc44919fdc8c0b7ce83))
    - Bump version to 0.2.2 ([`c608a4c`](https://github.com/spmadden/irox/commit/c608a4c0995d087749df00ff3b3053b25edffcec))
</details>

## v0.2.1 (2023-10-07)

<csr-id-9725e632a2d780013e1ac3b58449c4be9ffd951d/>

### Chore

 - <csr-id-9725e632a2d780013e1ac3b58449c4be9ffd951d/> locking deps to current versions
   This should enable us to verify behaviors through tests, and not rely on a dev's goodwill and semver versioning to ensure the tools and libraries work.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 19 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.2.1, irox-carto v0.2.1, irox-egui-extras v0.2.1, irox-gpx v0.1.0, irox-types v0.2.1, irox-structs_derive v0.2.1, irox-raymarine-sonar v0.1.0, irox-stats v0.2.1, irox-winlocation-api v0.1.1, irox v0.2.1 ([`68d770b`](https://github.com/spmadden/irox/commit/68d770bb78abe49bf30364ca17ddb6f7bfda05d9))
    - Locking deps to current versions ([`9725e63`](https://github.com/spmadden/irox/commit/9725e632a2d780013e1ac3b58449c4be9ffd951d))
</details>

## v0.2.0 (2023-09-17)

<csr-id-f03d8a3ec997d53470bfdeb5e76b71925aac3f10/>
<csr-id-80d2b88bdcb553faaeafc09673c31d7ebedafd19/>
<csr-id-1a365333397b02a5f911d0897c3bf0c80f6c2b80/>
<csr-id-dde7377b87baf75bfe664155378ae1a0b9639fcd/>

### Chore

 - <csr-id-f03d8a3ec997d53470bfdeb5e76b71925aac3f10/> clean up code with additional lints
 - <csr-id-80d2b88bdcb553faaeafc09673c31d7ebedafd19/> update cargo.tomls to add repository
 - <csr-id-1a365333397b02a5f911d0897c3bf0c80f6c2b80/> setting up blank changelogs for the modules

### New Features

 - <csr-id-1d102501e7a44c37461e77184031897b0ab07bc0/> update README.md and Cargo.toml
 - <csr-id-c48f8ae52f898bde42818ffaf42f140b1cc68bd8/> new struct serialization modules

### Bug Fixes

 - <csr-id-74f6a74ed05e38eefa6b6298e8bd2835fdb29618/> add specific version

### Refactor

 - <csr-id-dde7377b87baf75bfe664155378ae1a0b9639fcd/> There's no reason to have a separate error type, so it just returns std::io::Error now

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 21 calendar days.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-enums_derive v0.2.0, irox-enums v0.2.0, irox-tools v0.2.0, irox-units v0.2.0, irox-carto v0.2.0, irox-csv v0.2.0, irox-egui-extras v0.2.0, irox-networking v0.2.0, irox-types v0.2.0, irox-influxdb_v1 v0.2.0, irox-structs_derive v0.2.0, irox-structs v0.2.0, irox-nmea0183 v0.1.0, irox-sirf v0.2.0, irox-stats v0.2.0, irox-winlocation-api v0.1.0, irox v0.2.0, safety bump 10 crates ([`6a72204`](https://github.com/spmadden/irox/commit/6a722046661ceef02a66c2067e2c5c15ce102e04))
    - Clean up code with additional lints ([`f03d8a3`](https://github.com/spmadden/irox/commit/f03d8a3ec997d53470bfdeb5e76b71925aac3f10))
    - Update cargo.tomls to add repository ([`80d2b88`](https://github.com/spmadden/irox/commit/80d2b88bdcb553faaeafc09673c31d7ebedafd19))
    - Setting up blank changelogs for the modules ([`1a36533`](https://github.com/spmadden/irox/commit/1a365333397b02a5f911d0897c3bf0c80f6c2b80))
    - Update README.md and Cargo.toml ([`1d10250`](https://github.com/spmadden/irox/commit/1d102501e7a44c37461e77184031897b0ab07bc0))
    - There's no reason to have a separate error type, so it just returns std::io::Error now ([`dde7377`](https://github.com/spmadden/irox/commit/dde7377b87baf75bfe664155378ae1a0b9639fcd))
    - Add specific version ([`74f6a74`](https://github.com/spmadden/irox/commit/74f6a74ed05e38eefa6b6298e8bd2835fdb29618))
    - New struct serialization modules ([`c48f8ae`](https://github.com/spmadden/irox/commit/c48f8ae52f898bde42818ffaf42f140b1cc68bd8))
</details>

