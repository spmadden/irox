

## v0.3.0 (2026-11-04)

### Chore

 - <csr-id-306ecdf97f797b33cc68f485b0bd7aebd5b6212c/> hide log behind internal export in threading & progress
 - <csr-id-622b21017e004b29d1107b7471339306083bcc4a/> fix debug lints

### Bug Fixes

 - <csr-id-2b93d9d46592e0e3e2291a497d1cb5e944060ea2/> push progress & threading behind logging facade

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 613 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-enums_derive v0.3.0, safety bump 29 crates ([`b5c9e4a`](https://github.com/spmadden/irox/commit/b5c9e4aca7410c9d0c2cb670f49c9dd9dc711bf3))
    - Push progress & threading behind logging facade ([`2b93d9d`](https://github.com/spmadden/irox/commit/2b93d9d46592e0e3e2291a497d1cb5e944060ea2))
    - Hide log behind internal export in threading & progress ([`306ecdf`](https://github.com/spmadden/irox/commit/306ecdf97f797b33cc68f485b0bd7aebd5b6212c))
    - Fix debug lints ([`622b210`](https://github.com/spmadden/irox/commit/622b21017e004b29d1107b7471339306083bcc4a))
</details>

## v0.2.6 (2025-03-01)

### Chore

 - <csr-id-dfe010053f41bf0531816c19c3229ce5a7e90a1c/> cleanup remaining misc lints
 - <csr-id-de5e67fb00da4d87ac75adb7592f4848ba2399b2/> elude all the lifetimes!

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 13 calendar days.
 - 246 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-threading v0.2.6 ([`cf47177`](https://github.com/spmadden/irox/commit/cf471777356becc38b281cf897e287a9edc7a3b4))
    - Adjusting changelogs prior to release of irox-threading v0.2.6, irox-winlocation-api v0.5.3, irox v0.8.4 ([`73f7a77`](https://github.com/spmadden/irox/commit/73f7a7776dbcd26a8b8b835acb1c0132b7d3aada))
    - Cleanup remaining misc lints ([`dfe0100`](https://github.com/spmadden/irox/commit/dfe010053f41bf0531816c19c3229ce5a7e90a1c))
    - Elude all the lifetimes! ([`de5e67f`](https://github.com/spmadden/irox/commit/de5e67fb00da4d87ac75adb7592f4848ba2399b2))
</details>

## v0.2.5 (2024-06-28)

<csr-id-7b8a2bfcae359473feeeb4eb7098f9eab14685ee/>

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 37 calendar days.
 - 41 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-threading v0.2.5 ([`10215f4`](https://github.com/spmadden/irox/commit/10215f4d6e28558ed97ffd6ff21c28c75c5d0beb))
    - Fixup some 1.78 lint warnings ([`7b8a2bf`](https://github.com/spmadden/irox/commit/7b8a2bfcae359473feeeb4eb7098f9eab14685ee))
</details>

## v0.2.4 (2024-05-18)

### Bug Fixes

 - <csr-id-0b4029db8c0c9882e2c248bdd24b7a708a85d8d3/> fix issue in current thread executor where a task would mark that it should be rescheduled, but the executor would override it.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 13 calendar days.
 - 76 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-threading v0.2.4 ([`42fa14b`](https://github.com/spmadden/irox/commit/42fa14b1a9bf98289ba745f9f03a7c209be0b496))
    - Fix issue in current thread executor where a task would mark that it should be rescheduled, but the executor would override it. ([`0b4029d`](https://github.com/spmadden/irox/commit/0b4029db8c0c9882e2c248bdd24b7a708a85d8d3))
</details>

## v0.2.3 (2024-03-03)

<csr-id-7b3a6247e34af3ad754a61c1bd15b03b1abdf82a/>
<csr-id-300356f119c976f98a230fc37ce7c43e6bd1a9e0/>

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 6 calendar days.
 - 69 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-threading v0.2.3 ([`e7c45f3`](https://github.com/spmadden/irox/commit/e7c45f37a9e53c535ab4385420547130d701994c))
    - Bump deps to latest versions ([`7b3a624`](https://github.com/spmadden/irox/commit/7b3a6247e34af3ad754a61c1bd15b03b1abdf82a))
    - Clean up new lints for 1.75 ([`300356f`](https://github.com/spmadden/irox/commit/300356f119c976f98a230fc37ce7c43e6bd1a9e0))
</details>

## v0.2.2 (2023-12-25)

### New Features

 - <csr-id-3d915d93f2f99a851e3c5a830da20d2e0ae8b541/> CompletableTask is now async

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 2 calendar days.
 - 26 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-threading v0.2.2 ([`4ee9f9f`](https://github.com/spmadden/irox/commit/4ee9f9f7e75357203f586193dd9f2f71d397da20))
    - CompletableTask is now async ([`3d915d9`](https://github.com/spmadden/irox/commit/3d915d93f2f99a851e3c5a830da20d2e0ae8b541))
</details>

## v0.2.1 (2023-11-29)

<csr-id-88ebfb5deea5508ca54f4aaab62f6fd5a36f531c/>
<csr-id-8dac28062fabe59a155f04de03a0f2429a655f6a/>

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 21 calendar days.
 - 23 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-threading v0.2.1 ([`94e762c`](https://github.com/spmadden/irox/commit/94e762cce4ceffcef7b6bfb7bc09e7640bcfb211))
    - Pivot to using Cargo.toml workspace lints ([`88ebfb5`](https://github.com/spmadden/irox/commit/88ebfb5deea5508ca54f4aaab62f6fd5a36f531c))
    - Remove unused dependencies ([`8dac280`](https://github.com/spmadden/irox/commit/8dac28062fabe59a155f04de03a0f2429a655f6a))
</details>

## v0.2.0 (2023-11-06)

### New Features

 - <csr-id-1fa38e7e4d6a95fe6590fc3a94b917715fc6bc1e/> new multi-threaded executor
 - <csr-id-82ebc8e0a346ce6b938f895d491c230f4f4c90f8/> Impl new SingleThreadExecutor
 - <csr-id-7092bf0fdf023775913529398105f465ec7fdb43/> Impl 'run until complete' for CurrentThreadExecutor
 - <csr-id-f8b45682a3fc53de2aa1adaaae417bcde907149f/> Cleaning up docs for Task

### New Features (BREAKING)

 - <csr-id-4749a7ed346d6d355666e1a6abda37257e5682de/> current thread takes a ref to itself rather than consumed

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 8 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-threading v0.2.0 ([`f8d0662`](https://github.com/spmadden/irox/commit/f8d0662f753a86a21ebd9674fcc18fb5274cbe3a))
    - Release irox-tools v0.3.2, irox-time v0.3.0, irox-log v0.1.0, safety bump 8 crates ([`9c08793`](https://github.com/spmadden/irox/commit/9c0879320a17a94fa7a4169426de4d9d3b62395e))
    - New multi-threaded executor ([`1fa38e7`](https://github.com/spmadden/irox/commit/1fa38e7e4d6a95fe6590fc3a94b917715fc6bc1e))
    - Current thread takes a ref to itself rather than consumed ([`4749a7e`](https://github.com/spmadden/irox/commit/4749a7ed346d6d355666e1a6abda37257e5682de))
    - Impl new SingleThreadExecutor ([`82ebc8e`](https://github.com/spmadden/irox/commit/82ebc8e0a346ce6b938f895d491c230f4f4c90f8))
    - Impl 'run until complete' for CurrentThreadExecutor ([`7092bf0`](https://github.com/spmadden/irox/commit/7092bf0fdf023775913529398105f465ec7fdb43))
    - Cleaning up docs for Task ([`f8b4568`](https://github.com/spmadden/irox/commit/f8b45682a3fc53de2aa1adaaae417bcde907149f))
</details>

## v0.1.0 (2023-10-29)

### New Features

 - <csr-id-d495715a0cfd5c30b1fe659382f16210e8dad538/> new async/threading module

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-threading v0.1.0 ([`08f47a9`](https://github.com/spmadden/irox/commit/08f47a91b2ddfb3f1b229fe7c7202e3d443bd71d))
    - New async/threading module ([`d495715`](https://github.com/spmadden/irox/commit/d495715a0cfd5c30b1fe659382f16210e8dad538))
</details>

