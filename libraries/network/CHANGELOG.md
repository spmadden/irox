


## v0.4.5 (2024-12-13)

### New Features

 - <csr-id-79b4c0111cfb4daff7419dda335fca312e4afa4e/> bump MSRV to 1.82

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 50 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump MSRV to 1.82 ([`79b4c01`](https://github.com/spmadden/irox/commit/79b4c0111cfb4daff7419dda335fca312e4afa4e))
</details>

## v0.4.4 (2024-10-24)

### New Features

 - <csr-id-ac7d990b556e3cebf5e22cf41f2ffba2dc5d79d0/> Basic impl of RFC3912/Whois

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 158 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-networking v0.4.4 ([`aab795f`](https://github.com/spmadden/irox/commit/aab795f497881d8276215efdd75be9eb3c250f3a))
    - Basic impl of RFC3912/Whois ([`ac7d990`](https://github.com/spmadden/irox/commit/ac7d990b556e3cebf5e22cf41f2ffba2dc5d79d0))
</details>

## v0.4.3 (2024-05-19)

### Bug Fixes

 - <csr-id-ebb6c803a1b5a59f19862aa156fa91dab8007392/> add irox-tools/std feature to networking

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 76 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-networking v0.4.3 ([`a8ba268`](https://github.com/spmadden/irox/commit/a8ba268103427be5c3eae8f210eb9b0c16138fb2))
    - Add irox-tools/std feature to networking ([`ebb6c80`](https://github.com/spmadden/irox/commit/ebb6c803a1b5a59f19862aa156fa91dab8007392))
</details>

## v0.4.2 (2024-03-03)

<csr-id-300356f119c976f98a230fc37ce7c43e6bd1a9e0/>
<csr-id-b95c39f94603a7c42353fe65114e95cf6a37a4bb/>

### Chore

 - <csr-id-300356f119c976f98a230fc37ce7c43e6bd1a9e0/> clean up new lints for 1.75

### Refactor

 - <csr-id-b95c39f94603a7c42353fe65114e95cf6a37a4bb/> deconflict network/example and build-rs/example

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 50 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-networking v0.4.2 ([`010a9a4`](https://github.com/spmadden/irox/commit/010a9a4907e3000d4d537e85590d4e5b2f049e71))
    - Deconflict network/example and build-rs/example ([`b95c39f`](https://github.com/spmadden/irox/commit/b95c39f94603a7c42353fe65114e95cf6a37a4bb))
    - Clean up new lints for 1.75 ([`300356f`](https://github.com/spmadden/irox/commit/300356f119c976f98a230fc37ce7c43e6bd1a9e0))
</details>

## v0.4.1 (2024-01-13)

<csr-id-902df4173eb9542972a3712c8cbcb5b99612613f/>
<csr-id-e1a2beccad777886c8495d0d15a72d4d666bbc2f/>
<csr-id-6a313ea002129ab2470e2b4c8543b2fc8d1a6a6f/>
<csr-id-663496c6dee3b0bd7c6bc2e2abf00fba851b786d/>

### Chore

 - <csr-id-902df4173eb9542972a3712c8cbcb5b99612613f/> networking to 0.4.1
 - <csr-id-e1a2beccad777886c8495d0d15a72d4d666bbc2f/> cleanup some lints on networking before snap release

### New Features

 - <csr-id-01ef5caa98730e8c1c4a9415ea7f6395543178dc/> impl Display for AddressError
 - <csr-id-8f320425909ce9d9db1b2c3d3e9e1dafe1b160af/> impl FromStr for HttpProtocol
 - <csr-id-4381c274ae9858202cf43dac4c1574b4105805fd/> finish impl FromStr for HttpCodes
 - <csr-id-bbf2b845292f6ddee8cc3d06c0e0e982649827fd/> new error module using the new macros
 - <csr-id-c5511546b4615d7739b2d4c481e4a400c4d84cbf/> snap WIP http/ws client

### Bug Fixes

 - <csr-id-be738eef324236d125b8b1bc235cbfd0278f51db/> cleanup lints/unwraps in client and response
 - <csr-id-8ccfa7133187fdedbd5e271548857ecc29735922/> use $crate ident in url macro

### Other

 - <csr-id-6a313ea002129ab2470e2b4c8543b2fc8d1a6a6f/> wip snap HTTP client, basic interaction works
 - <csr-id-663496c6dee3b0bd7c6bc2e2abf00fba851b786d/> wip snap HTTP client, basic interaction works

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release.
 - 11 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-networking v0.4.1 ([`1236d8f`](https://github.com/spmadden/irox/commit/1236d8fd2b7b312a96202e27aa36069d95ed6d31))
    - Networking to 0.4.1 ([`902df41`](https://github.com/spmadden/irox/commit/902df4173eb9542972a3712c8cbcb5b99612613f))
    - Release irox-networking v0.4.0 ([`1560cb6`](https://github.com/spmadden/irox/commit/1560cb63a92efeb8701e17334d6b09c24614881f))
    - Release irox-tools v0.5.2 ([`89b01ec`](https://github.com/spmadden/irox/commit/89b01ec20e8637408a1497fa88a30452233efc97))
    - Cleanup lints/unwraps in client and response ([`be738ee`](https://github.com/spmadden/irox/commit/be738eef324236d125b8b1bc235cbfd0278f51db))
    - Impl Display for AddressError ([`01ef5ca`](https://github.com/spmadden/irox/commit/01ef5caa98730e8c1c4a9415ea7f6395543178dc))
    - Impl FromStr for HttpProtocol ([`8f32042`](https://github.com/spmadden/irox/commit/8f320425909ce9d9db1b2c3d3e9e1dafe1b160af))
    - Use $crate ident in url macro ([`8ccfa71`](https://github.com/spmadden/irox/commit/8ccfa7133187fdedbd5e271548857ecc29735922))
    - Finish impl FromStr for HttpCodes ([`4381c27`](https://github.com/spmadden/irox/commit/4381c274ae9858202cf43dac4c1574b4105805fd))
    - New error module using the new macros ([`bbf2b84`](https://github.com/spmadden/irox/commit/bbf2b845292f6ddee8cc3d06c0e0e982649827fd))
    - Cleanup some lints on networking before snap release ([`e1a2bec`](https://github.com/spmadden/irox/commit/e1a2beccad777886c8495d0d15a72d4d666bbc2f))
    - Wip snap HTTP client, basic interaction works ([`6a313ea`](https://github.com/spmadden/irox/commit/6a313ea002129ab2470e2b4c8543b2fc8d1a6a6f))
    - Wip snap HTTP client, basic interaction works ([`663496c`](https://github.com/spmadden/irox/commit/663496c6dee3b0bd7c6bc2e2abf00fba851b786d))
    - Snap WIP http/ws client ([`c551154`](https://github.com/spmadden/irox/commit/c5511546b4615d7739b2d4c481e4a400c4d84cbf))
</details>

## v0.4.0 (2024-01-13)

<csr-id-6a313ea002129ab2470e2b4c8543b2fc8d1a6a6f/>
<csr-id-663496c6dee3b0bd7c6bc2e2abf00fba851b786d/>
<csr-id-e1a2beccad777886c8495d0d15a72d4d666bbc2f/>

### Other

 - <csr-id-6a313ea002129ab2470e2b4c8543b2fc8d1a6a6f/> wip snap HTTP client, basic interaction works
 - <csr-id-663496c6dee3b0bd7c6bc2e2abf00fba851b786d/> wip snap HTTP client, basic interaction works

### Bug Fixes

 - <csr-id-be738eef324236d125b8b1bc235cbfd0278f51db/> cleanup lints/unwraps in client and response
 - <csr-id-8ccfa7133187fdedbd5e271548857ecc29735922/> use $crate ident in url macro

### New Features

 - <csr-id-01ef5caa98730e8c1c4a9415ea7f6395543178dc/> impl Display for AddressError
 - <csr-id-8f320425909ce9d9db1b2c3d3e9e1dafe1b160af/> impl FromStr for HttpProtocol
 - <csr-id-4381c274ae9858202cf43dac4c1574b4105805fd/> finish impl FromStr for HttpCodes
 - <csr-id-bbf2b845292f6ddee8cc3d06c0e0e982649827fd/> new error module using the new macros
 - <csr-id-c5511546b4615d7739b2d4c481e4a400c4d84cbf/> snap WIP http/ws client

### Chore

 - <csr-id-e1a2beccad777886c8495d0d15a72d4d666bbc2f/> cleanup some lints on networking before snap release

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 38 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.5.0, safety bump 17 crates ([`a46e9e2`](https://github.com/spmadden/irox/commit/a46e9e2da699f6ccd3a85b660014f0e15e59c0d0))
</details>

## v0.3.2 (2023-12-05)

### New Features

 - <csr-id-4e408cab037480839fb013f3692e753dc824ecb5/> put serde behind a feature gate

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 6 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-networking v0.3.2 ([`23110f9`](https://github.com/spmadden/irox/commit/23110f90aecb29115923ce4c0b749854caee7fde))
    - Put serde behind a feature gate ([`4e408ca`](https://github.com/spmadden/irox/commit/4e408cab037480839fb013f3692e753dc824ecb5))
</details>

## v0.3.1 (2023-11-28)

<csr-id-88ebfb5deea5508ca54f4aaab62f6fd5a36f531c/>

### Chore

 - <csr-id-88ebfb5deea5508ca54f4aaab62f6fd5a36f531c/> pivot to using Cargo.toml workspace lints

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 30 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-networking v0.3.1 ([`6a1d3d2`](https://github.com/spmadden/irox/commit/6a1d3d227697ea28db8086b7c246d45714d3267a))
    - Pivot to using Cargo.toml workspace lints ([`88ebfb5`](https://github.com/spmadden/irox/commit/88ebfb5deea5508ca54f4aaab62f6fd5a36f531c))
</details>

## v0.3.0 (2023-10-29)

### New Features

 - <csr-id-9aadde4fd718bb3c2aaf095eaf3507cb9f9315cb/> switch address to use new longest_consecutive_values fn
 - <csr-id-f8b7c123400808017affc4f02754708044a41ded/> new IPv6Address basic impl
 - <csr-id-43889bfa327f789f8913ced2394262d0a7039555/> new network addressing tools

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 22 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-networking v0.3.0 ([`ac69e64`](https://github.com/spmadden/irox/commit/ac69e64aa08e1fd95935325b612289828423cbb6))
    - Release irox-tools v0.3.0, safety bump 12 crates ([`eb83b27`](https://github.com/spmadden/irox/commit/eb83b27b20c23e51e5b0fc3b7b3704e2c03af46c))
    - Switch address to use new longest_consecutive_values fn ([`9aadde4`](https://github.com/spmadden/irox/commit/9aadde4fd718bb3c2aaf095eaf3507cb9f9315cb))
    - New IPv6Address basic impl ([`f8b7c12`](https://github.com/spmadden/irox/commit/f8b7c123400808017affc4f02754708044a41ded))
    - New network addressing tools ([`43889bf`](https://github.com/spmadden/irox/commit/43889bfa327f789f8913ced2394262d0a7039555))
</details>

## v0.2.1 (2023-10-07)

### Bug Fixes

 - <csr-id-fc10cc83d34183fb0e7be80f6a521fac8f6cf933/> bump version to pull in new dep

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
    - Release irox-networking v0.2.1, irox v0.2.1 ([`5303955`](https://github.com/spmadden/irox/commit/5303955be8ac39766f2ba2ff3bde32e7d031a7f4))
    - Bump version to pull in new dep ([`fc10cc8`](https://github.com/spmadden/irox/commit/fc10cc83d34183fb0e7be80f6a521fac8f6cf933))
</details>

## v0.2.0 (2023-09-17)

<csr-id-80d2b88bdcb553faaeafc09673c31d7ebedafd19/>
<csr-id-1a365333397b02a5f911d0897c3bf0c80f6c2b80/>
<csr-id-5c178560becc0b665d70be2d99a1cffad3ba4284/>
<csr-id-d42d2489478ed55560afd87bd0cad63f25224e93/>

### Chore

 - <csr-id-80d2b88bdcb553faaeafc09673c31d7ebedafd19/> update cargo.tomls to add repository
 - <csr-id-1a365333397b02a5f911d0897c3bf0c80f6c2b80/> setting up blank changelogs for the modules

### New Features

 - <csr-id-df3fcaa0fcea819d40fea6b2487fe25024b4194c/> add keywords & categories to Cargo.toml
 - <csr-id-e0d4ac9a99b2ed65cd7b0c4ca4333e0d09e5cfd3/> New TCP Connection Pool impl
 - <csr-id-c088de020214e47f28391d0af5a64abe56ad185b/> prohibit unsafe code
 - <csr-id-5a3e305fea33b80cffc446fc9c7773939ae6baf9/> new network tools module

### Bug Fixes

 - <csr-id-d5f4d2f189d2dfe3d26ddf668d4e3dc042218a93/> Pool actually accepts clients now

### Other

 - <csr-id-5c178560becc0b665d70be2d99a1cffad3ba4284/> cleaning up clippy warnings
 - <csr-id-d42d2489478ed55560afd87bd0cad63f25224e93/> influx pings

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release.
 - 9 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-enums_derive v0.2.0, irox-enums v0.2.0, irox-tools v0.2.0, irox-units v0.2.0, irox-carto v0.2.0, irox-csv v0.2.0, irox-egui-extras v0.2.0, irox-networking v0.2.0, irox-types v0.2.0, irox-influxdb_v1 v0.2.0, irox-structs_derive v0.2.0, irox-structs v0.2.0, irox-nmea0183 v0.1.0, irox-sirf v0.2.0, irox-stats v0.2.0, irox-winlocation-api v0.1.0, irox v0.2.0, safety bump 10 crates ([`6a72204`](https://github.com/spmadden/irox/commit/6a722046661ceef02a66c2067e2c5c15ce102e04))
    - Update cargo.tomls to add repository ([`80d2b88`](https://github.com/spmadden/irox/commit/80d2b88bdcb553faaeafc09673c31d7ebedafd19))
    - Setting up blank changelogs for the modules ([`1a36533`](https://github.com/spmadden/irox/commit/1a365333397b02a5f911d0897c3bf0c80f6c2b80))
    - Add keywords & categories to Cargo.toml ([`df3fcaa`](https://github.com/spmadden/irox/commit/df3fcaa0fcea819d40fea6b2487fe25024b4194c))
    - Pool actually accepts clients now ([`d5f4d2f`](https://github.com/spmadden/irox/commit/d5f4d2f189d2dfe3d26ddf668d4e3dc042218a93))
    - New TCP Connection Pool impl ([`e0d4ac9`](https://github.com/spmadden/irox/commit/e0d4ac9a99b2ed65cd7b0c4ca4333e0d09e5cfd3))
    - Cleaning up clippy warnings ([`5c17856`](https://github.com/spmadden/irox/commit/5c178560becc0b665d70be2d99a1cffad3ba4284))
    - Prohibit unsafe code ([`c088de0`](https://github.com/spmadden/irox/commit/c088de020214e47f28391d0af5a64abe56ad185b))
    - Influx pings ([`d42d248`](https://github.com/spmadden/irox/commit/d42d2489478ed55560afd87bd0cad63f25224e93))
    - New network tools module ([`5a3e305`](https://github.com/spmadden/irox/commit/5a3e305fea33b80cffc446fc9c7773939ae6baf9))
</details>

