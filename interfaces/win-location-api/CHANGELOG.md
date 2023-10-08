


## v0.1.1 (2023-10-08)

### New Features

 - <csr-id-9e06dc9c340a48f7a36f1510186e2fed5a151f76/> add ability to watch location status
 - <csr-id-05b284ba559e96c77f9152be01f54ce1af31aea5/> adding request for geolocation status

### Bug Fixes

 - <csr-id-c368a7f0d50cbf852b3068d6be18de02ae5cc2b8/> switching to manual convert of nanos in timestamp
 - <csr-id-db5ef9abe171c2d02ad8d6066687097daf1a37a3/> using micros instead of nanos for wintime
 - <csr-id-099c216acb827aae3d7f31483b14c2a707f7186a/> and yet another fix for the timestamp
 - <csr-id-f9777b281a1b997d02de04131dd1dbc8e7d1d54c/> actually add timestamp
 - <csr-id-0ca8619638f1928d2bf402550f99972e79140016/> add timestamp to coordinate if not present already
 - <csr-id-4670aaaa26d055e080f6a72bdf86da3293d6a409/> check for NaN & Inf in spd and hdg
 - <csr-id-01adbbd31fe97922c50c2b15414bc7cc181467f2/> add additional trace logging to the location handler

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 19 calendar days.
 - 19 days passed between releases.
 - 9 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Switching to manual convert of nanos in timestamp ([`c368a7f`](https://github.com/spmadden/irox/commit/c368a7f0d50cbf852b3068d6be18de02ae5cc2b8))
    - Using micros instead of nanos for wintime ([`db5ef9a`](https://github.com/spmadden/irox/commit/db5ef9abe171c2d02ad8d6066687097daf1a37a3))
    - And yet another fix for the timestamp ([`099c216`](https://github.com/spmadden/irox/commit/099c216acb827aae3d7f31483b14c2a707f7186a))
    - Actually add timestamp ([`f9777b2`](https://github.com/spmadden/irox/commit/f9777b281a1b997d02de04131dd1dbc8e7d1d54c))
    - Add ability to watch location status ([`9e06dc9`](https://github.com/spmadden/irox/commit/9e06dc9c340a48f7a36f1510186e2fed5a151f76))
    - Add timestamp to coordinate if not present already ([`0ca8619`](https://github.com/spmadden/irox/commit/0ca8619638f1928d2bf402550f99972e79140016))
    - Adding request for geolocation status ([`05b284b`](https://github.com/spmadden/irox/commit/05b284ba559e96c77f9152be01f54ce1af31aea5))
    - Check for NaN & Inf in spd and hdg ([`4670aaa`](https://github.com/spmadden/irox/commit/4670aaaa26d055e080f6a72bdf86da3293d6a409))
    - Add additional trace logging to the location handler ([`01adbbd`](https://github.com/spmadden/irox/commit/01adbbd31fe97922c50c2b15414bc7cc181467f2))
</details>

## v0.1.0 (2023-09-18)

<csr-id-f03d8a3ec997d53470bfdeb5e76b71925aac3f10/>
<csr-id-80d2b88bdcb553faaeafc09673c31d7ebedafd19/>
<csr-id-1a365333397b02a5f911d0897c3bf0c80f6c2b80/>

### Chore

 - <csr-id-f03d8a3ec997d53470bfdeb5e76b71925aac3f10/> clean up code with additional lints
 - <csr-id-80d2b88bdcb553faaeafc09673c31d7ebedafd19/> update cargo.tomls to add repository
 - <csr-id-1a365333397b02a5f911d0897c3bf0c80f6c2b80/> setting up blank changelogs for the modules

### New Features

 - <csr-id-a70805c7577dc7678755adf65e343f370ed45a68/> update README.md and Cargo.toml
 - <csr-id-f740c7385ec66313cd5a6df02ae6aa15f8294b13/> New module - Windows Location API

### Bug Fixes

 - <csr-id-c6381a9e8d6f927297e6f6874a0b84c2787d7f8f/> Fix fmt
 - <csr-id-13570aa9130707362ae8aa72a31cf2c5ebee1968/> really fix it this time
 - <csr-id-08a4155a0ebcb8bd8538f7ea0f2cabea812ca5bd/> Introduce module with cfg_os

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 15 calendar days.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-enums_derive v0.2.0, irox-enums v0.2.0, irox-tools v0.2.0, irox-units v0.2.0, irox-carto v0.2.0, irox-csv v0.2.0, irox-egui-extras v0.2.0, irox-networking v0.2.0, irox-types v0.2.0, irox-influxdb_v1 v0.2.0, irox-structs_derive v0.2.0, irox-structs v0.2.0, irox-nmea0183 v0.1.0, irox-sirf v0.2.0, irox-stats v0.2.0, irox-winlocation-api v0.1.0, irox v0.2.0, safety bump 10 crates ([`6a72204`](https://github.com/spmadden/irox/commit/6a722046661ceef02a66c2067e2c5c15ce102e04))
    - Clean up code with additional lints ([`f03d8a3`](https://github.com/spmadden/irox/commit/f03d8a3ec997d53470bfdeb5e76b71925aac3f10))
    - Update cargo.tomls to add repository ([`80d2b88`](https://github.com/spmadden/irox/commit/80d2b88bdcb553faaeafc09673c31d7ebedafd19))
    - Setting up blank changelogs for the modules ([`1a36533`](https://github.com/spmadden/irox/commit/1a365333397b02a5f911d0897c3bf0c80f6c2b80))
    - Update README.md and Cargo.toml ([`a70805c`](https://github.com/spmadden/irox/commit/a70805c7577dc7678755adf65e343f370ed45a68))
    - Fix fmt ([`c6381a9`](https://github.com/spmadden/irox/commit/c6381a9e8d6f927297e6f6874a0b84c2787d7f8f))
    - Really fix it this time ([`13570aa`](https://github.com/spmadden/irox/commit/13570aa9130707362ae8aa72a31cf2c5ebee1968))
    - Introduce module with cfg_os ([`08a4155`](https://github.com/spmadden/irox/commit/08a4155a0ebcb8bd8538f7ea0f2cabea812ca5bd))
    - New module - Windows Location API ([`f740c73`](https://github.com/spmadden/irox/commit/f740c7385ec66313cd5a6df02ae6aa15f8294b13))
</details>

