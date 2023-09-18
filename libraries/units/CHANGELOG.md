

## v0.2.0 (2023-09-18)

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

 - 40 commits contributed to the release over the course of 60 calendar days.
 - 40 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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

