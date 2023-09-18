

## v0.2.0 (2023-09-18)

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

 - 26 commits contributed to the release over the course of 60 calendar days.
 - 26 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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

