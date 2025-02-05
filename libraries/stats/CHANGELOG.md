


## v0.3.2 (2025-02-05)

### Chore

 - <csr-id-dd46f2088abb692774a56d900fb986fd3fa425f0/> add lifetime to squash lint warning

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 9 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add lifetime to squash lint warning (dd46f20)
</details>

## v0.3.1 (2025-01-26)

### Bug Fixes

 - <csr-id-9cd9e624907cded75b72e0aa9734909ded0f93d1/> hopefully fix docsrs builds for cargo, log, stats, units

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-stats v0.3.1 (ed39cf2)
    - Hopefully fix docsrs builds for cargo, log, stats, units (9cd9e62)
</details>

## v0.3.0 (2025-01-26)

<csr-id-45a936a015f24871d007c91f9051377a3dfc1fe5/>
<csr-id-8852ee8e0dfa88b92d0c5ff0d5f1ba6b30da18ca/>
<csr-id-7957bd7ac707c9c6bfe0ba2fd32612341fd8cb57/>

### Chore

 - <csr-id-45a936a015f24871d007c91f9051377a3dfc1fe5/> cleanup some unused egui features
 - <csr-id-8852ee8e0dfa88b92d0c5ff0d5f1ba6b30da18ca/> fix lints around using Arc<Vec<T>> instead of Arc<[T]>

### Documentation

 - <csr-id-af1182afe32faf36f9f00954b0cf92dee60f8c8b/> hopefully fix the stats/units docsrs builds

### New Features

 - <csr-id-f039caac8a8960d249a754b99c19f38c64429eb3/> decoding a Coded stream passes basic tests
 - <csr-id-18e0246c5173dc368e03681404f3eb8834b4a853/> fixup nostd support in stats
 - <csr-id-13990089e681ebf7eb3ca5e86620ea81fb7cb4d6/> beginnings of a CodedTimeSeriesReader
 - <csr-id-f45c79ad8097c998eb4794119f1cdcb31ff32065/> some basic decoders, ZagZig, Inflater
 - <csr-id-dc687d8ebbc0a1bd62a10565ff491ef1f2cabfd4/> new CodedTimeSeriesWriter to compress time-series sensor data
 - <csr-id-aef170881b7afcb4914e37c551bf3555b96fd48e/> simplify adding a counter vs a fn in StreamStats
 - <csr-id-8ec4046bfec2b28efed7b5bdbe798cb60a0badf3/> rework StreamStats to take a boxed fn rather than a counter directly
 - <csr-id-1460ffd9fb2aee608ba9974e87656c50a77b68ee/> bunch of work on TSDF and float compression
 - <csr-id-5a8ca800c9fcf306271348355678eaecd344be82/> new basic alpha-beta and alpha-beta-gamma filter impls
 - <csr-id-7dbfca1fa8be625a32613e7271922a9fbdfe21a0/> linreg curve fitting using standard least squares method
 - <csr-id-511fa52cf26ed82ec2a2f4236ee6c822d5a28566/> add SavitzkyGolay1DerivOrder34 filter
 - <csr-id-c4f025f35f68a954eb4fdb418418faf66eb4956e/> ability to break a rectangle into quadrants
 - <csr-id-016d9e40273d24d1149eca0121e25565cddc71e4/> time window downsampling
 - <csr-id-07a16c737337a6cdfafeca041e0178b991d6360e/> add time window filters

### Bug Fixes

 - <csr-id-4c1164835f66e16e242912dfe889aba04bab6346/> USOS doesn't default to zero when no data
 - <csr-id-80518f578cbc22b2ff0572856825be11056a66e2/> squash rustrover misidentified error.
 - <csr-id-a4ad50d17385224c4b0ee57b9bf569027aebb2fa/> rejigger filter to align with sago binning scheme
 - <csr-id-16f1c7a7f9d79cf0108d00d5b84abb1c79ffccd1/> fix an issue with streaming mean
 - <csr-id-6e5e222a6a72cf565c8da73bb4bb157dfcd409dd/> updating examples from breaking api changes
 - <csr-id-a535cafdcbbcb6644fb57b402997bf49b2e00539/> clean up some issues with streaming min/max and points

### Other

 - <csr-id-7957bd7ac707c9c6bfe0ba2fd32612341fd8cb57/> Update readme & some documentation for the stats module

### New Features (BREAKING)

 - <csr-id-cf18819735eecc7e8512ec587f59fcbed385d712/> return usize instead of () in WriteToBEBits

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 27 commits contributed to the release over the course of 36 calendar days.
 - 41 days passed between releases.
 - 25 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-stats v0.3.0 (97dfe5a)
    - Release irox-bits v0.4.0 (a6de5f0)
    - Decoding a Coded stream passes basic tests (f039caa)
    - Fixup nostd support in stats (18e0246)
    - USOS doesn't default to zero when no data (4c11648)
    - Beginnings of a CodedTimeSeriesReader (1399008)
    - Some basic decoders, ZagZig, Inflater (f45c79a)
    - New CodedTimeSeriesWriter to compress time-series sensor data (dc687d8)
    - Simplify adding a counter vs a fn in StreamStats (aef1708)
    - Squash rustrover misidentified error. (80518f5)
    - Rework StreamStats to take a boxed fn rather than a counter directly (8ec4046)
    - Return usize instead of () in WriteToBEBits (cf18819)
    - Bunch of work on TSDF and float compression (1460ffd)
    - Hopefully fix the stats/units docsrs builds (af1182a)
    - Update readme & some documentation for the stats module (7957bd7)
    - New basic alpha-beta and alpha-beta-gamma filter impls (5a8ca80)
    - Cleanup some unused egui features (45a936a)
    - Linreg curve fitting using standard least squares method (7dbfca1)
    - Rejigger filter to align with sago binning scheme (a4ad50d)
    - Add SavitzkyGolay1DerivOrder34 filter (511fa52)
    - Fix an issue with streaming mean (16f1c7a)
    - Ability to break a rectangle into quadrants (c4f025f)
    - Updating examples from breaking api changes (6e5e222)
    - Time window downsampling (016d9e4)
    - Clean up some issues with streaming min/max and points (a535caf)
    - Add time window filters (07a16c7)
    - Fix lints around using Arc<Vec<T>> instead of Arc<[T]> (8852ee8)
</details>

## v0.2.9 (2024-12-15)

<csr-id-f0cb38d6ad407000405cecc9d94ed9157d639faa/>

### Chore

 - <csr-id-f0cb38d6ad407000405cecc9d94ed9157d639faa/> clean up lints in stats

### New Features

 - <csr-id-ba92c8669b749a681057904e57c2697c70b7a250/> Samples, Points, Rects, Windows, and some graphing

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 day passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-stats v0.2.9 (701453c)
    - Samples, Points, Rects, Windows, and some graphing (ba92c86)
    - Clean up lints in stats (f0cb38d)
</details>

## v0.2.8 (2024-12-13)

### New Features

 - <csr-id-472f1c98d523299552677374e3d07778688a71a9/> pivot to SPDP encoding scheme for tsdf
 - <csr-id-e82a848777e2f524482fa92915a99e503bc0231e/> pivot to using 'bestspeed' rather than 'bestcompression' because there's only a 5% gain but a massive perf hit.
 - <csr-id-79b4c0111cfb4daff7419dda335fca312e4afa4e/> bump MSRV to 1.82

### Bug Fixes

 - <csr-id-eb1707c86468b410415aab2e42fde0bf68b87a0b/> stackoverflow with too many compressors on the stack

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 20 calendar days.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-stats v0.2.8 (5915cd4)
    - Pivot to SPDP encoding scheme for tsdf (472f1c9)
    - Stackoverflow with too many compressors on the stack (eb1707c)
    - Pivot to using 'bestspeed' rather than 'bestcompression' because there's only a 5% gain but a massive perf hit. (e82a848)
    - Bump MSRV to 1.82 (79b4c01)
</details>

## v0.2.7 (2024-10-29)

<csr-id-2747f689e2206435cdd1ee8bab43ad9442415f20/>

### Chore

 - <csr-id-2747f689e2206435cdd1ee8bab43ad9442415f20/> update deps

### New Features

 - <csr-id-b21947ab6d854b37712535f92681beed2759c7a2/> new Summary struct and one second streaming windows stats

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 5 calendar days.
 - 5 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-stats v0.2.7 (d68794a)
    - New Summary struct and one second streaming windows stats (b21947a)
    - Update deps (2747f68)
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

 - 10 commits contributed to the release over the course of 47 calendar days.
 - 159 days passed between releases.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-stats v0.2.6 (9f599be)
    - Remove unused tdigest module (58b51d5)
    - Release irox-bits v0.2.0 (3ed7b85)
    - Fix stats tests (e8ef0ac)
    - More work on TimeSeriesDataWriters, TSDF gets "varying floats" about 2x compressed than straight deflate (d8da320)
    - Improved layering ability for the statistical compression streams (58c16d1)
    - New statistical lossless streaming encoders (f1c45cd)
    - New Point2D trait, with impls Float2D, Double2D, Quad2D (df11e3d)
    - Fixup lints & formatting (a359753)
    - Add new unlimited/paged buffer that does not reallocate (98d5046)
</details>

## v0.2.5 (2024-05-18)

### New Features

 - <csr-id-943df24540ecbce2a596363476d37b39f6a0018f/> add streaming-min, tdigest mod

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 28 calendar days.
 - 47 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-stats v0.2.5 (38375e2)
    - Add streaming-min, tdigest mod (943df24)
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

 - 7 commits contributed to the release over the course of 2 calendar days.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-stats v0.2.4 (77715c7)
    - Release irox-tools v0.6.1 (091c8ac)
    - Add eps value to tests (b2f572d)
    - New streaming statistics features (96c41ef)
    - Updating docs for stats (46dc6bf)
    - Add version to stats dep to quiet warning (2a25738)
    - Stats is now no_std (3bae95b)
</details>

## v0.2.3 (2024-03-03)

<csr-id-0fc37b1a2d545e8d6479443f2a55b3ad64bf5a39/>

### Chore

 - <csr-id-0fc37b1a2d545e8d6479443f2a55b3ad64bf5a39/> fixup newline formatting

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 50 calendar days.
 - 95 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-stats v0.2.3 (ef99183)
    - Fixup newline formatting (0fc37b1)
</details>

## v0.2.2 (2023-11-29)

<csr-id-88ebfb5deea5508ca54f4aaab62f6fd5a36f531c/>

### Chore

 - <csr-id-88ebfb5deea5508ca54f4aaab62f6fd5a36f531c/> pivot to using Cargo.toml workspace lints

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 3 calendar days.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-stats v0.2.2 (77e1ba9)
    - Pivot to using Cargo.toml workspace lints (88ebfb5)
</details>

## v0.2.1 (2023-10-07)

### New Features

 - <csr-id-f3d2d7bb7a12f1c49c5f732cd800afc1e5dd01fe/> new convolution and filtering module
 - <csr-id-cc0585f380c7ae6a25b1dd490489182a741fe3f8/> decay can use a duration now

### Bug Fixes

 - <csr-id-aa196c427363cde6523eb2b509b972ea0a089bb3/> can actually create a gaussian now.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 12 calendar days.
 - 19 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.2.1, irox-carto v0.2.1, irox-egui-extras v0.2.1, irox-gpx v0.1.0, irox-types v0.2.1, irox-structs_derive v0.2.1, irox-raymarine-sonar v0.1.0, irox-stats v0.2.1, irox-winlocation-api v0.1.1, irox v0.2.1 (68d770b)
    - New convolution and filtering module (f3d2d7b)
    - Can actually create a gaussian now. (aa196c4)
    - Decay can use a duration now (cc0585f)
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

 - 5 commits contributed to the release over the course of 29 calendar days.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-enums_derive v0.2.0, irox-enums v0.2.0, irox-tools v0.2.0, irox-units v0.2.0, irox-carto v0.2.0, irox-csv v0.2.0, irox-egui-extras v0.2.0, irox-networking v0.2.0, irox-types v0.2.0, irox-influxdb_v1 v0.2.0, irox-structs_derive v0.2.0, irox-structs v0.2.0, irox-nmea0183 v0.1.0, irox-sirf v0.2.0, irox-stats v0.2.0, irox-winlocation-api v0.1.0, irox v0.2.0, safety bump 10 crates (6a72204)
    - Update cargo.tomls to add repository (80d2b88)
    - Setting up blank changelogs for the modules (1a36533)
    - Prohibit unsafe code (c088de0)
    - New stats module (31e1ea4)
</details>

