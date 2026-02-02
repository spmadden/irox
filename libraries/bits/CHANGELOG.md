

## v0.5.0 (2026-02-01)

### Chore

 - <csr-id-dc305634f1eda45a6a3137a6d270f9d406579c7c/> bump version to 0.5.0

### New Features

 - <csr-id-79cd221db336001a43bf474011f6779a30db731e/> impl u32 to/from [u8;64]
 - <csr-id-4cd4f7b3c81c27d30335b98f48a493f24f5cac90/> Impl Read for BitsWrapper
 - <csr-id-8b214d075998dbeb3860d4d5071058792dbfc1fc/> consume_until now returns a bool to if it found the search string
 - <csr-id-9bcd25c7ccf9798c6f83c1ee71996cfaf8d9bb21/> improved alloc/std bits passthroughs
 - <csr-id-c741ea689400b84dffebc67e0cd001136d998c1a/> new Bits::remaining function
 - <csr-id-89ea040cd9eb02c73be86cd7c09960d1a48cd0e3/> Ability to flush the BitsBuffer periodically
 - <csr-id-ab5451d8710710536ec66c62761c75ef73f1936c/> New backtrace feature for BitsError to show where it came from
 - <csr-id-8e7c85bcb84c3c9c2c16fd9780d1460908d41075/> new BitsBuffer struct, that wraps & buffers a Bits impl
 - <csr-id-e756d343f862b1b593ca5a6302379a4f4f82b01c/> better TCP stream buffering
 - <csr-id-5c38bc11a69d10824a056b34582a8cf0113e81a8/> new read_str_nul_terminated fns
 - <csr-id-9c9a598cc6a9ce6e5543209456945a8c381ee2d5/> more impls of WriteToBEBits
 - <csr-id-39931e44889ad1f02af308721533fb8c0ec60b01/> new 'read_filling' method
 - <csr-id-e3e39b9dbbfc1bf627e62d4ac207ad228f7a8454/> adding LE, peek, and delegation to BitStream
 - <csr-id-04e6a38e72b10f7db063b0cc1881849e8cd390b1/> new SerializeToBits and DeserializeFromBits traits for specific encoding formats
 - <csr-id-a7bddcec69b0ab5a04b718f24881818339e1b449/> impl ToBEBytes for some array lengths

### Bug Fixes

 - <csr-id-0fbe42ff747075b5bf60e0ba3294d6d9da73b9da/> clean imports for non-standard os's

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 17 commits contributed to the release.
 - 337 days passed between releases.
 - 17 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version to 0.5.0 ([`dc30563`](https://github.com/spmadden/irox/commit/dc305634f1eda45a6a3137a6d270f9d406579c7c))
    - Impl u32 to/from [u8;64] ([`79cd221`](https://github.com/spmadden/irox/commit/79cd221db336001a43bf474011f6779a30db731e))
    - Impl Read for BitsWrapper ([`4cd4f7b`](https://github.com/spmadden/irox/commit/4cd4f7b3c81c27d30335b98f48a493f24f5cac90))
    - Consume_until now returns a bool to if it found the search string ([`8b214d0`](https://github.com/spmadden/irox/commit/8b214d075998dbeb3860d4d5071058792dbfc1fc))
    - Improved alloc/std bits passthroughs ([`9bcd25c`](https://github.com/spmadden/irox/commit/9bcd25c7ccf9798c6f83c1ee71996cfaf8d9bb21))
    - Clean imports for non-standard os's ([`0fbe42f`](https://github.com/spmadden/irox/commit/0fbe42ff747075b5bf60e0ba3294d6d9da73b9da))
    - New Bits::remaining function ([`c741ea6`](https://github.com/spmadden/irox/commit/c741ea689400b84dffebc67e0cd001136d998c1a))
    - Ability to flush the BitsBuffer periodically ([`89ea040`](https://github.com/spmadden/irox/commit/89ea040cd9eb02c73be86cd7c09960d1a48cd0e3))
    - New backtrace feature for BitsError to show where it came from ([`ab5451d`](https://github.com/spmadden/irox/commit/ab5451d8710710536ec66c62761c75ef73f1936c))
    - New BitsBuffer struct, that wraps & buffers a Bits impl ([`8e7c85b`](https://github.com/spmadden/irox/commit/8e7c85bcb84c3c9c2c16fd9780d1460908d41075))
    - Better TCP stream buffering ([`e756d34`](https://github.com/spmadden/irox/commit/e756d343f862b1b593ca5a6302379a4f4f82b01c))
    - New read_str_nul_terminated fns ([`5c38bc1`](https://github.com/spmadden/irox/commit/5c38bc11a69d10824a056b34582a8cf0113e81a8))
    - More impls of WriteToBEBits ([`9c9a598`](https://github.com/spmadden/irox/commit/9c9a598cc6a9ce6e5543209456945a8c381ee2d5))
    - New 'read_filling' method ([`39931e4`](https://github.com/spmadden/irox/commit/39931e44889ad1f02af308721533fb8c0ec60b01))
    - Adding LE, peek, and delegation to BitStream ([`e3e39b9`](https://github.com/spmadden/irox/commit/e3e39b9dbbfc1bf627e62d4ac207ad228f7a8454))
    - New SerializeToBits and DeserializeFromBits traits for specific encoding formats ([`04e6a38`](https://github.com/spmadden/irox/commit/04e6a38e72b10f7db063b0cc1881849e8cd390b1))
    - Impl ToBEBytes for some array lengths ([`a7bddce`](https://github.com/spmadden/irox/commit/a7bddcec69b0ab5a04b718f24881818339e1b449))
</details>

## v0.4.2 (2025-03-01)

<csr-id-1f48b70c64fb0cde031bf379fe3d6b5b276b6f51/>
<csr-id-de5e67fb00da4d87ac75adb7592f4848ba2399b2/>

### Chore

 - <csr-id-1f48b70c64fb0cde031bf379fe3d6b5b276b6f51/> cleanup bitmask order-of-operations
 - <csr-id-de5e67fb00da4d87ac75adb7592f4848ba2399b2/> elude all the lifetimes!

### New Features

 - <csr-id-f91538d1cb3ebae7eaee1273cbaaca08979b99e2/> add impl WriteToBEBits for Arc<String>
 - <csr-id-4f516d71e4db3ff1874da92d226939ad2e1478de/> fix lints for 1.84, update to 1.84

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 13 calendar days.
 - 17 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-bits v0.4.2 ([`da45a93`](https://github.com/spmadden/irox/commit/da45a93d8a0e1621f4ac63dc77f8e00528cffba4))
    - Add impl WriteToBEBits for Arc<String> ([`f91538d`](https://github.com/spmadden/irox/commit/f91538d1cb3ebae7eaee1273cbaaca08979b99e2))
    - Fix lints for 1.84, update to 1.84 ([`4f516d7`](https://github.com/spmadden/irox/commit/4f516d71e4db3ff1874da92d226939ad2e1478de))
    - Cleanup bitmask order-of-operations ([`1f48b70`](https://github.com/spmadden/irox/commit/1f48b70c64fb0cde031bf379fe3d6b5b276b6f51))
    - Elude all the lifetimes! ([`de5e67f`](https://github.com/spmadden/irox/commit/de5e67fb00da4d87ac75adb7592f4848ba2399b2))
</details>

## v0.4.1 (2025-02-12)

### New Features

 - <csr-id-d428007e46592c8b1c15d2e83abd34f94a7a8540/> Add LE bit encoding/decoding for strings
   Introduced `WriteToLEBits` for `&str` and `ReadFromLEBits` for `String` to support little-endian bit operations. These implementations handle length-prefixed string serialization and deserialization efficiently.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 17 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-bits v0.4.1 ([`ff0885f`](https://github.com/spmadden/irox/commit/ff0885f4c2955b71b920efe59df0ca0dd7e5424b))
    - Add LE bit encoding/decoding for strings ([`d428007`](https://github.com/spmadden/irox/commit/d428007e46592c8b1c15d2e83abd34f94a7a8540))
</details>

## v0.4.0 (2025-01-26)

### New Features

 - <csr-id-5fe9aca83691818e241c808034706288f8366538/> rework codec into a template to impl more conversions
 - <csr-id-c1ba59f17d2937c2aea90f2ff90a1fdace864dcc/> impl LE for f32/f64
 - <csr-id-0a2df5a5c5e83ffd2ab6fed96c0d5b2aecb00dad/> Add new Tee struct which copies data to multiple outputs simultaneously
 - <csr-id-01bfda3f7f60e219d99e53d9db25d91da9d0fcfb/> new string encoding WriteToBEBits, ReadFromBEBits
 - <csr-id-f62f28cdd74e919909661f3e22be520b917b502b/> new read_X_blob_into functions in Bits
 - <csr-id-8b258ddec7c7cffed8df3c58a3e46e7f0eac898c/> new ReadFromBEBits trait, the inverse of WriteToBeBits
 - <csr-id-47550e97462f3100d11b34884e8d7a56bba39eef/> new flush() in MutBits
 - <csr-id-e34948deae97e45b1da777dcc616eee983cf6487/> new CountingBits struct, move SharedROCounter to bits from tools
 - <csr-id-1ed5ace17b2670059fa9ca8dfafd8f8a307e2423/> new BufBits trait, like BufRead
 - <csr-id-90367606a72590d15b9ab172af537570b97d08fa/> new default write_all_into fn in MutBits
 - <csr-id-f77e9ec544ecdc3ee37fbc38f0d5ad294d6e6bb1/> impl Bits,MutBits for BufRead,BufWrite

### Bug Fixes

 - <csr-id-3491afb67a5d45feac9cf1865900c2a196d76f4f/> fix issue where WriteToBEBytes returned the wrong length.
 - <csr-id-f2a9641da129188ced1bfcc02c53d638b9d095a3/> fix alloc compliation

### New Features (BREAKING)

 - <csr-id-cf18819735eecc7e8512ec587f59fcbed385d712/> return usize instead of () in WriteToBEBits

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 25 calendar days.
 - 43 days passed between releases.
 - 14 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-bits v0.4.0 ([`a6de5f0`](https://github.com/spmadden/irox/commit/a6de5f0f1280adf860333e8c066b145e3166ef4b))
    - Fix issue where WriteToBEBytes returned the wrong length. ([`3491afb`](https://github.com/spmadden/irox/commit/3491afb67a5d45feac9cf1865900c2a196d76f4f))
    - Rework codec into a template to impl more conversions ([`5fe9aca`](https://github.com/spmadden/irox/commit/5fe9aca83691818e241c808034706288f8366538))
    - Impl LE for f32/f64 ([`c1ba59f`](https://github.com/spmadden/irox/commit/c1ba59f17d2937c2aea90f2ff90a1fdace864dcc))
    - Add new Tee struct which copies data to multiple outputs simultaneously ([`0a2df5a`](https://github.com/spmadden/irox/commit/0a2df5a5c5e83ffd2ab6fed96c0d5b2aecb00dad))
    - Fix alloc compliation ([`f2a9641`](https://github.com/spmadden/irox/commit/f2a9641da129188ced1bfcc02c53d638b9d095a3))
    - New string encoding WriteToBEBits, ReadFromBEBits ([`01bfda3`](https://github.com/spmadden/irox/commit/01bfda3f7f60e219d99e53d9db25d91da9d0fcfb))
    - New read_X_blob_into functions in Bits ([`f62f28c`](https://github.com/spmadden/irox/commit/f62f28cdd74e919909661f3e22be520b917b502b))
    - New ReadFromBEBits trait, the inverse of WriteToBeBits ([`8b258dd`](https://github.com/spmadden/irox/commit/8b258ddec7c7cffed8df3c58a3e46e7f0eac898c))
    - Return usize instead of () in WriteToBEBits ([`cf18819`](https://github.com/spmadden/irox/commit/cf18819735eecc7e8512ec587f59fcbed385d712))
    - New flush() in MutBits ([`47550e9`](https://github.com/spmadden/irox/commit/47550e97462f3100d11b34884e8d7a56bba39eef))
    - New CountingBits struct, move SharedROCounter to bits from tools ([`e34948d`](https://github.com/spmadden/irox/commit/e34948deae97e45b1da777dcc616eee983cf6487))
    - New BufBits trait, like BufRead ([`1ed5ace`](https://github.com/spmadden/irox/commit/1ed5ace17b2670059fa9ca8dfafd8f8a307e2423))
    - New default write_all_into fn in MutBits ([`9036760`](https://github.com/spmadden/irox/commit/90367606a72590d15b9ab172af537570b97d08fa))
    - Impl Bits,MutBits for BufRead,BufWrite ([`f77e9ec`](https://github.com/spmadden/irox/commit/f77e9ec544ecdc3ee37fbc38f0d5ad294d6e6bb1))
</details>

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

 - 7 commits contributed to the release over the course of 13 calendar days.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-bits v0.3.0 ([`32e7b8d`](https://github.com/spmadden/irox/commit/32e7b8dbcb854c7eaebe3473145cbe2a4ad35ac0))
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

 - 10 commits contributed to the release over the course of 11 calendar days.
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

 - 2 commits contributed to the release over the course of 11 calendar days.
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

