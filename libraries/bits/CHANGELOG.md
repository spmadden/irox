

## v0.1.0 (2024-04-21)

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

 - 5 commits contributed to the release.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Fix removing u8 from wrong end of struct for String and Vec with horribly inefficient solution ([`91da08c`](https://github.com/spmadden/irox/commit/91da08c9b855233a77b7e0df369d70736eaeabc9))
    - New read_exact function ([`a63f11e`](https://github.com/spmadden/irox/commit/a63f11ed48e14aac3ac0d9cd058c78771c5c2d8c))
    - Feature std requires alloc ([`5618cf8`](https://github.com/spmadden/irox/commit/5618cf8e7290900915b3cd23d87191253de4cd3e))
    - Expose error info ([`a63123a`](https://github.com/spmadden/irox/commit/a63123ab8fc28fa775ad02aacfb13081f8c03faf))
    - Pull out tools/bits into own module, no-std, no-alloc ([`05c5b84`](https://github.com/spmadden/irox/commit/05c5b84578474138d78211db2763e11a7bb3a925))
</details>

