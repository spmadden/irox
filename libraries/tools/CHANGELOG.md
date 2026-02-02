


## v0.11.1 (2026-02-01)

### Bug Fixes

 - <csr-id-13e25487cc41d3ba530be94fa054d59ff1553398/> readding unsafe forbidden flag

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Readding unsafe forbidden flag ([`13e2548`](https://github.com/spmadden/irox/commit/13e25487cc41d3ba530be94fa054d59ff1553398))
</details>

## v0.11.0 (2026-02-01)

<csr-id-e89ac5f4fe6579db999e18db9b02f8e49668bb08/>
<csr-id-b7b95a377765d1af3c529cdf9bcf901579d74545/>
<csr-id-691776a610c7e169cdbf888f4d18892a5cf2377c/>
<csr-id-789f6fbb81e815454d18c68c310d9a8e58f969c5/>
<csr-id-84ff4e9c2fb4577b9e009f5ec6c9b40ec3a26acc/>
<csr-id-80339f6fd1474cf4182bca867517b3e9f83a98ca/>
<csr-id-f10ad9347376d5321920d23da0ebe787c3fe5150/>
<csr-id-626b51ace277f02658e47da7c4beeac66912ea40/>
<csr-id-2de510fd54646200cf214dadcd2323273bd1db94/>
<csr-id-5beb610d54822c4045d088b378604ba3caab7d10/>
<csr-id-159001f900b2ca97dd398671a85c548c90fc07c4/>
<csr-id-59f35131513a8b43d7e288832276943532e6bfee/>
<csr-id-79458eaa00099c41ce5908191fee46a52af942a2/>

### Chore

 - <csr-id-e89ac5f4fe6579db999e18db9b02f8e49668bb08/> bump version to 0.11.0
 - <csr-id-b7b95a377765d1af3c529cdf9bcf901579d74545/> fix lints & bump to 1.91
 - <csr-id-691776a610c7e169cdbf888f4d18892a5cf2377c/> fix lints & bump to 1.89
 - <csr-id-789f6fbb81e815454d18c68c310d9a8e58f969c5/> fix lints & bump to 1.87
 - <csr-id-84ff4e9c2fb4577b9e009f5ec6c9b40ec3a26acc/> fix lints & bump to 1.86
 - <csr-id-80339f6fd1474cf4182bca867517b3e9f83a98ca/> cleaning up the remainder of the feature cleanup detritus
 - <csr-id-f10ad9347376d5321920d23da0ebe787c3fe5150/> cleanup test code

### New Features

 - <csr-id-29b09c4ae1ab79ba5a1ffcb52dcced42db0e2f09/> Implementation of CORDIC
 - <csr-id-08ca5c08d742fd1c2a5e64c0a164166f084a4e6f/> Floatish gets clamp()
 - <csr-id-70d5ca91070c8981d1d59ac3ebad2343dee3b257/> lock FloatIsh to FloatExt<Self>
 - <csr-id-2ad1d74789b1668398f8cac8ff567d0a1e47b231/> add Levenshtein distance fn
 - <csr-id-7b3f53bf3c7c871bc2d0f8b1942fecbdcbb8fe1c/> new FromF64, PrimitiveMath, and FloatIsh traits
 - <csr-id-5979ef38b46c3e2e514b89d0476a9fc13d221d95/> add algorithm to HashDigest trait
 - <csr-id-012bde174864e1f8047be4b5fa29acb9441d0b2b/> new traits Cast, FloatIsh
 - <csr-id-af61b0eba0d8bfb0ea718026f22268c88d0251e3/> Scanner gets Whitespace and the ability to skip empty/repeated tokens
 - <csr-id-4b66cd71025161a4195ae1ae9ba1950ab0af4904/> BChunks - chunks for Bits
 - <csr-id-1df2807c5f34e3da0aec4532e66e939acb9e0613/> add len() and is_empty() to Buffer
 - <csr-id-fbc278e417f3f8697262a99f9b67b73bc40c67a5/> Impl Bits for FixedU8Buf
 - <csr-id-9c836149bedbba16eb21cffb8105d5ad7f20286d/> generalize Matrix for both f32 and f64
 - <csr-id-306fc7c3079c38fb33cb5052442cf595468d9a94/> add Sin and Cos to FloatExt trait
 - <csr-id-0a9869677df28963b091825801ff31f97a2b444a/> Add One and Zero traits
 - <csr-id-9c9ccbac6937da99e31e23486ffddd61215a073b/> add string float parsing from FixedU8Buf
 - <csr-id-faff2898a93fc7848c5ea99f182baca435616e3b/> matrix augmentation and splitting
 - <csr-id-698c6d00c29351bbb5158c80e64d0f89fbf9eca7/> rejigger to be more compatible with wasm32
 - <csr-id-cb129abe0ebc07cfd64dc0e9882aac6433564f6a/> additional tempdir functionality
 - <csr-id-9e31376a3aa89f60f024fe71a7ff7c3acbcfd860/> building out tempfiles
 - <csr-id-2887b672c485b972abea89f29010bb2aeebaae50/> implementing Serialize/Deserialize for UUID, UnixTimestamp, UTCDateTime
 - <csr-id-190d91082e9da4d6869d2cc23ba1ef48bb48376f/> Temp files helpers
 - <csr-id-095699e932c2c44ed2da8f845dabd9a4b72f0a55/> Random numbers without modulo-bias
 - <csr-id-03368d146a331e2b89f5915a3f0e0bf600ef9118/> easier whole-file hashing for HashAlgorithm
 - <csr-id-31c8a6152bc733ab18b1ca3dae835efeb0d5c38a/> add string convolution comparison
 - <csr-id-bb68b29e1a425d07a1656962f1109cc4311637fc/> more ansi control codes work
 - <csr-id-a0b36095d77f8b6ace1d16b1ca5646d8d1cdb86b/> add LUPDecomposition to matrix
 - <csr-id-8f155ebacc33237b2b2fef644887af8539ff2064/> cleaning up vbyte and le conversions
 - <csr-id-06dc86fd1ef69b20f936684620d58cb36a1150dd/> hashes implement Clone
 - <csr-id-c2c1a9cc40acf1adab60914261810fdeb1eca745/> new MaybeLocked enumeration for lifetime & borrowing easement
 - <csr-id-4fe2f667c655a6e19078272c1527cd6dd71fcdf9/> Hasher gets a 'hash_file' fn
 - <csr-id-500bde65794df78da584d8ffa63ff331375ae041/> adding docs to static_init!
 - <csr-id-d95374ba3a6bbfbd934c4a80510ef0faecc2242c/> new HasherCounting to track written bytes
 - <csr-id-aba995e9a914ff7abe86452623181892ad3ec755/> less copies in murmur3 for perf
 - <csr-id-781a33d7a20502a1f425d7ca8cba1d6997db0e12/> tweak FixedU8Buf for perf
 - <csr-id-ba50d4f13f7e9b84d8d59541050e229597764bf0/> new Hasher generic hash function struct
 - <csr-id-836dc742c8fd1e90d7a6902efeee2d0a6ec393a3/> impl ZeroedBuffer for Box<[T]>
 - <csr-id-027a9d4ca0f21dc2dc3c098ed4d0aac4b8fa30b9/> blake2 now actually passes all test vectors
 - <csr-id-d907fdf428dc92feab30af38868029f7162562d2/> add 'from_slice' to FixedU8Buf
 - <csr-id-78a6bcf9968385c367a02a2abcdfe0a6f65eedca/> rejigger md5 for perf
 - <csr-id-2c0ff36cf1ccbe1890d99d9613f2c1b899208ac8/> rejigger murmur3 for perf
 - <csr-id-4a8154a8efe25b1200264179bbeb53fde10492d7/> impl blake2b/blake2s
 - <csr-id-17c0da06ed7cd216b8580af9be4d55894aeeecd0/> new ArrayTools and SliceTools traits for bonus helper functions
 - <csr-id-d562dd01591c4988ad5f595282634c6b2b64e4a3/> some checks for invalid curve inputs

### Bug Fixes

 - <csr-id-765876d9dca00cd5a6f71e8b2a06223bf6b41a72/> lock pagefile and multistream behind os features
 - <csr-id-6f8080966a5e5e259d47bf556d7599f3409e8def/> fix missing box import
 - <csr-id-c81b779514e19fc9e459d8c1db3c4b3380c634a8/> fix compilation problem around alloc
 - <csr-id-380026234f60a4beaa213d711ee359c63d93e877/> fix issue in sha2 where buffer wasn't fully counted

### Other

 - <csr-id-626b51ace277f02658e47da7c4beeac66912ea40/> units & graphing
 - <csr-id-2de510fd54646200cf214dadcd2323273bd1db94/> cryptids

### Refactor

 - <csr-id-5beb610d54822c4045d088b378604ba3caab7d10/> Drop self parameter from HashDigest::algorithm
 - <csr-id-159001f900b2ca97dd398671a85c548c90fc07c4/> move eframe behind feature
 - <csr-id-59f35131513a8b43d7e288832276943532e6bfee/> hexdump read -> bits
 - <csr-id-79458eaa00099c41ce5908191fee46a52af942a2/> rejigger ArrayBuf for performance

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 61 commits contributed to the release.
 - 337 days passed between releases.
 - 60 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.11.0 ([`b0589f9`](https://github.com/spmadden/irox/commit/b0589f9eae1dcd1486d9aa817406c1e9f9516d7f))
    - Bump version to 0.11.0 ([`e89ac5f`](https://github.com/spmadden/irox/commit/e89ac5f4fe6579db999e18db9b02f8e49668bb08))
    - Implementation of CORDIC ([`29b09c4`](https://github.com/spmadden/irox/commit/29b09c4ae1ab79ba5a1ffcb52dcced42db0e2f09))
    - Floatish gets clamp() ([`08ca5c0`](https://github.com/spmadden/irox/commit/08ca5c08d742fd1c2a5e64c0a164166f084a4e6f))
    - Lock FloatIsh to FloatExt<Self> ([`70d5ca9`](https://github.com/spmadden/irox/commit/70d5ca91070c8981d1d59ac3ebad2343dee3b257))
    - Add Levenshtein distance fn ([`2ad1d74`](https://github.com/spmadden/irox/commit/2ad1d74789b1668398f8cac8ff567d0a1e47b231))
    - Drop self parameter from HashDigest::algorithm ([`5beb610`](https://github.com/spmadden/irox/commit/5beb610d54822c4045d088b378604ba3caab7d10))
    - New FromF64, PrimitiveMath, and FloatIsh traits ([`7b3f53b`](https://github.com/spmadden/irox/commit/7b3f53bf3c7c871bc2d0f8b1942fecbdcbb8fe1c))
    - Add algorithm to HashDigest trait ([`5979ef3`](https://github.com/spmadden/irox/commit/5979ef38b46c3e2e514b89d0476a9fc13d221d95))
    - New traits Cast, FloatIsh ([`012bde1`](https://github.com/spmadden/irox/commit/012bde174864e1f8047be4b5fa29acb9441d0b2b))
    - Units & graphing ([`626b51a`](https://github.com/spmadden/irox/commit/626b51ace277f02658e47da7c4beeac66912ea40))
    - Move eframe behind feature ([`159001f`](https://github.com/spmadden/irox/commit/159001f900b2ca97dd398671a85c548c90fc07c4))
    - Hexdump read -> bits ([`59f3513`](https://github.com/spmadden/irox/commit/59f35131513a8b43d7e288832276943532e6bfee))
    - Scanner gets Whitespace and the ability to skip empty/repeated tokens ([`af61b0e`](https://github.com/spmadden/irox/commit/af61b0eba0d8bfb0ea718026f22268c88d0251e3))
    - BChunks - chunks for Bits ([`4b66cd7`](https://github.com/spmadden/irox/commit/4b66cd71025161a4195ae1ae9ba1950ab0af4904))
    - Add len() and is_empty() to Buffer ([`1df2807`](https://github.com/spmadden/irox/commit/1df2807c5f34e3da0aec4532e66e939acb9e0613))
    - Impl Bits for FixedU8Buf ([`fbc278e`](https://github.com/spmadden/irox/commit/fbc278e417f3f8697262a99f9b67b73bc40c67a5))
    - Fix lints & bump to 1.91 ([`b7b95a3`](https://github.com/spmadden/irox/commit/b7b95a377765d1af3c529cdf9bcf901579d74545))
    - Fix lints & bump to 1.89 ([`691776a`](https://github.com/spmadden/irox/commit/691776a610c7e169cdbf888f4d18892a5cf2377c))
    - Fix lints & bump to 1.87 ([`789f6fb`](https://github.com/spmadden/irox/commit/789f6fbb81e815454d18c68c310d9a8e58f969c5))
    - Fix lints & bump to 1.86 ([`84ff4e9`](https://github.com/spmadden/irox/commit/84ff4e9c2fb4577b9e009f5ec6c9b40ec3a26acc))
    - Generalize Matrix for both f32 and f64 ([`9c83614`](https://github.com/spmadden/irox/commit/9c836149bedbba16eb21cffb8105d5ad7f20286d))
    - Add Sin and Cos to FloatExt trait ([`306fc7c`](https://github.com/spmadden/irox/commit/306fc7c3079c38fb33cb5052442cf595468d9a94))
    - Add One and Zero traits ([`0a98696`](https://github.com/spmadden/irox/commit/0a9869677df28963b091825801ff31f97a2b444a))
    - Add string float parsing from FixedU8Buf ([`9c9ccba`](https://github.com/spmadden/irox/commit/9c9ccbac6937da99e31e23486ffddd61215a073b))
    - Matrix augmentation and splitting ([`faff289`](https://github.com/spmadden/irox/commit/faff2898a93fc7848c5ea99f182baca435616e3b))
    - Rejigger to be more compatible with wasm32 ([`698c6d0`](https://github.com/spmadden/irox/commit/698c6d00c29351bbb5158c80e64d0f89fbf9eca7))
    - Lock pagefile and multistream behind os features ([`765876d`](https://github.com/spmadden/irox/commit/765876d9dca00cd5a6f71e8b2a06223bf6b41a72))
    - Additional tempdir functionality ([`cb129ab`](https://github.com/spmadden/irox/commit/cb129abe0ebc07cfd64dc0e9882aac6433564f6a))
    - Building out tempfiles ([`9e31376`](https://github.com/spmadden/irox/commit/9e31376a3aa89f60f024fe71a7ff7c3acbcfd860))
    - Cleaning up the remainder of the feature cleanup detritus ([`80339f6`](https://github.com/spmadden/irox/commit/80339f6fd1474cf4182bca867517b3e9f83a98ca))
    - Implementing Serialize/Deserialize for UUID, UnixTimestamp, UTCDateTime ([`2887b67`](https://github.com/spmadden/irox/commit/2887b672c485b972abea89f29010bb2aeebaae50))
    - Temp files helpers ([`190d910`](https://github.com/spmadden/irox/commit/190d91082e9da4d6869d2cc23ba1ef48bb48376f))
    - Random numbers without modulo-bias ([`095699e`](https://github.com/spmadden/irox/commit/095699e932c2c44ed2da8f845dabd9a4b72f0a55))
    - Easier whole-file hashing for HashAlgorithm ([`03368d1`](https://github.com/spmadden/irox/commit/03368d146a331e2b89f5915a3f0e0bf600ef9118))
    - Add string convolution comparison ([`31c8a61`](https://github.com/spmadden/irox/commit/31c8a6152bc733ab18b1ca3dae835efeb0d5c38a))
    - More ansi control codes work ([`bb68b29`](https://github.com/spmadden/irox/commit/bb68b29e1a425d07a1656962f1109cc4311637fc))
    - Cleanup test code ([`f10ad93`](https://github.com/spmadden/irox/commit/f10ad9347376d5321920d23da0ebe787c3fe5150))
    - Add LUPDecomposition to matrix ([`a0b3609`](https://github.com/spmadden/irox/commit/a0b36095d77f8b6ace1d16b1ca5646d8d1cdb86b))
    - Cleaning up vbyte and le conversions ([`8f155eb`](https://github.com/spmadden/irox/commit/8f155ebacc33237b2b2fef644887af8539ff2064))
    - Hashes implement Clone ([`06dc86f`](https://github.com/spmadden/irox/commit/06dc86fd1ef69b20f936684620d58cb36a1150dd))
    - New MaybeLocked enumeration for lifetime & borrowing easement ([`c2c1a9c`](https://github.com/spmadden/irox/commit/c2c1a9cc40acf1adab60914261810fdeb1eca745))
    - Hasher gets a 'hash_file' fn ([`4fe2f66`](https://github.com/spmadden/irox/commit/4fe2f667c655a6e19078272c1527cd6dd71fcdf9))
    - Adding docs to static_init! ([`500bde6`](https://github.com/spmadden/irox/commit/500bde65794df78da584d8ffa63ff331375ae041))
    - Fix missing box import ([`6f80809`](https://github.com/spmadden/irox/commit/6f8080966a5e5e259d47bf556d7599f3409e8def))
    - New HasherCounting to track written bytes ([`d95374b`](https://github.com/spmadden/irox/commit/d95374ba3a6bbfbd934c4a80510ef0faecc2242c))
    - Less copies in murmur3 for perf ([`aba995e`](https://github.com/spmadden/irox/commit/aba995e9a914ff7abe86452623181892ad3ec755))
    - Tweak FixedU8Buf for perf ([`781a33d`](https://github.com/spmadden/irox/commit/781a33d7a20502a1f425d7ca8cba1d6997db0e12))
    - New Hasher generic hash function struct ([`ba50d4f`](https://github.com/spmadden/irox/commit/ba50d4f13f7e9b84d8d59541050e229597764bf0))
    - Impl ZeroedBuffer for Box<[T]> ([`836dc74`](https://github.com/spmadden/irox/commit/836dc742c8fd1e90d7a6902efeee2d0a6ec393a3))
    - Blake2 now actually passes all test vectors ([`027a9d4`](https://github.com/spmadden/irox/commit/027a9d4ca0f21dc2dc3c098ed4d0aac4b8fa30b9))
    - Add 'from_slice' to FixedU8Buf ([`d907fdf`](https://github.com/spmadden/irox/commit/d907fdf428dc92feab30af38868029f7162562d2))
    - Rejigger md5 for perf ([`78a6bcf`](https://github.com/spmadden/irox/commit/78a6bcf9968385c367a02a2abcdfe0a6f65eedca))
    - Rejigger murmur3 for perf ([`2c0ff36`](https://github.com/spmadden/irox/commit/2c0ff36cf1ccbe1890d99d9613f2c1b899208ac8))
    - Rejigger ArrayBuf for performance ([`79458ea`](https://github.com/spmadden/irox/commit/79458eaa00099c41ce5908191fee46a52af942a2))
    - Impl blake2b/blake2s ([`4a8154a`](https://github.com/spmadden/irox/commit/4a8154a8efe25b1200264179bbeb53fde10492d7))
    - Fix compilation problem around alloc ([`c81b779`](https://github.com/spmadden/irox/commit/c81b779514e19fc9e459d8c1db3c4b3380c634a8))
    - Fix issue in sha2 where buffer wasn't fully counted ([`3800262`](https://github.com/spmadden/irox/commit/380026234f60a4beaa213d711ee359c63d93e877))
    - New ArrayTools and SliceTools traits for bonus helper functions ([`17c0da0`](https://github.com/spmadden/irox/commit/17c0da06ed7cd216b8580af9be4d55894aeeecd0))
    - Some checks for invalid curve inputs ([`d562dd0`](https://github.com/spmadden/irox/commit/d562dd01591c4988ad5f595282634c6b2b64e4a3))
    - Cryptids ([`2de510f`](https://github.com/spmadden/irox/commit/2de510fd54646200cf214dadcd2323273bd1db94))
</details>

## v0.10.3 (2025-03-01)

<csr-id-fef64a162b568961be8445ec418918d64f3cadde/>
<csr-id-dfe010053f41bf0531816c19c3229ce5a7e90a1c/>
<csr-id-1f48b70c64fb0cde031bf379fe3d6b5b276b6f51/>
<csr-id-de5e67fb00da4d87ac75adb7592f4848ba2399b2/>

### Chore

 - <csr-id-fef64a162b568961be8445ec418918d64f3cadde/> fixing lints
 - <csr-id-dfe010053f41bf0531816c19c3229ce5a7e90a1c/> cleanup remaining misc lints
 - <csr-id-1f48b70c64fb0cde031bf379fe3d6b5b276b6f51/> cleanup bitmask order-of-operations
 - <csr-id-de5e67fb00da4d87ac75adb7592f4848ba2399b2/> elude all the lifetimes!

### New Features

 - <csr-id-47084c92d924079e50d8b020d411ece48cda76f9/> better iteration for UnlimitedBuffer
 - <csr-id-2794f0f8209f681621fb462accc30ec8ae65ecc9/> Add Murmur3_32 implementation and hashing benchmarks
   This commit introduces the Murmur3_32 struct and its associated functionality. It also refactors parts of the Murmur3_128 implementation and integrates both into benchmarking tests. Additionally, new benchmarks were added to evaluate Murmur3 and SHA hashing performance.
 - <csr-id-cae124cb47519325cbd8ee1938faf5b8867c532d/> Add `copy_subset` function for copying slices into statically sized arrays
   Introduce `copy_subset`, a utility function to copy elements from a slice into a fixed-size array. This function leverages `copy_from_slice` and enforces bounds, panicking if the slice is too short.
 - <csr-id-acff02b726f6e03e142d6bca5825d4fc15a82a5d/> Add `take` method to FixedU8Buf to return and consume the inner buffer
   The new `take` method allows users to retrieve the inner buffer while consuming the object. This provides a cleaner way to extract the buffer content when the object is no longer needed.
 - <csr-id-f9f52242a4229873e44d154e9e35e25b10b212bb/> Add `try_from_hex_str` function to parse hex into static arrays
   This new function attempts to parse a hex string into a fixed-size static array buffer, enhancing usability for scenarios requiring precise buffer fits. Updated imports and minor adjustments ensure compatibility and improved error handling.
 - <csr-id-4a7a1aad86732249eb3de36cb42b1a3f44225e3d/> Add `Zipping` and `Windows` iterator utilities
   Introduce flexible `Zipping` for combining multiple iterators and `Windows` for sliding window iteration over slices. These utilities enhance iterator capabilities with support for constant generic sizes and comprehensive test coverage.
 - <csr-id-f2dfce6798354251ef30cd6500feaae729326c89/> Add iterator support to RoundBuffer and update iterators
   Introduce an `Iter` struct and associated methods for iterating over `RoundBuffer`. Enhance `FixedBufIterMut` and `FixedU8BufIterMut` with updated lifetimes for iterator methods. These changes improve the usability and flexibility of buffer structures.
 - <csr-id-80f28233580410443440763d577b92b2a590746e/> Refactor LendingIterator and add Moderator iterator
   Updated the LendingIterator trait to simplify lifetimes, improving clarity and flexibility. Introduced a new Moderator iterator with modular arithmetic and optional limits, including tests to validate its functionality.
 - <csr-id-ed10f3e02cf7bb2e432c25067be2cee58a57778e/> Add `array!` macro for array construction
   This new macro simplifies the creation of arrays with repeated elements. It supports various sizes using recursive accumulation logic, improving code clarity and reducing repetitive boilerplate.
 - <csr-id-f313e1a342d03f422c9dfbd625e380b8a7885dc3/> Add `cfg_docs!` macro for documentation-specific items
   This macro simplifies conditional compilation for items only relevant during documentation generation. It ensures compatibility with `doc` and `docsrs` configurations, streamlining code targeting documentation builds.

### Bug Fixes

 - <csr-id-a0d60b0b5ff1c1414b77ed3bee43af75c5663858/> Fix type casting in zigzag_impl to ensure correct output
   The type casting was moved to the final result of the computation to avoid unintended behavior. This ensures that the operation produces the correct type and improves overall code safety.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 14 calendar days.
 - 17 days passed between releases.
 - 15 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.10.3 ([`f6c9070`](https://github.com/spmadden/irox/commit/f6c9070254c2494cebd3d3dc7d4572f303a8b969))
    - Better iteration for UnlimitedBuffer ([`47084c9`](https://github.com/spmadden/irox/commit/47084c92d924079e50d8b020d411ece48cda76f9))
    - Add Murmur3_32 implementation and hashing benchmarks ([`2794f0f`](https://github.com/spmadden/irox/commit/2794f0f8209f681621fb462accc30ec8ae65ecc9))
    - Add `copy_subset` function for copying slices into statically sized arrays ([`cae124c`](https://github.com/spmadden/irox/commit/cae124cb47519325cbd8ee1938faf5b8867c532d))
    - Add `take` method to FixedU8Buf to return and consume the inner buffer ([`acff02b`](https://github.com/spmadden/irox/commit/acff02b726f6e03e142d6bca5825d4fc15a82a5d))
    - Add `try_from_hex_str` function to parse hex into static arrays ([`f9f5224`](https://github.com/spmadden/irox/commit/f9f52242a4229873e44d154e9e35e25b10b212bb))
    - Fixing lints ([`fef64a1`](https://github.com/spmadden/irox/commit/fef64a162b568961be8445ec418918d64f3cadde))
    - Add `Zipping` and `Windows` iterator utilities ([`4a7a1aa`](https://github.com/spmadden/irox/commit/4a7a1aad86732249eb3de36cb42b1a3f44225e3d))
    - Add iterator support to RoundBuffer and update iterators ([`f2dfce6`](https://github.com/spmadden/irox/commit/f2dfce6798354251ef30cd6500feaae729326c89))
    - Refactor LendingIterator and add Moderator iterator ([`80f2823`](https://github.com/spmadden/irox/commit/80f28233580410443440763d577b92b2a590746e))
    - Add `array!` macro for array construction ([`ed10f3e`](https://github.com/spmadden/irox/commit/ed10f3e02cf7bb2e432c25067be2cee58a57778e))
    - Cleanup remaining misc lints ([`dfe0100`](https://github.com/spmadden/irox/commit/dfe010053f41bf0531816c19c3229ce5a7e90a1c))
    - Cleanup bitmask order-of-operations ([`1f48b70`](https://github.com/spmadden/irox/commit/1f48b70c64fb0cde031bf379fe3d6b5b276b6f51))
    - Elude all the lifetimes! ([`de5e67f`](https://github.com/spmadden/irox/commit/de5e67fb00da4d87ac75adb7592f4848ba2399b2))
    - Add `cfg_docs!` macro for documentation-specific items ([`f313e1a`](https://github.com/spmadden/irox/commit/f313e1a342d03f422c9dfbd625e380b8a7885dc3))
    - Fix type casting in zigzag_impl to ensure correct output ([`a0d60b0`](https://github.com/spmadden/irox/commit/a0d60b0b5ff1c1414b77ed3bee43af75c5663858))
</details>

## v0.10.2 (2025-02-12)

<csr-id-6e72be2bb9d1b96561fadd46e5190c80f223a694/>
<csr-id-b3c08ca4dc5c6a20c9661248b7c42a1a7dfceccd/>

### Chore

 - <csr-id-6e72be2bb9d1b96561fadd46e5190c80f223a694/> cleanup some lints

### New Features

 - <csr-id-e3b6d01e1c085303d86114c996690e6be3666fe1/> Add `StrWrapper` abstraction for flexible string handling
   Introduce `StrWrapper` that supports shared, owned, and borrowed string representations. It provides conversion methods, trait implementations, and mutability features for efficient and flexible string manipulation. Includes integration with bit operations for serialization and deserialization.
 - <csr-id-1e197c5b019b398bd1570429d225d2bf91018a0b/> Add `ToU64` trait and implement it for common types.
   Introduced a `ToU64` trait to allow conversion of various types to `u64`. Implemented the trait for several primitive types, including integers, floats, `bool`, and `char`. This provides a unified interface for such conversions.
 - <csr-id-cd01a72ea62b17ad72c22a05d7ae975486f520cc/> Refactor `Copy` to `Clone` for improved trait flexibility
   Replaced the `Copy` trait with `Clone` in several structs and methods to better handle non-trivial types. This change ensures consistency and allows for more robust usage of `CodeDictionary` and related implementations. Updated all necessary logic to work with `Clone` accordingly.
 - <csr-id-b719387d6e52f1665eb2cde643d95957aaccae3c/> add basic hash visualization
 - <csr-id-077cd4518e68c35659e5d3eaef7bb607f47dda6c/> add basic base conversions
 - <csr-id-03c7ac5d1977ea18095d974737db55765410f07e/> add reverse method to FixedU8Buf
 - <csr-id-c7a8c574cdd6635ef242cc44792210a08e9564a9/> add add_str methods to FixedU8Buf

### Refactor

 - <csr-id-b3c08ca4dc5c6a20c9661248b7c42a1a7dfceccd/> move sixwords to hashes

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 1 calendar day.
 - 7 days passed between releases.
 - 9 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.10.2 ([`7e94f76`](https://github.com/spmadden/irox/commit/7e94f76451914e7bedc51f5ebb769b8837865699))
    - Add `StrWrapper` abstraction for flexible string handling ([`e3b6d01`](https://github.com/spmadden/irox/commit/e3b6d01e1c085303d86114c996690e6be3666fe1))
    - Add `ToU64` trait and implement it for common types. ([`1e197c5`](https://github.com/spmadden/irox/commit/1e197c5b019b398bd1570429d225d2bf91018a0b))
    - Refactor `Copy` to `Clone` for improved trait flexibility ([`cd01a72`](https://github.com/spmadden/irox/commit/cd01a72ea62b17ad72c22a05d7ae975486f520cc))
    - Cleanup some lints ([`6e72be2`](https://github.com/spmadden/irox/commit/6e72be2bb9d1b96561fadd46e5190c80f223a694))
    - Add basic hash visualization ([`b719387`](https://github.com/spmadden/irox/commit/b719387d6e52f1665eb2cde643d95957aaccae3c))
    - Add basic base conversions ([`077cd45`](https://github.com/spmadden/irox/commit/077cd4518e68c35659e5d3eaef7bb607f47dda6c))
    - Add reverse method to FixedU8Buf ([`03c7ac5`](https://github.com/spmadden/irox/commit/03c7ac5d1977ea18095d974737db55765410f07e))
    - Add add_str methods to FixedU8Buf ([`c7a8c57`](https://github.com/spmadden/irox/commit/c7a8c574cdd6635ef242cc44792210a08e9564a9))
    - Move sixwords to hashes ([`b3c08ca`](https://github.com/spmadden/irox/commit/b3c08ca4dc5c6a20c9661248b7c42a1a7dfceccd))
</details>

## v0.10.1 (2025-02-05)

### New Features

 - <csr-id-d35ec29c046a31bc40e6365f07620d985db2c2e9/> adding sixwords encoding/decoding

### Bug Fixes

 - <csr-id-f3b8c0c58e3ea34c2ff073555efd7929ae84ab87/> fix noalloc in tools

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 2 calendar days.
 - 10 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release irox-tools v0.10.1 ([`5393a2c`](https://github.com/spmadden/irox/commit/5393a2c8e457b4db8763f1069a5c02b40a7e4c7c))
    - Adding sixwords encoding/decoding ([`d35ec29`](https://github.com/spmadden/irox/commit/d35ec29c046a31bc40e6365f07620d985db2c2e9))
    - Fix noalloc in tools ([`f3b8c0c`](https://github.com/spmadden/irox/commit/f3b8c0c58e3ea34c2ff073555efd7929ae84ab87))
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
    - Release irox-tools v0.10.0 ([`06629ef`](https://github.com/spmadden/irox/commit/06629ef062edd1cfe60a0ba1d8b0783a6c43e454))
    - Fix multi_stream on linux, seek_write doesn't modify the underlying position of the FP like it does on windows. ([`a7d97e0`](https://github.com/spmadden/irox/commit/a7d97e08afba83769389842156c78e9792037e66))
    - Fix edge case issues in multi_stream around the end of a stream. ([`507a83c`](https://github.com/spmadden/irox/commit/507a83ccd5831745d983fcbddf6a9da9f83df9c5))
    - Impl new,debug,clone in RoundBuffer ([`b71c23c`](https://github.com/spmadden/irox/commit/b71c23c77672f7efcb500268167874ec9f0403d3))
    - Switch to RoundBuf in UnlimitedBuf for performance ([`2e05702`](https://github.com/spmadden/irox/commit/2e05702b0f90563b76be1caa2f0666d90158dbdd))
    - Fix off-by-one in roundu8 buffer limiting ([`f9a5bdd`](https://github.com/spmadden/irox/commit/f9a5bdd6dd5f9b06a48b8fa0d1e46986cc2d1aa1))
    - HMAC-SHA2 with test vectors ([`79883f4`](https://github.com/spmadden/irox/commit/79883f420af37979cf0b19d70d7960df7b358703))
    - Fix off-by-one where key was same length as block size. ([`4096723`](https://github.com/spmadden/irox/commit/4096723f47280285db82c2ce82afc21c23cfcb95))
    - Some asserts can now take test name/message as last parameter ([`3355368`](https://github.com/spmadden/irox/commit/33553680fe493ec285286310c2bac2a29d60b90d))
    - Adding sha1 test vectors ([`80726de`](https://github.com/spmadden/irox/commit/80726de6cd543fbad5faa4bb182f534bb231a078))
    - Fix impls of sha2/384/512 and add NIST test vectors ([`b607383`](https://github.com/spmadden/irox/commit/b60738301ce6d50bcd91b40bf8da7225ad3ff1f2))
    - Fixup nostd support in carto ([`8270cef`](https://github.com/spmadden/irox/commit/8270cefc0903fafcf39d8b206a250403df3c79e9))
    - New Matrix::transpose() operation ([`5458021`](https://github.com/spmadden/irox/commit/54580210d482718c4e6a876b0081485eca0d5b44))
    - Silence warning about unused trait in Matrix ([`f327a89`](https://github.com/spmadden/irox/commit/f327a89b1f398e1552758b0a9bd49506bcabbd90))
    - Tests for rotating matrices ([`d0c3882`](https://github.com/spmadden/irox/commit/d0c38822ffb13c9a7fb300a402161a43e339816c))
    - Matrix rotations behind feature std for sin/cos ([`dbf8ef8`](https://github.com/spmadden/irox/commit/dbf8ef898ba6c089582caf3d6d7fe7f8f166e845))
    - Fix test assumption the buffer wasn't cleared. ([`e7bba1e`](https://github.com/spmadden/irox/commit/e7bba1e8df7cffe0bf8a30addee93218500c3a34))
    - New ZeroedBuffer trait to create empty buffers ([`904ba25`](https://github.com/spmadden/irox/commit/904ba2577ddd001a459326fb2741436f8dfab923))
    - Zero out any part of RoundU8Buffer when consumed ([`8b816e9`](https://github.com/spmadden/irox/commit/8b816e9cdc2228c9de03e406dd4b1e8454c030db))
    - Yaaay matrix transforms :> ([`7bf897d`](https://github.com/spmadden/irox/commit/7bf897d6381c453936022d5ea985cd5e3fcc4930))
    - New negative_one fn in ToSigned to allow easy inversion ([`6321e4e`](https://github.com/spmadden/irox/commit/6321e4e643eca9541801dee69bd19a8f66e63bdc))
    - New OrderedHashMap that replicates elements in insertion order. ([`223ce68`](https://github.com/spmadden/irox/commit/223ce686b44ef9bf39fa83ea06c02bfbe5d814b4))
    - New ZagZig impls, fix alloc in vbyte ([`be2700b`](https://github.com/spmadden/irox/commit/be2700b42dba1c7e88a7db6d131d17e26bde840d))
    - Impl BufBits for StreamReader ([`fd99495`](https://github.com/spmadden/irox/commit/fd99495373be8ef4adda57cbece5e0dcd2f53f20))
    - New ToSigned and ToUnsigned traits for the primitives ([`e3caf23`](https://github.com/spmadden/irox/commit/e3caf23f1b80eafdb449e92ce8d1fda89afa91b3))
    - Refactoring StrBuf into FixedU8Buf ([`e73848b`](https://github.com/spmadden/irox/commit/e73848b677f582ad68a22cd92afb0d0fd4cdb1c5))
    - Fix alloc feature build in tools ([`ee5face`](https://github.com/spmadden/irox/commit/ee5face00fb5d16bf1a23b9fff50852b1eafacf2))
    - Add an atomic counter to CodeDictionary for stats ([`fdbf627`](https://github.com/spmadden/irox/commit/fdbf627db9f5614d9b92a135812dc6c05adf85ad))
    - New shared version of the code dictionary and encoder ([`616ab4d`](https://github.com/spmadden/irox/commit/616ab4de9cadc5c0caf2ae381e42c12d9d70cefa))
    - New GroupVarintCodeDecoder ([`15873e5`](https://github.com/spmadden/irox/commit/15873e543c1009ee5b863f3e058ca9a35a742847))
    - Impl From<[u8;N]> for RoundU8Buffer ([`ef314f4`](https://github.com/spmadden/irox/commit/ef314f4261f6f8f84a6571b0d844801c045ffc7f))
    - New CodeDictionary and GroupVarintCodeEncoder capabilities ([`0a8a6c6`](https://github.com/spmadden/irox/commit/0a8a6c62ebfb1ea2228fc6317af920ca9fb7c3a3))
    - New varint encoding capability ([`57d04be`](https://github.com/spmadden/irox/commit/57d04be3a4becf00779eaed39351b8c68bcd2ee1))
    - New vbyte predictive length functions ([`a7e15d7`](https://github.com/spmadden/irox/commit/a7e15d7475435f05e57fb13310ed1bb0d4f88ef2))
    - Impl WriteToBEBits for IntegerValue ([`d1e54a5`](https://github.com/spmadden/irox/commit/d1e54a55a16da755bb2c3b9d420e3bd0b447c08f))
    - Further flushing out of Fixed & Round u8 buffers ([`247c322`](https://github.com/spmadden/irox/commit/247c3228f154f10eb36a6bd96b1ec14c3965da71))
    - Return usize instead of () in WriteToBEBits ([`cf18819`](https://github.com/spmadden/irox/commit/cf18819735eecc7e8512ec587f59fcbed385d712))
    - New CountingBits struct, move SharedROCounter to bits from tools ([`e34948d`](https://github.com/spmadden/irox/commit/e34948deae97e45b1da777dcc616eee983cf6487))
    - Better buffer usage in the RoundU8Buffer, nearly usable now for reading & writing. ([`138fe85`](https://github.com/spmadden/irox/commit/138fe8565dc368755498c2f1426779e227cbc54a))
    - New basic matrix implementation ([`83a5f50`](https://github.com/spmadden/irox/commit/83a5f506b8a49930e8d18d07cc9dd5f311d0db8d))
    - New debug_assert_eq_eps assert ([`cd5e251`](https://github.com/spmadden/irox/commit/cd5e251008bbd88c72c7bd56793cf72cd85a503a))
    - Fixup lints ([`73c770e`](https://github.com/spmadden/irox/commit/73c770e4c24a06ebd1938f68027b32354a8a6c29))
    - New functions in PRNG to make generating spreads of numbers easier ([`c459c43`](https://github.com/spmadden/irox/commit/c459c43514fd3d240686ddd23ecd3645883b8f95))
    - New bencher to check performance of atomics on a hardware config ([`33bee42`](https://github.com/spmadden/irox/commit/33bee42679e32d4b1e2e48bc18a072dcce881a51))
    - New thread-safe performant one-way non-blocking data exchanger ([`fbd4a63`](https://github.com/spmadden/irox/commit/fbd4a63b40a6d1955e446a0480ea183a01e7ada9))
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
    - Release irox-tools v0.9.3 ([`7df8024`](https://github.com/spmadden/irox/commit/7df8024c2166824c4b9d4f85d780c923192f3166))
    - New assert_eq_eps_slice! macro for testing slices of floating points to within epsilon values ([`d72868b`](https://github.com/spmadden/irox/commit/d72868b00c3b8ae018badf809dce12584a1186a5))
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
    - Release irox-tools v0.9.2 ([`219aa3a`](https://github.com/spmadden/irox/commit/219aa3ac7b7265fbf636a9d4d25d475a86d5122a))
    - New 'ToF64' trait ([`0ac28bf`](https://github.com/spmadden/irox/commit/0ac28bf47dd59480b3c7b840f2876d7c44bbcd0f))
    - New WrappingAdd/Sub/Mul for the primitives. ([`55d1a9a`](https://github.com/spmadden/irox/commit/55d1a9ad0e2d94780baf7c85ff9b43b93c2da0db))
    - New FixedU8Buf with optimizations for a straight u8 ([`a02debb`](https://github.com/spmadden/irox/commit/a02debb360ce750c741a940eac1fedb25a4900e0))
    - Re-pull in the 'once' module behind the std feature flag ([`a85828d`](https://github.com/spmadden/irox/commit/a85828d283e6a0a2528140562bfe94ad6776aeaa))
    - Fix assumption that error types are called 'Error' ([`7893feb`](https://github.com/spmadden/irox/commit/7893feb3e314ef7b6f8dc61d42edf07bf86f7a2d))
    - Bump MSRV to 1.82 ([`79b4c01`](https://github.com/spmadden/irox/commit/79b4c0111cfb4daff7419dda335fca312e4afa4e))
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
    - Release irox-tools v0.9.1 ([`233bb8f`](https://github.com/spmadden/irox/commit/233bb8f8eefe006b06264940f5a5493118a373dc))
    - Add hexarray function and nibble_to_hex_char ([`f4f2cb5`](https://github.com/spmadden/irox/commit/f4f2cb5289f2cb311f583a2af1af10699c20ef6b))
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
    - Release irox-tools v0.9.0 ([`7e9a935`](https://github.com/spmadden/irox/commit/7e9a935d9d1060f712e6ec2c5cacff048cbb1036))
    - Fix issue where MultiStream would return garbage if the stream didn't end on a block boundary ([`93fb4fb`](https://github.com/spmadden/irox/commit/93fb4fb43b97cfb6bdc6d3258862a3a2fdb3b125))
    - Add 'limit' capability to RoundU8Buffer to artificially limit the output ([`c3fcecc`](https://github.com/spmadden/irox/commit/c3fceccc71f1617c4376c684029e89f2da4c4630))
    - Fix issue in RoundU8Buffer where the head pointer wouldn't increment if the buffer was full ([`6e48657`](https://github.com/spmadden/irox/commit/6e48657b59c217477e65bc2066ac87b08df7ef1c))
    - MultiStreamWriter returns an Arc, add len and is_empty ([`a2e3838`](https://github.com/spmadden/irox/commit/a2e38381e8de3faa2bd3ea29d7d0f8c2f20587d3))
    - Impl bits & mutbits for unlimited buffer, fix lengths & push_back ([`7bf1542`](https://github.com/spmadden/irox/commit/7bf1542894179a6005a28819032adf21f458ee5e))
    - Impl signed vbyte/zigzag encoding ([`09104c6`](https://github.com/spmadden/irox/commit/09104c6821dcbaf5d1f02376b54e5c50f78ad979))
    - Add 'to_bits', 'exponent', and 'significand' to FloatExt trait ([`aa7909f`](https://github.com/spmadden/irox/commit/aa7909f619e332d37cb2b6099b3bba2be52fccc4))
    - Breaking: refactor BitsWrapper to have an owned and borrowed variant. ([`d256059`](https://github.com/spmadden/irox/commit/d256059f37bcfc75dc8ba556e35343cb3cb18add))
    - Impl EncodeVByteTo for u8 ([`fb0ed97`](https://github.com/spmadden/irox/commit/fb0ed97e95a61348d3b72915f3404ca99ca27a39))
    - Random now has a fill() method ([`897fc63`](https://github.com/spmadden/irox/commit/897fc6368878d09d9cc91b752cc1a6f6b318ac8b))
    - New trait EncodeVByteTo for writing encoded VBytes (varints) to MutBits impls ([`b2d313d`](https://github.com/spmadden/irox/commit/b2d313d3dbf852a2c7ca635065fc5bc3495e9de3))
    - FixedBuf gets an iterator and an impl of WriteToBEBits ([`1c09689`](https://github.com/spmadden/irox/commit/1c0968904aae747527b9c8d5c17f76ba35db3b4d))
    - Multi-Stream Reader impls. ([`c3c4ade`](https://github.com/spmadden/irox/commit/c3c4adea1d8d876620b8f38e72f00be253448bac))
    - Decode vbytes directly from Bits impls. ([`cb9d4e6`](https://github.com/spmadden/irox/commit/cb9d4e6e60f90eb2b6ff9db76e13ece93e548c7d))
    - New Round Buffer optimized for u8's ([`5075479`](https://github.com/spmadden/irox/commit/5075479018db38b8944a0d554d88154c767fdb7a))
    - New Multi-Stream File writer ([`5fb2f75`](https://github.com/spmadden/irox/commit/5fb2f75cbee4960b86829e0f7493dc977847a07f))
    - Vbyte encode_u128 uses same encoding method as generally accepted. ([`2aa6728`](https://github.com/spmadden/irox/commit/2aa6728b1789faf420f7c9bc4d9ac3bc666156a5))
    - Buffer gets new 'is_full' method ([`d19bfdc`](https://github.com/spmadden/irox/commit/d19bfdc570ea6010e2ed446cc135d551f33bef03))
    - FixedBuf gets MutBits and LendingIterator support. ([`6e808d6`](https://github.com/spmadden/irox/commit/6e808d638afbca28a5fc2daea955aa1b55bfdff2))
    - Lending iterator trait ([`5488bb2`](https://github.com/spmadden/irox/commit/5488bb2fd5ef691e38bfc0cbaa7ff5eae2a635f8))
    - More work on multi-streams ([`0301108`](https://github.com/spmadden/irox/commit/03011089653d97ae0c6ec0624aec627285d3c19a))
    - Fix garbage constants in random generators ([`b1542db`](https://github.com/spmadden/irox/commit/b1542dbc8f81c8dd486d71b24b4b5b11dafa9a32))
    - Pivot vbyte to use fixedbuf internally ([`5eaec49`](https://github.com/spmadden/irox/commit/5eaec4960399ffa235f2b720b4efd49d975fb91c))
    - Better vbyte impl ([`9c38e72`](https://github.com/spmadden/irox/commit/9c38e72895be6a00382ad1f8c42bbffb1a51759e))
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
    - Release irox-tools v0.8.6 ([`1c99a16`](https://github.com/spmadden/irox/commit/1c99a1610eb84fb7127a28f34628b2dec5045415))
    - New SyncFlag signalling ([`57bccdb`](https://github.com/spmadden/irox/commit/57bccdba5ee3921ef359ca35953d6c6bd8929a92))
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
    - Release irox-tools v0.8.5 ([`3261257`](https://github.com/spmadden/irox/commit/3261257312d17a0c0703550f5dcaa538f153a11f))
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

 - 2 commits contributed to the release over the course of 11 calendar days.
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

 - 15 commits contributed to the release over the course of 33 calendar days.
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

 - 9 commits contributed to the release over the course of 13 calendar days.
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

 - 12 commits contributed to the release over the course of 2 calendar days.
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

 - 7 commits contributed to the release over the course of 35 calendar days.
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

 - 25 commits contributed to the release over the course of 8 calendar days.
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

 - 3 commits contributed to the release over the course of 4 calendar days.
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

 - 7 commits contributed to the release over the course of 14 calendar days.
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

 - 15 commits contributed to the release over the course of 1 calendar day.
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

 - 7 commits contributed to the release over the course of 7 calendar days.
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

 - 6 commits contributed to the release over the course of 12 calendar days.
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

 - 27 commits contributed to the release over the course of 62 calendar days.
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

