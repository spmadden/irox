

## v0.2.0 (2026-11-04)

### Chore

 - <csr-id-50b5d0e18a05b2e7b1da475efbccecf2c883af71/> update to rust 1.97, cleanup lints
 - <csr-id-30c0363684f1c7e5f3cdb61f8cab8f4cda290cd4/> clean up some lints
 - <csr-id-691776a610c7e169cdbf888f4d18892a5cf2377c/> fix lints & bump to 1.89
 - <csr-id-191b4f9be1a45386e952d684694c33263ace7189/> remove unused libraries
 - <csr-id-622b21017e004b29d1107b7471339306083bcc4a/> fix debug lints

### New Features

 - <csr-id-c8d4a081f3285949934ec24296de3c8e63fcc4b1/> some prelim work on autogenerating colors
 - <csr-id-2cdbd427c685b0d394dd3b20aa617377b6c924f0/> add additional colormaps
 - <csr-id-43d82baa4752d6c7c287a46cd292e6ae86ac9a71/> add HSL, Display, invert_black_white() to Color
 - <csr-id-e41b178bb113ae682bdb8ffa1b2d6910f1c7a0ca/> Add basic PNG reading capability
 - <csr-id-492d29a5fe26142695e7d0646020aa4035c0bd17/> new basic ansi color theming support
 - <csr-id-7711f5d8b8199f71e339dca598ca79f224181fd1/> x11 color codes
 - <csr-id-9b53e3db74471c975bc98d30ee5999d0a6827826/> some improved color conversions
 - <csr-id-4734b79873699d56c09c55f295c2c440610853c5/> color luminance and inversion
 - <csr-id-09f26cf9ca0c74d08aa809028f8d820f6e348e9d/> HSV
 - <csr-id-d1a814d74e19323a90053db33cf0b7db11f7e545/> add x11 color definition data
 - <csr-id-f65e7683acc039d39bef2d94ead25a13db294abf/> new small terminal raster fonts
 - <csr-id-e88dd8d8462d12d8d60c0b857f8280628eabc77c/> better egui interop
 - <csr-id-9afc16d087521217e15069bfe66d8f8f0ab0b81f/> test image generation
 - <csr-id-339de44602521de8b105ce7afa6cf9778c094206/> colors and basic image storage

### Bug Fixes

 - <csr-id-8d06ac750cf9d28af857570cef9ea459d58de5e0/> fix alloc build in png
 - <csr-id-820b2838d4ad16c3d955c660f75ec2f2c8de67b4/> use new hex formatting functions
 - <csr-id-a1f1d5dae42145dc186c7f2a0fd2368269b7637d/> drop 'round' call in color in favor of cast to preserve constness of fn
 - <csr-id-f547ea29c889714dc4dfb8800288fcc6c74f82ba/> clean up features
 - <csr-id-bb2294b57f3be7b4da9002f2377e8f1da1aa40af/> fix no-std build errors
 - <csr-id-fcedd4b14cbecc6d41f2153d280483ce58eda95f/> rejigger HSV to actually compile

### New Features (BREAKING)

 - <csr-id-170a4ac732fd9524798c20960fbe7106a76e65e9/> bump egui to 0.34

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 27 commits contributed to the release.
 - 613 days passed between releases.
 - 26 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-enums_derive v0.3.0, safety bump 29 crates ([`b5c9e4a`](https://github.com/spmadden/irox/commit/b5c9e4aca7410c9d0c2cb670f49c9dd9dc711bf3))
    - Update to rust 1.97, cleanup lints ([`50b5d0e`](https://github.com/spmadden/irox/commit/50b5d0e18a05b2e7b1da475efbccecf2c883af71))
    - Some prelim work on autogenerating colors ([`c8d4a08`](https://github.com/spmadden/irox/commit/c8d4a081f3285949934ec24296de3c8e63fcc4b1))
    - Fix alloc build in png ([`8d06ac7`](https://github.com/spmadden/irox/commit/8d06ac750cf9d28af857570cef9ea459d58de5e0))
    - Use new hex formatting functions ([`820b283`](https://github.com/spmadden/irox/commit/820b2838d4ad16c3d955c660f75ec2f2c8de67b4))
    - Drop 'round' call in color in favor of cast to preserve constness of fn ([`a1f1d5d`](https://github.com/spmadden/irox/commit/a1f1d5dae42145dc186c7f2a0fd2368269b7637d))
    - Add additional colormaps ([`2cdbd42`](https://github.com/spmadden/irox/commit/2cdbd427c685b0d394dd3b20aa617377b6c924f0))
    - Add HSL, Display, invert_black_white() to Color ([`43d82ba`](https://github.com/spmadden/irox/commit/43d82baa4752d6c7c287a46cd292e6ae86ac9a71))
    - Add basic PNG reading capability ([`e41b178`](https://github.com/spmadden/irox/commit/e41b178bb113ae682bdb8ffa1b2d6910f1c7a0ca))
    - New basic ansi color theming support ([`492d29a`](https://github.com/spmadden/irox/commit/492d29a5fe26142695e7d0646020aa4035c0bd17))
    - Clean up features ([`f547ea2`](https://github.com/spmadden/irox/commit/f547ea29c889714dc4dfb8800288fcc6c74f82ba))
    - Bump egui to 0.34 ([`170a4ac`](https://github.com/spmadden/irox/commit/170a4ac732fd9524798c20960fbe7106a76e65e9))
    - Clean up some lints ([`30c0363`](https://github.com/spmadden/irox/commit/30c0363684f1c7e5f3cdb61f8cab8f4cda290cd4))
    - X11 color codes ([`7711f5d`](https://github.com/spmadden/irox/commit/7711f5d8b8199f71e339dca598ca79f224181fd1))
    - Some improved color conversions ([`9b53e3d`](https://github.com/spmadden/irox/commit/9b53e3db74471c975bc98d30ee5999d0a6827826))
    - Fix no-std build errors ([`bb2294b`](https://github.com/spmadden/irox/commit/bb2294b57f3be7b4da9002f2377e8f1da1aa40af))
    - Color luminance and inversion ([`4734b79`](https://github.com/spmadden/irox/commit/4734b79873699d56c09c55f295c2c440610853c5))
    - Rejigger HSV to actually compile ([`fcedd4b`](https://github.com/spmadden/irox/commit/fcedd4b14cbecc6d41f2153d280483ce58eda95f))
    - HSV ([`09f26cf`](https://github.com/spmadden/irox/commit/09f26cf9ca0c74d08aa809028f8d820f6e348e9d))
    - Add x11 color definition data ([`d1a814d`](https://github.com/spmadden/irox/commit/d1a814d74e19323a90053db33cf0b7db11f7e545))
    - New small terminal raster fonts ([`f65e768`](https://github.com/spmadden/irox/commit/f65e7683acc039d39bef2d94ead25a13db294abf))
    - Better egui interop ([`e88dd8d`](https://github.com/spmadden/irox/commit/e88dd8d8462d12d8d60c0b857f8280628eabc77c))
    - Fix lints & bump to 1.89 ([`691776a`](https://github.com/spmadden/irox/commit/691776a610c7e169cdbf888f4d18892a5cf2377c))
    - Test image generation ([`9afc16d`](https://github.com/spmadden/irox/commit/9afc16d087521217e15069bfe66d8f8f0ab0b81f))
    - Colors and basic image storage ([`339de44`](https://github.com/spmadden/irox/commit/339de44602521de8b105ce7afa6cf9778c094206))
    - Remove unused libraries ([`191b4f9`](https://github.com/spmadden/irox/commit/191b4f9be1a45386e952d684694c33263ace7189))
    - Fix debug lints ([`622b210`](https://github.com/spmadden/irox/commit/622b21017e004b29d1107b7471339306083bcc4a))
</details>

## v0.1.3 (2025-03-01)

### Chore

 - <csr-id-e77e9a5d2460581f0f4140a350ca5fe0b283cc05/> cleanup remaining misc lints

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 13 calendar days.
 - 34 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-imagery v0.1.3 ([`4c12afd`](https://github.com/spmadden/irox/commit/4c12afdefd9b858c2c6bc060b6ad438423e39034))
    - Cleanup remaining misc lints ([`e77e9a5`](https://github.com/spmadden/irox/commit/e77e9a5d2460581f0f4140a350ca5fe0b283cc05))
</details>

## v0.1.2 (2025-01-26)

### New Features

 - <csr-id-d1cf63488838f82f9c64755256fb75ca27851801/> cleanup nostd support in imagery
 - <csr-id-3faf80368fcaaa9f4151b03b8f6c971adc7d69e6/> reorder the classic 10 color map so the colors are more diverse

### Bug Fixes

 - <csr-id-d14960aea6509ae8ffbf4218a7b761f05b20d2f6/> ignore test requiring some local data for the moment

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 35 calendar days.
 - 42 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-imagery v0.1.2 ([`e9e4201`](https://github.com/spmadden/irox/commit/e9e420196950a89558180faf34146e9f9670fc87))
    - Cleanup nostd support in imagery ([`d1cf634`](https://github.com/spmadden/irox/commit/d1cf63488838f82f9c64755256fb75ca27851801))
    - Reorder the classic 10 color map so the colors are more diverse ([`3faf803`](https://github.com/spmadden/irox/commit/3faf80368fcaaa9f4151b03b8f6c971adc7d69e6))
    - Ignore test requiring some local data for the moment ([`d14960a`](https://github.com/spmadden/irox/commit/d14960aea6509ae8ffbf4218a7b761f05b20d2f6))
</details>

## v0.1.1 (2024-12-15)

### New Features

 - <csr-id-f2302e18dd620eba54e66732e402acca14523afb/> cleaning up some deps for wasm

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
    - Release irox-imagery v0.1.1 ([`4c3e865`](https://github.com/spmadden/irox/commit/4c3e865bf619a292633579022f4cafee4334f8d9))
    - Cleaning up some deps for wasm ([`f2302e1`](https://github.com/spmadden/irox/commit/f2302e18dd620eba54e66732e402acca14523afb))
</details>

## v0.1.0 (2024-12-13)

<csr-id-80dc3eb3869ca82d62e63358f04836babdba4584/>

### New Features

 - <csr-id-79981651b3d355c012a7b4ea088960637562c3f1/> adding colors and some colormaps
 - <csr-id-a2309e710d677b8dd2d0626506e5b639456f60c1/> more robust tiff impls
 - <csr-id-5dbff7ad9ba8cf56d18413d0cbf23370e1a21b88/> improved tiff support, still WIP.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 13 calendar days.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-imagery v0.1.0 ([`0df3c7c`](https://github.com/spmadden/irox/commit/0df3c7c303a6b7dc609eac986060fec33824021f))
    - Release irox-bits v0.3.0 ([`32e7b8d`](https://github.com/spmadden/irox/commit/32e7b8dbcb854c7eaebe3473145cbe2a4ad35ac0))
    - Adding colors and some colormaps ([`7998165`](https://github.com/spmadden/irox/commit/79981651b3d355c012a7b4ea088960637562c3f1))
    - More robust tiff impls ([`a2309e7`](https://github.com/spmadden/irox/commit/a2309e710d677b8dd2d0626506e5b639456f60c1))
    - Improved tiff support, still WIP. ([`5dbff7a`](https://github.com/spmadden/irox/commit/5dbff7ad9ba8cf56d18413d0cbf23370e1a21b88))
    - Imagery snap ([`80dc3eb`](https://github.com/spmadden/irox/commit/80dc3eb3869ca82d62e63358f04836babdba4584))
</details>

