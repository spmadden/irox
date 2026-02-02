

## v0.1.2 (2026-02-01)

### Bug Fixes

 - <csr-id-442dc8cc475b26d8e6801a3a1ccbb85c99100520/> docsrs fix round 2

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Docsrs fix round 2 ([`442dc8c`](https://github.com/spmadden/irox/commit/442dc8cc475b26d8e6801a3a1ccbb85c99100520))
</details>

## v0.1.1 (2026-02-01)

### Bug Fixes

 - <csr-id-841968f65860c4b70f41d3391b66457164c1fad9/> fix docsrs build :<

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-cryptids v0.1.1 ([`c2a26ad`](https://github.com/spmadden/irox/commit/c2a26ada67146c4e827877619d222da1954859e6))
    - Fix docsrs build :< ([`841968f`](https://github.com/spmadden/irox/commit/841968f65860c4b70f41d3391b66457164c1fad9))
</details>

## v0.1.0 (2026-02-01)

<csr-id-691776a610c7e169cdbf888f4d18892a5cf2377c/>
<csr-id-de5e67fb00da4d87ac75adb7592f4848ba2399b2/>
<csr-id-2de510fd54646200cf214dadcd2323273bd1db94/>
<csr-id-d2df47065c23ebaf632143cb1c8a4cb6dad559cc/>
<csr-id-759bf9c7de15076346de3c8f5c93b493b81729d8/>

### Chore

 - <csr-id-691776a610c7e169cdbf888f4d18892a5cf2377c/> fix lints & bump to 1.89
 - <csr-id-de5e67fb00da4d87ac75adb7592f4848ba2399b2/> elude all the lifetimes!

### New Features

 - <csr-id-e1dba89b4e14ebcff14d88760352c85690441a00/> Salsa rounds & key functions
 - <csr-id-5a2fd2d0a9e5f3bd564946dab3594b180fb4c2fc/> Ed25519 option for different hash alg than SHA512
 - <csr-id-dd88b0d52b87e6a563aab351e3baedbf9850504c/> rejigger to be more compatible with wasm32
 - <csr-id-e006dd4a762b051d81fcfd91cec495ac45bf9fe4/> generate random keys, more conversions
 - <csr-id-841296828e211d0195a2e650dd5579f36b910e0f/> adding new crng utility methods
 - <csr-id-8b74e91500e229023f9c8e47761d975674c98e10/> new crng based on ChaCha20
 - <csr-id-dabee1ee8e4b8467c8d289dc5a69bc5f63f880b9/> add real feature 'std'
 - <csr-id-43c2df500cc7cc96d080693ed4f27b8e337482ac/> more PubKey creation types
 - <csr-id-2654f386674d7cbb05755e3767ca0403402a175d/> AEAD ChaCha20Poly1305-Dec
 - <csr-id-f8a5370247817d2a490dd391de69f173b0705eb9/> AEAD ChaCha20Poly1305-Enc
 - <csr-id-8e89ce272834ee7c52c3b188277fb13b4a814a11/> rework poly1305 to be buffer based
 - <csr-id-764de5b1ceaae30b61d9bbe7ae2d751af8d0e8b1/> poly1305 passes rfc8349 test cases
 - <csr-id-70c325dcbcfe2af58ee4a7453f0d6beeac8a9a8d/> improve usability of signatures, add extra curve params
 - <csr-id-d562dd01591c4988ad5f595282634c6b2b64e4a3/> some checks for invalid curve inputs
 - <csr-id-385518615f8d1daf8a22a76bf0ada44527e1ba44/> can create valid ed25519 signatures
 - <csr-id-6154f88dfd0b4fe9c9ad092c4bdf7e792ad597d7/> toggle on no_std
 - <csr-id-c21d85755c59e7b30d04621b33465c05f2ff49ad/> ed25519 passes the 1024 signature verification tests.
 - <csr-id-25c51600d9f709e35b70a4d08531bce2c2407e6e/> first pass at a functioning ed25519 impl
 - <csr-id-c6b5bc93ace65add86d4b49f14e3f60a327aa6eb/> remove branches in the x25519 inversion code
 - <csr-id-f3e28cc8c3268a9634b67f6989dbeb1030dbd385/> clean up copying of secret data in x25519
 - <csr-id-cdfcc8e1fdc44ceedda8619ad808ee8b81db9791/> first pass at a functional Curve25519 - needs validations
 - <csr-id-164b2fa94bce9ab6eff8d4aaa4e180f243cafa66/> adding more pbkdf2 test vectors
 - <csr-id-b9a58e94c511aae0cfe327cdf40f8261522cbd10/> new pbkdf2 impl
 - <csr-id-01f712be472dab6501934a7669c618b8d9aa4bff/> adding the full set of RFC test cases to chacha20
 - <csr-id-795f0a98f6b1a15fccfe2f1df41df6f4e3d05095/> new ChaCha20 impl based on RFC8439
 - <csr-id-e1ec9580d82f7f8b01d77d6c0452aef4dc4e3509/> basic chacha20 impl
 - <csr-id-757ed35f0df71f067b5b45f1b05a2053eebe028e/> new awful crypto module that shouldn't be used by anyone.

### Bug Fixes

 - <csr-id-066ee5f0f67755c99fc6ee25d91d35e07a25822d/> fix import for build
 - <csr-id-35092c06065baf9e2508e47e8680b0d297947e47/> add missing std feature
 - <csr-id-270063ee47835e139f347832717fbc74cbbf46b0/> fix regression caused from futzing with FixedU8Buf
 - <csr-id-b7fa87a606780d8d8855a44b0b9c8e0b1e39fd90/> fix x25519 bencher
 - <csr-id-348890dc3f736ba34297142b3fc0af03aa2c59fc/> fix aes test compilation error
 - <csr-id-99d549d56286fbca875ec3fcfa2caee297319fe0/> ignore currently failing test - will fix

### Other

 - <csr-id-2de510fd54646200cf214dadcd2323273bd1db94/> cryptids
 - <csr-id-d2df47065c23ebaf632143cb1c8a4cb6dad559cc/> poly1305
 - <csr-id-759bf9c7de15076346de3c8f5c93b493b81729d8/> update readme to be useful

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 41 commits contributed to the release over the course of 428 calendar days.
 - 38 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-cryptids v0.1.0 ([`cdc5c9a`](https://github.com/spmadden/irox/commit/cdc5c9a54d33ce83dc72a5316244948e14946762))
    - Salsa rounds & key functions ([`e1dba89`](https://github.com/spmadden/irox/commit/e1dba89b4e14ebcff14d88760352c85690441a00))
    - Ed25519 option for different hash alg than SHA512 ([`5a2fd2d`](https://github.com/spmadden/irox/commit/5a2fd2d0a9e5f3bd564946dab3594b180fb4c2fc))
    - Fix lints & bump to 1.89 ([`691776a`](https://github.com/spmadden/irox/commit/691776a610c7e169cdbf888f4d18892a5cf2377c))
    - Rejigger to be more compatible with wasm32 ([`dd88b0d`](https://github.com/spmadden/irox/commit/dd88b0d52b87e6a563aab351e3baedbf9850504c))
    - Fix import for build ([`066ee5f`](https://github.com/spmadden/irox/commit/066ee5f0f67755c99fc6ee25d91d35e07a25822d))
    - Generate random keys, more conversions ([`e006dd4`](https://github.com/spmadden/irox/commit/e006dd4a762b051d81fcfd91cec495ac45bf9fe4))
    - Adding new crng utility methods ([`8412968`](https://github.com/spmadden/irox/commit/841296828e211d0195a2e650dd5579f36b910e0f))
    - Add missing std feature ([`35092c0`](https://github.com/spmadden/irox/commit/35092c06065baf9e2508e47e8680b0d297947e47))
    - New crng based on ChaCha20 ([`8b74e91`](https://github.com/spmadden/irox/commit/8b74e91500e229023f9c8e47761d975674c98e10))
    - Add real feature 'std' ([`dabee1e`](https://github.com/spmadden/irox/commit/dabee1ee8e4b8467c8d289dc5a69bc5f63f880b9))
    - More PubKey creation types ([`43c2df5`](https://github.com/spmadden/irox/commit/43c2df500cc7cc96d080693ed4f27b8e337482ac))
    - Fix regression caused from futzing with FixedU8Buf ([`270063e`](https://github.com/spmadden/irox/commit/270063ee47835e139f347832717fbc74cbbf46b0))
    - AEAD ChaCha20Poly1305-Dec ([`2654f38`](https://github.com/spmadden/irox/commit/2654f386674d7cbb05755e3767ca0403402a175d))
    - AEAD ChaCha20Poly1305-Enc ([`f8a5370`](https://github.com/spmadden/irox/commit/f8a5370247817d2a490dd391de69f173b0705eb9))
    - Rework poly1305 to be buffer based ([`8e89ce2`](https://github.com/spmadden/irox/commit/8e89ce272834ee7c52c3b188277fb13b4a814a11))
    - Poly1305 passes rfc8349 test cases ([`764de5b`](https://github.com/spmadden/irox/commit/764de5b1ceaae30b61d9bbe7ae2d751af8d0e8b1))
    - Improve usability of signatures, add extra curve params ([`70c325d`](https://github.com/spmadden/irox/commit/70c325dcbcfe2af58ee4a7453f0d6beeac8a9a8d))
    - Some checks for invalid curve inputs ([`d562dd0`](https://github.com/spmadden/irox/commit/d562dd01591c4988ad5f595282634c6b2b64e4a3))
    - Cryptids ([`2de510f`](https://github.com/spmadden/irox/commit/2de510fd54646200cf214dadcd2323273bd1db94))
    - Can create valid ed25519 signatures ([`3855186`](https://github.com/spmadden/irox/commit/385518615f8d1daf8a22a76bf0ada44527e1ba44))
    - Release irox-bits v0.4.2 ([`da45a93`](https://github.com/spmadden/irox/commit/da45a93d8a0e1621f4ac63dc77f8e00528cffba4))
    - Fix x25519 bencher ([`b7fa87a`](https://github.com/spmadden/irox/commit/b7fa87a606780d8d8855a44b0b9c8e0b1e39fd90))
    - Toggle on no_std ([`6154f88`](https://github.com/spmadden/irox/commit/6154f88dfd0b4fe9c9ad092c4bdf7e792ad597d7))
    - Ed25519 passes the 1024 signature verification tests. ([`c21d857`](https://github.com/spmadden/irox/commit/c21d85755c59e7b30d04621b33465c05f2ff49ad))
    - First pass at a functioning ed25519 impl ([`25c5160`](https://github.com/spmadden/irox/commit/25c51600d9f709e35b70a4d08531bce2c2407e6e))
    - Remove branches in the x25519 inversion code ([`c6b5bc9`](https://github.com/spmadden/irox/commit/c6b5bc93ace65add86d4b49f14e3f60a327aa6eb))
    - Clean up copying of secret data in x25519 ([`f3e28cc`](https://github.com/spmadden/irox/commit/f3e28cc8c3268a9634b67f6989dbeb1030dbd385))
    - First pass at a functional Curve25519 - needs validations ([`cdfcc8e`](https://github.com/spmadden/irox/commit/cdfcc8e1fdc44ceedda8619ad808ee8b81db9791))
    - Elude all the lifetimes! ([`de5e67f`](https://github.com/spmadden/irox/commit/de5e67fb00da4d87ac75adb7592f4848ba2399b2))
    - Adding more pbkdf2 test vectors ([`164b2fa`](https://github.com/spmadden/irox/commit/164b2fa94bce9ab6eff8d4aaa4e180f243cafa66))
    - New pbkdf2 impl ([`b9a58e9`](https://github.com/spmadden/irox/commit/b9a58e94c511aae0cfe327cdf40f8261522cbd10))
    - Poly1305 ([`d2df470`](https://github.com/spmadden/irox/commit/d2df47065c23ebaf632143cb1c8a4cb6dad559cc))
    - Update readme to be useful ([`759bf9c`](https://github.com/spmadden/irox/commit/759bf9c7de15076346de3c8f5c93b493b81729d8))
    - Fix aes test compilation error ([`348890d`](https://github.com/spmadden/irox/commit/348890dc3f736ba34297142b3fc0af03aa2c59fc))
    - Adding the full set of RFC test cases to chacha20 ([`01f712b`](https://github.com/spmadden/irox/commit/01f712be472dab6501934a7669c618b8d9aa4bff))
    - New ChaCha20 impl based on RFC8439 ([`795f0a9`](https://github.com/spmadden/irox/commit/795f0a98f6b1a15fccfe2f1df41df6f4e3d05095))
    - Basic chacha20 impl ([`e1ec958`](https://github.com/spmadden/irox/commit/e1ec9580d82f7f8b01d77d6c0452aef4dc4e3509))
    - Release irox-bits v0.3.0 ([`32e7b8d`](https://github.com/spmadden/irox/commit/32e7b8dbcb854c7eaebe3473145cbe2a4ad35ac0))
    - Ignore currently failing test - will fix ([`99d549d`](https://github.com/spmadden/irox/commit/99d549d56286fbca875ec3fcfa2caee297319fe0))
    - New awful crypto module that shouldn't be used by anyone. ([`757ed35`](https://github.com/spmadden/irox/commit/757ed35f0df71f067b5b45f1b05a2053eebe028e))
</details>

