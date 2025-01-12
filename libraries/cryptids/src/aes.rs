// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![allow(clippy::indexing_slicing)]

use core::ops::{BitXor, BitXorAssign, Shl, Shr};
use irox_bits::{Bits, BitsWrapper, Error, FromBEBytes, MutBits, ToBEBytes};
use irox_tools::buf::Buffer;

static AES_SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];
fn sub_word(a: u32) -> u32 {
    let [a, b, c, d] = a.to_be_bytes();
    u32::from_be_bytes([
        AES_SBOX[a as usize],
        AES_SBOX[b as usize],
        AES_SBOX[c as usize],
        AES_SBOX[d as usize],
    ])
}
fn mul_by_x32(w: u32) -> u32 {
    let x = w & 0x7f7f7f7f;
    let y = w & 0x80808080;

    (x << 1) ^ ((y >> 7) * 0x1b)
}
fn mul_by_x(a: u8) -> u8 {
    (a << 1) ^ ((a >> 7) * 0x1Bu8)
}
fn mix_columns(r: u32) -> u32 {
    let mut b: [u8; 4] = u32::to_be_bytes(r);
    let a = b;
    b[0] = mul_by_x(b[0]);
    b[1] = mul_by_x(b[1]);
    b[2] = mul_by_x(b[2]);
    b[3] = mul_by_x(b[3]);

    let d = b[0] ^ a[3] ^ a[2] ^ b[1] ^ a[1]; /* 2 * a0 + a3 + a2 + 3 * a1 */
    let e = b[1] ^ a[0] ^ a[3] ^ b[2] ^ a[2]; /* 2 * a1 + a0 + a3 + 3 * a2 */
    let f = b[2] ^ a[1] ^ a[0] ^ b[3] ^ a[3]; /* 2 * a2 + a1 + a0 + 3 * a3 */
    let g = b[3] ^ a[2] ^ a[1] ^ b[0] ^ a[0]; /* 2 * a3 + a2 + a1 + 3 * a0 */
    u32::from_be_bytes([d, e, f, g])
}
#[allow(dead_code)]
fn mul_by_x2(a: u32) -> u32 {
    let x = a & 0x3F3F3F3F;
    let y = a & 0x80808080;
    let z = a & 0x40404040;
    x.shl(2u32).bitxor(
        y.shr(7u32)
            .wrapping_mul(0x36)
            .bitxor(z.shr(6u32))
            .wrapping_mul(0x1B),
    )
}
fn subshift(data: &[u32; 4], iter: usize) -> u32 {
    let mut data = *data;
    data.rotate_left(iter);
    let [a, b, c, d] = data;
    u32::from_be_bytes([
        AES_SBOX[((a >> 24) & 0xFF) as usize],
        AES_SBOX[((b >> 16) & 0xFF) as usize],
        AES_SBOX[((c >> 8) & 0xFF) as usize],
        AES_SBOX[(d & 0xFF) as usize],
    ])
}

pub struct Aes256Enc {
    key: [u32; 120],
}
impl Drop for Aes256Enc {
    fn drop(&mut self) {
        for x in &mut self.key {
            *x = 0;
        }
    }
}

impl Aes256Enc {
    pub fn new(in_key: [u8; 32]) -> Aes256Enc {
        let mut enc = Aes256Enc { key: [0; 120] };
        let mut rdr = in_key.as_slice();
        for i in 0..8 {
            enc.key[i] = rdr.read_be_u32().unwrap_or_default();
        }
        let v = &mut enc.key;
        let mut rc = 1;
        for i in 0..14 {
            rc = mul_by_x32(rc);

            let idx = i * 8;
            let odx = idx + 8;
            v[odx] = sub_word(v[idx + 7])
                .rotate_right(8)
                .bitxor(rc)
                .bitxor(v[idx]);
            v[odx + 1] = v[odx].bitxor(v[idx + 1]);
            v[odx + 2] = v[odx + 1].bitxor(v[idx + 2]);
            v[odx + 3] = v[odx + 2].bitxor(v[idx + 3]);
            v[odx + 4] = sub_word(v[idx + 3]).bitxor(v[idx + 4]);
            v[odx + 5] = v[odx + 4].bitxor(v[idx + 5]);
            v[odx + 6] = v[odx + 5].bitxor(v[idx + 6]);
            v[odx + 7] = v[odx + 5].bitxor(v[idx + 7]);
        }
        enc
    }

    pub fn encrypt(&self, block: &[u8; 16]) -> [u8; 16] {
        let mut state: [u32; 4] = FromBEBytes::from_be_bytes(*block);
        state[0].bitxor_assign(self.key[0]);
        state[1].bitxor_assign(self.key[1]);
        state[2].bitxor_assign(self.key[2]);
        state[3].bitxor_assign(self.key[3]);

        let mut rkp = 4;
        let mut s2 = state;
        for _ in 0..13 {
            s2[0] = mix_columns(subshift(&state, 0)).bitxor(self.key[rkp]);
            s2[1] = mix_columns(subshift(&state, 1)).bitxor(self.key[rkp + 1]);
            s2[2] = mix_columns(subshift(&state, 2)).bitxor(self.key[rkp + 2]);
            s2[3] = mix_columns(subshift(&state, 3)).bitxor(self.key[rkp + 3]);
            state = s2;
            rkp += 4;
        }
        [
            subshift(&state, 0).bitxor(self.key[rkp + 4]),
            subshift(&state, 1).bitxor(self.key[rkp + 5]),
            subshift(&state, 2).bitxor(self.key[rkp + 6]),
            subshift(&state, 3).bitxor(self.key[rkp + 7]),
        ]
        .to_be_bytes()
    }
}
pub struct Aes256EncStream<'a, T> {
    enc: Aes256Enc,
    buf: irox_tools::buf::FixedBuf<16, u8>,
    wrapped: BitsWrapper<'a, T>,
}
impl<'a, T> Aes256EncStream<'a, T> {
    pub fn new(key: [u8; 32], wrapped: BitsWrapper<'a, T>) -> Aes256EncStream<'a, T> {
        Self {
            enc: Aes256Enc::new(key),
            buf: Default::default(),
            wrapped,
        }
    }
}
impl<'a, T: MutBits> MutBits for Aes256EncStream<'a, T> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        self.buf.write_u8(val)?;
        if self.buf.is_full() {
            let enc = self.enc.encrypt(&self.buf.into_buf_default());
            self.wrapped.write_all_bytes(&enc)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::aes::{mix_columns, subshift};
    use crate::Aes256EncStream;
    use core::ops::BitXor;
    use irox_bits::{Bits, BitsWrapper, MutBits, ToBEBytes};
    use irox_tools::assert_eq_hex;
    use std::collections::VecDeque;

    #[test]
    pub fn test_aes_subshift() {
        assert_eq_hex!(
            0x2B7E7062,
            subshift(&[0x0BFC55F2, 0x3B8AEE28, 0xC24ED0E1, 0xF6EE60AB], 0)
        );
        assert_eq_hex!(
            0xE22FD089,
            subshift(&[0x0BFC55F2, 0x3B8AEE28, 0xC24ED0E1, 0xF6EE60AB], 1)
        );
        assert_eq_hex!(
            0x2528FC34,
            subshift(&[0x0BFC55F2, 0x3B8AEE28, 0xC24ED0E1, 0xF6EE60AB], 2)
        );
        assert_eq_hex!(
            0x42B028F8,
            subshift(&[0x0BFC55F2, 0x3B8AEE28, 0xC24ED0E1, 0xF6EE60AB], 3)
        );
    }
    #[test]
    pub fn test_aes_mixcolumns() {
        assert_eq_hex!(0xC62513B7, mix_columns(0x2B7E7062));
    }
    #[test]
    pub fn test_aes_keyadd() {
        assert_eq_hex!(0xD9103FB0, mix_columns(0x2B7E7062).bitxor(0x1F352C07))
    }

    #[test]
    #[ignore]
    pub fn test_aes256_enc1() {
        let key: [u8; 32] = [
            0x603DEB10u32,
            0x15CA71BE,
            0x2B73AEF0,
            0x857D7781,
            0x1F352C07,
            0x3B6108D7,
            0x2D9810A3,
            0x0914DFF4,
        ]
        .to_be_bytes();
        let mut buf = VecDeque::<u8>::new();
        let mut alg = Aes256EncStream::new(key, BitsWrapper::Borrowed(&mut buf));

        for v in [
            0x6BC1BEE2, 0x2E409F96, 0xE93D7E11, 0x7393172A, 0xAE2D8A57, 0x1E03AC9C, 0x9EB76FAC,
            0x45AF8E51, 0x30C81C46, 0xA35CE411, 0xE5FBC119, 0x1A0A52EF, 0xF69F2445, 0xDF4F9B17,
            0xAD2B417B, 0xE66C3710,
        ] {
            alg.write_be_u32(v).unwrap();
        }

        for exp in [
            0xF3EED1BD, 0xB5D2A03C, 0x064B5A7E, 0x3DB181F8, 0x591CCB10, 0xD410ED26, 0xDC5BA74A,
            0x31362870, 0xB6ED21B9, 0x9CA6F4F9, 0xF153E7B1, 0xBEAFED1D, 0x23304B7A, 0x39F9F3FF,
            0x067D8D8F, 0x9E24ECC7,
        ] {
            assert_eq_hex!(exp, buf.read_be_u32().unwrap());
        }
    }
}
