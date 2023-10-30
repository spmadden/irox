


## v0.4.0 (2023-10-30)

### Bug Fixes

 - <csr-id-47dec1ce858b6792544264c7e109e5494b05682c/> fix issue with CSV interspersing double/single quotes by default

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 14 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.3.0, safety bump 12 crates ([`eb83b27`](https://github.com/spmadden/irox/commit/eb83b27b20c23e51e5b0fc3b7b3704e2c03af46c))
    - Fix issue with CSV interspersing double/single quotes by default ([`47dec1c`](https://github.com/spmadden/irox/commit/47dec1ce858b6792544264c7e109e5494b05682c))
</details>

## v0.3.0 (2023-10-16)

### Documentation

 - <csr-id-13ae74c7a318037939a4604a28a1cf33d87741a0/> update docs for rustdoc-lints

### New Features

 - <csr-id-7090af7410fa95c6a4f09d4f6e99826a7eef8ec4/> add dialect option to reader
 - <csr-id-51050d666b2d523776eab99531e6b85d5a7e1596/> Moved reader and writer into separate modules
 - <csr-id-7c509d006010ac421429b08b3cfb51afbfa1f0c4/> Created Dialects and Tokenizers

### New Features (BREAKING)

 - <csr-id-2e4a584642a34fe7dee9b477b4fc0a86fe62bfe4/> Add the ability to have comment lines
 - <csr-id-6b24c66b7208048fa6ef8c951a0e1a3dbc23c98f/> rework writer to use dialects, remove builder
 - <csr-id-8d8140c7164638c859e88229fe42a9b2ba0b5d41/> rework dialects
 - <csr-id-a2bed65772914f4981d0c7dc060bfd3bb5e15b57/> Upgraded to use new, more powerful scanner.
   This will open the way to new CSV dialects.

### Bug Fixes (BREAKING)

 - <csr-id-28e07a41df63f6d5551db7b0135aface45d91fa4/> actually compliant with RFC4180, CRLF instead of LF for line endings

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 7 calendar days.
 - 27 days passed between releases.
 - 9 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-carto v0.3.0, irox-csv v0.3.0, irox-egui-extras v0.3.0, irox-gpx v0.2.0, irox-influxdb_v1 v0.3.0, irox-nmea0183 v0.2.0, irox-raymarine-sonar v0.2.0, irox-time v0.1.0, irox-winlocation-api v0.2.0, irox v0.3.0 ([`dfa6258`](https://github.com/spmadden/irox/commit/dfa6258b8f93f6d27b85d2f3f4e209599a8168ad))
    - Release irox-units v0.3.0, irox-carto v0.3.0, irox-csv v0.3.0, irox-egui-extras v0.3.0, irox-gpx v0.2.0, irox-influxdb_v1 v0.3.0, irox-nmea0183 v0.2.0, irox-raymarine-sonar v0.2.0, irox-time v0.1.0, irox-winlocation-api v0.2.0, irox v0.3.0, safety bump 2 crates ([`a6c0a5f`](https://github.com/spmadden/irox/commit/a6c0a5fcfc4070b8cbc1442192b7eaef275e80f2))
    - Release irox-tools v0.2.2 ([`f49db4f`](https://github.com/spmadden/irox/commit/f49db4fc702003b0e464b0dbcc65cdcf0c629935))
    - Update docs for rustdoc-lints ([`13ae74c`](https://github.com/spmadden/irox/commit/13ae74c7a318037939a4604a28a1cf33d87741a0))
    - Add the ability to have comment lines ([`2e4a584`](https://github.com/spmadden/irox/commit/2e4a584642a34fe7dee9b477b4fc0a86fe62bfe4))
    - Actually compliant with RFC4180, CRLF instead of LF for line endings ([`28e07a4`](https://github.com/spmadden/irox/commit/28e07a41df63f6d5551db7b0135aface45d91fa4))
    - Add dialect option to reader ([`7090af7`](https://github.com/spmadden/irox/commit/7090af7410fa95c6a4f09d4f6e99826a7eef8ec4))
    - Rework writer to use dialects, remove builder ([`6b24c66`](https://github.com/spmadden/irox/commit/6b24c66b7208048fa6ef8c951a0e1a3dbc23c98f))
    - Rework dialects ([`8d8140c`](https://github.com/spmadden/irox/commit/8d8140c7164638c859e88229fe42a9b2ba0b5d41))
    - Upgraded to use new, more powerful scanner. ([`a2bed65`](https://github.com/spmadden/irox/commit/a2bed65772914f4981d0c7dc060bfd3bb5e15b57))
    - Moved reader and writer into separate modules ([`51050d6`](https://github.com/spmadden/irox/commit/51050d666b2d523776eab99531e6b85d5a7e1596))
    - Created Dialects and Tokenizers ([`7c509d0`](https://github.com/spmadden/irox/commit/7c509d006010ac421429b08b3cfb51afbfa1f0c4))
</details>

## v0.2.1 (2023-09-18)

<csr-id-f99614a5ce3368072b4d44dacede0e6e847b0b2e/>

### Chore

 - <csr-id-f99614a5ce3368072b4d44dacede0e6e847b0b2e/> Fix up the readmes for publishing

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-csv v0.2.1 ([`c6d09fa`](https://github.com/spmadden/irox/commit/c6d09fa4965c8f6fa3d78bd7c1231e7982118b8c))
    - Fix up the readmes for publishing ([`f99614a`](https://github.com/spmadden/irox/commit/f99614a5ce3368072b4d44dacede0e6e847b0b2e))
</details>

## v0.2.0 (2023-09-18)

<csr-id-f03d8a3ec997d53470bfdeb5e76b71925aac3f10/>
<csr-id-80d2b88bdcb553faaeafc09673c31d7ebedafd19/>
<csr-id-1a365333397b02a5f911d0897c3bf0c80f6c2b80/>
<csr-id-5c178560becc0b665d70be2d99a1cffad3ba4284/>
<csr-id-6d1d9a937390e9c89c4a1c66ae55f547d22e63df/>
<csr-id-e720f74b2427c4e02a92f384eaa93a28b9de28c3/>

### Chore

 - <csr-id-f03d8a3ec997d53470bfdeb5e76b71925aac3f10/> clean up code with additional lints
 - <csr-id-80d2b88bdcb553faaeafc09673c31d7ebedafd19/> update cargo.tomls to add repository
 - <csr-id-1a365333397b02a5f911d0897c3bf0c80f6c2b80/> setting up blank changelogs for the modules

### New Features

 - <csr-id-4a8470c0e380c48074bc51d1a1d178e2e7adeabb/> handle quotes within fields properly
 - <csr-id-3e154fb17540a26faf18719aeb753e38e7a03ce0/> finish implementing csv writer
 - <csr-id-c088de020214e47f28391d0af5a64abe56ad185b/> prohibit unsafe code
 - <csr-id-175d88ed8477654ef0b42e4af2c541d1a78ad4ad/> new 'for_each' function
 - <csr-id-1a158e6a3fcfcb9c7968bdd594848df0ccbbb8b1/> full/better CSV parsing
 - <csr-id-c943d81155c5eea1ec848648ff80178f5fa27211/> new writer builder
 - <csr-id-bf6140c24fa8240fe6960b504e3bc1ac48ffef72/> new csv reader/writer module

### Bug Fixes

 - <csr-id-e91b38d650a1f4cae2bf4b5b3c31717c2d9de83c/> fix fmt

### Other

 - <csr-id-5c178560becc0b665d70be2d99a1cffad3ba4284/> cleaning up clippy warnings
 - <csr-id-6d1d9a937390e9c89c4a1c66ae55f547d22e63df/> started tokenizer

### Refactor

 - <csr-id-e720f74b2427c4e02a92f384eaa93a28b9de28c3/> use buffer, ignore repeated newlines, add test

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 29 calendar days.
 - 14 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-enums_derive v0.2.0, irox-enums v0.2.0, irox-tools v0.2.0, irox-units v0.2.0, irox-carto v0.2.0, irox-csv v0.2.0, irox-egui-extras v0.2.0, irox-networking v0.2.0, irox-types v0.2.0, irox-influxdb_v1 v0.2.0, irox-structs_derive v0.2.0, irox-structs v0.2.0, irox-nmea0183 v0.1.0, irox-sirf v0.2.0, irox-stats v0.2.0, irox-winlocation-api v0.1.0, irox v0.2.0, safety bump 10 crates ([`6a72204`](https://github.com/spmadden/irox/commit/6a722046661ceef02a66c2067e2c5c15ce102e04))
    - Clean up code with additional lints ([`f03d8a3`](https://github.com/spmadden/irox/commit/f03d8a3ec997d53470bfdeb5e76b71925aac3f10))
    - Update cargo.tomls to add repository ([`80d2b88`](https://github.com/spmadden/irox/commit/80d2b88bdcb553faaeafc09673c31d7ebedafd19))
    - Feat!(csv): more robust testing, renames, documentation ([`b0f36ac`](https://github.com/spmadden/irox/commit/b0f36ac6b0ca5011f68274c0e90b5362e1b8f151))
    - Handle quotes within fields properly ([`4a8470c`](https://github.com/spmadden/irox/commit/4a8470c0e380c48074bc51d1a1d178e2e7adeabb))
    - Use buffer, ignore repeated newlines, add test ([`e720f74`](https://github.com/spmadden/irox/commit/e720f74b2427c4e02a92f384eaa93a28b9de28c3))
    - Setting up blank changelogs for the modules ([`1a36533`](https://github.com/spmadden/irox/commit/1a365333397b02a5f911d0897c3bf0c80f6c2b80))
    - Finish implementing csv writer ([`3e154fb`](https://github.com/spmadden/irox/commit/3e154fb17540a26faf18719aeb753e38e7a03ce0))
    - Cleaning up clippy warnings ([`5c17856`](https://github.com/spmadden/irox/commit/5c178560becc0b665d70be2d99a1cffad3ba4284))
    - Prohibit unsafe code ([`c088de0`](https://github.com/spmadden/irox/commit/c088de020214e47f28391d0af5a64abe56ad185b))
    - New 'for_each' function ([`175d88e`](https://github.com/spmadden/irox/commit/175d88ed8477654ef0b42e4af2c541d1a78ad4ad))
    - Fix fmt ([`e91b38d`](https://github.com/spmadden/irox/commit/e91b38d650a1f4cae2bf4b5b3c31717c2d9de83c))
    - Full/better CSV parsing ([`1a158e6`](https://github.com/spmadden/irox/commit/1a158e6a3fcfcb9c7968bdd594848df0ccbbb8b1))
    - Started tokenizer ([`6d1d9a9`](https://github.com/spmadden/irox/commit/6d1d9a937390e9c89c4a1c66ae55f547d22e63df))
    - New writer builder ([`c943d81`](https://github.com/spmadden/irox/commit/c943d81155c5eea1ec848648ff80178f5fa27211))
    - New csv reader/writer module ([`bf6140c`](https://github.com/spmadden/irox/commit/bf6140c24fa8240fe6960b504e3bc1ac48ffef72))
</details>

