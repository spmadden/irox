

## v0.2.0 (2023-09-18)

### Chore

 - <csr-id-f03d8a3ec997d53470bfdeb5e76b71925aac3f10/> clean up code with additional lints
 - <csr-id-80d2b88bdcb553faaeafc09673c31d7ebedafd19/> update cargo.tomls to add repository
 - <csr-id-b9a0ae0ccb51682bd9c36e9ab198f38634a62ade/> fix new formatting errors from rust 1.72 upgrade
 - <csr-id-1a365333397b02a5f911d0897c3bf0c80f6c2b80/> setting up blank changelogs for the modules

### New Features

 - <csr-id-38072c3b5e21ad15a1de0642c8fb06f599690a28/> Impl MaybeFrom for DilutionOfPrecision
 - <csr-id-9a92cc2420933c45fa9afe594c6ce61591548f6c/> Display impls for several structs
 - <csr-id-b180e4ee3e371453a2c9604b4647c12b8220538d/> new GPS receiver struct GPSFixTime and improvements to DOPs
 - <csr-id-3403b5e704481f5aed4a0b6bd115e3e2606b7afc/> Quick WGS84 check in EllipticalShape
 - <csr-id-1b6eeebb4b2f81e4d711f2eb7a4d2de75cbd86d4/> Impl Display for multiple types
 - <csr-id-6aa9d74db23a17bbf207e3f62be3b2314f9916bc/> Impl 'short_name' for AltitudeReferenceFrame
 - <csr-id-a72d1f7c975b038c275f76783138b1574b3c6744/> Impl 'name' for EllipticalShape
 - <csr-id-55233e236cd2f4f92a4d5e247dff7870c056ab40/> Refactor EllipticalShape EPSG into a u32 code
 - <csr-id-f4a505475ced355286aad1bf5a352b36beae9a00/> New ConvertError struct
 - <csr-id-91a282d78e559d6bb366176de68f96f5ce095ccc/> New GPS Dilution of Precision struct
 - <csr-id-c41735befd0db3522ab10fe307a1ca09b304d644/> New Elliptical Coordinate builder
 - <csr-id-c7f4ed6cd4b8caea41b285c9eae9c8e13d2f37f3/> New more-specific Absolute and Relative CoordinateTypes
 - <csr-id-219fa8665f7c822f262d2808c18a0641d2c44278/> New Elevation angle type
 - <csr-id-b6e6c2f0e7e28e802d66ccf5814a918f0eaf3562/> Uncertainties in EllipticalCoordinate
 - <csr-id-ed9b49d74156e41f64320c46995aefe85378f8e9/> Derive PartialEQ and others for Coordinate Types
 - <csr-id-23809bda079b871491e54c433ba0bb70936d8a77/> New 'Altitude' type with reference frames.
 - <csr-id-c088de020214e47f28391d0af5a64abe56ad185b/> prohibit unsafe code
 - <csr-id-6303361462db005a3b4a35a09e86ff7dc73e0a31/> Updating coordinates
 - <csr-id-f54b4332d0b784f29913fb0672b37d3d42fbb77e/> Add name to standard ellipsoids
 - <csr-id-4c29c32713706ec1fee5d5181f2fe81238396511/> Add additional GRS80, Airy ellipsoids
 - <csr-id-bcd3c93146a1dd96e0db7e589ed0fd4a06448640/> Inv TM tests
 - <csr-id-8b5f0f2146953c4abd1dc9a74f2f5a188a538ec3/> TM unidirectional
 - <csr-id-0d954875723b480c5738335764aefbdf94775936/> Basic EPSG3857 (SphericalMercator) impl
 - <csr-id-4850e84d1f4b418bfce5fc9941b86605178b3321/> First impls of projections

### Bug Fixes

 - <csr-id-527e19e5dfa73b2cd32fc88a30a3855d28d79333/> Cleanup formatting
 - <csr-id-24ebfad3be38a87c10b98b24cb75565fd010c4df/> fix clippy lint for slices
 - <csr-id-94039ac9daaed4e8c131f80007a11687653316a3/> grump. silly clippy fix off-by-one
 - <csr-id-a84feba7fc3566be534a3d572d93985bd3d773a9/> fix clippy lints
 - <csr-id-8a9f899448f9b0b995a3510510398828d49dda9e/> fmt
 - <csr-id-6a6902f1d0a8c3cd6cc839bbeb483eb3ed421690/> Fixing issue with 3857

### Other

 - <csr-id-5c178560becc0b665d70be2d99a1cffad3ba4284/> cleaning up clippy warnings
 - <csr-id-6fa2e180f0c44bb4cbf76738acdda5631ecea20e/> remove '*' versions
 - <csr-id-8ef5bb6167b6fae09c73e2ccfe8ff4fe862c7ac9/> exclude docs folder from publish
 - <csr-id-5d31a592ac1abf6e3e616e65f7a5b8d699558fb2/> add license headers
 - <csr-id-49d55665ffd9ebcfe0394e40cb36bcc35a6a72f9/> update metadata, prepare for release

### Refactor

 - <csr-id-553cfcabf7e0a3066eeb646952f8271ac0887208/> Move coordinate & geo from units

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 42 commits contributed to the release over the course of 60 calendar days.
 - 40 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Clean up code with additional lints ([`f03d8a3`](https://github.com/spmadden/irox/commit/f03d8a3ec997d53470bfdeb5e76b71925aac3f10))
    - Update cargo.tomls to add repository ([`80d2b88`](https://github.com/spmadden/irox/commit/80d2b88bdcb553faaeafc09673c31d7ebedafd19))
    - Fix new formatting errors from rust 1.72 upgrade ([`b9a0ae0`](https://github.com/spmadden/irox/commit/b9a0ae0ccb51682bd9c36e9ab198f38634a62ade))
    - Setting up blank changelogs for the modules ([`1a36533`](https://github.com/spmadden/irox/commit/1a365333397b02a5f911d0897c3bf0c80f6c2b80))
    - Impl MaybeFrom for DilutionOfPrecision ([`38072c3`](https://github.com/spmadden/irox/commit/38072c3b5e21ad15a1de0642c8fb06f599690a28))
    - Display impls for several structs ([`9a92cc2`](https://github.com/spmadden/irox/commit/9a92cc2420933c45fa9afe594c6ce61591548f6c))
    - New GPS receiver struct GPSFixTime and improvements to DOPs ([`b180e4e`](https://github.com/spmadden/irox/commit/b180e4ee3e371453a2c9604b4647c12b8220538d))
    - Quick WGS84 check in EllipticalShape ([`3403b5e`](https://github.com/spmadden/irox/commit/3403b5e704481f5aed4a0b6bd115e3e2606b7afc))
    - Cleanup formatting ([`527e19e`](https://github.com/spmadden/irox/commit/527e19e5dfa73b2cd32fc88a30a3855d28d79333))
    - Impl Display for multiple types ([`1b6eeeb`](https://github.com/spmadden/irox/commit/1b6eeebb4b2f81e4d711f2eb7a4d2de75cbd86d4))
    - Impl 'short_name' for AltitudeReferenceFrame ([`6aa9d74`](https://github.com/spmadden/irox/commit/6aa9d74db23a17bbf207e3f62be3b2314f9916bc))
    - Impl 'name' for EllipticalShape ([`a72d1f7`](https://github.com/spmadden/irox/commit/a72d1f7c975b038c275f76783138b1574b3c6744))
    - Fixup windowscarto ([`b294607`](https://github.com/spmadden/irox/commit/b294607d583d8e9d21423eab3e7a706f6cdb2a9b))
    - Fixup converterror ([`9dbf74a`](https://github.com/spmadden/irox/commit/9dbf74aa36195f1e93f0db4ee3d1ed1f0eee6880))
    - Refactor EllipticalShape EPSG into a u32 code ([`55233e2`](https://github.com/spmadden/irox/commit/55233e236cd2f4f92a4d5e247dff7870c056ab40))
    - New ConvertError struct ([`f4a5054`](https://github.com/spmadden/irox/commit/f4a505475ced355286aad1bf5a352b36beae9a00))
    - New GPS Dilution of Precision struct ([`91a282d`](https://github.com/spmadden/irox/commit/91a282d78e559d6bb366176de68f96f5ce095ccc))
    - New Elliptical Coordinate builder ([`c41735b`](https://github.com/spmadden/irox/commit/c41735befd0db3522ab10fe307a1ca09b304d644))
    - New more-specific Absolute and Relative CoordinateTypes ([`c7f4ed6`](https://github.com/spmadden/irox/commit/c7f4ed6cd4b8caea41b285c9eae9c8e13d2f37f3))
    - New Elevation angle type ([`219fa86`](https://github.com/spmadden/irox/commit/219fa8665f7c822f262d2808c18a0641d2c44278))
    - Uncertainties in EllipticalCoordinate ([`b6e6c2f`](https://github.com/spmadden/irox/commit/b6e6c2f0e7e28e802d66ccf5814a918f0eaf3562))
    - Derive PartialEQ and others for Coordinate Types ([`ed9b49d`](https://github.com/spmadden/irox/commit/ed9b49d74156e41f64320c46995aefe85378f8e9))
    - New 'Altitude' type with reference frames. ([`23809bd`](https://github.com/spmadden/irox/commit/23809bda079b871491e54c433ba0bb70936d8a77))
    - Fix clippy lint for slices ([`24ebfad`](https://github.com/spmadden/irox/commit/24ebfad3be38a87c10b98b24cb75565fd010c4df))
    - Cleaning up clippy warnings ([`5c17856`](https://github.com/spmadden/irox/commit/5c178560becc0b665d70be2d99a1cffad3ba4284))
    - Grump. silly clippy fix off-by-one ([`94039ac`](https://github.com/spmadden/irox/commit/94039ac9daaed4e8c131f80007a11687653316a3))
    - Fix clippy lints ([`a84feba`](https://github.com/spmadden/irox/commit/a84feba7fc3566be534a3d572d93985bd3d773a9))
    - Prohibit unsafe code ([`c088de0`](https://github.com/spmadden/irox/commit/c088de020214e47f28391d0af5a64abe56ad185b))
    - Remove '*' versions ([`6fa2e18`](https://github.com/spmadden/irox/commit/6fa2e180f0c44bb4cbf76738acdda5631ecea20e))
    - Exclude docs folder from publish ([`8ef5bb6`](https://github.com/spmadden/irox/commit/8ef5bb6167b6fae09c73e2ccfe8ff4fe862c7ac9))
    - Add license headers ([`5d31a59`](https://github.com/spmadden/irox/commit/5d31a592ac1abf6e3e616e65f7a5b8d699558fb2))
    - Update metadata, prepare for release ([`49d5566`](https://github.com/spmadden/irox/commit/49d55665ffd9ebcfe0394e40cb36bcc35a6a72f9))
    - Fmt ([`8a9f899`](https://github.com/spmadden/irox/commit/8a9f899448f9b0b995a3510510398828d49dda9e))
    - Updating coordinates ([`6303361`](https://github.com/spmadden/irox/commit/6303361462db005a3b4a35a09e86ff7dc73e0a31))
    - Add name to standard ellipsoids ([`f54b433`](https://github.com/spmadden/irox/commit/f54b4332d0b784f29913fb0672b37d3d42fbb77e))
    - Add additional GRS80, Airy ellipsoids ([`4c29c32`](https://github.com/spmadden/irox/commit/4c29c32713706ec1fee5d5181f2fe81238396511))
    - Inv TM tests ([`bcd3c93`](https://github.com/spmadden/irox/commit/bcd3c93146a1dd96e0db7e589ed0fd4a06448640))
    - TM unidirectional ([`8b5f0f2`](https://github.com/spmadden/irox/commit/8b5f0f2146953c4abd1dc9a74f2f5a188a538ec3))
    - Move coordinate & geo from units ([`553cfca`](https://github.com/spmadden/irox/commit/553cfcabf7e0a3066eeb646952f8271ac0887208))
    - Fixing issue with 3857 ([`6a6902f`](https://github.com/spmadden/irox/commit/6a6902f1d0a8c3cd6cc839bbeb483eb3ed421690))
    - Basic EPSG3857 (SphericalMercator) impl ([`0d95487`](https://github.com/spmadden/irox/commit/0d954875723b480c5738335764aefbdf94775936))
    - First impls of projections ([`4850e84`](https://github.com/spmadden/irox/commit/4850e84d1f4b418bfce5fc9941b86605178b3321))
</details>

