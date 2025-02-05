


## v0.4.2 (2025-02-05)

### Bug Fixes

 - <csr-id-5d478b9ee7e14bde66a7673e5d807f42dad91344/> sirf needs tools[alloc]

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release over the course of 2 calendar days.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Sirf needs tools[alloc] (5d478b9)
</details>

## v0.4.1 (2024-05-18)

### Bug Fixes

 - <csr-id-4ad1c3fefb261538487bcf0c9f08c40c5280cde4/> add dep on bits/std
 - <csr-id-9b798b2acf8fda000e4d2e15cca9515ae2a66f7e/> pull in irox_structs::alloc
 - <csr-id-9d0f6eb1a44cbe7bbf29bab46ec4c31473cc821a/> convert to using bits error

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 28 calendar days.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-sirf v0.4.1 (35bd139)
    - Add dep on bits/std (4ad1c3f)
    - Pull in irox_structs::alloc (9b798b2)
    - Convert to using bits error (9d0f6eb)
</details>

## v0.4.0 (2024-03-03)

<csr-id-0fc37b1a2d545e8d6479443f2a55b3ad64bf5a39/>
<csr-id-7dccca7f878c10302fa2f6c71b08be6c564276dc/>

### Chore

 - <csr-id-0fc37b1a2d545e8d6479443f2a55b3ad64bf5a39/> fixup newline formatting

### Refactor (BREAKING)

 - <csr-id-7dccca7f878c10302fa2f6c71b08be6c564276dc/> update to use Bits rather than Read

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 50 calendar days.
 - 95 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-sirf v0.4.0 (45a3fea)
    - Update to use Bits rather than Read (7dccca7)
    - Release irox-tools v0.5.0, safety bump 17 crates (a46e9e2)
    - Fixup newline formatting (0fc37b1)
</details>

## v0.3.1 (2023-11-29)

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
    - Release irox-sirf v0.3.1 (f429455)
    - Pivot to using Cargo.toml workspace lints (88ebfb5)
</details>

## v0.3.0 (2023-10-29)

### Bug Fixes (BREAKING)

 - <csr-id-bba2c45c261c24124b9f4cd260d7177364e0794e/> Bits and MutBits no longer require Read & Write
 - <csr-id-e981099e141ffbc884031fd40d4adcebc46faaec/> rename 'utc_seconds' to 'utc_milliseconds' in GeoNavData to reflect the actual contents of the field

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 9 calendar days.
 - 42 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-sirf v0.3.0 (3e12093)
    - Release irox-tools v0.3.0, safety bump 12 crates (eb83b27)
    - Bits and MutBits no longer require Read & Write (bba2c45)
    - Rename 'utc_seconds' to 'utc_milliseconds' in GeoNavData to reflect the actual contents of the field (e981099)
</details>

## v0.2.0 (2023-09-17)

<csr-id-f03d8a3ec997d53470bfdeb5e76b71925aac3f10/>
<csr-id-80d2b88bdcb553faaeafc09673c31d7ebedafd19/>
<csr-id-b9a0ae0ccb51682bd9c36e9ab198f38634a62ade/>
<csr-id-1a365333397b02a5f911d0897c3bf0c80f6c2b80/>
<csr-id-5c178560becc0b665d70be2d99a1cffad3ba4284/>
<csr-id-8ef5bb6167b6fae09c73e2ccfe8ff4fe862c7ac9/>
<csr-id-7fa187c565b024c1311fb8dcc0ed5bb5387557a1/>
<csr-id-49d55665ffd9ebcfe0394e40cb36bcc35a6a72f9/>

### Chore

 - <csr-id-f03d8a3ec997d53470bfdeb5e76b71925aac3f10/> clean up code with additional lints
 - <csr-id-80d2b88bdcb553faaeafc09673c31d7ebedafd19/> update cargo.tomls to add repository
 - <csr-id-b9a0ae0ccb51682bd9c36e9ab198f38634a62ade/> fix new formatting errors from rust 1.72 upgrade
 - <csr-id-1a365333397b02a5f911d0897c3bf0c80f6c2b80/> setting up blank changelogs for the modules

### New Features

 - <csr-id-eb7f5fa4f547ba8b59d3551b50dcebed53aa3a36/> More full impl of the SIRf spec
 - <csr-id-fdaf61f7de9e266c1d3aa25ca1c69a92b655f0ad/> new x38 msg and clippy lints fixed
 - <csr-id-95869b9bf0aa7619f97b3552d3de0658526ec32c/> derive 'struct' for parsing in SIRf
 - <csr-id-c088de020214e47f28391d0af5a64abe56ad185b/> prohibit unsafe code
 - <csr-id-091f484738eb46b1e9735440f4e11dc98abe6287/> Initial

### Bug Fixes

 - <csr-id-36ce7378e51dc93247379486952a7104329feceb/> fmt

### Other

 - <csr-id-5c178560becc0b665d70be2d99a1cffad3ba4284/> cleaning up clippy warnings
 - <csr-id-8ef5bb6167b6fae09c73e2ccfe8ff4fe862c7ac9/> exclude docs folder from publish
 - <csr-id-7fa187c565b024c1311fb8dcc0ed5bb5387557a1/> add license headers
 - <csr-id-49d55665ffd9ebcfe0394e40cb36bcc35a6a72f9/> update metadata, prepare for release

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 24 commits contributed to the release over the course of 44 calendar days.
 - 14 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-enums_derive v0.2.0, irox-enums v0.2.0, irox-tools v0.2.0, irox-units v0.2.0, irox-carto v0.2.0, irox-csv v0.2.0, irox-egui-extras v0.2.0, irox-networking v0.2.0, irox-types v0.2.0, irox-influxdb_v1 v0.2.0, irox-structs_derive v0.2.0, irox-structs v0.2.0, irox-nmea0183 v0.1.0, irox-sirf v0.2.0, irox-stats v0.2.0, irox-winlocation-api v0.1.0, irox v0.2.0, safety bump 10 crates (6a72204)
    - Clean up code with additional lints (f03d8a3)
    - Update cargo.tomls to add repository (80d2b88)
    - Fix new formatting errors from rust 1.72 upgrade (b9a0ae0)
    - Setting up blank changelogs for the modules (1a36533)
    - More full impl of the SIRf spec (eb7f5fa)
    - New x38 msg and clippy lints fixed (fdaf61f)
    - Derive 'struct' for parsing in SIRf (95869b9)
    - Cleaning up clippy warnings (5c17856)
    - Add 0xFF asciidata (885b594)
    - Add 0x09 CPU Throughput (7c27337)
    - Add 0x33-6 Tracker Load Statu (bd904ca)
    - Add 0x1C NavLibrary Measurement (ccc9637)
    - Add 0x29 Geodetic Nav Data (e634e35)
    - Add 0x32 SBAS Parameters (3aeb9b4)
    - Add 0x1E Nav SV State (3a718cd)
    - Add 0x08 50 BPS Data (8ece255)
    - Add 0x04 Measured Track Data (59d017d)
    - Prohibit unsafe code (c088de0)
    - Exclude docs folder from publish (8ef5bb6)
    - Add license headers (7fa187c)
    - Update metadata, prepare for release (49d5566)
    - Fmt (36ce737)
    - Initial (091f484)
</details>

