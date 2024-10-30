


## v0.2.7 (2024-10-29)

### Chore

 - <csr-id-2747f689e2206435cdd1ee8bab43ad9442415f20/> update deps

### New Features

 - <csr-id-b21947ab6d854b37712535f92681beed2759c7a2/> new Summary struct and one second streaming windows stats

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 5 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - New Summary struct and one second streaming windows stats ([`b21947a`](https://github.com/spmadden/irox/commit/b21947ab6d854b37712535f92681beed2759c7a2))
    - Update deps ([`2747f68`](https://github.com/spmadden/irox/commit/2747f689e2206435cdd1ee8bab43ad9442415f20))
</details>

## v0.2.6 (2024-10-24)

<csr-id-a35975360f42880d6e74ceb4443ccd4093c27975/>

### Chore

 - <csr-id-a35975360f42880d6e74ceb4443ccd4093c27975/> fixup lints & formatting

### New Features

 - <csr-id-d8da320deb6fade1ed8a1ae225af7d28d16714a3/> more work on TimeSeriesDataWriters, TSDF gets "varying floats" about 2x compressed than straight deflate
 - <csr-id-58c16d115737ad98dd8deb0852e170378b7c2a78/> improved layering ability for the statistical compression streams
 - <csr-id-f1c45cd905228e9f38f5c537148a329b971d3140/> new statistical lossless streaming encoders
 - <csr-id-df11e3da1d562ecb184f7bde002b608dd494f47f/> new Point2D trait, with impls Float2D, Double2D, Quad2D
 - <csr-id-98d5046d137ecb02f5270ff794de182df044c606/> add new unlimited/paged buffer that does not reallocate

### Bug Fixes

 - <csr-id-58b51d507bb1722c0dd2896a734a8ba6dfe884f2/> remove unused tdigest module
 - <csr-id-e8ef0ace18ba71f7aedb871040488b6a5ecaa680/> fix stats tests

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release.
 - 159 days passed between releases.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-stats v0.2.6 ([`9f599be`](https://github.com/spmadden/irox/commit/9f599bea75ab85beab95f646358fdb4074dcc1c5))
    - Remove unused tdigest module ([`58b51d5`](https://github.com/spmadden/irox/commit/58b51d507bb1722c0dd2896a734a8ba6dfe884f2))
    - Release irox-bits v0.2.0 ([`3ed7b85`](https://github.com/spmadden/irox/commit/3ed7b850a87bfc670ce18f5c824008f09b0af7b4))
    - Fix stats tests ([`e8ef0ac`](https://github.com/spmadden/irox/commit/e8ef0ace18ba71f7aedb871040488b6a5ecaa680))
    - More work on TimeSeriesDataWriters, TSDF gets "varying floats" about 2x compressed than straight deflate ([`d8da320`](https://github.com/spmadden/irox/commit/d8da320deb6fade1ed8a1ae225af7d28d16714a3))
    - Improved layering ability for the statistical compression streams ([`58c16d1`](https://github.com/spmadden/irox/commit/58c16d115737ad98dd8deb0852e170378b7c2a78))
    - New statistical lossless streaming encoders ([`f1c45cd`](https://github.com/spmadden/irox/commit/f1c45cd905228e9f38f5c537148a329b971d3140))
    - New Point2D trait, with impls Float2D, Double2D, Quad2D ([`df11e3d`](https://github.com/spmadden/irox/commit/df11e3da1d562ecb184f7bde002b608dd494f47f))
    - Fixup lints & formatting ([`a359753`](https://github.com/spmadden/irox/commit/a35975360f42880d6e74ceb4443ccd4093c27975))
    - Add new unlimited/paged buffer that does not reallocate ([`98d5046`](https://github.com/spmadden/irox/commit/98d5046d137ecb02f5270ff794de182df044c606))
</details>

## v0.2.5 (2024-05-18)

### New Features

 - <csr-id-943df24540ecbce2a596363476d37b39f6a0018f/> add streaming-min, tdigest mod

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 47 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-stats v0.2.5 ([`38375e2`](https://github.com/spmadden/irox/commit/38375e207080946ec8c97efe7e88e501a15f68c4))
    - Add streaming-min, tdigest mod ([`943df24`](https://github.com/spmadden/irox/commit/943df24540ecbce2a596363476d37b39f6a0018f))
</details>

## v0.2.4 (2024-04-01)

<csr-id-46dc6bfb95b3860eba5f9eb47394c92acd5ac502/>

### New Features

 - <csr-id-96c41ef90232cf89154a0fddb71af0f265daed84/> new streaming statistics features
 - <csr-id-3bae95bd404f99819a47ab95a7af3f97c2764f9b/> stats is now no_std

### Bug Fixes

 - <csr-id-b2f572d459b23fb761eff24daafda319ac8ecc7a/> add eps value to tests
 - <csr-id-2a25738da0f48bc7436054295dadaba15da6b64a/> add version to stats dep to quiet warning

### Other

 - <csr-id-46dc6bfb95b3860eba5f9eb47394c92acd5ac502/> updating docs for stats

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 28 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-stats v0.2.4 ([`77715c7`](https://github.com/spmadden/irox/commit/77715c7ed1e594fe97c18d879ea8357350ece160))
    - Release irox-tools v0.6.1 ([`091c8ac`](https://github.com/spmadden/irox/commit/091c8ac155297f942d1a46462e3d2d782d71993e))
    - Add eps value to tests ([`b2f572d`](https://github.com/spmadden/irox/commit/b2f572d459b23fb761eff24daafda319ac8ecc7a))
    - New streaming statistics features ([`96c41ef`](https://github.com/spmadden/irox/commit/96c41ef90232cf89154a0fddb71af0f265daed84))
    - Updating docs for stats ([`46dc6bf`](https://github.com/spmadden/irox/commit/46dc6bfb95b3860eba5f9eb47394c92acd5ac502))
    - Add version to stats dep to quiet warning ([`2a25738`](https://github.com/spmadden/irox/commit/2a25738da0f48bc7436054295dadaba15da6b64a))
    - Stats is now no_std ([`3bae95b`](https://github.com/spmadden/irox/commit/3bae95bd404f99819a47ab95a7af3f97c2764f9b))
</details>

## v0.2.3 (2024-03-03)

<csr-id-0fc37b1a2d545e8d6479443f2a55b3ad64bf5a39/>

### Chore

 - <csr-id-0fc37b1a2d545e8d6479443f2a55b3ad64bf5a39/> fixup newline formatting

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 95 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-stats v0.2.3 ([`ef99183`](https://github.com/spmadden/irox/commit/ef991830c2d652c26cb368f8ae3cb6ce414f1e7d))
    - Fixup newline formatting ([`0fc37b1`](https://github.com/spmadden/irox/commit/0fc37b1a2d545e8d6479443f2a55b3ad64bf5a39))
</details>

## v0.2.2 (2023-11-29)

<csr-id-88ebfb5deea5508ca54f4aaab62f6fd5a36f531c/>

### Chore

 - <csr-id-88ebfb5deea5508ca54f4aaab62f6fd5a36f531c/> pivot to using Cargo.toml workspace lints

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 52 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-stats v0.2.2 ([`77e1ba9`](https://github.com/spmadden/irox/commit/77e1ba9869cfe0701249d5c7c50935baf08bc206))
    - Pivot to using Cargo.toml workspace lints ([`88ebfb5`](https://github.com/spmadden/irox/commit/88ebfb5deea5508ca54f4aaab62f6fd5a36f531c))
</details>

## v0.2.1 (2023-10-07)

### New Features

 - <csr-id-f3d2d7bb7a12f1c49c5f732cd800afc1e5dd01fe/> new convolution and filtering module
 - <csr-id-cc0585f380c7ae6a25b1dd490489182a741fe3f8/> decay can use a duration now

### Bug Fixes

 - <csr-id-aa196c427363cde6523eb2b509b972ea0a089bb3/> can actually create a gaussian now.

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
    - New convolution and filtering module ([`f3d2d7b`](https://github.com/spmadden/irox/commit/f3d2d7bb7a12f1c49c5f732cd800afc1e5dd01fe))
    - Can actually create a gaussian now. ([`aa196c4`](https://github.com/spmadden/irox/commit/aa196c427363cde6523eb2b509b972ea0a089bb3))
    - Decay can use a duration now ([`cc0585f`](https://github.com/spmadden/irox/commit/cc0585f380c7ae6a25b1dd490489182a741fe3f8))
</details>

## v0.2.0 (2023-09-17)

<csr-id-80d2b88bdcb553faaeafc09673c31d7ebedafd19/>
<csr-id-1a365333397b02a5f911d0897c3bf0c80f6c2b80/>

### Chore

 - <csr-id-80d2b88bdcb553faaeafc09673c31d7ebedafd19/> update cargo.tomls to add repository
 - <csr-id-1a365333397b02a5f911d0897c3bf0c80f6c2b80/> setting up blank changelogs for the modules

### New Features

 - <csr-id-c088de020214e47f28391d0af5a64abe56ad185b/> prohibit unsafe code
 - <csr-id-31e1ea489779beab0398a89ecd630a1e3a6b3812/> new stats module

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-enums_derive v0.2.0, irox-enums v0.2.0, irox-tools v0.2.0, irox-units v0.2.0, irox-carto v0.2.0, irox-csv v0.2.0, irox-egui-extras v0.2.0, irox-networking v0.2.0, irox-types v0.2.0, irox-influxdb_v1 v0.2.0, irox-structs_derive v0.2.0, irox-structs v0.2.0, irox-nmea0183 v0.1.0, irox-sirf v0.2.0, irox-stats v0.2.0, irox-winlocation-api v0.1.0, irox v0.2.0, safety bump 10 crates ([`6a72204`](https://github.com/spmadden/irox/commit/6a722046661ceef02a66c2067e2c5c15ce102e04))
    - Update cargo.tomls to add repository ([`80d2b88`](https://github.com/spmadden/irox/commit/80d2b88bdcb553faaeafc09673c31d7ebedafd19))
    - Setting up blank changelogs for the modules ([`1a36533`](https://github.com/spmadden/irox/commit/1a365333397b02a5f911d0897c3bf0c80f6c2b80))
    - Prohibit unsafe code ([`c088de0`](https://github.com/spmadden/irox/commit/c088de020214e47f28391d0af5a64abe56ad185b))
    - New stats module ([`31e1ea4`](https://github.com/spmadden/irox/commit/31e1ea489779beab0398a89ecd630a1e3a6b3812))
</details>

