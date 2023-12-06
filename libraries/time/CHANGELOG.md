

## v0.3.5 (2023-12-06)

### Bug Fixes

 - <csr-id-cbe19212886560100de6cfa1c7403aaf1efbeeb4/> leading zeros may not appear for fractional seconds in ISO8601

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 6 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Leading zeros may not appear for fractional seconds in ISO8601 ([`cbe1921`](https://github.com/spmadden/irox/commit/cbe19212886560100de6cfa1c7403aaf1efbeeb4))
</details>

## v0.3.4 (2023-11-29)

<csr-id-88ebfb5deea5508ca54f4aaab62f6fd5a36f531c/>

### Chore

 - <csr-id-88ebfb5deea5508ca54f4aaab62f6fd5a36f531c/> pivot to using Cargo.toml workspace lints

### New Features

 - <csr-id-6f75e3f77356bd8ebb9ffdcd3b1b073f8948e477/> new Time::as_hms_f64 method which returns the final seconds as an f64

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 14 calendar days.
 - 20 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-time v0.3.4 ([`f2d62c7`](https://github.com/spmadden/irox/commit/f2d62c7241e6f7cb5ae002eef8533ed0f559b62e))
    - Pivot to using Cargo.toml workspace lints ([`88ebfb5`](https://github.com/spmadden/irox/commit/88ebfb5deea5508ca54f4aaab62f6fd5a36f531c))
    - New Time::as_hms_f64 method which returns the final seconds as an f64 ([`6f75e3f`](https://github.com/spmadden/irox/commit/6f75e3f77356bd8ebb9ffdcd3b1b073f8948e477))
</details>

## v0.3.3 (2023-11-08)

### Bug Fixes

 - <csr-id-87d603a6aedcdf9c150cd3c61ede327c1c516c1b/> Fix duration math in Sub<Duration> for UTCDateTime

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-time v0.3.3 ([`2b9a37f`](https://github.com/spmadden/irox/commit/2b9a37f3e23cae2c64fffe1a4ece21891c3bda4e))
    - Fix duration math in Sub<Duration> for UTCDateTime ([`87d603a`](https://github.com/spmadden/irox/commit/87d603a6aedcdf9c150cd3c61ede327c1c516c1b))
</details>

## v0.3.2 (2023-11-08)

### New Features

 - <csr-id-2cbc2e1cdbdb5d211b48ef1b113ff67732450ef9/> Impl Add, AddAssign for Date, UTCDateTime
 - <csr-id-b615236b420d596cb4a12368f4061c9da9560be2/> Impl 'wrapping_add' for Time

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-units v0.3.2, irox-time v0.3.2 ([`320bab3`](https://github.com/spmadden/irox/commit/320bab39f3dea37a8c464f29459dbda1f962af4a))
    - Impl Add, AddAssign for Date, UTCDateTime ([`2cbc2e1`](https://github.com/spmadden/irox/commit/2cbc2e1cdbdb5d211b48ef1b113ff67732450ef9))
    - Impl 'wrapping_add' for Time ([`b615236`](https://github.com/spmadden/irox/commit/b615236b420d596cb4a12368f4061c9da9560be2))
</details>

## v0.3.1 (2023-11-06)

### New Features

 - <csr-id-358ee9680adc36ab0061e4b43915aca5a0a3c8fb/> Much more complete (and correct) ISO8601 impl
 - <csr-id-5ea2b40643882d7003c6f7267f13f5ea821aa17c/> impl Display for Date using ISO8601
 - <csr-id-ecd70fb5f6d9c212e334c7b6f24289f458e20179/> new 'try_from_values' for UTCDateTime

### Bug Fixes

 - <csr-id-cc630d312b9cdd6b0dcbddc2d7717a52d4dac821/> fix inverted nanosecond calc in Time

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-time v0.3.1 ([`b16680c`](https://github.com/spmadden/irox/commit/b16680cb5843e54f4e8f39e04aeee27a09d3edd0))
    - Much more complete (and correct) ISO8601 impl ([`358ee96`](https://github.com/spmadden/irox/commit/358ee9680adc36ab0061e4b43915aca5a0a3c8fb))
    - Fix inverted nanosecond calc in Time ([`cc630d3`](https://github.com/spmadden/irox/commit/cc630d312b9cdd6b0dcbddc2d7717a52d4dac821))
    - Impl Display for Date using ISO8601 ([`5ea2b40`](https://github.com/spmadden/irox/commit/5ea2b40643882d7003c6f7267f13f5ea821aa17c))
    - New 'try_from_values' for UTCDateTime ([`ecd70fb`](https://github.com/spmadden/irox/commit/ecd70fb5f6d9c212e334c7b6f24289f458e20179))
</details>

## v0.3.0 (2023-11-05)

### New Features

 - <csr-id-d97683b8b34669c5efbc7464279ef172c4584c22/> impl Alternate display for Time to show fractional seconds to six digits
 - <csr-id-d12df244ed837501d37d63903d5357c5c2ca14a7/> Impl basic formats for UTCDateTime

### Bug Fixes

 - <csr-id-b8b5ccb189d5fec9878de4e9cebce6a6e4786183/> Fix nanos frac calculation in Time

### New Features (BREAKING)

 - <csr-id-1654133411d46a4bf1697ffb6045946542e047f9/> Switching Format/FormatParser traits from types to generics so they can be implemented multiple times on a single format struct.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 6 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.3.2, irox-time v0.3.0, irox-log v0.1.0, safety bump 8 crates ([`9c08793`](https://github.com/spmadden/irox/commit/9c0879320a17a94fa7a4169426de4d9d3b62395e))
    - Impl Alternate display for Time to show fractional seconds to six digits ([`d97683b`](https://github.com/spmadden/irox/commit/d97683b8b34669c5efbc7464279ef172c4584c22))
    - Fix nanos frac calculation in Time ([`b8b5ccb`](https://github.com/spmadden/irox/commit/b8b5ccb189d5fec9878de4e9cebce6a6e4786183))
    - Impl basic formats for UTCDateTime ([`d12df24`](https://github.com/spmadden/irox/commit/d12df244ed837501d37d63903d5357c5c2ca14a7))
    - Switching Format/FormatParser traits from types to generics so they can be implemented multiple times on a single format struct. ([`1654133`](https://github.com/spmadden/irox/commit/1654133411d46a4bf1697ffb6045946542e047f9))
</details>

## v0.2.0 (2023-10-30)

<csr-id-62a00cc08ddb4edc44b587e91db9d372ace2dcd8/>

### Chore

 - <csr-id-62a00cc08ddb4edc44b587e91db9d372ace2dcd8/> ALL THE LINTS.

### New Features

 - <csr-id-15dd99ef00993e250220096016e140db9a978f11/> ISO8601 Duration Formatting
 - <csr-id-7748f6820921d4174bd9644c2c877a1c6924f097/> Publicly exporting Duration now
 - <csr-id-82141a00b6fc20fef675f4e203dda698cb2f56f2/> Impl Add/Sub<Duration> for JulianDates
 - <csr-id-decbe2e08a0cb69691b231078e34ebea311e2697/> Impl better From<Date> conversions for Date
 - <csr-id-2e2196548859bfd2827ca2b62503af0121d13fc9/> Impl Add/Sub<Duration> for Timestamps
 - <csr-id-cdf0d92242a0fd3f68b10bf1e3f6d96f39e0d8c5/> Impl Sub<Self> and conversions for UTCDateTime
 - <csr-id-fb3c890fd6037e1def55298751fcb36ed6470e33/> from H/M/S variants for Time
 - <csr-id-cd69a1e1b1d173995421c3765e812d5090e38393/> Derives for the Julian structs
 - <csr-id-cdac4b00114a71d573e50ec1df088cdb8c82a42b/> impl Display, math, and Julian conversions in Date
 - <csr-id-fd784e6a38face1958ecb0c4f68b132e9f551c60/> impl Timestamp conversions between different Epochs
 - <csr-id-3848add99a2e03b2b79aa378f1ff058329110ba4/> impl Display and add format for UTCDateTime

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release over the course of 9 calendar days.
 - 14 days passed between releases.
 - 12 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-time v0.2.0 ([`7901d52`](https://github.com/spmadden/irox/commit/7901d52aac33bb3d967b840a8d0855fd3c2f5505))
    - Release irox-tools v0.3.0, safety bump 12 crates ([`eb83b27`](https://github.com/spmadden/irox/commit/eb83b27b20c23e51e5b0fc3b7b3704e2c03af46c))
    - ISO8601 Duration Formatting ([`15dd99e`](https://github.com/spmadden/irox/commit/15dd99ef00993e250220096016e140db9a978f11))
    - Publicly exporting Duration now ([`7748f68`](https://github.com/spmadden/irox/commit/7748f6820921d4174bd9644c2c877a1c6924f097))
    - Impl Add/Sub<Duration> for JulianDates ([`82141a0`](https://github.com/spmadden/irox/commit/82141a00b6fc20fef675f4e203dda698cb2f56f2))
    - Impl better From<Date> conversions for Date ([`decbe2e`](https://github.com/spmadden/irox/commit/decbe2e08a0cb69691b231078e34ebea311e2697))
    - Impl Add/Sub<Duration> for Timestamps ([`2e21965`](https://github.com/spmadden/irox/commit/2e2196548859bfd2827ca2b62503af0121d13fc9))
    - Impl Sub<Self> and conversions for UTCDateTime ([`cdf0d92`](https://github.com/spmadden/irox/commit/cdf0d92242a0fd3f68b10bf1e3f6d96f39e0d8c5))
    - ALL THE LINTS. ([`62a00cc`](https://github.com/spmadden/irox/commit/62a00cc08ddb4edc44b587e91db9d372ace2dcd8))
    - From H/M/S variants for Time ([`fb3c890`](https://github.com/spmadden/irox/commit/fb3c890fd6037e1def55298751fcb36ed6470e33))
    - Derives for the Julian structs ([`cd69a1e`](https://github.com/spmadden/irox/commit/cd69a1e1b1d173995421c3765e812d5090e38393))
    - Impl Display, math, and Julian conversions in Date ([`cdac4b0`](https://github.com/spmadden/irox/commit/cdac4b00114a71d573e50ec1df088cdb8c82a42b))
    - Impl Timestamp conversions between different Epochs ([`fd784e6`](https://github.com/spmadden/irox/commit/fd784e6a38face1958ecb0c4f68b132e9f551c60))
    - Impl Display and add format for UTCDateTime ([`3848add`](https://github.com/spmadden/irox/commit/3848add99a2e03b2b79aa378f1ff058329110ba4))
</details>

## v0.1.0 (2023-10-16)

### Documentation

 - <csr-id-13ae74c7a318037939a4604a28a1cf33d87741a0/> update docs for rustdoc-lints

### New Features (BREAKING)

 - <csr-id-9245b11a33fe56f75028ef5d2faa09efa4f40626/> break out time into own module

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-carto v0.3.0, irox-csv v0.3.0, irox-egui-extras v0.3.0, irox-gpx v0.2.0, irox-influxdb_v1 v0.3.0, irox-nmea0183 v0.2.0, irox-raymarine-sonar v0.2.0, irox-time v0.1.0, irox-winlocation-api v0.2.0, irox v0.3.0 ([`dfa6258`](https://github.com/spmadden/irox/commit/dfa6258b8f93f6d27b85d2f3f4e209599a8168ad))
    - Release irox-units v0.3.0, irox-carto v0.3.0, irox-csv v0.3.0, irox-egui-extras v0.3.0, irox-gpx v0.2.0, irox-influxdb_v1 v0.3.0, irox-nmea0183 v0.2.0, irox-raymarine-sonar v0.2.0, irox-time v0.1.0, irox-winlocation-api v0.2.0, irox v0.3.0, safety bump 2 crates ([`a6c0a5f`](https://github.com/spmadden/irox/commit/a6c0a5fcfc4070b8cbc1442192b7eaef275e80f2))
    - Update docs for rustdoc-lints ([`13ae74c`](https://github.com/spmadden/irox/commit/13ae74c7a318037939a4604a28a1cf33d87741a0))
    - Break out time into own module ([`9245b11`](https://github.com/spmadden/irox/commit/9245b11a33fe56f75028ef5d2faa09efa4f40626))
</details>

