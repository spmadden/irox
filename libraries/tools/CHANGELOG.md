


## v0.10.1 (2025-02-05)

### New Features

 - <csr-id-d35ec29c046a31bc40e6365f07620d985db2c2e9/> adding sixwords encoding/decoding

### Bug Fixes

 - <csr-id-f3b8c0c58e3ea34c2ff073555efd7929ae84ab87/> fix noalloc in tools

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 2 calendar days.
 - 10 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Adding sixwords encoding/decoding (d35ec29)
    - Fix noalloc in tools (f3b8c0c)
</details>

## v0.10.0 (2025-01-26)

<csr-id-73c770e4c24a06ebd1938f68027b32354a8a6c29/>

### Chore

 - <csr-id-73c770e4c24a06ebd1938f68027b32354a8a6c29/> fixup lints

### New Features

 - <csr-id-b71c23c77672f7efcb500268167874ec9f0403d3/> impl new,debug,clone in RoundBuffer
 - <csr-id-79883f420af37979cf0b19d70d7960df7b358703/> HMAC-SHA2 with test vectors
 - <csr-id-33553680fe493ec285286310c2bac2a29d60b90d/> some asserts can now take test name/message as last parameter
 - <csr-id-80726de6cd543fbad5faa4bb182f534bb231a078/> adding sha1 test vectors
 - <csr-id-8270cefc0903fafcf39d8b206a250403df3c79e9/> fixup nostd support in carto
 - <csr-id-54580210d482718c4e6a876b0081485eca0d5b44/> new Matrix::transpose() operation
 - <csr-id-d0c38822ffb13c9a7fb300a402161a43e339816c/> tests for rotating matrices
 - <csr-id-904ba2577ddd001a459326fb2741436f8dfab923/> new ZeroedBuffer trait to create empty buffers
 - <csr-id-8b816e9cdc2228c9de03e406dd4b1e8454c030db/> zero out any part of RoundU8Buffer when consumed
 - <csr-id-7bf897d6381c453936022d5ea985cd5e3fcc4930/> yaaay matrix transforms :>
 - <csr-id-6321e4e643eca9541801dee69bd19a8f66e63bdc/> new negative_one fn in ToSigned to allow easy inversion
 - <csr-id-223ce686b44ef9bf39fa83ea06c02bfbe5d814b4/> new OrderedHashMap that replicates elements in insertion order.
 - <csr-id-be2700b42dba1c7e88a7db6d131d17e26bde840d/> new ZagZig impls, fix alloc in vbyte
 - <csr-id-fd99495373be8ef4adda57cbece5e0dcd2f53f20/> impl BufBits for StreamReader
 - <csr-id-e3caf23f1b80eafdb449e92ce8d1fda89afa91b3/> new ToSigned and ToUnsigned traits for the primitives
 - <csr-id-e73848b677f582ad68a22cd92afb0d0fd4cdb1c5/> refactoring StrBuf into FixedU8Buf
 - <csr-id-fdbf627db9f5614d9b92a135812dc6c05adf85ad/> add an atomic counter to CodeDictionary for stats
 - <csr-id-616ab4de9cadc5c0caf2ae381e42c12d9d70cefa/> new shared version of the code dictionary and encoder
 - <csr-id-15873e543c1009ee5b863f3e058ca9a35a742847/> new GroupVarintCodeDecoder
 - <csr-id-ef314f4261f6f8f84a6571b0d844801c045ffc7f/> impl From<[u8;N]> for RoundU8Buffer
 - <csr-id-0a8a6c62ebfb1ea2228fc6317af920ca9fb7c3a3/> new CodeDictionary and GroupVarintCodeEncoder capabilities
 - <csr-id-57d04be3a4becf00779eaed39351b8c68bcd2ee1/> new varint encoding capability
 - <csr-id-a7e15d7475435f05e57fb13310ed1bb0d4f88ef2/> new vbyte predictive length functions
 - <csr-id-d1e54a55a16da755bb2c3b9d420e3bd0b447c08f/> impl WriteToBEBits for IntegerValue
 - <csr-id-247c3228f154f10eb36a6bd96b1ec14c3965da71/> further flushing out of Fixed & Round u8 buffers
 - <csr-id-e34948deae97e45b1da777dcc616eee983cf6487/> new CountingBits struct, move SharedROCounter to bits from tools
 - <csr-id-138fe8565dc368755498c2f1426779e227cbc54a/> Better buffer usage in the RoundU8Buffer, nearly usable now for reading & writing.
 - <csr-id-83a5f506b8a49930e8d18d07cc9dd5f311d0db8d/> new basic matrix implementation
 - <csr-id-cd5e251008bbd88c72c7bd56793cf72cd85a503a/> new debug_assert_eq_eps assert
 - <csr-id-c459c43514fd3d240686ddd23ecd3645883b8f95/> new functions in PRNG to make generating spreads of numbers easier
 - <csr-id-33bee42679e32d4b1e2e48bc18a072dcce881a51/> new bencher to check performance of atomics on a hardware config
 - <csr-id-fbd4a63b40a6d1955e446a0480ea183a01e7ada9/> new thread-safe performant one-way non-blocking data exchanger

### Bug Fixes

 - <csr-id-a7d97e08afba83769389842156c78e9792037e66/> fix multi_stream on linux, seek_write doesn't modify the underlying position of the FP like it does on windows.
 - <csr-id-507a83ccd5831745d983fcbddf6a9da9f83df9c5/> fix edge case issues in multi_stream around the end of a stream.
 - <csr-id-2e05702b0f90563b76be1caa2f0666d90158dbdd/> switch to RoundBuf in UnlimitedBuf for performance
 - <csr-id-f9a5bdd6dd5f9b06a48b8fa0d1e46986cc2d1aa1/> fix off-by-one in roundu8 buffer limiting
 - <csr-id-4096723f47280285db82c2ce82afc21c23cfcb95/> fix off-by-one where key was same length as block size.
 - <csr-id-b60738301ce6d50bcd91b40bf8da7225ad3ff1f2/> fix impls of sha2/384/512 and add NIST test vectors
 - <csr-id-f327a89b1f398e1552758b0a9bd49506bcabbd90/> silence warning about unused trait in Matrix
 - <csr-id-dbf8ef898ba6c089582caf3d6d7fe7f8f166e845/> matrix rotations behind feature std for sin/cos
 - <csr-id-e7bba1e8df7cffe0bf8a30addee93218500c3a34/> fix test assumption the buffer wasn't cleared.
 - <csr-id-ee5face00fb5d16bf1a23b9fff50852b1eafacf2/> fix alloc feature build in tools

### New Features (BREAKING)

 - <csr-id-cf18819735eecc7e8512ec587f59fcbed385d712/> return usize instead of () in WriteToBEBits

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 45 commits contributed to the release over the course of 40 calendar days.
 - 41 days passed between releases.
 - 44 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.10.0 (06629ef)
    - Fix multi_stream on linux, seek_write doesn't modify the underlying position of the FP like it does on windows. (a7d97e0)
    - Fix edge case issues in multi_stream around the end of a stream. (507a83c)
    - Impl new,debug,clone in RoundBuffer (b71c23c)
    - Switch to RoundBuf in UnlimitedBuf for performance (2e05702)
    - Fix off-by-one in roundu8 buffer limiting (f9a5bdd)
    - HMAC-SHA2 with test vectors (79883f4)
    - Fix off-by-one where key was same length as block size. (4096723)
    - Some asserts can now take test name/message as last parameter (3355368)
    - Adding sha1 test vectors (80726de)
    - Fix impls of sha2/384/512 and add NIST test vectors (b607383)
    - Fixup nostd support in carto (8270cef)
    - New Matrix::transpose() operation (5458021)
    - Silence warning about unused trait in Matrix (f327a89)
    - Tests for rotating matrices (d0c3882)
    - Matrix rotations behind feature std for sin/cos (dbf8ef8)
    - Fix test assumption the buffer wasn't cleared. (e7bba1e)
    - New ZeroedBuffer trait to create empty buffers (904ba25)
    - Zero out any part of RoundU8Buffer when consumed (8b816e9)
    - Yaaay matrix transforms :> (7bf897d)
    - New negative_one fn in ToSigned to allow easy inversion (6321e4e)
    - New OrderedHashMap that replicates elements in insertion order. (223ce68)
    - New ZagZig impls, fix alloc in vbyte (be2700b)
    - Impl BufBits for StreamReader (fd99495)
    - New ToSigned and ToUnsigned traits for the primitives (e3caf23)
    - Refactoring StrBuf into FixedU8Buf (e73848b)
    - Fix alloc feature build in tools (ee5face)
    - Add an atomic counter to CodeDictionary for stats (fdbf627)
    - New shared version of the code dictionary and encoder (616ab4d)
    - New GroupVarintCodeDecoder (15873e5)
    - Impl From<[u8;N]> for RoundU8Buffer (ef314f4)
    - New CodeDictionary and GroupVarintCodeEncoder capabilities (0a8a6c6)
    - New varint encoding capability (57d04be)
    - New vbyte predictive length functions (a7e15d7)
    - Impl WriteToBEBits for IntegerValue (d1e54a5)
    - Further flushing out of Fixed & Round u8 buffers (247c322)
    - Return usize instead of () in WriteToBEBits (cf18819)
    - New CountingBits struct, move SharedROCounter to bits from tools (e34948d)
    - Better buffer usage in the RoundU8Buffer, nearly usable now for reading & writing. (138fe85)
    - New basic matrix implementation (83a5f50)
    - New debug_assert_eq_eps assert (cd5e251)
    - Fixup lints (73c770e)
    - New functions in PRNG to make generating spreads of numbers easier (c459c43)
    - New bencher to check performance of atomics on a hardware config (33bee42)
    - New thread-safe performant one-way non-blocking data exchanger (fbd4a63)
</details>

## v0.9.3 (2024-12-15)

### New Features

 - <csr-id-d72868b00c3b8ae018badf809dce12584a1186a5/> new assert_eq_eps_slice! macro for testing slices of floating points to within epsilon values

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
    - Release irox-tools v0.9.3 (7df8024)
    - New assert_eq_eps_slice! macro for testing slices of floating points to within epsilon values (d72868b)
</details>

## v0.9.2 (2024-12-13)

### New Features

 - <csr-id-0ac28bf47dd59480b3c7b840f2876d7c44bbcd0f/> new 'ToF64' trait
 - <csr-id-55d1a9ad0e2d94780baf7c85ff9b43b93c2da0db/> new WrappingAdd/Sub/Mul for the primitives.
 - <csr-id-a02debb360ce750c741a940eac1fedb25a4900e0/> new FixedU8Buf with optimizations for a straight u8
 - <csr-id-a85828d283e6a0a2528140562bfe94ad6776aeaa/> re-pull in the 'once' module behind the std feature flag
 - <csr-id-79b4c0111cfb4daff7419dda335fca312e4afa4e/> bump MSRV to 1.82

### Bug Fixes

 - <csr-id-7893feb3e314ef7b6f8dc61d42edf07bf86f7a2d/> fix assumption that error types are called 'Error'

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 20 calendar days.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.9.2 (219aa3a)
    - New 'ToF64' trait (0ac28bf)
    - New WrappingAdd/Sub/Mul for the primitives. (55d1a9a)
    - New FixedU8Buf with optimizations for a straight u8 (a02debb)
    - Re-pull in the 'once' module behind the std feature flag (a85828d)
    - Fix assumption that error types are called 'Error' (7893feb)
    - Bump MSRV to 1.82 (79b4c01)
</details>

## v0.9.1 (2024-10-29)

### New Features

 - <csr-id-f4f2cb5289f2cb311f583a2af1af10699c20ef6b/> add hexarray function and nibble_to_hex_char

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
    - Release irox-tools v0.9.1 (233bb8f)
    - Add hexarray function and nibble_to_hex_char (f4f2cb5)
</details>

## v0.9.0 (2024-10-24)

<csr-id-03011089653d97ae0c6ec0624aec627285d3c19a/>

### New Features

 - <csr-id-c3fceccc71f1617c4376c684029e89f2da4c4630/> add 'limit' capability to RoundU8Buffer to artificially limit the output
 - <csr-id-a2e38381e8de3faa2bd3ea29d7d0f8c2f20587d3/> MultiStreamWriter returns an Arc, add len and is_empty
 - <csr-id-7bf1542894179a6005a28819032adf21f458ee5e/> impl bits & mutbits for unlimited buffer, fix lengths & push_back
 - <csr-id-09104c6821dcbaf5d1f02376b54e5c50f78ad979/> impl signed vbyte/zigzag encoding
 - <csr-id-aa7909f619e332d37cb2b6099b3bba2be52fccc4/> add 'to_bits', 'exponent', and 'significand' to FloatExt trait
 - <csr-id-fb0ed97e95a61348d3b72915f3404ca99ca27a39/> impl EncodeVByteTo for u8
 - <csr-id-897fc6368878d09d9cc91b752cc1a6f6b318ac8b/> random now has a fill() method
 - <csr-id-b2d313d3dbf852a2c7ca635065fc5bc3495e9de3/> new trait EncodeVByteTo for writing encoded VBytes (varints) to MutBits impls
 - <csr-id-1c0968904aae747527b9c8d5c17f76ba35db3b4d/> FixedBuf gets an iterator and an impl of WriteToBEBits
 - <csr-id-c3c4adea1d8d876620b8f38e72f00be253448bac/> Multi-Stream Reader impls.
 - <csr-id-cb9d4e6e60f90eb2b6ff9db76e13ece93e548c7d/> Decode vbytes directly from Bits impls.
 - <csr-id-5075479018db38b8944a0d554d88154c767fdb7a/> new Round Buffer optimized for u8's
 - <csr-id-5fb2f75cbee4960b86829e0f7493dc977847a07f/> New Multi-Stream File writer
 - <csr-id-2aa6728b1789faf420f7c9bc4d9ac3bc666156a5/> vbyte encode_u128 uses same encoding method as generally accepted.
 - <csr-id-d19bfdc570ea6010e2ed446cc135d551f33bef03/> Buffer gets new 'is_full' method
 - <csr-id-6e808d638afbca28a5fc2daea955aa1b55bfdff2/> FixedBuf gets MutBits and LendingIterator support.
 - <csr-id-5488bb2fd5ef691e38bfc0cbaa7ff5eae2a635f8/> lending iterator trait
 - <csr-id-5eaec4960399ffa235f2b720b4efd49d975fb91c/> pivot vbyte to use fixedbuf internally
 - <csr-id-9c38e72895be6a00382ad1f8c42bbffb1a51759e/> better vbyte impl

### Bug Fixes

 - <csr-id-93fb4fb43b97cfb6bdc6d3258862a3a2fdb3b125/> Fix issue where MultiStream would return garbage if the stream didn't end on a block boundary
 - <csr-id-6e48657b59c217477e65bc2066ac87b08df7ef1c/> fix issue in RoundU8Buffer where the head pointer wouldn't increment if the buffer was full
 - <csr-id-b1542dbc8f81c8dd486d71b24b4b5b11dafa9a32/> fix garbage constants in random generators

### Other

 - <csr-id-03011089653d97ae0c6ec0624aec627285d3c19a/> more work on multi-streams

### New Features (BREAKING)

 - <csr-id-d256059f37bcfc75dc8ba556e35343cb3cb18add/> Breaking: refactor BitsWrapper to have an owned and borrowed variant.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 25 commits contributed to the release over the course of 16 calendar days.
 - 37 days passed between releases.
 - 24 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.9.0 (7e9a935)
    - Fix issue where MultiStream would return garbage if the stream didn't end on a block boundary (93fb4fb)
    - Add 'limit' capability to RoundU8Buffer to artificially limit the output (c3fcecc)
    - Fix issue in RoundU8Buffer where the head pointer wouldn't increment if the buffer was full (6e48657)
    - MultiStreamWriter returns an Arc, add len and is_empty (a2e3838)
    - Impl bits & mutbits for unlimited buffer, fix lengths & push_back (7bf1542)
    - Impl signed vbyte/zigzag encoding (09104c6)
    - Add 'to_bits', 'exponent', and 'significand' to FloatExt trait (aa7909f)
    - Breaking: refactor BitsWrapper to have an owned and borrowed variant. (d256059)
    - Impl EncodeVByteTo for u8 (fb0ed97)
    - Random now has a fill() method (897fc63)
    - New trait EncodeVByteTo for writing encoded VBytes (varints) to MutBits impls (b2d313d)
    - FixedBuf gets an iterator and an impl of WriteToBEBits (1c09689)
    - Multi-Stream Reader impls. (c3c4ade)
    - Decode vbytes directly from Bits impls. (cb9d4e6)
    - New Round Buffer optimized for u8's (5075479)
    - New Multi-Stream File writer (5fb2f75)
    - Vbyte encode_u128 uses same encoding method as generally accepted. (2aa6728)
    - Buffer gets new 'is_full' method (d19bfdc)
    - FixedBuf gets MutBits and LendingIterator support. (6e808d6)
    - Lending iterator trait (5488bb2)
    - More work on multi-streams (0301108)
    - Fix garbage constants in random generators (b1542db)
    - Pivot vbyte to use fixedbuf internally (5eaec49)
    - Better vbyte impl (9c38e72)
</details>

## v0.8.6 (2024-09-16)

### New Features

 - <csr-id-57bccdba5ee3921ef359ca35953d6c6bd8929a92/> new SyncFlag signalling

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
    - Release irox-tools v0.8.6 (1c99a16)
    - New SyncFlag signalling (57bccdb)
</details>

## v0.8.5 (2024-09-11)

<csr-id-a35975360f42880d6e74ceb4443ccd4093c27975/>
<csr-id-53f8eaa1d223ce33a9898de829d2557ca30832ed/>

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

 - 6 commits contributed to the release over the course of 4 calendar days.
 - 40 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.8.5 (3261257)
    - New SharedCell wrapper around Arc<RwLock<Option<T>>> (15ab622)
    - Ignore the documentation that's not rustdoc tests in errors (53f8eaa)
    - Fixup lints & formatting (a359753)
    - Add new unlimited/paged buffer that does not reallocate (98d5046)
    - Derive clone on fixedbuf (165dc19)
</details>

## v0.8.4 (2024-08-01)

### New Features

 - <csr-id-08ae6281049fcea8ac12536cce1792bf52c7d735/> new const hex! macro for compile-time hex string literals

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
    - Release irox-tools v0.8.4 (62055d3)
    - New const hex! macro for compile-time hex string literals (08ae628)
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
    - Release irox-tools v0.8.3 (8a633e9)
    - Fix docsrs for irox-tools (d9679d0)
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

 - 15 commits contributed to the release over the course of 33 calendar days.
 - 36 days passed between releases.
 - 14 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.8.2 (e85d66a)
    - Improvements across the board for docs.rs docs (c434f69)
    - Bump to rust 1.79 (7d369bf)
    - New testing '_toobig-tests' feature to ignore certain tests that are expensive & slow (1459e34)
    - Playing with buffers in sha2 for perf. (0999f97)
    - Clean up lints (5ce4b19)
    - New implementation of SHA2 (2e3ec33)
    - Impl HashDigest for MD5 (11ff4aa)
    - Impl HashDigest for SHA1 (0ab1678)
    - Implementation of HMAC (a7d6f47)
    - Improved perf for stack-allocated RoundBuf (167144f)
    - Fix typo in feature name guard preventing use of hex::from_hex_str (5e2c0bd)
    - New assert::assert_eq_hex_slice method (f8a4651)
    - Clean up new lints (7809b2d)
    - Fixup some 1.78 lint warnings (7b8a2bf)
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

 - 9 commits contributed to the release over the course of 13 calendar days.
 - 27 days passed between releases.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.8.1 (a77a177)
    - Put pagefile behind right feature (bits/std) (2c526b4)
    - Fix urls in primitives (c853a91)
    - Move buf back into module, expose StrBuf (826ce7f)
    - Fix pagefile compiling/function on Linux (835f4d7)
    - Clean up some test warnings in irox-tools (b8c91df)
    - Disable lints for random's test code (62b40dc)
    - Add new (basic) pagefile wrapper. (45b145a)
    - Fix debug math panic in Random (1c413bb)
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
    - Release irox-tools v0.8.0 (00325aa)
    - New feature: alloc, change default features to be nil rather than everything (bb44251)
    - New no-std/no-alloc fixed-size stack impls FixedBuf and RoundBuf (2537e8c)
    - Pull out tools/bits into own module, no-std, no-alloc (05c5b84)
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
    - Release irox-tools v0.7.0 (a6a3ae3)
    - New joining and joining_multi itertools methods (66cb1ad)
    - Rename write_*_blob methods to have 'be' and 'le' variants (fbd6a72)
    - New read/write length-prefixed strings functions (e8864e6)
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

 - 12 commits contributed to the release over the course of 2 calendar days.
 - 11 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.6.1 (091c8ac)
    - Accuracy of no_std FloatExt funcs (915cd9d)
    - Impl FloatExt on feature 'std' as well (be13dba)
    - Adding impl of SHA1 (efd9775)
    - Fixed issue where Bits::read_exact_vec was not sized correctly (3b2c7a2)
    - More support for little-endian in bits (40d00a6)
    - Adding impl of MD5 (a606b9a)
    - Adding assert_eq_hex! macro for hex printing assertions (9711bc3)
    - Adding u32 primitive FromArray and ToArray traits (2da7921)
    - Switch to different (better) alg for ln, imp loop perf (403f928)
    - Only run no_std tests on no_std build (1baab61)
    - Impl exp() ln() powi(), powf() and sqrt() for f64, f32 in no_std (dbdc371)
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

 - 7 commits contributed to the release over the course of 35 calendar days.
 - 48 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.6.0 (0560dc1)
    - Tweak alloc imports in fs, fix readme in cargo.toml (d012d64)
    - Clean up new lints for 1.75 (300356f)
    - Recursive justfiles (7902f54)
    - Refactor packetio traits to use Bits rather than std::io::*.  Is now no_std compliant. (ca214f0)
    - New 'Readerator' to turn Read into an Iterator. (c771cbc)
    - Big refactor of (Mut)Bits.  Default impls for std::io::{Read,Write} removed. (2c04083)
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
    - Release irox-tools v0.5.3 (eb01ead)
    - New fs mod for filename/filesystem utils (a3849cc)
    - New eventually complete ecosystem (464a2db)
    - New lazy static initialization macro (e599d63)
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
    - Release irox-tools v0.5.2 (89b01ec)
    - Bump tools to 0.5.2 (7bb9983)
    - Update readme for error macros (350e0f8)
    - New errors macros to accelerate error conversions (55f7016)
    - Update docs & readme for tools (5be0cba)
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
    - Release irox-tools v0.5.1 (c51a7a0)
    - Bump tools to 0.5.1 (aa59052)
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

 - 25 commits contributed to the release over the course of 8 calendar days.
 - 37 days passed between releases.
 - 24 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.5.0, safety bump 17 crates (a46e9e2)
    - Add additional PCG random impls (098c51c)
    - Starting new vbyte impls (0b67aa6)
    - Refactor random to have PRNG trait, impl new random algorithm (59aa3f6)
    - Mv codec.rs into codec/mod.rs (00bc196)
    - Rename 'sync.rs' into 'sync/mod.rs' (081d769)
    - Rename 'read.rs' into 'read/mod.rs' (cf0bff7)
    - HexDump is now no-std (4000e53)
    - MutBits can now be used with write!() (65021dc)
    - Move base64, bits, id, scanner, uuid to 'utils' module (ca28aa6)
    - Move associated primitives into primitives module (e753938)
    - Bits, Codec, Base64 now fully no_std (f43a929)
    - Clean up lints for no-std compliance (4160ef3)
    - Clean up lints for no-std compliance (af90660)
    - Bump u32 to u64 in fmt to fix rollover in tests (194adf3)
    - Added missing readme? (4dc4b9d)
    - New easy Line ending scanners (26986c9)
    - New ReadAny and ReadEmpty, for converting random stuff into Read's (94bbd1d)
    - Module docs for arrays, assert, options (c9bb397)
    - Fix busted impl of Read in Buffer (2b00a47)
    - New f32 and f64 traits for no-std (23d81e5)
    - New 'Base64' conversions, compatible with RFC4648 (91705d8)
    - New 'Codec' trait for byte encoding conversions (7febef2)
    - Now 'no_std' capable (without the 'std' feature) (1ad20e1)
    - Now 'no_std' capable (without the 'std' feature) (a9c45bf)
</details>

## v0.4.1 (2023-12-05)

### New Features

 - <csr-id-59c9d9821ced4b102b3f6a63fbb647d201ee82b1/> new synchronization primitive 'SynchronizedOptional' like 'OnceLock', but different.
 - <csr-id-4e6c8961dc8820c39ccabc0e8283f5d50aefed2f/> new hexdump module

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 4 calendar days.
 - 6 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.4.1 (367158e)
    - New synchronization primitive 'SynchronizedOptional' like 'OnceLock', but different. (59c9d98)
    - New hexdump module (4e6c896)
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

 - 7 commits contributed to the release over the course of 14 calendar days.
 - 23 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.4.0 (6abaf62)
    - Bits now has Optional 'next' methods (9e13976)
    - Scanner now scans across block boundaries correctly (08e634e)
    - New buffer extending BufReader to span multiple blocks (0320a56)
    - Pivot to using Cargo.toml workspace lints (88ebfb5)
    - Bits and MutBits can now read & write size-prefixed blobs (0dd7845)
    - New DecimalFormatF32/F64 with ability to specify number of digits precision (965b956)
</details>

## v0.3.2 (2023-11-05)

### New Features

 - <csr-id-262121de9bc30c7501f3d1a7382a90c556137cd0/> new ANSI Color Codes

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.3.2, irox-time v0.3.0, irox-log v0.1.0, safety bump 8 crates (9c08793)
    - New ANSI Color Codes (262121d)
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
    - Release irox-tools v0.3.1 (f7eaa32)
    - Add new RetainTake for Vec and VecDeque (1f0e241)
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

 - 15 commits contributed to the release over the course of 1 calendar day.
 - 14 days passed between releases.
 - 14 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.3.0, safety bump 12 crates (eb83b27)
    - New 'ReadCounting' struct to count bytes read from a Read (991cff0)
    - Fix fmt (3793f05)
    - ALL THE LINTS. (f8ab392)
    - New array scanning utils, max_index and longest_consecutive_values (c8e6ba6)
    - New U16 utilities, FromU16Array and ToU16Array (9fc9e98)
    - New lint for unwrap_or_default() (8ea01ea)
    - Fix rustfmt (7680bf8)
    - New Identifier type to allow multi-IDs of types (edd4b81)
    - Add borrowed From's for UUID (d3e2baa)
    - MurmurHash3-128 now passes reasonable tests (fd6aa90)
    - New PRNG based on PCG-XSH-RR (ccf4aae)
    - New UUID struct (159ec4e)
    - Bits and MutBits no longer require Read & Write (cbaa8f4)
    - Bits and MutBits no longer require Read & Write (b791aca)
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

 - 7 commits contributed to the release over the course of 7 calendar days.
 - 7 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.2.2 (f49db4f)
    - Remove extra clone in scanner (b225754)
    - Update docs for rustdoc-lints (13ae74c)
    - Add collect_next_chunk method to itertools (5d0ee4c)
    - Scanner can read & return data now. (73b5397)
    - Refactor scanner to have multiple tokens (763b01e)
    - New Scanner to scan a data stream for tokens (c6b8e0f)
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

 - 6 commits contributed to the release over the course of 12 calendar days.
 - 19 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.2.1, irox-carto v0.2.1, irox-egui-extras v0.2.1, irox-gpx v0.1.0, irox-types v0.2.1, irox-structs_derive v0.2.1, irox-raymarine-sonar v0.1.0, irox-stats v0.2.1, irox-winlocation-api v0.1.1, irox v0.2.1 (68d770b)
    - Fix fmt in vec (c1fe9a1)
    - Added additional static functions to Bits (d9f3dc8)
    - UpperHex for PrettyVec and new PrettyVecDeque (d280045)
    - Partially working murmur3_128 hash fn. (c2db7b0)
    - Some new static helper functions (6465a08)
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

 - 27 commits contributed to the release over the course of 62 calendar days.
 - 26 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-enums_derive v0.2.0, irox-enums v0.2.0, irox-tools v0.2.0, irox-units v0.2.0, irox-carto v0.2.0, irox-csv v0.2.0, irox-egui-extras v0.2.0, irox-networking v0.2.0, irox-types v0.2.0, irox-influxdb_v1 v0.2.0, irox-structs_derive v0.2.0, irox-structs v0.2.0, irox-nmea0183 v0.1.0, irox-sirf v0.2.0, irox-stats v0.2.0, irox-winlocation-api v0.1.0, irox v0.2.0, safety bump 10 crates (6a72204)
    - Clean up code with additional lints (f03d8a3)
    - Update cargo.tomls to add repository (80d2b88)
    - Setting up blank changelogs for the modules (1a36533)
    - New vec module with PrettyVec (f62ae9c)
    - New collect_exact, collect_exact_or, collect_exact_or_default methods in itertools (c63cb79)
    - PacketIO no longer allows a unique error - just std::io::Error (b39d283)
    - Read_until no longer includes the delimiter in the returned result (d840941)
    - New MaybeFrom impls for primitives, MaybeMap (65ca576)
    - New 'MaybeFrom' and 'MaybeInto' traits, which are semantically equal to 'Result<T, ()>' (b175302)
    - New read_until, read_exact_into, and read_exact_into_sized functions (0eb5f64)
    - Fix clippy lint for slices (0d50f60)
    - Refactor types from tools into it's own module (ce70857)
    - Cleaning up clippy warnings (5c17856)
    - Prohibit unsafe code (c088de0)
    - New reflection/primitive types (a979b1e)
    - Add license headers (211951e)
    - Update metadata, prepare for release (49d5566)
    - Repair invalid tools Cargo.toml (e70cf86)
    - Fixing doctest (b120f68)
    - New Read utilities module (3c5a6f9)
    - PacketIO uses Bits/MutBits now + PacketData (e852ff0)
    - Add MutBits trait (fc284bb)
    - Add PacketIO module with packetization (5ddc215)
    - Test asserts (c5bfdf3)
    - Adding bits from other project (012691d)
    - Adding first set of tools, min_max and looping iterator (c128a3c)
</details>

