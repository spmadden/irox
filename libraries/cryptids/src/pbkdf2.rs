// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use core::marker::PhantomData;
use irox_bits::MutBits;
use irox_tools::buf::FixedU8Buf;
use irox_tools::hash::{HashDigest, HMAC, SHA1, SHA224, SHA256, SHA384, SHA512};

#[allow(non_camel_case_types)]
pub type PBKDF_HMAC_SHA1<const ITER: usize, const OUTPUT_SIZE: usize> =
    PBKDF2<ITER, OUTPUT_SIZE, 64, 20, SHA1>;
#[allow(non_camel_case_types)]
pub type PBKDF_HMAC_SHA224<const ITER: usize, const OUTPUT_SIZE: usize> =
    PBKDF2<ITER, OUTPUT_SIZE, 64, 28, SHA224>;
#[allow(non_camel_case_types)]
pub type PBKDF_HMAC_SHA256<const ITER: usize, const OUTPUT_SIZE: usize> =
    PBKDF2<ITER, OUTPUT_SIZE, 64, 32, SHA256>;
#[allow(non_camel_case_types)]
pub type PBKDF_HMAC_SHA384<const ITER: usize, const OUTPUT_SIZE: usize> =
    PBKDF2<ITER, OUTPUT_SIZE, 128, 48, SHA384>;
#[allow(non_camel_case_types)]
pub type PBKDF_HMAC_SHA512<const ITER: usize, const OUTPUT_SIZE: usize> =
    PBKDF2<ITER, OUTPUT_SIZE, 128, 64, SHA512>;

pub struct PBKDF2<
    const ITER: usize,
    const OUTPUT_LEN: usize,
    const BLOCK_LEN: usize,
    const DIGEST_LEN: usize,
    T: HashDigest<BLOCK_LEN, DIGEST_LEN>,
> {
    _phan: PhantomData<T>,
}

impl<
        const ITER: usize,
        const OUTPUT_LEN: usize,
        const BLOCK_LEN: usize,
        const DIGEST_LEN: usize,
        T: HashDigest<BLOCK_LEN, DIGEST_LEN>,
    > PBKDF2<ITER, OUTPUT_LEN, BLOCK_LEN, DIGEST_LEN, T>
{
    const L: usize = OUTPUT_LEN.div_ceil(BLOCK_LEN);
    const _R: usize = OUTPUT_LEN - (OUTPUT_LEN.div_ceil(BLOCK_LEN) - 1usize) * BLOCK_LEN;

    pub fn pbkdf2(key: &[u8], salt: &[u8]) -> [u8; OUTPUT_LEN] {
        let mut out = FixedU8Buf::<OUTPUT_LEN>::new();
        for l in 0..=Self::L {
            let mut block = [0u8; DIGEST_LEN];
            let mut last = {
                let mut a = HMAC::<BLOCK_LEN, DIGEST_LEN, T>::new(key);
                a.write(salt);
                a.write((l as u32 + 1).to_be_bytes().as_slice());
                let last = a.finish();
                for (a, b) in block.iter_mut().zip(last.iter()) {
                    *a ^= *b;
                }
                last
            };
            for _c in 1..ITER {
                let mut a = HMAC::<BLOCK_LEN, DIGEST_LEN, T>::new(key);
                a.write(&last);
                last = a.finish();
                for (a, b) in block.iter_mut().zip(last.iter()) {
                    *a ^= *b;
                }
            }
            let _ = out.write_all_bytes(&block);
        }

        out.as_buf_default()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        PBKDF_HMAC_SHA1, PBKDF_HMAC_SHA224, PBKDF_HMAC_SHA256, PBKDF_HMAC_SHA384, PBKDF_HMAC_SHA512,
    };
    use irox_tools::assert_eq_hex_slice;

    #[test]
    pub fn tv1() {
        let salt = b"salt";
        let password = b"password";
        let out = PBKDF_HMAC_SHA1::<1, 20>::pbkdf2(password, salt);
        assert_eq_hex_slice!(
            &[
                0x0c, 0x60, 0xc8, 0x0f, 0x96, 0x1f, 0x0e, 0x71, 0xf3, 0xa9, 0xb5, 0x24, 0xaf, 0x60,
                0x12, 0x06, 0x2f, 0xe0, 0x37, 0xa6
            ],
            &out
        );
        let out = PBKDF_HMAC_SHA224::<1, 20>::pbkdf2(password, salt);
        assert_eq_hex_slice!(
            &[
                0x3c, 0x19, 0x8c, 0xbd, 0xb9, 0x46, 0x4b, 0x78, 0x57, 0x96, 0x6b, 0xd0, 0x5b, 0x7b,
                0xc9, 0x2b, 0xc1, 0xcc, 0x4e, 0x6e
            ],
            &out
        );
        let out = PBKDF_HMAC_SHA256::<1, 20>::pbkdf2(password, salt);
        assert_eq_hex_slice!(
            &[
                0x12, 0x0f, 0xb6, 0xcf, 0xfc, 0xf8, 0xb3, 0x2c, 0x43, 0xe7, 0x22, 0x52, 0x56, 0xc4,
                0xf8, 0x37, 0xa8, 0x65, 0x48, 0xc9
            ],
            &out
        );
        let out = PBKDF_HMAC_SHA384::<1, 20>::pbkdf2(password, salt);
        assert_eq_hex_slice!(
            &[
                0xc0, 0xe1, 0x4f, 0x06, 0xe4, 0x9e, 0x32, 0xd7, 0x3f, 0x9f, 0x52, 0xdd, 0xf1, 0xd0,
                0xc5, 0xc7, 0x19, 0x16, 0x09, 0x23
            ],
            &out
        );
        let out = PBKDF_HMAC_SHA512::<1, 20>::pbkdf2(password, salt);
        assert_eq_hex_slice!(
            &[
                0x86, 0x7f, 0x70, 0xcf, 0x1a, 0xde, 0x02, 0xcf, 0xf3, 0x75, 0x25, 0x99, 0xa3, 0xa5,
                0x3d, 0xc4, 0xaf, 0x34, 0xc7, 0xa6
            ],
            &out
        );
    }
    #[test]
    pub fn tv2() {
        let salt = b"salt";
        let password = b"password";
        let out = PBKDF_HMAC_SHA1::<2, 20>::pbkdf2(password, salt);
        assert_eq_hex_slice!(
            &[
                0xea, 0x6c, 0x01, 0x4d, 0xc7, 0x2d, 0x6f, 0x8c, 0xcd, 0x1e, 0xd9, 0x2a, 0xce, 0x1d,
                0x41, 0xf0, 0xd8, 0xde, 0x89, 0x57
            ],
            &out
        );
    }

    #[test]
    pub fn tv3() {
        let salt = b"salt";
        let password = b"password";
        let out = PBKDF_HMAC_SHA1::<4096, 20>::pbkdf2(password, salt);
        assert_eq_hex_slice!(
            &[
                0x4b, 0x00, 0x79, 0x01, 0xb7, 0x65, 0x48, 0x9a, 0xbe, 0xad, 0x49, 0xd9, 0x26, 0xf7,
                0x21, 0xd0, 0x65, 0xa4, 0x29, 0xc1
            ],
            &out
        );
    }

    #[test]
    #[ignore]
    pub fn tv4() {
        let salt = b"salt";
        let password = b"password";
        let out = PBKDF_HMAC_SHA1::<16777216, 20>::pbkdf2(password, salt);
        assert_eq_hex_slice!(
            &[
                0xee, 0xfe, 0x3d, 0x61, 0xcd, 0x4d, 0xa4, 0xe4, 0xe9, 0x94, 0x5b, 0x3d, 0x6b, 0xa2,
                0x15, 0x8c, 0x26, 0x34, 0xe9, 0x84
            ],
            &out
        );
    }

    #[test]
    pub fn tv5() {
        let salt = b"saltSALTsaltSALTsaltSALTsaltSALTsalt";
        let password = b"passwordPASSWORDpassword";
        let out = PBKDF_HMAC_SHA1::<4096, 25>::pbkdf2(password, salt);
        assert_eq_hex_slice!(
            &[
                0x3d, 0x2e, 0xec, 0x4f, 0xe4, 0x1c, 0x84, 0x9b, 0x80, 0xc8, 0xd8, 0x36, 0x62, 0xc0,
                0xe4, 0x4a, 0x8b, 0x29, 0x1a, 0x96, 0x4c, 0xf2, 0xf0, 0x70, 0x38
            ],
            &out
        );
    }

    #[test]
    pub fn tv6() {
        let salt = b"sa\0lt";
        let password = b"pass\0word";
        let out = PBKDF_HMAC_SHA1::<4096, 16>::pbkdf2(password, salt);
        assert_eq_hex_slice!(
            &[
                0x56, 0xfa, 0x6a, 0xa7, 0x55, 0x48, 0x09, 0x9d, 0xcc, 0x37, 0xd7, 0xf0, 0x34, 0x25,
                0xe0, 0xc3
            ],
            &out
        );
    }
}
