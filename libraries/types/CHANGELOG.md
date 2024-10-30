


## v0.4.4 (2024-10-29)

### New Features

 - <csr-id-7263abdd9e0eec30dd24c7ab7dcebc5d5f6e4f11/> Primitives can go to and from u8 now.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 5 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Primitives can go to and from u8 now. ([`7263abd`](https://github.com/spmadden/irox/commit/7263abdd9e0eec30dd24c7ab7dcebc5d5f6e4f11))
</details>

## v0.4.3 (2024-10-24)

### New Features

 - <csr-id-cc714380db2c733153ccf938df2ec939c4c8ddd4/> require FloatExt for 'AnyFloat'
 - <csr-id-421971094cd6b1b3288c71c3452931e211df7696/> impl Into<PrimitiveValue> for lots of stuff
 - <csr-id-a53e614d4029a5512b5a5605ad4925a8f1926500/> Add traits describing the number primitives

### Bug Fixes

 - <csr-id-1a87db6fda93f333ff81c62080ef1c9859bc4dbf/> cleaning up the NumberTraits to remove a bunch of lifetimes dicking with intended uses.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 118 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-types v0.4.3 ([`d729ed9`](https://github.com/spmadden/irox/commit/d729ed97cff8ccd01e684798b109c9e88185c7cc))
    - Require FloatExt for 'AnyFloat' ([`cc71438`](https://github.com/spmadden/irox/commit/cc714380db2c733153ccf938df2ec939c4c8ddd4))
    - Impl Into<PrimitiveValue> for lots of stuff ([`4219710`](https://github.com/spmadden/irox/commit/421971094cd6b1b3288c71c3452931e211df7696))
    - Cleaning up the NumberTraits to remove a bunch of lifetimes dicking with intended uses. ([`1a87db6`](https://github.com/spmadden/irox/commit/1a87db6fda93f333ff81c62080ef1c9859bc4dbf))
    - Add traits describing the number primitives ([`a53e614`](https://github.com/spmadden/irox/commit/a53e614d4029a5512b5a5605ad4925a8f1926500))
</details>

## v0.4.2 (2024-06-28)

### New Features

 - <csr-id-fe504a4675476e31bc550165b0d65c7872d82812/> New Type Schemas struct
 - <csr-id-cb5e5b1be16504b4e54a40b13701c36f439f1dd2/> impl Hash for PrimitiveValue
 - <csr-id-f0a4067befb8ad716a441244a22e4a0448ba69e0/> improved ability to encode/decode variable types

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 68 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-types v0.4.2 ([`490f3d7`](https://github.com/spmadden/irox/commit/490f3d76f8f675180b62415b413dda7f8d125fa3))
    - New Type Schemas struct ([`fe504a4`](https://github.com/spmadden/irox/commit/fe504a4675476e31bc550165b0d65c7872d82812))
    - Impl Hash for PrimitiveValue ([`cb5e5b1`](https://github.com/spmadden/irox/commit/cb5e5b1be16504b4e54a40b13701c36f439f1dd2))
    - Improved ability to encode/decode variable types ([`f0a4067`](https://github.com/spmadden/irox/commit/f0a4067befb8ad716a441244a22e4a0448ba69e0))
</details>

## v0.4.1 (2024-04-21)

### New Features

 - <csr-id-198a9e727016b05d6f6d3537035d8af80ac2073e/> new variable value type

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 20 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-types v0.4.1 ([`071d012`](https://github.com/spmadden/irox/commit/071d0125b198ff9cfb28a0bdade122dda6fbe0f6))
    - New variable value type ([`198a9e7`](https://github.com/spmadden/irox/commit/198a9e727016b05d6f6d3537035d8af80ac2073e))
</details>

## v0.4.0 (2024-04-01)

<csr-id-4e8bd7e566d5eb0eda3e7a4e0992abcc05da389c/>

### Refactor (BREAKING)

 - <csr-id-4e8bd7e566d5eb0eda3e7a4e0992abcc05da389c/> Moved the variably sized elements from Primitives into new enum

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 123 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-types v0.4.0 ([`d81a518`](https://github.com/spmadden/irox/commit/d81a518812f731224867d9409c8140be47ec95ac))
    - Moved the variably sized elements from Primitives into new enum ([`4e8bd7e`](https://github.com/spmadden/irox/commit/4e8bd7e566d5eb0eda3e7a4e0992abcc05da389c))
</details>

## v0.3.0 (2023-11-28)

<csr-id-88ebfb5deea5508ca54f4aaab62f6fd5a36f531c/>

### Chore

 - <csr-id-88ebfb5deea5508ca54f4aaab62f6fd5a36f531c/> pivot to using Cargo.toml workspace lints

### New Features

 - <csr-id-6919741e9ff5ca0b599e80a84c701b2fdb80a6af/> support converting a 'Vec<u8>' into a Primitives::u32_blob

### New Features (BREAKING)

 - <csr-id-69dd29057e64e9313e95d7e3be7ec3b948f9c1a9/> refactor 'Primitives::blob' into sized types

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 30 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-types v0.3.0 ([`16dce5d`](https://github.com/spmadden/irox/commit/16dce5d78a0cde18db160731b536059e204877a1))
    - Pivot to using Cargo.toml workspace lints ([`88ebfb5`](https://github.com/spmadden/irox/commit/88ebfb5deea5508ca54f4aaab62f6fd5a36f531c))
    - Support converting a 'Vec<u8>' into a Primitives::u32_blob ([`6919741`](https://github.com/spmadden/irox/commit/6919741e9ff5ca0b599e80a84c701b2fdb80a6af))
    - Refactor 'Primitives::blob' into sized types ([`69dd290`](https://github.com/spmadden/irox/commit/69dd29057e64e9313e95d7e3be7ec3b948f9c1a9))
</details>

## v0.2.2 (2023-10-29)

<csr-id-645afe2dca3b6f200aa6a8ed73c157316d30943c/>

### Chore

 - <csr-id-645afe2dca3b6f200aa6a8ed73c157316d30943c/> ALL THE LINTS.

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
    - Release irox-types v0.2.2 ([`9776f77`](https://github.com/spmadden/irox/commit/9776f7787fb3cbdfe84d96d5e0c2360d59f64e57))
    - ALL THE LINTS. ([`645afe2`](https://github.com/spmadden/irox/commit/645afe2dca3b6f200aa6a8ed73c157316d30943c))
</details>

## v0.2.1 (2023-10-07)

### New Features

 - <csr-id-6d31e4a52e0fc72a70de4970e414b6980e670316/> impl ToString for PrimitiveValue
 - <csr-id-4e0bc913e685e2e7d02a6186b68282cb3aaaae2e/> add 'blob' type

### Bug Fixes

 - <csr-id-9f90435643db234a3c4403d56926f813ecbafce2/> fix blob to_string

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 19 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.2.1, irox-carto v0.2.1, irox-egui-extras v0.2.1, irox-gpx v0.1.0, irox-types v0.2.1, irox-structs_derive v0.2.1, irox-raymarine-sonar v0.1.0, irox-stats v0.2.1, irox-winlocation-api v0.1.1, irox v0.2.1 ([`68d770b`](https://github.com/spmadden/irox/commit/68d770bb78abe49bf30364ca17ddb6f7bfda05d9))
    - Fix blob to_string ([`9f90435`](https://github.com/spmadden/irox/commit/9f90435643db234a3c4403d56926f813ecbafce2))
    - Impl ToString for PrimitiveValue ([`6d31e4a`](https://github.com/spmadden/irox/commit/6d31e4a52e0fc72a70de4970e414b6980e670316))
    - Add 'blob' type ([`4e0bc91`](https://github.com/spmadden/irox/commit/4e0bc913e685e2e7d02a6186b68282cb3aaaae2e))
</details>

## v0.2.0 (2023-09-17)

<csr-id-80d2b88bdcb553faaeafc09673c31d7ebedafd19/>
<csr-id-1a365333397b02a5f911d0897c3bf0c80f6c2b80/>

### Chore

 - <csr-id-80d2b88bdcb553faaeafc09673c31d7ebedafd19/> update cargo.tomls to add repository
 - <csr-id-1a365333397b02a5f911d0897c3bf0c80f6c2b80/> setting up blank changelogs for the modules

### New Features

 - <csr-id-f8e9213f63cc0931a4d94cb3fc26ad1229d0d644/> add keywords & categories to Cargo.toml, update README.md
 - <csr-id-302da92cc76fb50c883bc9dd7fe29fcc5492183a/> derive EnumName and others for types module
 - <csr-id-ce70857680aa5243227f45db5ecbaee132b7ab68/> refactor types from tools into it's own module

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-enums_derive v0.2.0, irox-enums v0.2.0, irox-tools v0.2.0, irox-units v0.2.0, irox-carto v0.2.0, irox-csv v0.2.0, irox-egui-extras v0.2.0, irox-networking v0.2.0, irox-types v0.2.0, irox-influxdb_v1 v0.2.0, irox-structs_derive v0.2.0, irox-structs v0.2.0, irox-nmea0183 v0.1.0, irox-sirf v0.2.0, irox-stats v0.2.0, irox-winlocation-api v0.1.0, irox v0.2.0, safety bump 10 crates ([`6a72204`](https://github.com/spmadden/irox/commit/6a722046661ceef02a66c2067e2c5c15ce102e04))
    - Update cargo.tomls to add repository ([`80d2b88`](https://github.com/spmadden/irox/commit/80d2b88bdcb553faaeafc09673c31d7ebedafd19))
    - Setting up blank changelogs for the modules ([`1a36533`](https://github.com/spmadden/irox/commit/1a365333397b02a5f911d0897c3bf0c80f6c2b80))
    - Add keywords & categories to Cargo.toml, update README.md ([`f8e9213`](https://github.com/spmadden/irox/commit/f8e9213f63cc0931a4d94cb3fc26ad1229d0d644))
    - Derive EnumName and others for types module ([`302da92`](https://github.com/spmadden/irox/commit/302da92cc76fb50c883bc9dd7fe29fcc5492183a))
    - Refactor types from tools into it's own module ([`ce70857`](https://github.com/spmadden/irox/commit/ce70857680aa5243227f45db5ecbaee132b7ab68))
</details>

