


## v0.8.5 (2024-09-11)

### Chore

 - <csr-id-a35975360f42880d6e74ceb4443ccd4093c27975/> fixup lints & formatting

### New Features

 - <csr-id-15ab6224386e56d53472b3991f2078d5f469e83e/> new SharedCell wrapper around Arc<RwLock<Option<T>>>
 - <csr-id-98d5046d137ecb02f5270ff794de182df044c606/> add new unlimited/paged buffer that does not reallocate
 - <csr-id-165dc1952bc470b07ab44a4834dc31edb4300a04/> derive clone on fixedbuf

### Other

 - <csr-id-53f8eaa1d223ce33a9898de829d2557ca30832ed/> ignore the documentation that's not rustdoc tests in errors

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 40 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - New SharedCell wrapper around Arc<RwLock<Option<T>>> ([`15ab622`](https://github.com/spmadden/irox/commit/15ab6224386e56d53472b3991f2078d5f469e83e))
    - Ignore the documentation that's not rustdoc tests in errors ([`53f8eaa`](https://github.com/spmadden/irox/commit/53f8eaa1d223ce33a9898de829d2557ca30832ed))
    - Fixup lints & formatting ([`a359753`](https://github.com/spmadden/irox/commit/a35975360f42880d6e74ceb4443ccd4093c27975))
    - Add new unlimited/paged buffer that does not reallocate ([`98d5046`](https://github.com/spmadden/irox/commit/98d5046d137ecb02f5270ff794de182df044c606))
    - Derive clone on fixedbuf ([`165dc19`](https://github.com/spmadden/irox/commit/165dc1952bc470b07ab44a4834dc31edb4300a04))
</details>

## v0.8.4 (2024-08-01)

### New Features

 - <csr-id-08ae6281049fcea8ac12536cce1792bf52c7d735/> new const hex! macro for compile-time hex string literals

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 13 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.8.4 ([`62055d3`](https://github.com/spmadden/irox/commit/62055d336f90405466124924e9dee3da59f8916c))
    - New const hex! macro for compile-time hex string literals ([`08ae628`](https://github.com/spmadden/irox/commit/08ae6281049fcea8ac12536cce1792bf52c7d735))
</details>

## v0.8.3 (2024-07-19)

<csr-id-d9679d01898271cdbffd1b56df072317da2d6880/>

### Other

 - <csr-id-d9679d01898271cdbffd1b56df072317da2d6880/> fix docsrs for irox-tools

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 25 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.8.3 ([`8a633e9`](https://github.com/spmadden/irox/commit/8a633e99da0353163e345f270e6273739d9a733f))
    - Fix docsrs for irox-tools ([`d9679d0`](https://github.com/spmadden/irox/commit/d9679d01898271cdbffd1b56df072317da2d6880))
</details>

## v0.8.2 (2024-06-24)

<csr-id-7809b2d2af9bd4b9767c701782530d7fde558421/>
<csr-id-7b8a2bfcae359473feeeb4eb7098f9eab14685ee/>
<csr-id-0999f97c86fd40009c321c0100d6db5db735b3c2/>

### Chore

 - <csr-id-7809b2d2af9bd4b9767c701782530d7fde558421/> Clean up new lints
 - <csr-id-7b8a2bfcae359473feeeb4eb7098f9eab14685ee/> fixup some 1.78 lint warnings

### New Features

 - <csr-id-c434f69700976ca796b29e6e9e213ae44ccf4e02/> improvements across the board for docs.rs docs
 - <csr-id-7d369bf4d9f753411be6eab864fd7f29d4fd888c/> bump to rust 1.79
 - <csr-id-2e3ec3339b9fa5597bdd39b3c5c6c7a3442dcef2/> new implementation of SHA2
 - <csr-id-11ff4aaf228d218c9c451e7cb5fcf9b776be7505/> impl HashDigest for MD5
 - <csr-id-0ab16782054679e07c99e52f96b8ab4109b3ec7d/> impl HashDigest for SHA1
 - <csr-id-a7d6f47b35f015ef43d7a4431982ac7f0b95bb8f/> implementation of HMAC
 - <csr-id-167144fabb5e619e925001b204fbe73d795570c3/> improved perf for stack-allocated RoundBuf
 - <csr-id-f8a4651a0e1eb2a8c8e61431eb58493169d92ca8/> new assert::assert_eq_hex_slice method

### Bug Fixes

 - <csr-id-5ce4b19d2e1d9759ae087b1ef2a14144f7b03076/> clean up lints
 - <csr-id-5e2c0bdbeee6a09d9b150fed5475c014890b9f2f/> fix typo in feature name guard preventing use of hex::from_hex_str

### Performance

 - <csr-id-1459e3484dbf2d36ba0964fb903a9a2b10fdf2bb/> new testing '_toobig-tests' feature to ignore certain tests that are expensive & slow

### Refactor

 - <csr-id-0999f97c86fd40009c321c0100d6db5db735b3c2/> playing with buffers in sha2 for perf.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release.
 - 36 days passed between releases.
 - 14 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.8.2 ([`e85d66a`](https://github.com/spmadden/irox/commit/e85d66a55434a6ece62506d1aefbdf7f02ab0c35))
    - Improvements across the board for docs.rs docs ([`c434f69`](https://github.com/spmadden/irox/commit/c434f69700976ca796b29e6e9e213ae44ccf4e02))
    - Bump to rust 1.79 ([`7d369bf`](https://github.com/spmadden/irox/commit/7d369bf4d9f753411be6eab864fd7f29d4fd888c))
    - New testing '_toobig-tests' feature to ignore certain tests that are expensive & slow ([`1459e34`](https://github.com/spmadden/irox/commit/1459e3484dbf2d36ba0964fb903a9a2b10fdf2bb))
    - Playing with buffers in sha2 for perf. ([`0999f97`](https://github.com/spmadden/irox/commit/0999f97c86fd40009c321c0100d6db5db735b3c2))
    - Clean up lints ([`5ce4b19`](https://github.com/spmadden/irox/commit/5ce4b19d2e1d9759ae087b1ef2a14144f7b03076))
    - New implementation of SHA2 ([`2e3ec33`](https://github.com/spmadden/irox/commit/2e3ec3339b9fa5597bdd39b3c5c6c7a3442dcef2))
    - Impl HashDigest for MD5 ([`11ff4aa`](https://github.com/spmadden/irox/commit/11ff4aaf228d218c9c451e7cb5fcf9b776be7505))
    - Impl HashDigest for SHA1 ([`0ab1678`](https://github.com/spmadden/irox/commit/0ab16782054679e07c99e52f96b8ab4109b3ec7d))
    - Implementation of HMAC ([`a7d6f47`](https://github.com/spmadden/irox/commit/a7d6f47b35f015ef43d7a4431982ac7f0b95bb8f))
    - Improved perf for stack-allocated RoundBuf ([`167144f`](https://github.com/spmadden/irox/commit/167144fabb5e619e925001b204fbe73d795570c3))
    - Fix typo in feature name guard preventing use of hex::from_hex_str ([`5e2c0bd`](https://github.com/spmadden/irox/commit/5e2c0bdbeee6a09d9b150fed5475c014890b9f2f))
    - New assert::assert_eq_hex_slice method ([`f8a4651`](https://github.com/spmadden/irox/commit/f8a4651a0e1eb2a8c8e61431eb58493169d92ca8))
    - Clean up new lints ([`7809b2d`](https://github.com/spmadden/irox/commit/7809b2d2af9bd4b9767c701782530d7fde558421))
    - Fixup some 1.78 lint warnings ([`7b8a2bf`](https://github.com/spmadden/irox/commit/7b8a2bfcae359473feeeb4eb7098f9eab14685ee))
</details>

## v0.8.1 (2024-05-18)

<csr-id-b8c91df14a0642426aca122ded0339b555f84ade/>
<csr-id-c853a9178bdb1a6c471b80f817cc13cb2b8a1958/>
<csr-id-826ce7f53cf2f8d84a251a83fd5909ae71e58a6c/>

### Chore

 - <csr-id-b8c91df14a0642426aca122ded0339b555f84ade/> clean up some test warnings in irox-tools

### New Features

 - <csr-id-45b145ade2a9fa5e4dedbfc53ec197ddb71d7469/> add new (basic) pagefile wrapper.

### Bug Fixes

 - <csr-id-2c526b417671a161582c0906f631a00b1988c633/> put pagefile behind right feature (bits/std)
 - <csr-id-835f4d7fb52581c4680e4f778409668f5e474fce/> fix pagefile compiling/function on Linux
 - <csr-id-62b40dc6b7c560153a6209ea3373aa9cb79ba27b/> disable lints for random's test code
 - <csr-id-1c413bba298204c593420a2d813291eb3997b054/> fix debug math panic in Random

### Other

 - <csr-id-c853a9178bdb1a6c471b80f817cc13cb2b8a1958/> fix urls in primitives

### Refactor

 - <csr-id-826ce7f53cf2f8d84a251a83fd5909ae71e58a6c/> move buf back into module, expose StrBuf

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release.
 - 27 days passed between releases.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.8.1 ([`a77a177`](https://github.com/spmadden/irox/commit/a77a17704f383c889450d7d21232a6bda447b26e))
    - Put pagefile behind right feature (bits/std) ([`2c526b4`](https://github.com/spmadden/irox/commit/2c526b417671a161582c0906f631a00b1988c633))
    - Fix urls in primitives ([`c853a91`](https://github.com/spmadden/irox/commit/c853a9178bdb1a6c471b80f817cc13cb2b8a1958))
    - Move buf back into module, expose StrBuf ([`826ce7f`](https://github.com/spmadden/irox/commit/826ce7f53cf2f8d84a251a83fd5909ae71e58a6c))
    - Fix pagefile compiling/function on Linux ([`835f4d7`](https://github.com/spmadden/irox/commit/835f4d7fb52581c4680e4f778409668f5e474fce))
    - Clean up some test warnings in irox-tools ([`b8c91df`](https://github.com/spmadden/irox/commit/b8c91df14a0642426aca122ded0339b555f84ade))
    - Disable lints for random's test code ([`62b40dc`](https://github.com/spmadden/irox/commit/62b40dc6b7c560153a6209ea3373aa9cb79ba27b))
    - Add new (basic) pagefile wrapper. ([`45b145a`](https://github.com/spmadden/irox/commit/45b145ade2a9fa5e4dedbfc53ec197ddb71d7469))
    - Fix debug math panic in Random ([`1c413bb`](https://github.com/spmadden/irox/commit/1c413bba298204c593420a2d813291eb3997b054))
</details>

## v0.8.0 (2024-04-21)

<csr-id-05c5b84578474138d78211db2763e11a7bb3a925/>

### New Features

 - <csr-id-bb44251a3eb5917a0b270880e4956700773da32f/> new feature: alloc, change default features to be nil rather than everything
 - <csr-id-2537e8c15422cee078684d2e01f0e0e4f7053316/> new no-std/no-alloc fixed-size stack impls FixedBuf and RoundBuf

### Refactor (BREAKING)

 - <csr-id-05c5b84578474138d78211db2763e11a7bb3a925/> pull out tools/bits into own module, no-std, no-alloc

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 20 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.8.0 ([`00325aa`](https://github.com/spmadden/irox/commit/00325aaa6a8ca13ad071bb0f931f002db1cb6517))
    - New feature: alloc, change default features to be nil rather than everything ([`bb44251`](https://github.com/spmadden/irox/commit/bb44251a3eb5917a0b270880e4956700773da32f))
    - New no-std/no-alloc fixed-size stack impls FixedBuf and RoundBuf ([`2537e8c`](https://github.com/spmadden/irox/commit/2537e8c15422cee078684d2e01f0e0e4f7053316))
    - Pull out tools/bits into own module, no-std, no-alloc ([`05c5b84`](https://github.com/spmadden/irox/commit/05c5b84578474138d78211db2763e11a7bb3a925))
</details>

## v0.7.0 (2024-04-01)

### New Features

 - <csr-id-66cb1ad87755b5dc2ccca9b0f856a43a13365096/> new joining and joining_multi itertools methods
 - <csr-id-e8864e656383096d6b3c4c3316b6d78d3746ab70/> new read/write length-prefixed strings functions

### New Features (BREAKING)

 - <csr-id-fbd6a72ac2ee6c6081bcfff0bab6e496b4d41ab2/> rename write_*_blob methods to have 'be' and 'le' variants

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.7.0 ([`a6a3ae3`](https://github.com/spmadden/irox/commit/a6a3ae33fa0cae813ea214e58f60fea85c9f0479))
    - New joining and joining_multi itertools methods ([`66cb1ad`](https://github.com/spmadden/irox/commit/66cb1ad87755b5dc2ccca9b0f856a43a13365096))
    - Rename write_*_blob methods to have 'be' and 'le' variants ([`fbd6a72`](https://github.com/spmadden/irox/commit/fbd6a72ac2ee6c6081bcfff0bab6e496b4d41ab2))
    - New read/write length-prefixed strings functions ([`e8864e6`](https://github.com/spmadden/irox/commit/e8864e656383096d6b3c4c3316b6d78d3746ab70))
</details>

## v0.6.1 (2024-04-01)

### New Features

 - <csr-id-be13dba6a728c32453fda64049ee1011dfe61c14/> impl FloatExt on feature 'std' as well
 - <csr-id-efd9775d64703aa24ef68b86ccdbd84c090acaa0/> adding impl of SHA1
 - <csr-id-40d00a68dd70dc52ee358f293168bce0c5f85f45/> more support for little-endian in bits
 - <csr-id-a606b9a21dc5ab1e96c588567a7b76efcc466d44/> adding impl of MD5
 - <csr-id-9711bc3cc3c9c6e88ef373c486e7382cdb5cd996/> adding assert_eq_hex! macro for hex printing assertions
 - <csr-id-2da79217703b5a33d9c7c086c412d7afbb75ef7f/> adding u32 primitive FromArray and ToArray traits
 - <csr-id-dbdc371dd862231c3a1a3c97fd54dd0d2c3e98ca/> impl exp() ln() powi(), powf() and sqrt() for f64, f32 in no_std

### Bug Fixes

 - <csr-id-915cd9d6a24111a16edf3e546d6e79a783ee6146/> accuracy of no_std FloatExt funcs
 - <csr-id-3b2c7a237dfa62486596ee1e54069605a1d3c7a3/> fixed issue where Bits::read_exact_vec was not sized correctly
 - <csr-id-403f9288190d88b2fa97891d715de1cb8a993202/> switch to different (better) alg for ln, imp loop perf
 - <csr-id-1baab6123103aabd1e1d9669887af7049a3272f9/> only run no_std tests on no_std build

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release.
 - 28 days passed between releases.
 - 11 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.6.1 ([`091c8ac`](https://github.com/spmadden/irox/commit/091c8ac155297f942d1a46462e3d2d782d71993e))
    - Accuracy of no_std FloatExt funcs ([`915cd9d`](https://github.com/spmadden/irox/commit/915cd9d6a24111a16edf3e546d6e79a783ee6146))
    - Impl FloatExt on feature 'std' as well ([`be13dba`](https://github.com/spmadden/irox/commit/be13dba6a728c32453fda64049ee1011dfe61c14))
    - Adding impl of SHA1 ([`efd9775`](https://github.com/spmadden/irox/commit/efd9775d64703aa24ef68b86ccdbd84c090acaa0))
    - Fixed issue where Bits::read_exact_vec was not sized correctly ([`3b2c7a2`](https://github.com/spmadden/irox/commit/3b2c7a237dfa62486596ee1e54069605a1d3c7a3))
    - More support for little-endian in bits ([`40d00a6`](https://github.com/spmadden/irox/commit/40d00a68dd70dc52ee358f293168bce0c5f85f45))
    - Adding impl of MD5 ([`a606b9a`](https://github.com/spmadden/irox/commit/a606b9a21dc5ab1e96c588567a7b76efcc466d44))
    - Adding assert_eq_hex! macro for hex printing assertions ([`9711bc3`](https://github.com/spmadden/irox/commit/9711bc3cc3c9c6e88ef373c486e7382cdb5cd996))
    - Adding u32 primitive FromArray and ToArray traits ([`2da7921`](https://github.com/spmadden/irox/commit/2da79217703b5a33d9c7c086c412d7afbb75ef7f))
    - Switch to different (better) alg for ln, imp loop perf ([`403f928`](https://github.com/spmadden/irox/commit/403f9288190d88b2fa97891d715de1cb8a993202))
    - Only run no_std tests on no_std build ([`1baab61`](https://github.com/spmadden/irox/commit/1baab6123103aabd1e1d9669887af7049a3272f9))
    - Impl exp() ln() powi(), powf() and sqrt() for f64, f32 in no_std ([`dbdc371`](https://github.com/spmadden/irox/commit/dbdc371dd862231c3a1a3c97fd54dd0d2c3e98ca))
</details>

## v0.6.0 (2024-03-03)

<csr-id-300356f119c976f98a230fc37ce7c43e6bd1a9e0/>
<csr-id-ca214f0f8b310c02e4009fcc37b51d04bda47368/>
<csr-id-2c04083563c31f6f260cbc1b5d9bf9ecea0b99d1/>

### Chore

 - <csr-id-300356f119c976f98a230fc37ce7c43e6bd1a9e0/> clean up new lints for 1.75

### New Features

 - <csr-id-7902f54162a5f33d0e452ff3760ef3a7e91ab704/> recursive justfiles
 - <csr-id-c771cbceee6789b445ec4ccf5c390601f857b52a/> new 'Readerator' to turn Read into an Iterator.

### Bug Fixes

 - <csr-id-d012d6459e4853ea48798b1b0d98196d0577f6ec/> tweak alloc imports in fs, fix readme in cargo.toml

### Refactor (BREAKING)

 - <csr-id-ca214f0f8b310c02e4009fcc37b51d04bda47368/> Refactor packetio traits to use Bits rather than std::io::*.  Is now no_std compliant.
 - <csr-id-2c04083563c31f6f260cbc1b5d9bf9ecea0b99d1/> big refactor of (Mut)Bits.  Default impls for std::io::{Read,Write} removed.
   Impls for most of the standard types provided.  New 'BitsWrapper' provided to wrap arbitrary impls of Read/Write.
   
   New Bits functions: 'read_exact_into', 'read_all_str_lossy', 'read_all_vec', 'read_all_into', 'read_some_into'.
   
   New MutBits function: 'write_some_bytes'
   
   Functions from 'read' have been moved into base Bits trait: consume_until, read_until

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 48 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.6.0 ([`0560dc1`](https://github.com/spmadden/irox/commit/0560dc130599c7355e1f57cbadd88395381c0033))
    - Tweak alloc imports in fs, fix readme in cargo.toml ([`d012d64`](https://github.com/spmadden/irox/commit/d012d6459e4853ea48798b1b0d98196d0577f6ec))
    - Clean up new lints for 1.75 ([`300356f`](https://github.com/spmadden/irox/commit/300356f119c976f98a230fc37ce7c43e6bd1a9e0))
    - Recursive justfiles ([`7902f54`](https://github.com/spmadden/irox/commit/7902f54162a5f33d0e452ff3760ef3a7e91ab704))
    - Refactor packetio traits to use Bits rather than std::io::*.  Is now no_std compliant. ([`ca214f0`](https://github.com/spmadden/irox/commit/ca214f0f8b310c02e4009fcc37b51d04bda47368))
    - New 'Readerator' to turn Read into an Iterator. ([`c771cbc`](https://github.com/spmadden/irox/commit/c771cbceee6789b445ec4ccf5c390601f857b52a))
    - Big refactor of (Mut)Bits.  Default impls for std::io::{Read,Write} removed. ([`2c04083`](https://github.com/spmadden/irox/commit/2c04083563c31f6f260cbc1b5d9bf9ecea0b99d1))
</details>

## v0.5.3 (2024-01-15)

### New Features

 - <csr-id-a3849cc09b4aec74df31d0e722cca2648bcc4bca/> new fs mod for filename/filesystem utils
 - <csr-id-464a2db730a363e79190823a72339177009e510f/> new eventually complete ecosystem
 - <csr-id-e599d63fec9c05c8ebc2aaa7d5e4ad59fa43d73e/> new lazy static initialization macro

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 1 day passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.5.3 ([`eb01ead`](https://github.com/spmadden/irox/commit/eb01ead38bd65ae52e5b1cfb534cbea16b3e13b7))
    - New fs mod for filename/filesystem utils ([`a3849cc`](https://github.com/spmadden/irox/commit/a3849cc09b4aec74df31d0e722cca2648bcc4bca))
    - New eventually complete ecosystem ([`464a2db`](https://github.com/spmadden/irox/commit/464a2db730a363e79190823a72339177009e510f))
    - New lazy static initialization macro ([`e599d63`](https://github.com/spmadden/irox/commit/e599d63fec9c05c8ebc2aaa7d5e4ad59fa43d73e))
</details>

## v0.5.2 (2024-01-13)

<csr-id-7bb9983b0e144be41483b4b9a3e610c773aa26f4/>
<csr-id-350e0f8529bdc3a936149b634ebd824abd440d2f/>
<csr-id-5be0cba49e4e0559570102152dfe0b4b73422158/>

### Chore

 - <csr-id-7bb9983b0e144be41483b4b9a3e610c773aa26f4/> bump tools to 0.5.2

### New Features

 - <csr-id-55f70163a3d46f63956eb935645e344ec9c3ee13/> new errors macros to accelerate error conversions

### Other

 - <csr-id-350e0f8529bdc3a936149b634ebd824abd440d2f/> update readme for error macros
 - <csr-id-5be0cba49e4e0559570102152dfe0b4b73422158/> update docs & readme for tools

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.5.2 ([`89b01ec`](https://github.com/spmadden/irox/commit/89b01ec20e8637408a1497fa88a30452233efc97))
    - Bump tools to 0.5.2 ([`7bb9983`](https://github.com/spmadden/irox/commit/7bb9983b0e144be41483b4b9a3e610c773aa26f4))
    - Update readme for error macros ([`350e0f8`](https://github.com/spmadden/irox/commit/350e0f8529bdc3a936149b634ebd824abd440d2f))
    - New errors macros to accelerate error conversions ([`55f7016`](https://github.com/spmadden/irox/commit/55f70163a3d46f63956eb935645e344ec9c3ee13))
    - Update docs & readme for tools ([`5be0cba`](https://github.com/spmadden/irox/commit/5be0cba49e4e0559570102152dfe0b4b73422158))
</details>

## v0.5.1 (2024-01-12)

<csr-id-aa590522fb7bd75591949813d08cf221b7b729dd/>
<csr-id-4160ef3c3d173bddc6688473113fb1a25a45a22a/>
<csr-id-af906604969d656432218f6843a8ac3f825b4a04/>
<csr-id-4dc4b9d0b73f0ebf5d97ff2685db6233e527cb92/>
<csr-id-c9bb39737d83b6d83376ce4700c088f20ec23b37/>
<csr-id-00bc196ddd2e8e0eb60fdb68fae661593752f3c0/>
<csr-id-081d7694415883c4569d762fe4da7864cbed8de4/>
<csr-id-cf0bff72254d93594a8b7ebd4067485f0434607a/>
<csr-id-ca28aa6647aa5425067b557f532844022546bb95/>
<csr-id-e753938da0ffabd720a5c91f63d8c998e2cec483/>

### Chore

 - <csr-id-aa590522fb7bd75591949813d08cf221b7b729dd/> bump tools to 0.5.1
 - <csr-id-4160ef3c3d173bddc6688473113fb1a25a45a22a/> clean up lints for no-std compliance
 - <csr-id-af906604969d656432218f6843a8ac3f825b4a04/> clean up lints for no-std compliance

### New Features

 - <csr-id-098c51c276447937fb20dd65c9c48ca1d6b84019/> add additional PCG random impls
 - <csr-id-0b67aa6b1dd893688862973d0b48029d6cce1924/> starting new vbyte impls
 - <csr-id-4000e53b0222938eed42e1534059f33efc337842/> HexDump is now no-std
 - <csr-id-65021dccf264ac2ac909dbc6fbd38b0f5a7186ec/> MutBits can now be used with write!()
 - <csr-id-f43a9291a7b8b2f8b092ed51bab40d685c8350ae/> Bits, Codec, Base64 now fully no_std
 - <csr-id-26986c9bd6c1ea2a7d9b8a174c2f10c9a03d847c/> new easy Line ending scanners
 - <csr-id-94bbd1d5ffb530559e016a8f3472d353d9f2d3ed/> new ReadAny and ReadEmpty, for converting random stuff into Read's
 - <csr-id-23d81e57f7cc5c044f827e8dbe644dbaa9c5a0d5/> new f32 and f64 traits for no-std
 - <csr-id-91705d832edccfc49849d0ba018f4cbf3210fd1d/> new 'Base64' conversions, compatible with RFC4648
 - <csr-id-7febef2493a70a7f25883b7e76a167afe6806836/> new 'Codec' trait for byte encoding conversions
 - <csr-id-1ad20e1bcd42018c5abbf62f9cb32d32456f107e/> Now 'no_std' capable (without the 'std' feature)
 - <csr-id-a9c45bfedc2d7b35a1f726ee5cd7fc555ebd6c81/> Now 'no_std' capable (without the 'std' feature)

### Bug Fixes

 - <csr-id-194adf37e50dd9677bcab9fe0540ea8a90c9f069/> bump u32 to u64 in fmt to fix rollover in tests
 - <csr-id-2b00a47d28d1856f2e2e00b03f2aa40ee5b2033a/> fix busted impl of Read in Buffer

### Other

 - <csr-id-4dc4b9d0b73f0ebf5d97ff2685db6233e527cb92/> Added missing readme?
 - <csr-id-c9bb39737d83b6d83376ce4700c088f20ec23b37/> Module docs for arrays, assert, options

### Refactor

 - <csr-id-00bc196ddd2e8e0eb60fdb68fae661593752f3c0/> mv codec.rs into codec/mod.rs
 - <csr-id-081d7694415883c4569d762fe4da7864cbed8de4/> rename 'sync.rs' into 'sync/mod.rs'
 - <csr-id-cf0bff72254d93594a8b7ebd4067485f0434607a/> rename 'read.rs' into 'read/mod.rs'
 - <csr-id-ca28aa6647aa5425067b557f532844022546bb95/> move base64, bits, id, scanner, uuid to 'utils' module
 - <csr-id-e753938da0ffabd720a5c91f63d8c998e2cec483/> move associated primitives into primitives module

### New Features (BREAKING)

 - <csr-id-59aa3f65a355740f793ea3db923629833fc0053c/> refactor random to have PRNG trait, impl new random algorithm

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.5.1 ([`c51a7a0`](https://github.com/spmadden/irox/commit/c51a7a0e503b78f6e576087178fdad2a227a7e04))
    - Bump tools to 0.5.1 ([`aa59052`](https://github.com/spmadden/irox/commit/aa590522fb7bd75591949813d08cf221b7b729dd))
</details>

## v0.5.0 (2024-01-12)

<csr-id-4160ef3c3d173bddc6688473113fb1a25a45a22a/>
<csr-id-af906604969d656432218f6843a8ac3f825b4a04/>
<csr-id-4dc4b9d0b73f0ebf5d97ff2685db6233e527cb92/>
<csr-id-c9bb39737d83b6d83376ce4700c088f20ec23b37/>
<csr-id-00bc196ddd2e8e0eb60fdb68fae661593752f3c0/>
<csr-id-081d7694415883c4569d762fe4da7864cbed8de4/>
<csr-id-cf0bff72254d93594a8b7ebd4067485f0434607a/>
<csr-id-ca28aa6647aa5425067b557f532844022546bb95/>
<csr-id-e753938da0ffabd720a5c91f63d8c998e2cec483/>

### Chore

 - <csr-id-4160ef3c3d173bddc6688473113fb1a25a45a22a/> clean up lints for no-std compliance
 - <csr-id-af906604969d656432218f6843a8ac3f825b4a04/> clean up lints for no-std compliance

### New Features

 - <csr-id-098c51c276447937fb20dd65c9c48ca1d6b84019/> add additional PCG random impls
 - <csr-id-0b67aa6b1dd893688862973d0b48029d6cce1924/> starting new vbyte impls
 - <csr-id-4000e53b0222938eed42e1534059f33efc337842/> HexDump is now no-std
 - <csr-id-65021dccf264ac2ac909dbc6fbd38b0f5a7186ec/> MutBits can now be used with write!()
 - <csr-id-f43a9291a7b8b2f8b092ed51bab40d685c8350ae/> Bits, Codec, Base64 now fully no_std
 - <csr-id-26986c9bd6c1ea2a7d9b8a174c2f10c9a03d847c/> new easy Line ending scanners
 - <csr-id-94bbd1d5ffb530559e016a8f3472d353d9f2d3ed/> new ReadAny and ReadEmpty, for converting random stuff into Read's
 - <csr-id-23d81e57f7cc5c044f827e8dbe644dbaa9c5a0d5/> new f32 and f64 traits for no-std
 - <csr-id-91705d832edccfc49849d0ba018f4cbf3210fd1d/> new 'Base64' conversions, compatible with RFC4648
 - <csr-id-7febef2493a70a7f25883b7e76a167afe6806836/> new 'Codec' trait for byte encoding conversions
 - <csr-id-1ad20e1bcd42018c5abbf62f9cb32d32456f107e/> Now 'no_std' capable (without the 'std' feature)
 - <csr-id-a9c45bfedc2d7b35a1f726ee5cd7fc555ebd6c81/> Now 'no_std' capable (without the 'std' feature)

### Bug Fixes

 - <csr-id-194adf37e50dd9677bcab9fe0540ea8a90c9f069/> bump u32 to u64 in fmt to fix rollover in tests
 - <csr-id-2b00a47d28d1856f2e2e00b03f2aa40ee5b2033a/> fix busted impl of Read in Buffer

### Other

 - <csr-id-4dc4b9d0b73f0ebf5d97ff2685db6233e527cb92/> Added missing readme?
 - <csr-id-c9bb39737d83b6d83376ce4700c088f20ec23b37/> Module docs for arrays, assert, options

### Refactor

 - <csr-id-00bc196ddd2e8e0eb60fdb68fae661593752f3c0/> mv codec.rs into codec/mod.rs
 - <csr-id-081d7694415883c4569d762fe4da7864cbed8de4/> rename 'sync.rs' into 'sync/mod.rs'
 - <csr-id-cf0bff72254d93594a8b7ebd4067485f0434607a/> rename 'read.rs' into 'read/mod.rs'
 - <csr-id-ca28aa6647aa5425067b557f532844022546bb95/> move base64, bits, id, scanner, uuid to 'utils' module
 - <csr-id-e753938da0ffabd720a5c91f63d8c998e2cec483/> move associated primitives into primitives module

### New Features (BREAKING)

 - <csr-id-59aa3f65a355740f793ea3db923629833fc0053c/> refactor random to have PRNG trait, impl new random algorithm

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 25 commits contributed to the release.
 - 37 days passed between releases.
 - 24 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.5.0, safety bump 17 crates ([`a46e9e2`](https://github.com/spmadden/irox/commit/a46e9e2da699f6ccd3a85b660014f0e15e59c0d0))
    - Add additional PCG random impls ([`098c51c`](https://github.com/spmadden/irox/commit/098c51c276447937fb20dd65c9c48ca1d6b84019))
    - Starting new vbyte impls ([`0b67aa6`](https://github.com/spmadden/irox/commit/0b67aa6b1dd893688862973d0b48029d6cce1924))
    - Refactor random to have PRNG trait, impl new random algorithm ([`59aa3f6`](https://github.com/spmadden/irox/commit/59aa3f65a355740f793ea3db923629833fc0053c))
    - Mv codec.rs into codec/mod.rs ([`00bc196`](https://github.com/spmadden/irox/commit/00bc196ddd2e8e0eb60fdb68fae661593752f3c0))
    - Rename 'sync.rs' into 'sync/mod.rs' ([`081d769`](https://github.com/spmadden/irox/commit/081d7694415883c4569d762fe4da7864cbed8de4))
    - Rename 'read.rs' into 'read/mod.rs' ([`cf0bff7`](https://github.com/spmadden/irox/commit/cf0bff72254d93594a8b7ebd4067485f0434607a))
    - HexDump is now no-std ([`4000e53`](https://github.com/spmadden/irox/commit/4000e53b0222938eed42e1534059f33efc337842))
    - MutBits can now be used with write!() ([`65021dc`](https://github.com/spmadden/irox/commit/65021dccf264ac2ac909dbc6fbd38b0f5a7186ec))
    - Move base64, bits, id, scanner, uuid to 'utils' module ([`ca28aa6`](https://github.com/spmadden/irox/commit/ca28aa6647aa5425067b557f532844022546bb95))
    - Move associated primitives into primitives module ([`e753938`](https://github.com/spmadden/irox/commit/e753938da0ffabd720a5c91f63d8c998e2cec483))
    - Bits, Codec, Base64 now fully no_std ([`f43a929`](https://github.com/spmadden/irox/commit/f43a9291a7b8b2f8b092ed51bab40d685c8350ae))
    - Clean up lints for no-std compliance ([`4160ef3`](https://github.com/spmadden/irox/commit/4160ef3c3d173bddc6688473113fb1a25a45a22a))
    - Clean up lints for no-std compliance ([`af90660`](https://github.com/spmadden/irox/commit/af906604969d656432218f6843a8ac3f825b4a04))
    - Bump u32 to u64 in fmt to fix rollover in tests ([`194adf3`](https://github.com/spmadden/irox/commit/194adf37e50dd9677bcab9fe0540ea8a90c9f069))
    - Added missing readme? ([`4dc4b9d`](https://github.com/spmadden/irox/commit/4dc4b9d0b73f0ebf5d97ff2685db6233e527cb92))
    - New easy Line ending scanners ([`26986c9`](https://github.com/spmadden/irox/commit/26986c9bd6c1ea2a7d9b8a174c2f10c9a03d847c))
    - New ReadAny and ReadEmpty, for converting random stuff into Read's ([`94bbd1d`](https://github.com/spmadden/irox/commit/94bbd1d5ffb530559e016a8f3472d353d9f2d3ed))
    - Module docs for arrays, assert, options ([`c9bb397`](https://github.com/spmadden/irox/commit/c9bb39737d83b6d83376ce4700c088f20ec23b37))
    - Fix busted impl of Read in Buffer ([`2b00a47`](https://github.com/spmadden/irox/commit/2b00a47d28d1856f2e2e00b03f2aa40ee5b2033a))
    - New f32 and f64 traits for no-std ([`23d81e5`](https://github.com/spmadden/irox/commit/23d81e57f7cc5c044f827e8dbe644dbaa9c5a0d5))
    - New 'Base64' conversions, compatible with RFC4648 ([`91705d8`](https://github.com/spmadden/irox/commit/91705d832edccfc49849d0ba018f4cbf3210fd1d))
    - New 'Codec' trait for byte encoding conversions ([`7febef2`](https://github.com/spmadden/irox/commit/7febef2493a70a7f25883b7e76a167afe6806836))
    - Now 'no_std' capable (without the 'std' feature) ([`1ad20e1`](https://github.com/spmadden/irox/commit/1ad20e1bcd42018c5abbf62f9cb32d32456f107e))
    - Now 'no_std' capable (without the 'std' feature) ([`a9c45bf`](https://github.com/spmadden/irox/commit/a9c45bfedc2d7b35a1f726ee5cd7fc555ebd6c81))
</details>

## v0.4.1 (2023-12-05)

### New Features

 - <csr-id-59c9d9821ced4b102b3f6a63fbb647d201ee82b1/> new synchronization primitive 'SynchronizedOptional' like 'OnceLock', but different.
 - <csr-id-4e6c8961dc8820c39ccabc0e8283f5d50aefed2f/> new hexdump module

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 6 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.4.1 ([`367158e`](https://github.com/spmadden/irox/commit/367158e54237e29b2e7203e1b33139684ac43086))
    - New synchronization primitive 'SynchronizedOptional' like 'OnceLock', but different. ([`59c9d98`](https://github.com/spmadden/irox/commit/59c9d9821ced4b102b3f6a63fbb647d201ee82b1))
    - New hexdump module ([`4e6c896`](https://github.com/spmadden/irox/commit/4e6c8961dc8820c39ccabc0e8283f5d50aefed2f))
</details>

## v0.4.0 (2023-11-28)

<csr-id-88ebfb5deea5508ca54f4aaab62f6fd5a36f531c/>

### Chore

 - <csr-id-88ebfb5deea5508ca54f4aaab62f6fd5a36f531c/> pivot to using Cargo.toml workspace lints

### New Features

 - <csr-id-0320a56d87ea14313690eed22f07af2faf754db5/> new buffer extending BufReader to span multiple blocks
 - <csr-id-0dd784548c69ac4e046210338f1d8bd71bd7479b/> Bits and MutBits can now read & write size-prefixed blobs
 - <csr-id-965b956e2b125d74e36cf72d3f92871cfba57b94/> new DecimalFormatF32/F64 with ability to specify number of digits precision

### Bug Fixes

 - <csr-id-08e634e08675453d3cd9960635476dc05f53bef6/> Scanner now scans across block boundaries correctly

### New Features (BREAKING)

 - <csr-id-9e13976bb33d1cb990a9841184637a28a038f66e/> bits now has Optional 'next' methods

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 23 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.4.0 ([`6abaf62`](https://github.com/spmadden/irox/commit/6abaf62cde62c90b2d68c14dbf6a4f86b599768c))
    - Bits now has Optional 'next' methods ([`9e13976`](https://github.com/spmadden/irox/commit/9e13976bb33d1cb990a9841184637a28a038f66e))
    - Scanner now scans across block boundaries correctly ([`08e634e`](https://github.com/spmadden/irox/commit/08e634e08675453d3cd9960635476dc05f53bef6))
    - New buffer extending BufReader to span multiple blocks ([`0320a56`](https://github.com/spmadden/irox/commit/0320a56d87ea14313690eed22f07af2faf754db5))
    - Pivot to using Cargo.toml workspace lints ([`88ebfb5`](https://github.com/spmadden/irox/commit/88ebfb5deea5508ca54f4aaab62f6fd5a36f531c))
    - Bits and MutBits can now read & write size-prefixed blobs ([`0dd7845`](https://github.com/spmadden/irox/commit/0dd784548c69ac4e046210338f1d8bd71bd7479b))
    - New DecimalFormatF32/F64 with ability to specify number of digits precision ([`965b956`](https://github.com/spmadden/irox/commit/965b956e2b125d74e36cf72d3f92871cfba57b94))
</details>

## v0.3.2 (2023-11-05)

### New Features

 - <csr-id-262121de9bc30c7501f3d1a7382a90c556137cd0/> new ANSI Color Codes

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 5 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.3.2, irox-time v0.3.0, irox-log v0.1.0, safety bump 8 crates ([`9c08793`](https://github.com/spmadden/irox/commit/9c0879320a17a94fa7a4169426de4d9d3b62395e))
    - New ANSI Color Codes ([`262121d`](https://github.com/spmadden/irox/commit/262121de9bc30c7501f3d1a7382a90c556137cd0))
</details>

## v0.3.1 (2023-10-31)

### New Features

 - <csr-id-1f0e241f6207cf7b91b74377d393a0f867c1bb92/> add new RetainTake for Vec and VecDeque

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
    - Release irox-tools v0.3.1 ([`f7eaa32`](https://github.com/spmadden/irox/commit/f7eaa327672c26b14a18869d6b10ce831a16a4a3))
    - Add new RetainTake for Vec and VecDeque ([`1f0e241`](https://github.com/spmadden/irox/commit/1f0e241f6207cf7b91b74377d393a0f867c1bb92))
</details>

## v0.3.0 (2023-10-29)

<csr-id-3793f0549be87bcce984d72c6153851be869cb43/>
<csr-id-f8ab392c14af57bb2f6198c45c82c602225ac356/>
<csr-id-7680bf804c1d6b4dd1352dc68b371eaf06bd29c5/>

### Chore

 - <csr-id-3793f0549be87bcce984d72c6153851be869cb43/> fix fmt
 - <csr-id-f8ab392c14af57bb2f6198c45c82c602225ac356/> ALL THE LINTS.
 - <csr-id-7680bf804c1d6b4dd1352dc68b371eaf06bd29c5/> fix rustfmt

### New Features

 - <csr-id-991cff0de29564748ccd2311eb080de249db40e9/> New 'ReadCounting' struct to count bytes read from a Read
 - <csr-id-c8e6ba69b07af9b3f4fb5ef44c7ff5b78062d4b1/> new array scanning utils, max_index and longest_consecutive_values
 - <csr-id-9fc9e98f31a35193859ffb53f5d5238a907afa76/> new U16 utilities, FromU16Array and ToU16Array
 - <csr-id-edd4b815d7f3e2ec599c7a467f17af398179fcbb/> New Identifier type to allow multi-IDs of types
 - <csr-id-d3e2baa73ad6e99f0b1fd816a11237f2e9dfda29/> Add borrowed From's for UUID
 - <csr-id-ccf4aae08cdaef94b0a2a542b32350e541b5bb37/> new PRNG based on PCG-XSH-RR
 - <csr-id-159ec4e01afcb3d3bc6c4005bc23351dddbc9906/> new UUID struct

### Bug Fixes

 - <csr-id-8ea01eae74dcc904503cc80f8d54c6f9575f5015/> new lint for unwrap_or_default()
 - <csr-id-fd6aa90ec9c2fba058a973282538681e49e12ea0/> MurmurHash3-128 now passes reasonable tests

### Bug Fixes (BREAKING)

 - <csr-id-cbaa8f43fb163b4022548b2733a187933e7fb2b5/> Bits and MutBits no longer require Read & Write
 - <csr-id-b791aca7dd8c0df67ed2912863d170b70684ae28/> Bits and MutBits no longer require Read & Write

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release.
 - 14 days passed between releases.
 - 14 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.3.0, safety bump 12 crates ([`eb83b27`](https://github.com/spmadden/irox/commit/eb83b27b20c23e51e5b0fc3b7b3704e2c03af46c))
    - New 'ReadCounting' struct to count bytes read from a Read ([`991cff0`](https://github.com/spmadden/irox/commit/991cff0de29564748ccd2311eb080de249db40e9))
    - Fix fmt ([`3793f05`](https://github.com/spmadden/irox/commit/3793f0549be87bcce984d72c6153851be869cb43))
    - ALL THE LINTS. ([`f8ab392`](https://github.com/spmadden/irox/commit/f8ab392c14af57bb2f6198c45c82c602225ac356))
    - New array scanning utils, max_index and longest_consecutive_values ([`c8e6ba6`](https://github.com/spmadden/irox/commit/c8e6ba69b07af9b3f4fb5ef44c7ff5b78062d4b1))
    - New U16 utilities, FromU16Array and ToU16Array ([`9fc9e98`](https://github.com/spmadden/irox/commit/9fc9e98f31a35193859ffb53f5d5238a907afa76))
    - New lint for unwrap_or_default() ([`8ea01ea`](https://github.com/spmadden/irox/commit/8ea01eae74dcc904503cc80f8d54c6f9575f5015))
    - Fix rustfmt ([`7680bf8`](https://github.com/spmadden/irox/commit/7680bf804c1d6b4dd1352dc68b371eaf06bd29c5))
    - New Identifier type to allow multi-IDs of types ([`edd4b81`](https://github.com/spmadden/irox/commit/edd4b815d7f3e2ec599c7a467f17af398179fcbb))
    - Add borrowed From's for UUID ([`d3e2baa`](https://github.com/spmadden/irox/commit/d3e2baa73ad6e99f0b1fd816a11237f2e9dfda29))
    - MurmurHash3-128 now passes reasonable tests ([`fd6aa90`](https://github.com/spmadden/irox/commit/fd6aa90ec9c2fba058a973282538681e49e12ea0))
    - New PRNG based on PCG-XSH-RR ([`ccf4aae`](https://github.com/spmadden/irox/commit/ccf4aae08cdaef94b0a2a542b32350e541b5bb37))
    - New UUID struct ([`159ec4e`](https://github.com/spmadden/irox/commit/159ec4e01afcb3d3bc6c4005bc23351dddbc9906))
    - Bits and MutBits no longer require Read & Write ([`cbaa8f4`](https://github.com/spmadden/irox/commit/cbaa8f43fb163b4022548b2733a187933e7fb2b5))
    - Bits and MutBits no longer require Read & Write ([`b791aca`](https://github.com/spmadden/irox/commit/b791aca7dd8c0df67ed2912863d170b70684ae28))
</details>

## v0.2.2 (2023-10-15)

### Documentation

 - <csr-id-13ae74c7a318037939a4604a28a1cf33d87741a0/> update docs for rustdoc-lints

### New Features

 - <csr-id-5d0ee4c0a813a180de0c1bd79d98d84518e509cf/> add collect_next_chunk method to itertools
 - <csr-id-73b539781d14681122263f5315940e67de6f3f2d/> Scanner can read & return data now.
 - <csr-id-763b01e2d5d6508cdaee71000de96c8748c02cf6/> Refactor scanner to have multiple tokens
 - <csr-id-c6b8e0f938b71b0da764a33b5ba837cd012a9928/> new Scanner to scan a data stream for tokens

### Bug Fixes

 - <csr-id-b2257546d7d9ca0d8620851fbc80d0d68e25ad10/> remove extra clone in scanner

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 7 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.2.2 ([`f49db4f`](https://github.com/spmadden/irox/commit/f49db4fc702003b0e464b0dbcc65cdcf0c629935))
    - Remove extra clone in scanner ([`b225754`](https://github.com/spmadden/irox/commit/b2257546d7d9ca0d8620851fbc80d0d68e25ad10))
    - Update docs for rustdoc-lints ([`13ae74c`](https://github.com/spmadden/irox/commit/13ae74c7a318037939a4604a28a1cf33d87741a0))
    - Add collect_next_chunk method to itertools ([`5d0ee4c`](https://github.com/spmadden/irox/commit/5d0ee4c0a813a180de0c1bd79d98d84518e509cf))
    - Scanner can read & return data now. ([`73b5397`](https://github.com/spmadden/irox/commit/73b539781d14681122263f5315940e67de6f3f2d))
    - Refactor scanner to have multiple tokens ([`763b01e`](https://github.com/spmadden/irox/commit/763b01e2d5d6508cdaee71000de96c8748c02cf6))
    - New Scanner to scan a data stream for tokens ([`c6b8e0f`](https://github.com/spmadden/irox/commit/c6b8e0f938b71b0da764a33b5ba837cd012a9928))
</details>

## v0.2.1 (2023-10-07)

<csr-id-c1fe9a1fe0a599202b7bf402bb6d81fc5eccc9e2/>

### Chore

 - <csr-id-c1fe9a1fe0a599202b7bf402bb6d81fc5eccc9e2/> fix fmt in vec

### New Features

 - <csr-id-d9f3dc8b63ad33e68b42517ad684c04ba5764218/> added additional static functions to Bits
 - <csr-id-d280045a9c918c9d94b77b9b812b1c43a9d918bd/> UpperHex for PrettyVec and new PrettyVecDeque
 - <csr-id-c2db7b0a2b8c9c989e16ff26dd9cb35823745090/> partially working murmur3_128 hash fn.
 - <csr-id-6465a082becb8f100184dd5cf166428c3d01e1b0/> some new static helper functions

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 19 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.2.1, irox-carto v0.2.1, irox-egui-extras v0.2.1, irox-gpx v0.1.0, irox-types v0.2.1, irox-structs_derive v0.2.1, irox-raymarine-sonar v0.1.0, irox-stats v0.2.1, irox-winlocation-api v0.1.1, irox v0.2.1 ([`68d770b`](https://github.com/spmadden/irox/commit/68d770bb78abe49bf30364ca17ddb6f7bfda05d9))
    - Fix fmt in vec ([`c1fe9a1`](https://github.com/spmadden/irox/commit/c1fe9a1fe0a599202b7bf402bb6d81fc5eccc9e2))
    - Added additional static functions to Bits ([`d9f3dc8`](https://github.com/spmadden/irox/commit/d9f3dc8b63ad33e68b42517ad684c04ba5764218))
    - UpperHex for PrettyVec and new PrettyVecDeque ([`d280045`](https://github.com/spmadden/irox/commit/d280045a9c918c9d94b77b9b812b1c43a9d918bd))
    - Partially working murmur3_128 hash fn. ([`c2db7b0`](https://github.com/spmadden/irox/commit/c2db7b0a2b8c9c989e16ff26dd9cb35823745090))
    - Some new static helper functions ([`6465a08`](https://github.com/spmadden/irox/commit/6465a082becb8f100184dd5cf166428c3d01e1b0))
</details>

## v0.2.0 (2023-09-17)

<csr-id-f03d8a3ec997d53470bfdeb5e76b71925aac3f10/>
<csr-id-80d2b88bdcb553faaeafc09673c31d7ebedafd19/>
<csr-id-1a365333397b02a5f911d0897c3bf0c80f6c2b80/>
<csr-id-5c178560becc0b665d70be2d99a1cffad3ba4284/>
<csr-id-211951e13f3d207f27dfa1ddbaa70157d019ad27/>
<csr-id-49d55665ffd9ebcfe0394e40cb36bcc35a6a72f9/>

### Chore

 - <csr-id-f03d8a3ec997d53470bfdeb5e76b71925aac3f10/> clean up code with additional lints
 - <csr-id-80d2b88bdcb553faaeafc09673c31d7ebedafd19/> update cargo.tomls to add repository
 - <csr-id-1a365333397b02a5f911d0897c3bf0c80f6c2b80/> setting up blank changelogs for the modules

### New Features

 - <csr-id-f62ae9c875cc0c915fc06a9003d72e50cfc013b7/> new vec module with PrettyVec
 - <csr-id-c63cb79f178337401d6a19111eef83229188b45d/> new collect_exact, collect_exact_or, collect_exact_or_default methods in itertools
 - <csr-id-b39d28314fd66c766cc07e3f66b8ef153e09b5a4/> PacketIO no longer allows a unique error - just std::io::Error
 - <csr-id-65ca5767a7d19db1ab2f638631d63410702e3f21/> new MaybeFrom impls for primitives, MaybeMap
 - <csr-id-b175302cc38146e5e4c10b99e8b50383f94c7589/> new 'MaybeFrom' and 'MaybeInto' traits, which are semantically equal to 'Result<T, ()>'
 - <csr-id-0eb5f64a97b2383434548873a0038f1d96b94bfb/> new read_until, read_exact_into, and read_exact_into_sized functions
 - <csr-id-ce70857680aa5243227f45db5ecbaee132b7ab68/> refactor types from tools into it's own module
 - <csr-id-c088de020214e47f28391d0af5a64abe56ad185b/> prohibit unsafe code
 - <csr-id-a979b1e4bb90754b27cc2bd19405226189d6d8e1/> new reflection/primitive types
 - <csr-id-3c5a6f9825aba1516ada921e1bbeb9b9615d374e/> New Read utilities module
 - <csr-id-e852ff0ee41064707e90fc7be182b43ab4d08d06/> PacketIO uses Bits/MutBits now + PacketData
 - <csr-id-fc284bbbe3ed66985911bc6ebd4923f81aac9393/> Add MutBits trait
 - <csr-id-5ddc215691f7bab8ffa4c1c52b8c71868ac9cf42/> Add PacketIO module with packetization
 - <csr-id-c5bfdf353166347c5297c8dd3da0a57fd7a373b6/> Test asserts
 - <csr-id-012691d48fd91392df883867a9f096dca1b912ee/> Adding bits from other project
 - <csr-id-c128a3c43208ee19c878e71e47fc398c2dd12cd1/> Adding first set of tools, min_max and looping iterator

### Bug Fixes

 - <csr-id-d8409416e47d755f5c73982d24bd252e487c8199/> read_until no longer includes the delimiter in the returned result
 - <csr-id-0d50f60ee51713f0e1deb0ce049303472b60d18c/> fix clippy lint for slices
 - <csr-id-e70cf86587258ad73b9a813b6c7878a231157921/> Repair invalid tools Cargo.toml
 - <csr-id-b120f6852df399749d5be1aa75644275ea193fcb/> Fixing doctest

### Other

 - <csr-id-5c178560becc0b665d70be2d99a1cffad3ba4284/> cleaning up clippy warnings
 - <csr-id-211951e13f3d207f27dfa1ddbaa70157d019ad27/> add license headers
 - <csr-id-49d55665ffd9ebcfe0394e40cb36bcc35a6a72f9/> update metadata, prepare for release

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 27 commits contributed to the release.
 - 26 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-enums_derive v0.2.0, irox-enums v0.2.0, irox-tools v0.2.0, irox-units v0.2.0, irox-carto v0.2.0, irox-csv v0.2.0, irox-egui-extras v0.2.0, irox-networking v0.2.0, irox-types v0.2.0, irox-influxdb_v1 v0.2.0, irox-structs_derive v0.2.0, irox-structs v0.2.0, irox-nmea0183 v0.1.0, irox-sirf v0.2.0, irox-stats v0.2.0, irox-winlocation-api v0.1.0, irox v0.2.0, safety bump 10 crates ([`6a72204`](https://github.com/spmadden/irox/commit/6a722046661ceef02a66c2067e2c5c15ce102e04))
    - Clean up code with additional lints ([`f03d8a3`](https://github.com/spmadden/irox/commit/f03d8a3ec997d53470bfdeb5e76b71925aac3f10))
    - Update cargo.tomls to add repository ([`80d2b88`](https://github.com/spmadden/irox/commit/80d2b88bdcb553faaeafc09673c31d7ebedafd19))
    - Setting up blank changelogs for the modules ([`1a36533`](https://github.com/spmadden/irox/commit/1a365333397b02a5f911d0897c3bf0c80f6c2b80))
    - New vec module with PrettyVec ([`f62ae9c`](https://github.com/spmadden/irox/commit/f62ae9c875cc0c915fc06a9003d72e50cfc013b7))
    - New collect_exact, collect_exact_or, collect_exact_or_default methods in itertools ([`c63cb79`](https://github.com/spmadden/irox/commit/c63cb79f178337401d6a19111eef83229188b45d))
    - PacketIO no longer allows a unique error - just std::io::Error ([`b39d283`](https://github.com/spmadden/irox/commit/b39d28314fd66c766cc07e3f66b8ef153e09b5a4))
    - Read_until no longer includes the delimiter in the returned result ([`d840941`](https://github.com/spmadden/irox/commit/d8409416e47d755f5c73982d24bd252e487c8199))
    - New MaybeFrom impls for primitives, MaybeMap ([`65ca576`](https://github.com/spmadden/irox/commit/65ca5767a7d19db1ab2f638631d63410702e3f21))
    - New 'MaybeFrom' and 'MaybeInto' traits, which are semantically equal to 'Result<T, ()>' ([`b175302`](https://github.com/spmadden/irox/commit/b175302cc38146e5e4c10b99e8b50383f94c7589))
    - New read_until, read_exact_into, and read_exact_into_sized functions ([`0eb5f64`](https://github.com/spmadden/irox/commit/0eb5f64a97b2383434548873a0038f1d96b94bfb))
    - Fix clippy lint for slices ([`0d50f60`](https://github.com/spmadden/irox/commit/0d50f60ee51713f0e1deb0ce049303472b60d18c))
    - Refactor types from tools into it's own module ([`ce70857`](https://github.com/spmadden/irox/commit/ce70857680aa5243227f45db5ecbaee132b7ab68))
    - Cleaning up clippy warnings ([`5c17856`](https://github.com/spmadden/irox/commit/5c178560becc0b665d70be2d99a1cffad3ba4284))
    - Prohibit unsafe code ([`c088de0`](https://github.com/spmadden/irox/commit/c088de020214e47f28391d0af5a64abe56ad185b))
    - New reflection/primitive types ([`a979b1e`](https://github.com/spmadden/irox/commit/a979b1e4bb90754b27cc2bd19405226189d6d8e1))
    - Add license headers ([`211951e`](https://github.com/spmadden/irox/commit/211951e13f3d207f27dfa1ddbaa70157d019ad27))
    - Update metadata, prepare for release ([`49d5566`](https://github.com/spmadden/irox/commit/49d55665ffd9ebcfe0394e40cb36bcc35a6a72f9))
    - Repair invalid tools Cargo.toml ([`e70cf86`](https://github.com/spmadden/irox/commit/e70cf86587258ad73b9a813b6c7878a231157921))
    - Fixing doctest ([`b120f68`](https://github.com/spmadden/irox/commit/b120f6852df399749d5be1aa75644275ea193fcb))
    - New Read utilities module ([`3c5a6f9`](https://github.com/spmadden/irox/commit/3c5a6f9825aba1516ada921e1bbeb9b9615d374e))
    - PacketIO uses Bits/MutBits now + PacketData ([`e852ff0`](https://github.com/spmadden/irox/commit/e852ff0ee41064707e90fc7be182b43ab4d08d06))
    - Add MutBits trait ([`fc284bb`](https://github.com/spmadden/irox/commit/fc284bbbe3ed66985911bc6ebd4923f81aac9393))
    - Add PacketIO module with packetization ([`5ddc215`](https://github.com/spmadden/irox/commit/5ddc215691f7bab8ffa4c1c52b8c71868ac9cf42))
    - Test asserts ([`c5bfdf3`](https://github.com/spmadden/irox/commit/c5bfdf353166347c5297c8dd3da0a57fd7a373b6))
    - Adding bits from other project ([`012691d`](https://github.com/spmadden/irox/commit/012691d48fd91392df883867a9f096dca1b912ee))
    - Adding first set of tools, min_max and looping iterator ([`c128a3c`](https://github.com/spmadden/irox/commit/c128a3c43208ee19c878e71e47fc398c2dd12cd1))
</details>

