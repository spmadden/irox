

## v0.3.0 (2024-12-13)

### New Features

 - <csr-id-067e6e3d3f0fce2f2667e8f2065a28e23083c6c7/> new bitstream struct for reading/writing a stream of individual bits
 - <csr-id-ca74e7d801a4f42d111987d2b2ee9d29e0bb0db4/> impl Bits and MutBits for Box<Bits> and Box<MutBits>
 - <csr-id-f6ebfe16cb12ee4716a6715bafa72b339f8f23d5/> pull in new 'cfg_feature_std macro'
 - <csr-id-a404f0528148edd9768d0206a8e39390ce1d4757/> Make no-std copy of 'Seek' and 'SeekFrom' types, impl for 'File'
 - <csr-id-ca66e3de64f1f9bb43b207d547ccf33b3d74155c/> rework a more generic array conversion logic.

### New Features (BREAKING)

 - <csr-id-a9800369d86f46905c1309ca4e790220195807ec/> add new dynamic byte ordering support in Bits

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 44 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - New bitstream struct for reading/writing a stream of individual bits ([`067e6e3`](https://github.com/spmadden/irox/commit/067e6e3d3f0fce2f2667e8f2065a28e23083c6c7))
    - Impl Bits and MutBits for Box<Bits> and Box<MutBits> ([`ca74e7d`](https://github.com/spmadden/irox/commit/ca74e7d801a4f42d111987d2b2ee9d29e0bb0db4))
    - Add new dynamic byte ordering support in Bits ([`a980036`](https://github.com/spmadden/irox/commit/a9800369d86f46905c1309ca4e790220195807ec))
    - Pull in new 'cfg_feature_std macro' ([`f6ebfe1`](https://github.com/spmadden/irox/commit/f6ebfe16cb12ee4716a6715bafa72b339f8f23d5))
    - Make no-std copy of 'Seek' and 'SeekFrom' types, impl for 'File' ([`a404f05`](https://github.com/spmadden/irox/commit/a404f0528148edd9768d0206a8e39390ce1d4757))
    - Rework a more generic array conversion logic. ([`ca66e3d`](https://github.com/spmadden/irox/commit/ca66e3de64f1f9bb43b207d547ccf33b3d74155c))
</details>

## v0.2.1 (2024-10-29)

### New Features

 - <csr-id-913fb3149cf855466ff2dd05845e132546c44023/> add const unsigned primitive conversions

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
    - Release irox-bits v0.2.1 ([`aa85502`](https://github.com/spmadden/irox/commit/aa855025191bd9d019b42e4012e28be24c0ffbdc))
    - Add const unsigned primitive conversions ([`913fb31`](https://github.com/spmadden/irox/commit/913fb3149cf855466ff2dd05845e132546c44023))
</details>

## v0.2.0 (2024-10-24)

### New Features

 - <csr-id-6e758eeead6cad3c16a8ce60e86ba4984f19514b/> impl Bits and MutBits for TcpStream
 - <csr-id-cac183713d52850472d3a5b90714938710796c56/> impl readline
 - <csr-id-f1c45cd905228e9f38f5c537148a329b971d3140/> new statistical lossless streaming encoders
 - <csr-id-fb3aca49374c55a105fd810b430031b5dc3bb55d/> new 'WriteToBEBits' trait and impls on primitives
 - <csr-id-031470c6ea39e78f319fd263c04f15a7ad8eab56/> new BitsArray and MutBitsArray wrappers around fixed stack-sized arrays.
 - <csr-id-89f0582bfad2a71a79b8d702a5f239a67efd357a/> SeekRead and SeekWrite now have seek_*_all methods to ensure the entire buffer is read/written.
 - <csr-id-40c370fe3b2be7e78a927f21a75a264942be193f/> new ToBEBytes and FromBEBytes const-ish traits for primitives

### Bug Fixes

 - <csr-id-223f172cdf729543c8f08c7ad34bce22e04ef1a9/> fix warning from alloc

### New Features (BREAKING)

 - <csr-id-d256059f37bcfc75dc8ba556e35343cb3cb18add/> Breaking: refactor BitsWrapper to have an owned and borrowed variant.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release.
 - 84 days passed between releases.
 - 9 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-bits v0.2.0 ([`3ed7b85`](https://github.com/spmadden/irox/commit/3ed7b850a87bfc670ce18f5c824008f09b0af7b4))
    - Breaking: refactor BitsWrapper to have an owned and borrowed variant. ([`d256059`](https://github.com/spmadden/irox/commit/d256059f37bcfc75dc8ba556e35343cb3cb18add))
    - Fix warning from alloc ([`223f172`](https://github.com/spmadden/irox/commit/223f172cdf729543c8f08c7ad34bce22e04ef1a9))
    - Impl Bits and MutBits for TcpStream ([`6e758ee`](https://github.com/spmadden/irox/commit/6e758eeead6cad3c16a8ce60e86ba4984f19514b))
    - Impl readline ([`cac1837`](https://github.com/spmadden/irox/commit/cac183713d52850472d3a5b90714938710796c56))
    - New statistical lossless streaming encoders ([`f1c45cd`](https://github.com/spmadden/irox/commit/f1c45cd905228e9f38f5c537148a329b971d3140))
    - New 'WriteToBEBits' trait and impls on primitives ([`fb3aca4`](https://github.com/spmadden/irox/commit/fb3aca49374c55a105fd810b430031b5dc3bb55d))
    - New BitsArray and MutBitsArray wrappers around fixed stack-sized arrays. ([`031470c`](https://github.com/spmadden/irox/commit/031470c6ea39e78f319fd263c04f15a7ad8eab56))
    - SeekRead and SeekWrite now have seek_*_all methods to ensure the entire buffer is read/written. ([`89f0582`](https://github.com/spmadden/irox/commit/89f0582bfad2a71a79b8d702a5f239a67efd357a))
    - New ToBEBytes and FromBEBytes const-ish traits for primitives ([`40c370f`](https://github.com/spmadden/irox/commit/40c370fe3b2be7e78a927f21a75a264942be193f))
</details>

## v0.1.5 (2024-08-01)

### New Features

 - <csr-id-9020f1174abfc895ad3438fb60cc3bc673a0a1b1/> BitsError can be used in const environments

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
    - Release irox-bits v0.1.5 ([`68fc27f`](https://github.com/spmadden/irox/commit/68fc27ff2ff0a347963cd36cab03930278b5c31f))
    - BitsError can be used in const environments ([`9020f11`](https://github.com/spmadden/irox/commit/9020f1174abfc895ad3438fb60cc3bc673a0a1b1))
</details>

## v0.1.4 (2024-07-19)

<csr-id-77677189e46aec6b857762f5a8ff0b49d6922ebf/>

### Other

 - <csr-id-77677189e46aec6b857762f5a8ff0b49d6922ebf/> fix docsrs for irox-bits

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
    - Release irox-bits v0.1.4 ([`f2464aa`](https://github.com/spmadden/irox/commit/f2464aa9392a50e3bdaeae56e4d7353099c36a15))
    - Fix docsrs for irox-bits ([`7767718`](https://github.com/spmadden/irox/commit/77677189e46aec6b857762f5a8ff0b49d6922ebf))
</details>

## v0.1.3 (2024-07-06)

### Documentation

 - <csr-id-9c6515d28c9e049ddeee33db099c81e21c608181/> improve docs.rs docs for bits

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
    - Release irox-bits v0.1.3 ([`305b602`](https://github.com/spmadden/irox/commit/305b6024e951595a1ce18f5a8d65d69301630aa9))
    - Improve docs.rs docs for bits ([`9c6515d`](https://github.com/spmadden/irox/commit/9c6515d28c9e049ddeee33db099c81e21c608181))
</details>

## v0.1.2 (2024-06-24)

### Documentation

 - <csr-id-ef4b515af54d9cf76b87f5493fd59cd386a27041/> vastly improved documentation

### New Features

 - <csr-id-cd042abec8394ea08c1bc4d14bd323e75cfd38fd/> improved support for i128/bool/char/i8
 - <csr-id-bbb9ff39051ec49313c297d6a4217532b9228390/> new SeekRead and SeekWrite traits to align Linux and Window's FileExt traits

### Bug Fixes

 - <csr-id-fcb40e9370dfb0246350252c2fd0d6158e4c5b59/> std leaked in :<

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 64 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-bits v0.1.2 ([`fcfb061`](https://github.com/spmadden/irox/commit/fcfb061ea219d32b6575115ee722aeb4416e7c63))
    - Std leaked in :< ([`fcb40e9`](https://github.com/spmadden/irox/commit/fcb40e9370dfb0246350252c2fd0d6158e4c5b59))
    - Vastly improved documentation ([`ef4b515`](https://github.com/spmadden/irox/commit/ef4b515af54d9cf76b87f5493fd59cd386a27041))
    - Improved support for i128/bool/char/i8 ([`cd042ab`](https://github.com/spmadden/irox/commit/cd042abec8394ea08c1bc4d14bd323e75cfd38fd))
    - Release irox-bits v0.1.1 ([`92fdded`](https://github.com/spmadden/irox/commit/92fdded1ffad1d268f65c1584e04967e0f04f524))
    - New SeekRead and SeekWrite traits to align Linux and Window's FileExt traits ([`bbb9ff3`](https://github.com/spmadden/irox/commit/bbb9ff39051ec49313c297d6a4217532b9228390))
</details>

## v0.1.1 (2024-05-18)

### New Features

 - <csr-id-bbb9ff39051ec49313c297d6a4217532b9228390/> new SeekRead and SeekWrite traits to align Linux and Window's FileExt traits

## v0.1.0 (2024-04-21)

<csr-id-05c5b84578474138d78211db2763e11a7bb3a925/>

### New Features

 - <csr-id-a63f11ed48e14aac3ac0d9cd058c78771c5c2d8c/> new read_exact function
 - <csr-id-5618cf8e7290900915b3cd23d87191253de4cd3e/> feature std requires alloc
 - <csr-id-a63123ab8fc28fa775ad02aacfb13081f8c03faf/> expose error info

### Bug Fixes

 - <csr-id-91da08c9b855233a77b7e0df369d70736eaeabc9/> fix removing u8 from wrong end of struct for String and Vec with horribly inefficient solution

### Refactor (BREAKING)

 - <csr-id-05c5b84578474138d78211db2763e11a7bb3a925/> pull out tools/bits into own module, no-std, no-alloc

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-bits v0.1.0 ([`6a51458`](https://github.com/spmadden/irox/commit/6a51458cde4fb1be4303792e68c476330b479a27))
    - Fix removing u8 from wrong end of struct for String and Vec with horribly inefficient solution ([`91da08c`](https://github.com/spmadden/irox/commit/91da08c9b855233a77b7e0df369d70736eaeabc9))
    - New read_exact function ([`a63f11e`](https://github.com/spmadden/irox/commit/a63f11ed48e14aac3ac0d9cd058c78771c5c2d8c))
    - Feature std requires alloc ([`5618cf8`](https://github.com/spmadden/irox/commit/5618cf8e7290900915b3cd23d87191253de4cd3e))
    - Expose error info ([`a63123a`](https://github.com/spmadden/irox/commit/a63123ab8fc28fa775ad02aacfb13081f8c03faf))
    - Pull out tools/bits into own module, no-std, no-alloc ([`05c5b84`](https://github.com/spmadden/irox/commit/05c5b84578474138d78211db2763e11a7bb3a925))
</details>

