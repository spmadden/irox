

## v0.1.4 (2024-07-19)

### Other

 - <csr-id-77677189e46aec6b857762f5a8ff0b49d6922ebf/> fix docsrs for irox-bits

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 12 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Fix docsrs for irox-bits ([`7767718`](https://github.com/spmadden/irox/commit/77677189e46aec6b857762f5a8ff0b49d6922ebf))
</details>

## v0.1.3 (2024-07-07)

### Documentation

 - <csr-id-9c6515d28c9e049ddeee33db099c81e21c608181/> improve docs.rs docs for bits

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
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

 - 6 commits contributed to the release over the course of 42 calendar days.
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

