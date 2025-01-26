


## v0.4.5 (2025-01-26)

### Bug Fixes

 - <csr-id-9cd9e624907cded75b72e0aa9734909ded0f93d1/> hopefully fix docsrs builds for cargo, log, stats, units

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Hopefully fix docsrs builds for cargo, log, stats, units ([`9cd9e62`](https://github.com/spmadden/irox/commit/9cd9e624907cded75b72e0aa9734909ded0f93d1))
</details>

## v0.4.4 (2025-01-26)

### Documentation

 - <csr-id-af1182afe32faf36f9f00954b0cf92dee60f8c8b/> hopefully fix the stats/units docsrs builds

### New Features

 - <csr-id-dd07c94554b9b667fb47a191944abfa686ae2069/> new assert_length_eq_eps! macro to compare lengths in tests
 - <csr-id-b5145eff50004ef8a959c8d346b00b4c57139de9/> Rework macro impls for units macro.  ALL HAIL THE MACROS.
 - <csr-id-903aa93424a1c310f810efa0ea0b35b6a6c70dd7/> push ord & eq up to all units
 - <csr-id-252e8c8348fe55a485d18da4e4b234bb046fe949/> impl Eq and Ord for Duration

### Bug Fixes

 - <csr-id-abc61257319eeadfcb45fdb251375ceace022b80/> fix issue with identifying a good prefix for negative numbers

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 40 calendar days.
 - 41 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-units v0.4.4 ([`af064b1`](https://github.com/spmadden/irox/commit/af064b132778f6add56b22d7d7705e6e8c92f68d))
    - Release irox-tools v0.10.0 ([`06629ef`](https://github.com/spmadden/irox/commit/06629ef062edd1cfe60a0ba1d8b0783a6c43e454))
    - New assert_length_eq_eps! macro to compare lengths in tests ([`dd07c94`](https://github.com/spmadden/irox/commit/dd07c94554b9b667fb47a191944abfa686ae2069))
    - Hopefully fix the stats/units docsrs builds ([`af1182a`](https://github.com/spmadden/irox/commit/af1182afe32faf36f9f00954b0cf92dee60f8c8b))
    - Rework macro impls for units macro.  ALL HAIL THE MACROS. ([`b5145ef`](https://github.com/spmadden/irox/commit/b5145eff50004ef8a959c8d346b00b4c57139de9))
    - Push ord & eq up to all units ([`903aa93`](https://github.com/spmadden/irox/commit/903aa93424a1c310f810efa0ea0b35b6a6c70dd7))
    - Fix issue with identifying a good prefix for negative numbers ([`abc6125`](https://github.com/spmadden/irox/commit/abc61257319eeadfcb45fdb251375ceace022b80))
    - Impl Eq and Ord for Duration ([`252e8c8`](https://github.com/spmadden/irox/commit/252e8c8348fe55a485d18da4e4b234bb046fe949))
</details>

## v0.4.3 (2024-12-15)

### New Features

 - <csr-id-3d7d1bff4b37383c1746119894431ee9afdb5288/> SI Prefixes and Quantities

### Bug Fixes

 - <csr-id-b5810ecb0c9e3736e3a5261d33827be8e8d416f2/> guard log10 behind alloc

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
    - Release irox-units v0.4.3 ([`d5c40da`](https://github.com/spmadden/irox/commit/d5c40da2c1827de911cbc20cd02792d526dc96da))
    - Guard log10 behind alloc ([`b5810ec`](https://github.com/spmadden/irox/commit/b5810ecb0c9e3736e3a5261d33827be8e8d416f2))
    - SI Prefixes and Quantities ([`3d7d1bf`](https://github.com/spmadden/irox/commit/3d7d1bff4b37383c1746119894431ee9afdb5288))
</details>

## v0.4.2 (2024-12-13)

### New Features

 - <csr-id-83c359bd88322ed9c130c54501550544803886de/> basic lcc impl - needs more tests
 - <csr-id-79b4c0111cfb4daff7419dda335fca312e4afa4e/> bump MSRV to 1.82

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 20 calendar days.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-units v0.4.2 ([`84f868b`](https://github.com/spmadden/irox/commit/84f868bae21e13fb6966af1678a2caf60a4d4c84))
    - Basic lcc impl - needs more tests ([`83c359b`](https://github.com/spmadden/irox/commit/83c359bd88322ed9c130c54501550544803886de))
    - Bump MSRV to 1.82 ([`79b4c01`](https://github.com/spmadden/irox/commit/79b4c0111cfb4daff7419dda335fca312e4afa4e))
</details>

## v0.4.1 (2024-08-04)

### New Features

 - <csr-id-611541ee1f65329c7800099c9faa9525e9f1c5b2/> add US Survey Foot to Length, finish impl

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 77 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-units v0.4.1 ([`5e0f8f5`](https://github.com/spmadden/irox/commit/5e0f8f5da7d565c5b4d01a3cb12adbf7c9a2da85))
    - Add US Survey Foot to Length, finish impl ([`611541e`](https://github.com/spmadden/irox/commit/611541ee1f65329c7800099c9faa9525e9f1c5b2))
</details>

## v0.4.0 (2024-05-19)

<csr-id-7a4c7dcdf5d871f561fb3ad8d30da358ae6ff39e/>

### Refactor (BREAKING)

 - <csr-id-7a4c7dcdf5d871f561fb3ad8d30da358ae6ff39e/> rename feature std_errors to std to align with other modules

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 48 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-units v0.4.0 ([`aacdec7`](https://github.com/spmadden/irox/commit/aacdec7fe68c21fc82cfa0cd0830292b2c5d1ea5))
    - Rename feature std_errors to std to align with other modules ([`7a4c7dc`](https://github.com/spmadden/irox/commit/7a4c7dcdf5d871f561fb3ad8d30da358ae6ff39e))
</details>

## v0.3.6 (2024-04-01)

### New Features

 - <csr-id-5c59f70540f178aa0f5ddc5e941942f601206513/> add new Duration::from_seconds_f64 method

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-units v0.3.6 ([`90bd31e`](https://github.com/spmadden/irox/commit/90bd31e63ed30080a726a87c05582d1301a1b32f))
    - Add new Duration::from_seconds_f64 method ([`5c59f70`](https://github.com/spmadden/irox/commit/5c59f70540f178aa0f5ddc5e941942f601206513))
</details>

## v0.3.5 (2024-03-03)

### Bug Fixes

 - <csr-id-cdea90ac4b594c7ddb640a67753030bcf8e76240/> 6 keywords -> 5 keywords for publish

### New Features

 - <csr-id-ecb3e74d7485cc3a0e1d27c53a70c7bcaf72a5db/> now no_std and no_alloc!

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 50 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-units v0.3.5 ([`a2e8f48`](https://github.com/spmadden/irox/commit/a2e8f48713e6ed2855fbe6e80fdb85767491e978))
    - 6 keywords -> 5 keywords for publish ([`cdea90a`](https://github.com/spmadden/irox/commit/cdea90ac4b594c7ddb640a67753030bcf8e76240))
    - Release irox-units v0.3.5 ([`5b3a8e2`](https://github.com/spmadden/irox/commit/5b3a8e2196cc6e1dbf2b32ed2a88b0cbb6df7d54))
    - Release irox-tools v0.6.0 ([`0560dc1`](https://github.com/spmadden/irox/commit/0560dc130599c7355e1f57cbadd88395381c0033))
    - Now no_std and no_alloc! ([`ecb3e74`](https://github.com/spmadden/irox/commit/ecb3e74d7485cc3a0e1d27c53a70c7bcaf72a5db))
</details>

## v0.3.4 (2024-01-12)

<csr-id-0fc37b1a2d545e8d6479443f2a55b3ad64bf5a39/>

### Chore

 - <csr-id-0fc37b1a2d545e8d6479443f2a55b3ad64bf5a39/> fixup newline formatting

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 44 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-units v0.3.4 ([`f9d100d`](https://github.com/spmadden/irox/commit/f9d100d8aa111d789a1921ebec5ce7729c626d6d))
    - Fixup newline formatting ([`0fc37b1`](https://github.com/spmadden/irox/commit/0fc37b1a2d545e8d6479443f2a55b3ad64bf5a39))
</details>

## v0.3.3 (2023-11-28)

<csr-id-88ebfb5deea5508ca54f4aaab62f6fd5a36f531c/>

### Chore

 - <csr-id-88ebfb5deea5508ca54f4aaab62f6fd5a36f531c/> pivot to using Cargo.toml workspace lints

### New Features

 - <csr-id-9457b54f25f571b98cb41be1aee5c46692bfa6a8/> add getters, docs to bounds

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 9 calendar days.
 - 20 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-units v0.3.3 ([`68456ec`](https://github.com/spmadden/irox/commit/68456ecdeac7235a36eef43712410626d0dae174))
    - Pivot to using Cargo.toml workspace lints ([`88ebfb5`](https://github.com/spmadden/irox/commit/88ebfb5deea5508ca54f4aaab62f6fd5a36f531c))
    - Add getters, docs to bounds ([`9457b54`](https://github.com/spmadden/irox/commit/9457b54f25f571b98cb41be1aee5c46692bfa6a8))
</details>

## v0.3.2 (2023-11-08)

### New Features

 - <csr-id-2cbc2e1cdbdb5d211b48ef1b113ff67732450ef9/> Impl Add, AddAssign for Date, UTCDateTime

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-units v0.3.2, irox-time v0.3.2 ([`320bab3`](https://github.com/spmadden/irox/commit/320bab39f3dea37a8c464f29459dbda1f962af4a))
    - Impl Add, AddAssign for Date, UTCDateTime ([`2cbc2e1`](https://github.com/spmadden/irox/commit/2cbc2e1cdbdb5d211b48ef1b113ff67732450ef9))
</details>

## v0.3.1 (2023-10-29)

<csr-id-f374b5ef4a7f69938753223712d28c52353bbb92/>

### Chore

 - <csr-id-f374b5ef4a7f69938753223712d28c52353bbb92/> ALL THE LINTS.

### New Features

 - <csr-id-7918120dc78e7b8a143d8b7e3175748fddbd21b8/> new Day/Hours/Minutes/Seconds extraction for Duration
 - <csr-id-8488f471efbd9083936eee4618eb3a37a28213f5/> Impl SubAssign in the units macro
 - <csr-id-8388e21e352e297da2470f048b38f254207c3032/> add Days and Years to duration
 - <csr-id-ebf4d595e6758d5451a951734354cda51ff1ef16/> allow bounds errors to upconvert primitives

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 9 calendar days.
 - 14 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-units v0.3.1 ([`e0690ec`](https://github.com/spmadden/irox/commit/e0690ec6af8cc52fd98a9d67073e6b7b1b3089eb))
    - New Day/Hours/Minutes/Seconds extraction for Duration ([`7918120`](https://github.com/spmadden/irox/commit/7918120dc78e7b8a143d8b7e3175748fddbd21b8))
    - Impl SubAssign in the units macro ([`8488f47`](https://github.com/spmadden/irox/commit/8488f471efbd9083936eee4618eb3a37a28213f5))
    - ALL THE LINTS. ([`f374b5e`](https://github.com/spmadden/irox/commit/f374b5ef4a7f69938753223712d28c52353bbb92))
    - Add Days and Years to duration ([`8388e21`](https://github.com/spmadden/irox/commit/8388e21e352e297da2470f048b38f254207c3032))
    - Allow bounds errors to upconvert primitives ([`ebf4d59`](https://github.com/spmadden/irox/commit/ebf4d595e6758d5451a951734354cda51ff1ef16))
</details>

## v0.3.0 (2023-10-15)

<csr-id-90e274aabca747c30b3a32823940fec6225f0a07/>

### Documentation

 - <csr-id-13ae74c7a318037939a4604a28a1cf33d87741a0/> update docs for rustdoc-lints
 - <csr-id-3236d70d95e05cb49ec3acbb4dacc80931f6e6e9/> Update docs again
 - <csr-id-faf539489ede2bc8613fbb389f932a2135b8d1db/> Update docs for epoch
 - <csr-id-30093ef600b7bbdf85cd1e2034a0ca15f91c727a/> Update module docs for Time
 - <csr-id-a0ef9bb353c0dd827f2fbcf1e2c53c61a4932d2c/> Update README and bounds docs
 - <csr-id-3850cefdd85f261e2199638d51b17c6e989b406f/> Update docs for shapes

### New Features

 - <csr-id-d6787aa42ab64d56f63112aa625ab2c53ae67070/> complete Temperature
 - <csr-id-45845793bbd64795044ec8f6b3d01c8a15d53773/> new UTCDateTime and conversions
 - <csr-id-c57c0cc4aefd03e957b82f7a7a2a2602892f3c7c/> Beginnings of Date & Time Formatting, ISO8601 and RFC3339
 - <csr-id-bb52e74b53cabae68ea09ddca7024120332a058f/> Add 'nanoseconds' to Time, more consts
 - <csr-id-fc046be558fb4c1f0825f15451d9e9cf5e572c5d/> Duration new is const
 - <csr-id-04979ee7e59954e4f3b6ec9706354e6544205fb7/> overhaul Gregorian
 - <csr-id-6b77fa774a88fbdc7779cde2b3d827ab5a56c4ce/> Add Time32, Time64, and Time128 structs
 - <csr-id-c0c14f148075424a7bc9f58853c2f0f12fa72dde/> Add the Prime and NTP Epochs
 - <csr-id-30a9225df81b84eafc9bf8a247ae4819b5ce9a90/> Add Julian Dates
 - <csr-id-4384de7c6e751b07bc7c4d476c10a0360b380123/> Add 'as_seconds_f64' to Timestamp
 - <csr-id-84f097469992adc3ab663f4a79b0cd6f56b61511/> Add Common Era Epoch
 - <csr-id-59d2f268c6df2680f19c5d461ee961928a1b745d/> Add readme to units
 - <csr-id-d3738fe4a2acc37c943acf2ea6a3718ea2b26bea/> Add 'new_seconds' to Duration
 - <csr-id-ad582e4fb0e713c2fdd1ad5eb183fe32033e884e/> Refactor Epochs to be like CompassHeading
 - <csr-id-eeb442365518a980e28995b761a536f69947bcfb/> add Gregorian Date setup
 - <csr-id-24bb6738ebbbc01df33dd7306c263db6dcd775c9/> add Epoch
 - <csr-id-b6d9b07319142b2e632c548c28e7874d12557c00/> impl more conversions and Display for Duration
 - <csr-id-b55264cf41eb38d06ff10c197fcc54e4b8e7b899/> new range checks in bounds.rs
 - <csr-id-9fd5a87095e3645e82561f11d436611db55a637a/> new Duration struct and unit
 - <csr-id-8dc3f98d6b32d735c009468feb0ba32dc367d49a/> bump versions for release

### Bug Fixes

 - <csr-id-6d58744849e01f5b3d3c31d65a6a0d681678aff2/> Impl extraction and display for Time
 - <csr-id-80d89db44dd7d8cea3affae4c1efde00f5837c34/> Impl PartialEq better for "equivalent units"
 - <csr-id-a7c461de86da33b33f5d79b3354e367cf0cbc4a4/> fix basic_unit macro export
 - <csr-id-806d3086ba751e7d2e12ebae98840ea548eaa583/> fix name for speed macro

### Other

 - <csr-id-90e274aabca747c30b3a32823940fec6225f0a07/> Update docs for angle and compass

### New Features (BREAKING)

 - <csr-id-9245b11a33fe56f75028ef5d2faa09efa4f40626/> break out time into own module
 - <csr-id-3fc1e4a1d85efbec426bfe8d1291551857090ced/> Update datasize docs and enum

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 34 commits contributed to the release over the course of 5 calendar days.
 - 27 days passed between releases.
 - 33 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-units v0.3.0, irox-carto v0.3.0, irox-csv v0.3.0, irox-egui-extras v0.3.0, irox-gpx v0.2.0, irox-influxdb_v1 v0.3.0, irox-nmea0183 v0.2.0, irox-raymarine-sonar v0.2.0, irox-time v0.1.0, irox-winlocation-api v0.2.0, irox v0.3.0, safety bump 2 crates ([`a6c0a5f`](https://github.com/spmadden/irox/commit/a6c0a5fcfc4070b8cbc1442192b7eaef275e80f2))
    - Bump versions for release ([`8dc3f98`](https://github.com/spmadden/irox/commit/8dc3f98d6b32d735c009468feb0ba32dc367d49a))
    - Update docs for rustdoc-lints ([`13ae74c`](https://github.com/spmadden/irox/commit/13ae74c7a318037939a4604a28a1cf33d87741a0))
    - Break out time into own module ([`9245b11`](https://github.com/spmadden/irox/commit/9245b11a33fe56f75028ef5d2faa09efa4f40626))
    - Complete Temperature ([`d6787aa`](https://github.com/spmadden/irox/commit/d6787aa42ab64d56f63112aa625ab2c53ae67070))
    - Update docs for angle and compass ([`90e274a`](https://github.com/spmadden/irox/commit/90e274aabca747c30b3a32823940fec6225f0a07))
    - Update datasize docs and enum ([`3fc1e4a`](https://github.com/spmadden/irox/commit/3fc1e4a1d85efbec426bfe8d1291551857090ced))
    - Update docs again ([`3236d70`](https://github.com/spmadden/irox/commit/3236d70d95e05cb49ec3acbb4dacc80931f6e6e9))
    - Update docs for epoch ([`faf5394`](https://github.com/spmadden/irox/commit/faf539489ede2bc8613fbb389f932a2135b8d1db))
    - Update module docs for Time ([`30093ef`](https://github.com/spmadden/irox/commit/30093ef600b7bbdf85cd1e2034a0ca15f91c727a))
    - Update README and bounds docs ([`a0ef9bb`](https://github.com/spmadden/irox/commit/a0ef9bb353c0dd827f2fbcf1e2c53c61a4932d2c))
    - New UTCDateTime and conversions ([`4584579`](https://github.com/spmadden/irox/commit/45845793bbd64795044ec8f6b3d01c8a15d53773))
    - Update docs for shapes ([`3850cef`](https://github.com/spmadden/irox/commit/3850cefdd85f261e2199638d51b17c6e989b406f))
    - Beginnings of Date & Time Formatting, ISO8601 and RFC3339 ([`c57c0cc`](https://github.com/spmadden/irox/commit/c57c0cc4aefd03e957b82f7a7a2a2602892f3c7c))
    - Add 'nanoseconds' to Time, more consts ([`bb52e74`](https://github.com/spmadden/irox/commit/bb52e74b53cabae68ea09ddca7024120332a058f))
    - Duration new is const ([`fc046be`](https://github.com/spmadden/irox/commit/fc046be558fb4c1f0825f15451d9e9cf5e572c5d))
    - Overhaul Gregorian ([`04979ee`](https://github.com/spmadden/irox/commit/04979ee7e59954e4f3b6ec9706354e6544205fb7))
    - Add Time32, Time64, and Time128 structs ([`6b77fa7`](https://github.com/spmadden/irox/commit/6b77fa774a88fbdc7779cde2b3d827ab5a56c4ce))
    - Add the Prime and NTP Epochs ([`c0c14f1`](https://github.com/spmadden/irox/commit/c0c14f148075424a7bc9f58853c2f0f12fa72dde))
    - Add Julian Dates ([`30a9225`](https://github.com/spmadden/irox/commit/30a9225df81b84eafc9bf8a247ae4819b5ce9a90))
    - Add 'as_seconds_f64' to Timestamp ([`4384de7`](https://github.com/spmadden/irox/commit/4384de7c6e751b07bc7c4d476c10a0360b380123))
    - Add Common Era Epoch ([`84f0974`](https://github.com/spmadden/irox/commit/84f097469992adc3ab663f4a79b0cd6f56b61511))
    - Add readme to units ([`59d2f26`](https://github.com/spmadden/irox/commit/59d2f268c6df2680f19c5d461ee961928a1b745d))
    - Impl extraction and display for Time ([`6d58744`](https://github.com/spmadden/irox/commit/6d58744849e01f5b3d3c31d65a6a0d681678aff2))
    - Impl PartialEq better for "equivalent units" ([`80d89db`](https://github.com/spmadden/irox/commit/80d89db44dd7d8cea3affae4c1efde00f5837c34))
    - Add 'new_seconds' to Duration ([`d3738fe`](https://github.com/spmadden/irox/commit/d3738fe4a2acc37c943acf2ea6a3718ea2b26bea))
    - Refactor Epochs to be like CompassHeading ([`ad582e4`](https://github.com/spmadden/irox/commit/ad582e4fb0e713c2fdd1ad5eb183fe32033e884e))
    - Add Gregorian Date setup ([`eeb4423`](https://github.com/spmadden/irox/commit/eeb442365518a980e28995b761a536f69947bcfb))
    - Add Epoch ([`24bb673`](https://github.com/spmadden/irox/commit/24bb6738ebbbc01df33dd7306c263db6dcd775c9))
    - Impl more conversions and Display for Duration ([`b6d9b07`](https://github.com/spmadden/irox/commit/b6d9b07319142b2e632c548c28e7874d12557c00))
    - New range checks in bounds.rs ([`b55264c`](https://github.com/spmadden/irox/commit/b55264cf41eb38d06ff10c197fcc54e4b8e7b899))
    - Fix basic_unit macro export ([`a7c461d`](https://github.com/spmadden/irox/commit/a7c461de86da33b33f5d79b3354e367cf0cbc4a4))
    - Fix name for speed macro ([`806d308`](https://github.com/spmadden/irox/commit/806d3086ba751e7d2e12ebae98840ea548eaa583))
    - New Duration struct and unit ([`9fd5a87`](https://github.com/spmadden/irox/commit/9fd5a87095e3645e82561f11d436611db55a637a))
</details>

## v0.2.0 (2023-09-17)

<csr-id-f03d8a3ec997d53470bfdeb5e76b71925aac3f10/>
<csr-id-80d2b88bdcb553faaeafc09673c31d7ebedafd19/>
<csr-id-1a365333397b02a5f911d0897c3bf0c80f6c2b80/>
<csr-id-5c178560becc0b665d70be2d99a1cffad3ba4284/>
<csr-id-8ef5bb6167b6fae09c73e2ccfe8ff4fe862c7ac9/>
<csr-id-6c088bdcb392c82ec09a9cf4288318b6933a4c35/>
<csr-id-49d55665ffd9ebcfe0394e40cb36bcc35a6a72f9/>
<csr-id-553cfcabf7e0a3066eeb646952f8271ac0887208/>
<csr-id-a1829550c5c0acdac004160b8050d69b4afdb3bd/>
<csr-id-05a5b43bfb2f907d36b17cf844a52ddc92c2dfde/>

### Chore

 - <csr-id-f03d8a3ec997d53470bfdeb5e76b71925aac3f10/> clean up code with additional lints
 - <csr-id-80d2b88bdcb553faaeafc09673c31d7ebedafd19/> update cargo.tomls to add repository
 - <csr-id-1a365333397b02a5f911d0897c3bf0c80f6c2b80/> setting up blank changelogs for the modules

### New Features

 - <csr-id-0bb78ee642c9952e70b5ec91d66d101db0263e9d/> impl ShortName and Display for Length
 - <csr-id-1052c7d262c61d06dc319b86a3649b9ab13e180e/> new DMS methods for Angle
 - <csr-id-662b3ae1634233cf5e886ec64c2a60d3e40c6d4a/> Add many Display impls
 - <csr-id-feca2571412607f6aa5513695863af6b82be3535/> New aggregating 'CompassDirection' enum type
 - <csr-id-bc7dace92f44194abd6b59544f907cc29d3c70c1/> New shapes module with Circular and Elliptical shapes
 - <csr-id-cfbc5951279a8954e24ba88da3fd0c26ca02158e/> Derive PartialEq, PartialOrd, where applicable
 - <csr-id-c088de020214e47f28391d0af5a64abe56ad185b/> prohibit unsafe code
 - <csr-id-1df20620dc1d53c5d6725bf194d2a2b96a7f4675/> Derive debug, clone for CompassOffset
 - <csr-id-350ddeab133e78ff4658857db28b2d00a30b9b81/> Update cargo workspaces to link units
 - <csr-id-f61a3f80fe38ce0e51e13d19f561744447f9835a/> Tweak gitlab ci settings
 - <csr-id-0f1ca086cf125ce9d2afcbd838f414b6c2b49ae2/> Getting gitlab-ci stood up
 - <csr-id-3b77c401e00a7068ee21324266bd15d07f9dce54/> Add NIST Constants ascii doc
 - <csr-id-ab7f120f7e21f9e6cd58a93395d8dd037f911f39/> Initial temperature
 - <csr-id-88607b44536c429b00689bfdf9301417db2a3ddf/> New Speed Unit
 - <csr-id-5de0aa77d7c3a54fdbe4413a951b5320f68fb846/> More length units
 - <csr-id-e5e2567a901cca41df7f5f7970504708eeb3b1d7/> More ops
 - <csr-id-7e2a3c73b601551c269a1a7473339c6d4dc8004d/> Further angle, length and Add/Mul/Div ops
 - <csr-id-0e6f2b21efe373e1f51d421283817f6efac029e3/> New compass impls
 - <csr-id-2228c7ff7e4b5401a2483aece579c21fd37f9807/> Add datasize conversions
 - <csr-id-788092a18e2207782215bb7be0f9a4057801f05c/> Add DataSizeUnits
 - <csr-id-3db2a313945098edd53c967f44edbf979702f80a/> Improving angle and coordinate with const
 - <csr-id-cc0ca42d3b1c30592e7367bcfd744a559fbca9f7/> More complete angle & length
 - <csr-id-1c06c1084a718caf7263ac5fad6d0dfa5b766964/> Adding NIST & NGA docs
 - <csr-id-bb3a0c2cd1d351648876e00d0327a05818ca87a2/> More units impls with WGS84
 - <csr-id-22e3352801b78ab5b57d6fe015253328cde46110/> More coordinate impls

### Bug Fixes

 - <csr-id-527e19e5dfa73b2cd32fc88a30a3855d28d79333/> Cleanup formatting
 - <csr-id-c21e5beffe2feb376e0f20076401dacbfd61b9fc/> allow certain clippy lints
 - <csr-id-b9a64189e4b2c4b359395a2ca313179fa76474e4/> Fix missing modules from units
 - <csr-id-1ee26b0b6b0ae9468d826b5ac82b56f6bcb37509/> Fixing fmt
 - <csr-id-d9df940c05bfafe2065c9a03015a6ced7a1eba77/> Fix backwards datasize

### Other

 - <csr-id-5c178560becc0b665d70be2d99a1cffad3ba4284/> cleaning up clippy warnings
 - <csr-id-8ef5bb6167b6fae09c73e2ccfe8ff4fe862c7ac9/> exclude docs folder from publish
 - <csr-id-6c088bdcb392c82ec09a9cf4288318b6933a4c35/> add license headers
 - <csr-id-49d55665ffd9ebcfe0394e40cb36bcc35a6a72f9/> update metadata, prepare for release

### Refactor

 - <csr-id-553cfcabf7e0a3066eeb646952f8271ac0887208/> Move coordinate & geo from units
 - <csr-id-a1829550c5c0acdac004160b8050d69b4afdb3bd/> Compass tweaks
 - <csr-id-05a5b43bfb2f907d36b17cf844a52ddc92c2dfde/> Move geo and coord to carto

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 41 commits contributed to the release over the course of 62 calendar days.
 - 40 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-enums_derive v0.2.0, irox-enums v0.2.0, irox-tools v0.2.0, irox-units v0.2.0, irox-carto v0.2.0, irox-csv v0.2.0, irox-egui-extras v0.2.0, irox-networking v0.2.0, irox-types v0.2.0, irox-influxdb_v1 v0.2.0, irox-structs_derive v0.2.0, irox-structs v0.2.0, irox-nmea0183 v0.1.0, irox-sirf v0.2.0, irox-stats v0.2.0, irox-winlocation-api v0.1.0, irox v0.2.0, safety bump 10 crates ([`6a72204`](https://github.com/spmadden/irox/commit/6a722046661ceef02a66c2067e2c5c15ce102e04))
    - Clean up code with additional lints ([`f03d8a3`](https://github.com/spmadden/irox/commit/f03d8a3ec997d53470bfdeb5e76b71925aac3f10))
    - Update cargo.tomls to add repository ([`80d2b88`](https://github.com/spmadden/irox/commit/80d2b88bdcb553faaeafc09673c31d7ebedafd19))
    - Setting up blank changelogs for the modules ([`1a36533`](https://github.com/spmadden/irox/commit/1a365333397b02a5f911d0897c3bf0c80f6c2b80))
    - Impl ShortName and Display for Length ([`0bb78ee`](https://github.com/spmadden/irox/commit/0bb78ee642c9952e70b5ec91d66d101db0263e9d))
    - New DMS methods for Angle ([`1052c7d`](https://github.com/spmadden/irox/commit/1052c7d262c61d06dc319b86a3649b9ab13e180e))
    - Cleanup formatting ([`527e19e`](https://github.com/spmadden/irox/commit/527e19e5dfa73b2cd32fc88a30a3855d28d79333))
    - Add many Display impls ([`662b3ae`](https://github.com/spmadden/irox/commit/662b3ae1634233cf5e886ec64c2a60d3e40c6d4a))
    - New aggregating 'CompassDirection' enum type ([`feca257`](https://github.com/spmadden/irox/commit/feca2571412607f6aa5513695863af6b82be3535))
    - New shapes module with Circular and Elliptical shapes ([`bc7dace`](https://github.com/spmadden/irox/commit/bc7dace92f44194abd6b59544f907cc29d3c70c1))
    - Derive PartialEq, PartialOrd, where applicable ([`cfbc595`](https://github.com/spmadden/irox/commit/cfbc5951279a8954e24ba88da3fd0c26ca02158e))
    - Allow certain clippy lints ([`c21e5be`](https://github.com/spmadden/irox/commit/c21e5beffe2feb376e0f20076401dacbfd61b9fc))
    - Cleaning up clippy warnings ([`5c17856`](https://github.com/spmadden/irox/commit/5c178560becc0b665d70be2d99a1cffad3ba4284))
    - Prohibit unsafe code ([`c088de0`](https://github.com/spmadden/irox/commit/c088de020214e47f28391d0af5a64abe56ad185b))
    - Derive debug, clone for CompassOffset ([`1df2062`](https://github.com/spmadden/irox/commit/1df20620dc1d53c5d6725bf194d2a2b96a7f4675))
    - Exclude docs folder from publish ([`8ef5bb6`](https://github.com/spmadden/irox/commit/8ef5bb6167b6fae09c73e2ccfe8ff4fe862c7ac9))
    - Add license headers ([`6c088bd`](https://github.com/spmadden/irox/commit/6c088bdcb392c82ec09a9cf4288318b6933a4c35))
    - Update metadata, prepare for release ([`49d5566`](https://github.com/spmadden/irox/commit/49d55665ffd9ebcfe0394e40cb36bcc35a6a72f9))
    - Move coordinate & geo from units ([`553cfca`](https://github.com/spmadden/irox/commit/553cfcabf7e0a3066eeb646952f8271ac0887208))
    - Fix missing modules from units ([`b9a6418`](https://github.com/spmadden/irox/commit/b9a64189e4b2c4b359395a2ca313179fa76474e4))
    - Update cargo workspaces to link units ([`350ddea`](https://github.com/spmadden/irox/commit/350ddeab133e78ff4658857db28b2d00a30b9b81))
    - Fixing fmt ([`1ee26b0`](https://github.com/spmadden/irox/commit/1ee26b0b6b0ae9468d826b5ac82b56f6bcb37509))
    - Tweak gitlab ci settings ([`f61a3f8`](https://github.com/spmadden/irox/commit/f61a3f80fe38ce0e51e13d19f561744447f9835a))
    - Getting gitlab-ci stood up ([`0f1ca08`](https://github.com/spmadden/irox/commit/0f1ca086cf125ce9d2afcbd838f414b6c2b49ae2))
    - Add NIST Constants ascii doc ([`3b77c40`](https://github.com/spmadden/irox/commit/3b77c401e00a7068ee21324266bd15d07f9dce54))
    - Initial temperature ([`ab7f120`](https://github.com/spmadden/irox/commit/ab7f120f7e21f9e6cd58a93395d8dd037f911f39))
    - Compass tweaks ([`a182955`](https://github.com/spmadden/irox/commit/a1829550c5c0acdac004160b8050d69b4afdb3bd))
    - New Speed Unit ([`88607b4`](https://github.com/spmadden/irox/commit/88607b44536c429b00689bfdf9301417db2a3ddf))
    - More length units ([`5de0aa7`](https://github.com/spmadden/irox/commit/5de0aa77d7c3a54fdbe4413a951b5320f68fb846))
    - More ops ([`e5e2567`](https://github.com/spmadden/irox/commit/e5e2567a901cca41df7f5f7970504708eeb3b1d7))
    - Further angle, length and Add/Mul/Div ops ([`7e2a3c7`](https://github.com/spmadden/irox/commit/7e2a3c73b601551c269a1a7473339c6d4dc8004d))
    - New compass impls ([`0e6f2b2`](https://github.com/spmadden/irox/commit/0e6f2b21efe373e1f51d421283817f6efac029e3))
    - Fix backwards datasize ([`d9df940`](https://github.com/spmadden/irox/commit/d9df940c05bfafe2065c9a03015a6ced7a1eba77))
    - Move geo and coord to carto ([`05a5b43`](https://github.com/spmadden/irox/commit/05a5b43bfb2f907d36b17cf844a52ddc92c2dfde))
    - Add datasize conversions ([`2228c7f`](https://github.com/spmadden/irox/commit/2228c7ff7e4b5401a2483aece579c21fd37f9807))
    - Add DataSizeUnits ([`788092a`](https://github.com/spmadden/irox/commit/788092a18e2207782215bb7be0f9a4057801f05c))
    - Improving angle and coordinate with const ([`3db2a31`](https://github.com/spmadden/irox/commit/3db2a313945098edd53c967f44edbf979702f80a))
    - More complete angle & length ([`cc0ca42`](https://github.com/spmadden/irox/commit/cc0ca42d3b1c30592e7367bcfd744a559fbca9f7))
    - Adding NIST & NGA docs ([`1c06c10`](https://github.com/spmadden/irox/commit/1c06c1084a718caf7263ac5fad6d0dfa5b766964))
    - More units impls with WGS84 ([`bb3a0c2`](https://github.com/spmadden/irox/commit/bb3a0c2cd1d351648876e00d0327a05818ca87a2))
    - More coordinate impls ([`22e3352`](https://github.com/spmadden/irox/commit/22e3352801b78ab5b57d6fe015253328cde46110))
</details>

