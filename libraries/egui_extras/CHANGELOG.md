


## v0.8.0 (2026-11-04)

### Chore

 - <csr-id-d852f3f634768b5f6632d9dc3d20f774badda2e2/> update to rust 1.98, cleanup lints
 - <csr-id-50b5d0e18a05b2e7b1da475efbccecf2c883af71/> update to rust 1.97, cleanup lints
 - <csr-id-93ecd14c3c1f844f8a76388e02452b182fb43b3a/> fix imports
 - <csr-id-b7b95a377765d1af3c529cdf9bcf901579d74545/> fix lints & bump to 1.91
 - <csr-id-691776a610c7e169cdbf888f4d18892a5cf2377c/> fix lints & bump to 1.89
 - <csr-id-789f6fbb81e815454d18c68c310d9a8e58f969c5/> fix lints & bump to 1.87

### New Features

 - <csr-id-a8db5e268bc04b97ba2c1676b425476a30da5edd/> rework of the boolean editor widget
 - <csr-id-4251f45dad4e177f1f2bcc1353f9c95d452cd61c/> basic editors
 - <csr-id-727a698b027559a877946b554140b6a65346f91d/> pivot to using Identifier, arcs return responses
 - <csr-id-82f28c83316951e07476fc1a52842d06a63ff8b0/> add toast widget, refactor gallery by features
 - <csr-id-0aea7db7b8d69e794c7fa3735046b0b1a396530f/> add arcwedge widget
 - <csr-id-105c444e060dd095578a5444aaf28e20d101d412/> add arcwedge and colors examples
 - <csr-id-523f014ce085ab6e43472ebbdd1a00d2f2f7166a/> arcwedges again
 - <csr-id-70f78e89cb27a24270135241fd749374e325974d/> Start of arcwedge example implementation
 - <csr-id-24940b7c225f0aa1e60d164f818b2c285fe8e178/> switch to using the nice built-in theme button.
 - <csr-id-e76348a16c9f4e4d3e0cc801f60050e2d6147c52/> Add basic PNG image tests
 - <csr-id-405acd2d8cdc965e6e1095dba9a4d6744049913b/> buncha new widgets
 - <csr-id-84f45ca6d391c1845338a0b8872ac0b704f30389/> random gets .choice() which returns a random element from the set
 - <csr-id-b6b852444ec0ddd2c63ae2805565c1b986fae00d/> expose Layer::scale_position
 - <csr-id-8f787a847d27d8a9931cfce339338c95eb1d6586/> arrow drawing
 - <csr-id-e41136f3a9766690a73ac4292676a2034e6edc21/> better interactions
 - <csr-id-b7206818384d8b82b05222fe2c276996d5f5e537/> test image example
 - <csr-id-c908820fda002d1c5ce2887897165c23bef53ec3/> improved profiling & composite app creation
 - <csr-id-5abb02edc5c8b0bd8898a411b478dd5dab8a347c/> rejigger fonts, add emojis
 - <csr-id-c2b61925ab265998e0919d1295f8b151ee4dafe7/> can disable dragging for other widgets
 - <csr-id-bf8faa6a6410fa98a400985c4c16688cad76e805/> ability to disable toolframe settings menu
 - <csr-id-d85675521504f1415d7107856c4d6ad84ca54b92/> new arc-ish bent line between two points based on distance
 - <csr-id-af2db7ecf3739af78d94ef4624c3e2a957a2ba8b/> option to disable mouse crosshairs in drawpanel
 - <csr-id-3e92b70ecd998fde8751a59823fbf7aa5fb4021c/> draw panel gets better location scaling options
 - <csr-id-dc99f3159abd0b5aacd55ffe2f01e6377baf7476/> reset to initial transform on double-click and store the world area off
 - <csr-id-b5a69f34b3eaf17bdccb2aaf104fc0396d259bdb/> new small terminal raster fonts
 - <csr-id-0ad56c45761e83598f59548ab8ec71ac5f4a7e9c/> new mesh-based font
 - <csr-id-6096a61fb0cbf628929fb2c59f980f336c0204ef/> ability to set fonts as default-egui fonts
 - <csr-id-9ce4bedc880becf88afdae8237fc25ce08febae1/> DrawPanels get layer support
 - <csr-id-cc7e6913467cabfa818b08a0891c1a3256e7e3e4/> remove indicatif dep RUSTSEC-2025-0119
 - <csr-id-abd5d4915f1b5110ad189ab72031593f5df419d9/> new test rendering image
 - <csr-id-dd6cdc080e4a3a5b2af01f2ac81b915edf0947b8/> add extra menu options to toolframe
 - <csr-id-631bcdcc1037af4cbe6d8e2b55144f6c21648138/> WIP draw panels
 - <csr-id-9afc16d087521217e15069bfe66d8f8f0ab0b81f/> test image generation
 - <csr-id-90221033f46472e52b4eb604183b2dc14f09005c/> tweak persistence behavior for composite apps, cleanup deps
 - <csr-id-2f4ab0cf269eb76666be7ebaeb42f3249b74099a/> easier all-fonts loading
 - <csr-id-4462f83776d2aa47681f8786667d09bc51da9ef2/> rejigger to be more compatible with wasm32
 - <csr-id-49093fc06cc2676b16d2244daceefa33cf9cc138/> also export eframe
 - <csr-id-2a7510a6cfe434cf8264b4885f5a54e40d475f4c/> new dialog infra, for messages & passwords

### Bug Fixes

 - <csr-id-379bf1c0d76872fb694d387049063e8588c1ae37/> Fix import of eframe vs egui in arrows
 - <csr-id-4347befc0255f48d228213db0112bf965d4c5602/> fix toolframe theming (maybe!)
 - <csr-id-40879d4963d129f822372793cb62dd2db04dd1c5/> fix theming button bug
 - <csr-id-03a1e18b85001f4448c87450571da9c513322956/> clean up features & imports
 - <csr-id-a2be1d9002ecab07ce63fe580b5338b588033fc3/> move bottom bar before windows
 - <csr-id-7b13ca4bfe1af1a6d8331aa366afb78f495ea50b/> remove default features
 - <csr-id-eec9bdb46b7a6b5979b72717f989a5813ac27eb0/> put profiling behind feature gate
 - <csr-id-e6bc47698c194864dc6e3941f81e77fe9bf7eae5/> default to wgpu
 - <csr-id-79da433927d358967e5ca0b72896d80c5e9c0af0/> fix wasm for toolframe
 - <csr-id-81032185a100f969cc6f26c7ee32472e1f92496b/> ensure NativeOptions isn't included for wasm32
 - <csr-id-be75cf2c3f0b4402d71ff4d74f30733f6287447a/> put irox-stats dep behind plots feature where it's used
 - <csr-id-18f86c9d7accff732208b0293dd2b118384e50b6/> add eframe wgpu, glow features

### Other

 - <csr-id-ce68d4d5577feb8f49c2a4fc31122bf1181dfbe5/> drawing in wasm

### Refactor

 - <csr-id-159001f900b2ca97dd398671a85c548c90fc07c4/> move eframe behind feature
 - <csr-id-59f35131513a8b43d7e288832276943532e6bfee/> hexdump read -> bits

### New Features (BREAKING)

 - <csr-id-170a4ac732fd9524798c20960fbe7106a76e65e9/> bump egui to 0.34

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 61 commits contributed to the release.
 - 605 days passed between releases.
 - 60 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-enums_derive v0.3.0, safety bump 29 crates ([`b5c9e4a`](https://github.com/spmadden/irox/commit/b5c9e4aca7410c9d0c2cb670f49c9dd9dc711bf3))
    - Update to rust 1.98, cleanup lints ([`d852f3f`](https://github.com/spmadden/irox/commit/d852f3f634768b5f6632d9dc3d20f774badda2e2))
    - Update to rust 1.97, cleanup lints ([`50b5d0e`](https://github.com/spmadden/irox/commit/50b5d0e18a05b2e7b1da475efbccecf2c883af71))
    - Rework of the boolean editor widget ([`a8db5e2`](https://github.com/spmadden/irox/commit/a8db5e268bc04b97ba2c1676b425476a30da5edd))
    - Fix imports ([`93ecd14`](https://github.com/spmadden/irox/commit/93ecd14c3c1f844f8a76388e02452b182fb43b3a))
    - Basic editors ([`4251f45`](https://github.com/spmadden/irox/commit/4251f45dad4e177f1f2bcc1353f9c95d452cd61c))
    - Pivot to using Identifier, arcs return responses ([`727a698`](https://github.com/spmadden/irox/commit/727a698b027559a877946b554140b6a65346f91d))
    - Add toast widget, refactor gallery by features ([`82f28c8`](https://github.com/spmadden/irox/commit/82f28c83316951e07476fc1a52842d06a63ff8b0))
    - Add arcwedge widget ([`0aea7db`](https://github.com/spmadden/irox/commit/0aea7db7b8d69e794c7fa3735046b0b1a396530f))
    - Add arcwedge and colors examples ([`105c444`](https://github.com/spmadden/irox/commit/105c444e060dd095578a5444aaf28e20d101d412))
    - Arcwedges again ([`523f014`](https://github.com/spmadden/irox/commit/523f014ce085ab6e43472ebbdd1a00d2f2f7166a))
    - Start of arcwedge example implementation ([`70f78e8`](https://github.com/spmadden/irox/commit/70f78e89cb27a24270135241fd749374e325974d))
    - Switch to using the nice built-in theme button. ([`24940b7`](https://github.com/spmadden/irox/commit/24940b7c225f0aa1e60d164f818b2c285fe8e178))
    - Add basic PNG image tests ([`e76348a`](https://github.com/spmadden/irox/commit/e76348a16c9f4e4d3e0cc801f60050e2d6147c52))
    - Fix import of eframe vs egui in arrows ([`379bf1c`](https://github.com/spmadden/irox/commit/379bf1c0d76872fb694d387049063e8588c1ae37))
    - Buncha new widgets ([`405acd2`](https://github.com/spmadden/irox/commit/405acd2d8cdc965e6e1095dba9a4d6744049913b))
    - Random gets .choice() which returns a random element from the set ([`84f45ca`](https://github.com/spmadden/irox/commit/84f45ca6d391c1845338a0b8872ac0b704f30389))
    - Fix toolframe theming (maybe!) ([`4347bef`](https://github.com/spmadden/irox/commit/4347befc0255f48d228213db0112bf965d4c5602))
    - Expose Layer::scale_position ([`b6b8524`](https://github.com/spmadden/irox/commit/b6b852444ec0ddd2c63ae2805565c1b986fae00d))
    - Arrow drawing ([`8f787a8`](https://github.com/spmadden/irox/commit/8f787a847d27d8a9931cfce339338c95eb1d6586))
    - Fix theming button bug ([`40879d4`](https://github.com/spmadden/irox/commit/40879d4963d129f822372793cb62dd2db04dd1c5))
    - Better interactions ([`e41136f`](https://github.com/spmadden/irox/commit/e41136f3a9766690a73ac4292676a2034e6edc21))
    - Clean up features & imports ([`03a1e18`](https://github.com/spmadden/irox/commit/03a1e18b85001f4448c87450571da9c513322956))
    - Bump egui to 0.34 ([`170a4ac`](https://github.com/spmadden/irox/commit/170a4ac732fd9524798c20960fbe7106a76e65e9))
    - Test image example ([`b720681`](https://github.com/spmadden/irox/commit/b7206818384d8b82b05222fe2c276996d5f5e537))
    - Move bottom bar before windows ([`a2be1d9`](https://github.com/spmadden/irox/commit/a2be1d9002ecab07ce63fe580b5338b588033fc3))
    - Remove default features ([`7b13ca4`](https://github.com/spmadden/irox/commit/7b13ca4bfe1af1a6d8331aa366afb78f495ea50b))
    - Put profiling behind feature gate ([`eec9bdb`](https://github.com/spmadden/irox/commit/eec9bdb46b7a6b5979b72717f989a5813ac27eb0))
    - Improved profiling & composite app creation ([`c908820`](https://github.com/spmadden/irox/commit/c908820fda002d1c5ce2887897165c23bef53ec3))
    - Rejigger fonts, add emojis ([`5abb02e`](https://github.com/spmadden/irox/commit/5abb02edc5c8b0bd8898a411b478dd5dab8a347c))
    - Can disable dragging for other widgets ([`c2b6192`](https://github.com/spmadden/irox/commit/c2b61925ab265998e0919d1295f8b151ee4dafe7))
    - Ability to disable toolframe settings menu ([`bf8faa6`](https://github.com/spmadden/irox/commit/bf8faa6a6410fa98a400985c4c16688cad76e805))
    - New arc-ish bent line between two points based on distance ([`d856755`](https://github.com/spmadden/irox/commit/d85675521504f1415d7107856c4d6ad84ca54b92))
    - Option to disable mouse crosshairs in drawpanel ([`af2db7e`](https://github.com/spmadden/irox/commit/af2db7ecf3739af78d94ef4624c3e2a957a2ba8b))
    - Draw panel gets better location scaling options ([`3e92b70`](https://github.com/spmadden/irox/commit/3e92b70ecd998fde8751a59823fbf7aa5fb4021c))
    - Reset to initial transform on double-click and store the world area off ([`dc99f31`](https://github.com/spmadden/irox/commit/dc99f3159abd0b5aacd55ffe2f01e6377baf7476))
    - Default to wgpu ([`e6bc476`](https://github.com/spmadden/irox/commit/e6bc47698c194864dc6e3941f81e77fe9bf7eae5))
    - New small terminal raster fonts ([`b5a69f3`](https://github.com/spmadden/irox/commit/b5a69f34b3eaf17bdccb2aaf104fc0396d259bdb))
    - New mesh-based font ([`0ad56c4`](https://github.com/spmadden/irox/commit/0ad56c45761e83598f59548ab8ec71ac5f4a7e9c))
    - Ability to set fonts as default-egui fonts ([`6096a61`](https://github.com/spmadden/irox/commit/6096a61fb0cbf628929fb2c59f980f336c0204ef))
    - DrawPanels get layer support ([`9ce4bed`](https://github.com/spmadden/irox/commit/9ce4bedc880becf88afdae8237fc25ce08febae1))
    - Move eframe behind feature ([`159001f`](https://github.com/spmadden/irox/commit/159001f900b2ca97dd398671a85c548c90fc07c4))
    - Hexdump read -> bits ([`59f3513`](https://github.com/spmadden/irox/commit/59f35131513a8b43d7e288832276943532e6bfee))
    - Fix lints & bump to 1.91 ([`b7b95a3`](https://github.com/spmadden/irox/commit/b7b95a377765d1af3c529cdf9bcf901579d74545))
    - Fix lints & bump to 1.89 ([`691776a`](https://github.com/spmadden/irox/commit/691776a610c7e169cdbf888f4d18892a5cf2377c))
    - Fix lints & bump to 1.87 ([`789f6fb`](https://github.com/spmadden/irox/commit/789f6fbb81e815454d18c68c310d9a8e58f969c5))
    - Remove indicatif dep RUSTSEC-2025-0119 ([`cc7e691`](https://github.com/spmadden/irox/commit/cc7e6913467cabfa818b08a0891c1a3256e7e3e4))
    - New test rendering image ([`abd5d49`](https://github.com/spmadden/irox/commit/abd5d4915f1b5110ad189ab72031593f5df419d9))
    - Add extra menu options to toolframe ([`dd6cdc0`](https://github.com/spmadden/irox/commit/dd6cdc080e4a3a5b2af01f2ac81b915edf0947b8))
    - WIP draw panels ([`631bcdc`](https://github.com/spmadden/irox/commit/631bcdcc1037af4cbe6d8e2b55144f6c21648138))
    - Test image generation ([`9afc16d`](https://github.com/spmadden/irox/commit/9afc16d087521217e15069bfe66d8f8f0ab0b81f))
    - Fix wasm for toolframe ([`79da433`](https://github.com/spmadden/irox/commit/79da433927d358967e5ca0b72896d80c5e9c0af0))
    - Tweak persistence behavior for composite apps, cleanup deps ([`9022103`](https://github.com/spmadden/irox/commit/90221033f46472e52b4eb604183b2dc14f09005c))
    - Drawing in wasm ([`ce68d4d`](https://github.com/spmadden/irox/commit/ce68d4d5577feb8f49c2a4fc31122bf1181dfbe5))
    - Easier all-fonts loading ([`2f4ab0c`](https://github.com/spmadden/irox/commit/2f4ab0cf269eb76666be7ebaeb42f3249b74099a))
    - Rejigger to be more compatible with wasm32 ([`4462f83`](https://github.com/spmadden/irox/commit/4462f83776d2aa47681f8786667d09bc51da9ef2))
    - Also export eframe ([`49093fc`](https://github.com/spmadden/irox/commit/49093fc06cc2676b16d2244daceefa33cf9cc138))
    - Ensure NativeOptions isn't included for wasm32 ([`8103218`](https://github.com/spmadden/irox/commit/81032185a100f969cc6f26c7ee32472e1f92496b))
    - Put irox-stats dep behind plots feature where it's used ([`be75cf2`](https://github.com/spmadden/irox/commit/be75cf2c3f0b4402d71ff4d74f30733f6287447a))
    - Add eframe wgpu, glow features ([`18f86c9`](https://github.com/spmadden/irox/commit/18f86c9d7accff732208b0293dd2b118384e50b6))
    - New dialog infra, for messages & passwords ([`2a7510a`](https://github.com/spmadden/irox/commit/2a7510a6cfe434cf8264b4885f5a54e40d475f4c))
</details>

## v0.7.0 (2025-03-09)

### Documentation

 - <csr-id-270a40465f1364e048c5ba9cc962f940070e252e/> add new progress gif

### Refactor (BREAKING)

 - <csr-id-8222675712d45fb15804713e9f557b993c71057f/> BREAKING: Move egui progress window into irox_egui_extras, isolating the dep to a single module.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 8 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.7.0 ([`76c9f80`](https://github.com/spmadden/irox/commit/76c9f8002e46fdb98f249ab569eddf0d2d6a5502))
    - Release irox-progress v0.6.0, safety bump 2 crates ([`d27f82c`](https://github.com/spmadden/irox/commit/d27f82c385255f496741b01be6dcb68e04ef65f4))
    - Add new progress gif ([`270a404`](https://github.com/spmadden/irox/commit/270a40465f1364e048c5ba9cc962f940070e252e))
    - BREAKING: Move egui progress window into irox_egui_extras, isolating the dep to a single module. ([`8222675`](https://github.com/spmadden/irox/commit/8222675712d45fb15804713e9f557b993c71057f))
</details>

## v0.6.2 (2025-03-01)

<csr-id-de5e67fb00da4d87ac75adb7592f4848ba2399b2/>

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 13 calendar days.
 - 17 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.6.2 ([`b0f78c2`](https://github.com/spmadden/irox/commit/b0f78c231299c1f7bdad7dce1da9915266bd6d90))
    - Elude all the lifetimes! ([`de5e67f`](https://github.com/spmadden/irox/commit/de5e67fb00da4d87ac75adb7592f4848ba2399b2))
</details>

## v0.6.1 (2025-02-12)

<csr-id-538e405c4960593413c72be7eb9429c98d92642b/>
<csr-id-e2387bc8ef08056c542b20f8b44e463755b55acb/>

### New Features

 - <csr-id-ca5ce821102f6f51d83d17680e32a9ac8631a7fc/> Make utility functions public in logplot.rs
   Expose `needs_rerendering`, `get_screen_range`, and `get_model_range` as public to improve access for external usage. This change facilitates integration and interaction with the `logplot` module by providing necessary data and states.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 3 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.6.1 ([`dc3f710`](https://github.com/spmadden/irox/commit/dc3f710fb37158cd706b93839b20b357c3a61c23))
    - Update rendering and dependencies for egui 0.31 upgrade ([`538e405`](https://github.com/spmadden/irox/commit/538e405c4960593413c72be7eb9429c98d92642b))
    - Make utility functions public in logplot.rs ([`ca5ce82`](https://github.com/spmadden/irox/commit/ca5ce821102f6f51d83d17680e32a9ac8631a7fc))
    - Fix some lints ([`e2387bc`](https://github.com/spmadden/irox/commit/e2387bc8ef08056c542b20f8b44e463755b55acb))
</details>

## v0.6.0 (2025-02-09)

<csr-id-38267a2559f76808f24d092ddaa2c6c002c319c3/>

### New Features

 - <csr-id-c419ac5c2c140c909341021143e94348d30799c7/> reworks buildrs to remove dep from irox_egui_extras

### Bug Fixes

 - <csr-id-7ab2f23fb4208834f81a9fb29c836bc353da79aa/> pivot from glow to wgpu
 - <csr-id-af2fc489d0826e438edac6ba956193129c23b607/> fix missing build feature for eframe.

### Bug Fixes (BREAKING)

 - <csr-id-f3f5245f45aa1ba0932b6c38e4f92a361c7c3556/> fix some rendering state bugs, refactor some user-facing axis data.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 1 calendar day.
 - 14 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.6.0 ([`7999c35`](https://github.com/spmadden/irox/commit/7999c358705a82bd6da098bf97c59bc9b35968c0))
    - Pivot from glow to wgpu ([`7ab2f23`](https://github.com/spmadden/irox/commit/7ab2f23fb4208834f81a9fb29c836bc353da79aa))
    - Fix missing build feature for eframe. ([`af2fc48`](https://github.com/spmadden/irox/commit/af2fc489d0826e438edac6ba956193129c23b607))
    - Fixup grammar in plots ([`38267a2`](https://github.com/spmadden/irox/commit/38267a2559f76808f24d092ddaa2c6c002c319c3))
    - Fix some rendering state bugs, refactor some user-facing axis data. ([`f3f5245`](https://github.com/spmadden/irox/commit/f3f5245f45aa1ba0932b6c38e4f92a361c7c3556))
    - Reworks buildrs to remove dep from irox_egui_extras ([`c419ac5`](https://github.com/spmadden/irox/commit/c419ac5c2c140c909341021143e94348d30799c7))
</details>

## v0.5.6 (2025-01-26)

### Bug Fixes

 - <csr-id-9cd9e624907cded75b72e0aa9734909ded0f93d1/> hopefully fix docsrs builds for cargo, log, stats, units

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.5.6 ([`8aa9990`](https://github.com/spmadden/irox/commit/8aa999068d9cf62bad7024fe08bdc61bd656e2cb))
    - Release irox-units v0.4.5 ([`731469d`](https://github.com/spmadden/irox/commit/731469da7b0ff6d41bf7488a59767f199d62fb57))
    - Hopefully fix docsrs builds for cargo, log, stats, units ([`9cd9e62`](https://github.com/spmadden/irox/commit/9cd9e624907cded75b72e0aa9734909ded0f93d1))
</details>

## v0.5.5 (2025-01-26)

<csr-id-45a936a015f24871d007c91f9051377a3dfc1fe5/>
<csr-id-8852ee8e0dfa88b92d0c5ff0d5f1ba6b30da18ca/>
<csr-id-9b3f63a99527aa22dccebac43a29057929b7493a/>

### New Features

 - <csr-id-8270cefc0903fafcf39d8b206a250403df3c79e9/> fixup nostd support in carto
 - <csr-id-76eaf78eda488c5a0a9207bad1807351cf0bc7e7/> use new errorbars and repainter
 - <csr-id-075db4dc85ea2437eb68f7a2a8c9ae8feb0b7fda/> add new stddev error bar type
 - <csr-id-d34faf971db9204a1a0cad479793e56dac6415af/> add new repaint limiter
 - <csr-id-9d668a1c87b9ce84855314c4bf8fac74449de20b/> upgrade egui to 0.30, fix profiling and fonts
 - <csr-id-8a21395de80191d7bd2c89258541755d1aec7c85/> add TROC options in plotperf
 - <csr-id-10d59a393cd9794cef83dd62903725c41cad87f7/> line highlight cycle now pauses in between loops
 - <csr-id-197846898e443f48ced447a2e4e6208ae4b83f60/> update perftool to include a epoch bias
 - <csr-id-f6d531a23cdff1ffc2d63228a427269bcb039a8d/> better axis alignment and ranging for right axis
 - <csr-id-5bc9bb3290781a2bc26cd51021f1533ea743959d/> plot can cycle through the lines
 - <csr-id-b24dffada0c756be63053ecf158d21ae933e7fc8/> pop hovered line to the front
 - <csr-id-ff847d5fae3376dca63e294289ffcbc4f8cecb05/> draw bonus min/max line
 - <csr-id-c475e0060a653c24d1ad3d8cc74e061c6002783e/> show multiple line quantity examples in plotsperf
 - <csr-id-ea9f262077b885297aae79749b819e6a6d50e9b8/> break up error bars mesh
 - <csr-id-407c46c35dacd4464cc54f48b9b3ed94ddb10914/> expose toolframe settings as defaults in constructor
 - <csr-id-fb3cc94725b10f17b39150d1575ea15dc2dceb7b/> big perf improvement by building and caching meshes by hand whenever the data updates, rather than every frame.
 - <csr-id-a9f48abe0ce7fd9351fbadfa6f434ab3d2398ddc/> add average lines to plotsperf
 - <csr-id-e992440ae06282a790859447c75b92a3bcfaba77/> add ability to have second y-axis
 - <csr-id-c614c13c659a2aa391e03b9ef91d9617c0d81828/> new plots performance checker example
 - <csr-id-c99179a7139826518bd4c615844fa340e9083166/> rejigger line label drawing
 - <csr-id-561032bc0ab15df35412d13cc358a76e4ed9a97c/> thread-safe out-of-band data updates for plots
 - <csr-id-9d4addf0948678b541a48a662d28653bf97ff316/> ability to mark individual lines as visible/invisible.
 - <csr-id-a9c96353d588db122ac290ad6f6a50383bab13cb/> new functions to build axis formatters for time-series physical data

### Bug Fixes

 - <csr-id-aba46d3516c51d883abd5f2fdfbc596187edf8f2/> fix irox-time imports within repainting
 - <csr-id-1627a04d8aff91644a7202d5b4cc8141c0667876/> fix the linux build by adding missing egui features
 - <csr-id-7fcb3e79aa80cc2d23c7de6733ff63d787af7a67/> show time rate of change
 - <csr-id-1eec345d032e64e8719067890adf708eac74a3fe/> tweak transparent shading to be more localized to individual lines rather than the whole range
 - <csr-id-023bc48c4c9179232558879ffef6acceb998883e/> allow extra stuff alongside the frame history in the bottom bar
 - <csr-id-6e5e222a6a72cf565c8da73bb4bb157dfcd409dd/> updating examples from breaking api changes
 - <csr-id-5061f5fc9bb6e46a431ac2dc92691d35f1492c4b/> only draw the hover within the plot grid area

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 34 commits contributed to the release over the course of 40 calendar days.
 - 42 days passed between releases.
 - 33 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.5.5 ([`a850656`](https://github.com/spmadden/irox/commit/a850656664f6e71a9543095377dfe191f0ae4d40))
    - Fix irox-time imports within repainting ([`aba46d3`](https://github.com/spmadden/irox/commit/aba46d3516c51d883abd5f2fdfbc596187edf8f2))
    - Fixup nostd support in carto ([`8270cef`](https://github.com/spmadden/irox/commit/8270cefc0903fafcf39d8b206a250403df3c79e9))
    - Use new errorbars and repainter ([`76eaf78`](https://github.com/spmadden/irox/commit/76eaf78eda488c5a0a9207bad1807351cf0bc7e7))
    - Add new stddev error bar type ([`075db4d`](https://github.com/spmadden/irox/commit/075db4dc85ea2437eb68f7a2a8c9ae8feb0b7fda))
    - Add new repaint limiter ([`d34faf9`](https://github.com/spmadden/irox/commit/d34faf971db9204a1a0cad479793e56dac6415af))
    - Fix the linux build by adding missing egui features ([`1627a04`](https://github.com/spmadden/irox/commit/1627a04d8aff91644a7202d5b4cc8141c0667876))
    - Cleanup some unused egui features ([`45a936a`](https://github.com/spmadden/irox/commit/45a936a015f24871d007c91f9051377a3dfc1fe5))
    - Upgrade egui to 0.30, fix profiling and fonts ([`9d668a1`](https://github.com/spmadden/irox/commit/9d668a1c87b9ce84855314c4bf8fac74449de20b))
    - Add TROC options in plotperf ([`8a21395`](https://github.com/spmadden/irox/commit/8a21395de80191d7bd2c89258541755d1aec7c85))
    - Move structure into own method ([`9b3f63a`](https://github.com/spmadden/irox/commit/9b3f63a99527aa22dccebac43a29057929b7493a))
    - Show time rate of change ([`7fcb3e7`](https://github.com/spmadden/irox/commit/7fcb3e79aa80cc2d23c7de6733ff63d787af7a67))
    - Tweak transparent shading to be more localized to individual lines rather than the whole range ([`1eec345`](https://github.com/spmadden/irox/commit/1eec345d032e64e8719067890adf708eac74a3fe))
    - Line highlight cycle now pauses in between loops ([`10d59a3`](https://github.com/spmadden/irox/commit/10d59a393cd9794cef83dd62903725c41cad87f7))
    - Update perftool to include a epoch bias ([`1978468`](https://github.com/spmadden/irox/commit/197846898e443f48ced447a2e4e6208ae4b83f60))
    - Better axis alignment and ranging for right axis ([`f6d531a`](https://github.com/spmadden/irox/commit/f6d531a23cdff1ffc2d63228a427269bcb039a8d))
    - Plot can cycle through the lines ([`5bc9bb3`](https://github.com/spmadden/irox/commit/5bc9bb3290781a2bc26cd51021f1533ea743959d))
    - Allow extra stuff alongside the frame history in the bottom bar ([`023bc48`](https://github.com/spmadden/irox/commit/023bc48c4c9179232558879ffef6acceb998883e))
    - Pop hovered line to the front ([`b24dffa`](https://github.com/spmadden/irox/commit/b24dffada0c756be63053ecf158d21ae933e7fc8))
    - Draw bonus min/max line ([`ff847d5`](https://github.com/spmadden/irox/commit/ff847d5fae3376dca63e294289ffcbc4f8cecb05))
    - Show multiple line quantity examples in plotsperf ([`c475e00`](https://github.com/spmadden/irox/commit/c475e0060a653c24d1ad3d8cc74e061c6002783e))
    - Break up error bars mesh ([`ea9f262`](https://github.com/spmadden/irox/commit/ea9f262077b885297aae79749b819e6a6d50e9b8))
    - Expose toolframe settings as defaults in constructor ([`407c46c`](https://github.com/spmadden/irox/commit/407c46c35dacd4464cc54f48b9b3ed94ddb10914))
    - Updating examples from breaking api changes ([`6e5e222`](https://github.com/spmadden/irox/commit/6e5e222a6a72cf565c8da73bb4bb157dfcd409dd))
    - Big perf improvement by building and caching meshes by hand whenever the data updates, rather than every frame. ([`fb3cc94`](https://github.com/spmadden/irox/commit/fb3cc94725b10f17b39150d1575ea15dc2dceb7b))
    - Add average lines to plotsperf ([`a9f48ab`](https://github.com/spmadden/irox/commit/a9f48abe0ce7fd9351fbadfa6f434ab3d2398ddc))
    - Fix lints around using Arc<Vec<T>> instead of Arc<[T]> ([`8852ee8`](https://github.com/spmadden/irox/commit/8852ee8e0dfa88b92d0c5ff0d5f1ba6b30da18ca))
    - Add ability to have second y-axis ([`e992440`](https://github.com/spmadden/irox/commit/e992440ae06282a790859447c75b92a3bcfaba77))
    - New plots performance checker example ([`c614c13`](https://github.com/spmadden/irox/commit/c614c13c659a2aa391e03b9ef91d9617c0d81828))
    - Rejigger line label drawing ([`c99179a`](https://github.com/spmadden/irox/commit/c99179a7139826518bd4c615844fa340e9083166))
    - Thread-safe out-of-band data updates for plots ([`561032b`](https://github.com/spmadden/irox/commit/561032bc0ab15df35412d13cc358a76e4ed9a97c))
    - Only draw the hover within the plot grid area ([`5061f5f`](https://github.com/spmadden/irox/commit/5061f5fc9bb6e46a431ac2dc92691d35f1492c4b))
    - Ability to mark individual lines as visible/invisible. ([`9d4addf`](https://github.com/spmadden/irox/commit/9d4addf0948678b541a48a662d28653bf97ff316))
    - New functions to build axis formatters for time-series physical data ([`a9c9635`](https://github.com/spmadden/irox/commit/a9c96353d588db122ac290ad6f6a50383bab13cb))
</details>

## v0.5.4 (2024-12-15)

### New Features

 - <csr-id-42724120f69243ffb0f93a1524986dc48c79091e/> expose data changed flag in plot

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.5.4 ([`fc569d0`](https://github.com/spmadden/irox/commit/fc569d0d513472f1fa43da6c817ed1e28498ee41))
    - Expose data changed flag in plot ([`4272412`](https://github.com/spmadden/irox/commit/42724120f69243ffb0f93a1524986dc48c79091e))
</details>

## v0.5.3 (2024-12-15)

### New Features

 - <csr-id-f2302e18dd620eba54e66732e402acca14523afb/> cleaning up some deps for wasm
 - <csr-id-22defec5feec02f5b4555a66a39aa6e97c9e4d35/> Plot speed improvements, drop egui_plot dep.
 - <csr-id-b46e8489365045efc963e137c324da44c71be5aa/> add ability to draw shapes on exact plot points
 - <csr-id-d3c8dff38f9b68843636ecb660c60af71891b1ff/> add snapping to graph mouseover
 - <csr-id-df5cccda93d8929266b682c1a3624a09e7482d45/> plots mouseover now use value formatters

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.5.3 ([`eb50a9c`](https://github.com/spmadden/irox/commit/eb50a9c91bab1d3ed56554081614dd6ca5ac9aad))
    - Cleaning up some deps for wasm ([`f2302e1`](https://github.com/spmadden/irox/commit/f2302e18dd620eba54e66732e402acca14523afb))
    - Plot speed improvements, drop egui_plot dep. ([`22defec`](https://github.com/spmadden/irox/commit/22defec5feec02f5b4555a66a39aa6e97c9e4d35))
    - Add ability to draw shapes on exact plot points ([`b46e848`](https://github.com/spmadden/irox/commit/b46e8489365045efc963e137c324da44c71be5aa))
    - Add snapping to graph mouseover ([`d3c8dff`](https://github.com/spmadden/irox/commit/d3c8dff38f9b68843636ecb660c60af71891b1ff))
    - Plots mouseover now use value formatters ([`df5cccd`](https://github.com/spmadden/irox/commit/df5cccda93d8929266b682c1a3624a09e7482d45))
</details>

## v0.5.2 (2024-12-14)

### Bug Fixes

 - <csr-id-091e86dd80a05abd833cbd7460c542bc777871fa/> fix dependency interaction for egui::Widget impl

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.5.2 ([`edab4ba`](https://github.com/spmadden/irox/commit/edab4ba46190aae8315e102bb1439c3cfc473d4b))
    - Fix dependency interaction for egui::Widget impl ([`091e86d`](https://github.com/spmadden/irox/commit/091e86dd80a05abd833cbd7460c542bc777871fa))
</details>

## v0.5.1 (2024-12-13)

### New Features

 - <csr-id-208e170334ed6ce08cbcd86cd503feb36ca18024/> multiple lines in plots, and colors!
 - <csr-id-759c0b9b0d0b6ba037afca7f69d6851df4e9722d/> adding additional fonts
 - <csr-id-79b4c0111cfb4daff7419dda335fca312e4afa4e/> bump MSRV to 1.82

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 20 calendar days.
 - 45 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.5.1 ([`67d16d5`](https://github.com/spmadden/irox/commit/67d16d505e7cebc942a7f99d8c7e88ebcb102299))
    - Multiple lines in plots, and colors! ([`208e170`](https://github.com/spmadden/irox/commit/208e170334ed6ce08cbcd86cd503feb36ca18024))
    - Adding additional fonts ([`759c0b9`](https://github.com/spmadden/irox/commit/759c0b9b0d0b6ba037afca7f69d6851df4e9722d))
    - Bump MSRV to 1.82 ([`79b4c01`](https://github.com/spmadden/irox/commit/79b4c0111cfb4daff7419dda335fca312e4afa4e))
</details>

## v0.5.0 (2024-10-29)

<csr-id-bf43c95acb312542bb94ca647366f0c23a692f19/>

### New Features

 - <csr-id-042f09d4a5463123b9ad02f2bb17b9226df11990/> update about window with build host variables.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 3 calendar days.
 - 42 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.5.0 ([`bbe46e7`](https://github.com/spmadden/irox/commit/bbe46e765791861109871c6e201bbf8f8a0e8c0e))
    - Upgrade to latest libraries, including egui 0.29.1 ([`bf43c95`](https://github.com/spmadden/irox/commit/bf43c95acb312542bb94ca647366f0c23a692f19))
    - Update about window with build host variables. ([`042f09d`](https://github.com/spmadden/irox/commit/042f09d4a5463123b9ad02f2bb17b9226df11990))
</details>

## v0.4.6 (2024-09-17)

### Bug Fixes

 - <csr-id-53ee70d68dfcebe2fdf134d5a7d0b6c860e346cb/> adding required Send trait to formatter function

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.4.6 ([`c501b0a`](https://github.com/spmadden/irox/commit/c501b0a4da16c8aaf816ba3f25381f3f78d0d263))
    - Adding required Send trait to formatter function ([`53ee70d`](https://github.com/spmadden/irox/commit/53ee70d68dfcebe2fdf134d5a7d0b6c860e346cb))
</details>

## v0.4.5 (2024-09-15)

### New Features

 - <csr-id-db1fcf488d8ec101ee79ebbf0c8363f30bad9a11/> new ability in logplot to format the axis detents.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 4 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.4.5 ([`9e0d778`](https://github.com/spmadden/irox/commit/9e0d7787e8d42de13cb85ac9e544afc838c1d0f7))
    - New ability in logplot to format the axis detents. ([`db1fcf4`](https://github.com/spmadden/irox/commit/db1fcf488d8ec101ee79ebbf0c8363f30bad9a11))
</details>

## v0.4.4 (2024-09-11)

### New Features

 - <csr-id-6927e8b9576c895bf41455d68c7263635bfa64cc/> add axis labels to BasicPlot
 - <csr-id-26394ddde84dd66c75d8197abf83d8ca0ddbacf0/> adding title to BasicPlot

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 9 calendar days.
 - 37 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.4.4 ([`0da8d38`](https://github.com/spmadden/irox/commit/0da8d38d68179c142db4df25a7591bb9f5ab2d1a))
    - Add axis labels to BasicPlot ([`6927e8b`](https://github.com/spmadden/irox/commit/6927e8b9576c895bf41455d68c7263635bfa64cc))
    - Adding title to BasicPlot ([`26394dd`](https://github.com/spmadden/irox/commit/26394ddde84dd66c75d8197abf83d8ca0ddbacf0))
</details>

## v0.4.3 (2024-08-05)

### Bug Fixes

 - <csr-id-62973622e93ec1c331d2e7a4d32779f1175302ea/> fix inverted if check in logplot zooming

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.4.3 ([`5826d07`](https://github.com/spmadden/irox/commit/5826d07803d94b3d231d5ea5a56ee2fb9298f25d))
    - Fix inverted if check in logplot zooming ([`6297362`](https://github.com/spmadden/irox/commit/62973622e93ec1c331d2e7a4d32779f1175302ea))
</details>

## v0.4.2 (2024-08-05)

<csr-id-29a9615441c4762311484e3b6ecf32745dc3c486/>

### New Features

 - <csr-id-dcdd6df468290e02abd9188f3ec68f5aaf3e49c3/> plots now zoom

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 1 day passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.4.2 ([`f4c2fea`](https://github.com/spmadden/irox/commit/f4c2fea0eb390e0174140c3beb08d7ff845f5627))
    - Release irox-egui-extras v0.4.2 ([`3ab0f78`](https://github.com/spmadden/irox/commit/3ab0f78936ee783ae3228044416b93ab140688d3))
    - Fix lints ([`29a9615`](https://github.com/spmadden/irox/commit/29a9615441c4762311484e3b6ecf32745dc3c486))
    - Plots now zoom ([`dcdd6df`](https://github.com/spmadden/irox/commit/dcdd6df468290e02abd9188f3ec68f5aaf3e49c3))
</details>

## v0.4.1 (2024-08-04)

### New Features

 - <csr-id-5c25d79fdfcb3cfd70bdbc472c790bb40c96c86e/> rework the logplot axes again to be more robust

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 3 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.4.1 ([`7edfabe`](https://github.com/spmadden/irox/commit/7edfabe6835dd203ecb4530a56a1224e82f6bae9))
    - Rework the logplot axes again to be more robust ([`5c25d79`](https://github.com/spmadden/irox/commit/5c25d79fdfcb3cfd70bdbc472c790bb40c96c86e))
</details>

## v0.4.0 (2024-08-01)

### New Features

 - <csr-id-0ab81237491d7797da62156f9c8a3fef0d16097e/> tweak the way that the logplot does autoscaling

### Bug Fixes

 - <csr-id-6d4065fde94c672170008017a30685ad7a293712/> fix lints

### New Features (BREAKING)

 - <csr-id-dfa11be274b8e5a61a40c9965096208d64e1fdba/> update egui to 0.28.0

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 10 calendar days.
 - 23 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.4.0 ([`74d6a21`](https://github.com/spmadden/irox/commit/74d6a21d05f69152f7872b55c1130db097748981))
    - Fix lints ([`6d4065f`](https://github.com/spmadden/irox/commit/6d4065fde94c672170008017a30685ad7a293712))
    - Tweak the way that the logplot does autoscaling ([`0ab8123`](https://github.com/spmadden/irox/commit/0ab81237491d7797da62156f9c8a3fef0d16097e))
    - Update egui to 0.28.0 ([`dfa11be`](https://github.com/spmadden/irox/commit/dfa11be274b8e5a61a40c9965096208d64e1fdba))
</details>

## v0.3.12 (2024-07-09)

### New Features

 - <csr-id-fc47b630c4ddda881af8a2da3a0917391088531f/> add about build button
 - <csr-id-317d6b7bd95c2ece9fee79c4a0521cd0f5dc2012/> add toggle buttons to gallery
 - <csr-id-ceeef3484020d9ab171979ec3a94201ebe32f607/> add mouseover capability for logplot
 - <csr-id-109a8c43908cf43a18bc38b2ae6eba859d1a32b8/> Add About Window using irox-build-rs
 - <csr-id-162f02e7597a778f7220903e14bf4ab56b49d852/> start to generate build info for egui-extras

### Bug Fixes

 - <csr-id-bbb2b9698cadb06c939daf857c0c82665990143b/> move WithAlpha trait to crate top-level to fix packaging error

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 2 calendar days.
 - 3 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.3.12 ([`51933cb`](https://github.com/spmadden/irox/commit/51933cb24f54862e1575cbce95ac386e67e21fdc))
    - Move WithAlpha trait to crate top-level to fix packaging error ([`bbb2b96`](https://github.com/spmadden/irox/commit/bbb2b9698cadb06c939daf857c0c82665990143b))
    - Add about build button ([`fc47b63`](https://github.com/spmadden/irox/commit/fc47b630c4ddda881af8a2da3a0917391088531f))
    - Add toggle buttons to gallery ([`317d6b7`](https://github.com/spmadden/irox/commit/317d6b7bd95c2ece9fee79c4a0521cd0f5dc2012))
    - Add mouseover capability for logplot ([`ceeef34`](https://github.com/spmadden/irox/commit/ceeef3484020d9ab171979ec3a94201ebe32f607))
    - Add About Window using irox-build-rs ([`109a8c4`](https://github.com/spmadden/irox/commit/109a8c43908cf43a18bc38b2ae6eba859d1a32b8))
    - Start to generate build info for egui-extras ([`162f02e`](https://github.com/spmadden/irox/commit/162f02e7597a778f7220903e14bf4ab56b49d852))
</details>

## v0.3.11 (2024-07-06)

### New Features

 - <csr-id-57dcb42a7f88f2d23bd1117d0ad59e9300b98ee9/> add drag highlight in logplot, soon zoom.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 8 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.3.11 ([`2b6f331`](https://github.com/spmadden/irox/commit/2b6f3318a8c9081721c9d948a6de4fe4191551f6))
    - Add drag highlight in logplot, soon zoom. ([`57dcb42`](https://github.com/spmadden/irox/commit/57dcb42a7f88f2d23bd1117d0ad59e9300b98ee9))
</details>

## v0.3.10 (2024-06-28)

<csr-id-cc3197e276912dd7aadf83c092086369381aad91/>
<csr-id-e2a48c22f0dab037c9db1710a3b3e4764e199404/>

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 26 calendar days.
 - 41 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.3.10 ([`726d000`](https://github.com/spmadden/irox/commit/726d00044987c5e42299213b0352797d6f3196d1))
    - Clean lints in egui_extras ([`cc3197e`](https://github.com/spmadden/irox/commit/cc3197e276912dd7aadf83c092086369381aad91))
    - Clean up new lints ([`e2a48c2`](https://github.com/spmadden/irox/commit/e2a48c22f0dab037c9db1710a3b3e4764e199404))
</details>

## v0.3.9 (2024-05-18)

### New Features

 - <csr-id-e90d10a12fb82c0a9070df65f94a1a1c977ca2ff/> add dB space scaling to plot
 - <csr-id-923576ef0c6d50f1ce719d1be73b3b44c5f22475/> much happier with the final state of the new plot widget.
 - <csr-id-d4b1c16431af7d952a4c799fb0f6b15bf39c4539/> add basic plot

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 13 calendar days.
 - 24 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.3.9 ([`d589ea2`](https://github.com/spmadden/irox/commit/d589ea2748f76c3f3fc33d69ae8ed63661ca9bf7))
    - Add dB space scaling to plot ([`e90d10a`](https://github.com/spmadden/irox/commit/e90d10a12fb82c0a9070df65f94a1a1c977ca2ff))
    - Much happier with the final state of the new plot widget. ([`923576e`](https://github.com/spmadden/irox/commit/923576ef0c6d50f1ce719d1be73b3b44c5f22475))
    - Add basic plot ([`d4b1c16`](https://github.com/spmadden/irox/commit/d4b1c16431af7d952a4c799fb0f6b15bf39c4539))
</details>

## v0.3.8 (2024-04-24)

<csr-id-169d25bed38e36f5ae87e69a2bac055290ea0ff6/>

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 52 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.3.8 ([`c38ade3`](https://github.com/spmadden/irox/commit/c38ade31d68081990a4d71bdb0d1457f858c1b65))
    - Bump version of egui-extras for deps ([`169d25b`](https://github.com/spmadden/irox/commit/169d25bed38e36f5ae87e69a2bac055290ea0ff6))
</details>

## v0.3.7 (2024-03-03)

### New Features

 - <csr-id-49a1d9e676a22ec0d83e5cb98916ddfbd2158d4b/> bump egui to 0.26.0
 - <csr-id-86f758445258f5942fb4b33fa75e2b7c0d64a17f/> updated readme and examples
 - <csr-id-855a5d0b0c8b984536f45299197a0ddfa2adb217/> new serde debug serializer to display objects
 - <csr-id-5ecf31f0f9d77e635569689ce35aab34e9cdadf0/> new toolframe components for easy tools

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 26 calendar days.
 - 51 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.3.7 ([`ea99a3f`](https://github.com/spmadden/irox/commit/ea99a3f39286e7670997f794dc67a0d7cebd8faa))
    - Bump egui to 0.26.0 ([`49a1d9e`](https://github.com/spmadden/irox/commit/49a1d9e676a22ec0d83e5cb98916ddfbd2158d4b))
    - Updated readme and examples ([`86f7584`](https://github.com/spmadden/irox/commit/86f758445258f5942fb4b33fa75e2b7c0d64a17f))
    - New serde debug serializer to display objects ([`855a5d0`](https://github.com/spmadden/irox/commit/855a5d0b0c8b984536f45299197a0ddfa2adb217))
    - New toolframe components for easy tools ([`5ecf31f`](https://github.com/spmadden/irox/commit/5ecf31f0f9d77e635569689ce35aab34e9cdadf0))
</details>

## v0.3.6 (2024-01-12)

<csr-id-0fc37b1a2d545e8d6479443f2a55b3ad64bf5a39/>

### New Features

 - <csr-id-303aca0c8fb37543c95ab4c14770e671db4d5b11/> playing with wasm, now can make webpages!

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 10 calendar days.
 - 26 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.3.6, irox-progress v0.3.0 ([`f23a34b`](https://github.com/spmadden/irox/commit/f23a34bf76874bd6565db16606a68a4cd1056f18))
    - Fixup newline formatting ([`0fc37b1`](https://github.com/spmadden/irox/commit/0fc37b1a2d545e8d6479443f2a55b3ad64bf5a39))
    - Playing with wasm, now can make webpages! ([`303aca0`](https://github.com/spmadden/irox/commit/303aca0c8fb37543c95ab4c14770e671db4d5b11))
</details>

## v0.3.5 (2023-12-17)

### Bug Fixes

 - <csr-id-b4a61eaee2797b459366487c8b2c60dd19b46f99/> fix packaging on linux

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 12 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.3.5 ([`c6bfcae`](https://github.com/spmadden/irox/commit/c6bfcae2dd3a43b06859a309639474829734afca))
    - Fix packaging on linux ([`b4a61ea`](https://github.com/spmadden/irox/commit/b4a61eaee2797b459366487c8b2c60dd19b46f99))
</details>

## v0.3.4 (2023-12-05)

### New Features

 - <csr-id-92294eb04dda490695d7ab4fde72de759be4045e/> progress bar now can draw text left, center, and right aligned

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
    - Release irox-egui-extras v0.3.4 ([`2858b60`](https://github.com/spmadden/irox/commit/2858b6001041e20427f878cd13df280406dac271))
    - Progress bar now can draw text left, center, and right aligned ([`92294eb`](https://github.com/spmadden/irox/commit/92294eb04dda490695d7ab4fde72de759be4045e))
</details>

## v0.3.3 (2023-11-29)

<csr-id-88ebfb5deea5508ca54f4aaab62f6fd5a36f531c/>

### New Features

 - <csr-id-d08939007d939152e532b84654208b0156b3a8d6/> update egui to 0.24
 - <csr-id-64c956eb3d26546caaf0f8d3e8c00d8ae44a74f1/> disable all default features for egui deps at workspace level

### Bug Fixes

 - <csr-id-33a3e98e1d2493e7703cf519d9a4514e0f7587a9/> put serde behind a feature gate to fix standalone checks
 - <csr-id-222158b51b32dbb38a20b548fe2b4efd1b1d0486/> fix lint in egui progressbar

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 14 calendar days.
 - 27 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.3.3 ([`bd453bc`](https://github.com/spmadden/irox/commit/bd453bc79951479f48db0be8f12dc035fab95ed0))
    - Put serde behind a feature gate to fix standalone checks ([`33a3e98`](https://github.com/spmadden/irox/commit/33a3e98e1d2493e7703cf519d9a4514e0f7587a9))
    - Pivot to using Cargo.toml workspace lints ([`88ebfb5`](https://github.com/spmadden/irox/commit/88ebfb5deea5508ca54f4aaab62f6fd5a36f531c))
    - Update egui to 0.24 ([`d089390`](https://github.com/spmadden/irox/commit/d08939007d939152e532b84654208b0156b3a8d6))
    - Disable all default features for egui deps at workspace level ([`64c956e`](https://github.com/spmadden/irox/commit/64c956eb3d26546caaf0f8d3e8c00d8ae44a74f1))
    - Fix lint in egui progressbar ([`222158b`](https://github.com/spmadden/irox/commit/222158b51b32dbb38a20b548fe2b4efd1b1d0486))
</details>

## v0.3.2 (2023-11-02)

### New Features

 - <csr-id-69bb1e4bcb962b22bcb7d46582c2b877a03b6322/> Add new progressbar impl with indeterminate mode

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 4 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.3.2, irox-progress v0.1.2 ([`73e0e72`](https://github.com/spmadden/irox/commit/73e0e7222582695151958708533c033c7eec61bc))
    - Add new progressbar impl with indeterminate mode ([`69bb1e4`](https://github.com/spmadden/irox/commit/69bb1e4bcb962b22bcb7d46582c2b877a03b6322))
</details>

## v0.3.1 (2023-10-29)

<csr-id-7218f70a16f1a21f1ed716bb7aaa712511645476/>

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 14 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-egui-extras v0.3.1 ([`8e46d34`](https://github.com/spmadden/irox/commit/8e46d34b6abc9fa1eb8379460dcbddcc8a2d4326))
    - ALL THE LINTS. ([`7218f70`](https://github.com/spmadden/irox/commit/7218f70a16f1a21f1ed716bb7aaa712511645476))
</details>

## v0.3.0 (2023-10-15)

### New Features (BREAKING)

 - <csr-id-ecf8c32dab7374550c63ad62aa3a6637238bdca9/> bump egui dep to 0.23.0

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 7 calendar days.
 - 8 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-carto v0.3.0, irox-csv v0.3.0, irox-egui-extras v0.3.0, irox-gpx v0.2.0, irox-influxdb_v1 v0.3.0, irox-nmea0183 v0.2.0, irox-raymarine-sonar v0.2.0, irox-time v0.1.0, irox-winlocation-api v0.2.0, irox v0.3.0 ([`dfa6258`](https://github.com/spmadden/irox/commit/dfa6258b8f93f6d27b85d2f3f4e209599a8168ad))
    - Release irox-units v0.3.0, irox-carto v0.3.0, irox-csv v0.3.0, irox-egui-extras v0.3.0, irox-gpx v0.2.0, irox-influxdb_v1 v0.3.0, irox-nmea0183 v0.2.0, irox-raymarine-sonar v0.2.0, irox-time v0.1.0, irox-winlocation-api v0.2.0, irox v0.3.0, safety bump 2 crates ([`a6c0a5f`](https://github.com/spmadden/irox/commit/a6c0a5fcfc4070b8cbc1442192b7eaef275e80f2))
    - Bump egui dep to 0.23.0 ([`ecf8c32`](https://github.com/spmadden/irox/commit/ecf8c32dab7374550c63ad62aa3a6637238bdca9))
</details>

## v0.2.1 (2023-10-07)

<csr-id-f99614a5ce3368072b4d44dacede0e6e847b0b2e/>

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 19 calendar days.
 - 20 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.2.1, irox-carto v0.2.1, irox-egui-extras v0.2.1, irox-gpx v0.1.0, irox-types v0.2.1, irox-structs_derive v0.2.1, irox-raymarine-sonar v0.1.0, irox-stats v0.2.1, irox-winlocation-api v0.1.1, irox v0.2.1 ([`68d770b`](https://github.com/spmadden/irox/commit/68d770bb78abe49bf30364ca17ddb6f7bfda05d9))
    - Fix up the readmes for publishing ([`f99614a`](https://github.com/spmadden/irox/commit/f99614a5ce3368072b4d44dacede0e6e847b0b2e))
</details>

## v0.2.0 (2023-09-17)

<csr-id-80d2b88bdcb553faaeafc09673c31d7ebedafd19/>
<csr-id-1a365333397b02a5f911d0897c3bf0c80f6c2b80/>
<csr-id-5c178560becc0b665d70be2d99a1cffad3ba4284/>

### New Features

 - <csr-id-f98cc23ab3b3270e55c70d59d5ca75e10e8acc62/> fix module name to be consistent

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 28 calendar days.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-enums_derive v0.2.0, irox-enums v0.2.0, irox-tools v0.2.0, irox-units v0.2.0, irox-carto v0.2.0, irox-csv v0.2.0, irox-egui-extras v0.2.0, irox-networking v0.2.0, irox-types v0.2.0, irox-influxdb_v1 v0.2.0, irox-structs_derive v0.2.0, irox-structs v0.2.0, irox-nmea0183 v0.1.0, irox-sirf v0.2.0, irox-stats v0.2.0, irox-winlocation-api v0.1.0, irox v0.2.0, safety bump 10 crates ([`6a72204`](https://github.com/spmadden/irox/commit/6a722046661ceef02a66c2067e2c5c15ce102e04))
    - Update cargo.tomls to add repository ([`80d2b88`](https://github.com/spmadden/irox/commit/80d2b88bdcb553faaeafc09673c31d7ebedafd19))
    - Setting up blank changelogs for the modules ([`1a36533`](https://github.com/spmadden/irox/commit/1a365333397b02a5f911d0897c3bf0c80f6c2b80))
    - Cleaning up clippy warnings ([`5c17856`](https://github.com/spmadden/irox/commit/5c178560becc0b665d70be2d99a1cffad3ba4284))
    - Fix module name to be consistent ([`f98cc23`](https://github.com/spmadden/irox/commit/f98cc23ab3b3270e55c70d59d5ca75e10e8acc62))
</details>

