// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![allow(clippy::indexing_slicing)]

use crate::x25519;
use crate::x25519::{invert, mul, sub, zeroize_u8, FieldElement};
use core::ops::MulAssign;
use irox_tools::hash::SHA512;

pub enum Ed25519Error {
    InvalidSignature,
    InvalidPublicKey,
    InvalidSecretKey,
    InvalidMessage,
    InvalidInput,
}

type GF4 = [FieldElement; 4];
const fn empty_gf4() -> GF4 {
    [
        FieldElement([0; 16]),
        FieldElement([0; 16]),
        FieldElement([0; 16]),
        FieldElement([0; 16]),
    ]
}

///
/// A public key generated using the algorithm in RFC 8032
#[derive(Eq, PartialEq)]
pub struct Ed25519PublicKey(pub [u8; 32]);
impl From<[u8; 32]> for Ed25519PublicKey {
    fn from(value: [u8; 32]) -> Self {
        Ed25519PublicKey(value)
    }
}
impl From<&[u8; 32]> for Ed25519PublicKey {
    fn from(value: &[u8; 32]) -> Self {
        Ed25519PublicKey(*value)
    }
}
///
/// Secret Key - usually random bytes.  Generation of a good random value is important here.
pub struct Ed25519SecretKey(pub [u8; 32]);
impl From<[u8; 32]> for Ed25519SecretKey {
    fn from(value: [u8; 32]) -> Self {
        Ed25519SecretKey(value)
    }
}
impl From<&[u8; 32]> for Ed25519SecretKey {
    fn from(value: &[u8; 32]) -> Self {
        Ed25519SecretKey(*value)
    }
}
impl Drop for Ed25519SecretKey {
    fn drop(&mut self) {
        zeroize_u8(&mut self.0);
    }
}
impl Ed25519SecretKey {
    pub fn generate_public_key(&self) -> Ed25519PublicKey {
        let mut d = SHA512::default().hash(&self.0);
        d[0] &= 0xF8;
        d[31] &= 0x7F;
        d[31] |= 0x40;
        let mut p = empty_gf4();
        scalarbase(&mut p, &d);
        Ed25519PublicKey(pack(&p))
    }
}
pub struct Ed25519KeyPair {
    pub public_key: Ed25519PublicKey,
    pub secret_key: Ed25519SecretKey,
}

impl Ed25519KeyPair {
    pub fn from_secret_bytes(sb: [u8; 32]) -> Ed25519KeyPair {
        let secret_key: Ed25519SecretKey = Ed25519SecretKey(sb);
        let public_key: Ed25519PublicKey = secret_key.generate_public_key();
        Ed25519KeyPair {
            secret_key,
            public_key,
        }
    }
    pub fn from_full_bytes_unchecked(mut fb: [u8; 64]) -> Ed25519KeyPair {
        let mut sk = [0u8; 32];
        let mut pk = [0u8; 32];
        sk.copy_from_slice(&fb[0..32]);
        pk.copy_from_slice(&fb[32..64]);
        zeroize_u8(&mut fb);
        Ed25519KeyPair {
            secret_key: Ed25519SecretKey(sk),
            public_key: Ed25519PublicKey(pk),
        }
    }
    pub fn from_full_bytes(fb: [u8; 64]) -> Result<Ed25519KeyPair, Ed25519Error> {
        let kp = Ed25519KeyPair::from_full_bytes_unchecked(fb);
        let gpk = kp.secret_key.generate_public_key();
        if kp.public_key != gpk {
            return Err(Ed25519Error::InvalidPublicKey);
        }
        Ok(kp)
    }
}

macro_rules! add {
    ($p:ident,$q:ident) => {{
        let mut a = FieldElement([0i64; 16]);
        let mut b = FieldElement([0i64; 16]);
        let mut c = FieldElement([0i64; 16]);
        let mut d = FieldElement([0i64; 16]);
        let mut e = FieldElement([0i64; 16]);
        let mut f = FieldElement([0i64; 16]);
        let mut g = FieldElement([0i64; 16]);
        let mut h = FieldElement([0i64; 16]);
        let mut t = FieldElement([0i64; 16]);

        sub(&mut a, &$p[1], &$p[0]);
        sub(&mut t, &$q[1], &$q[0]);
        a.mul_assign(&t);
        x25519::add(&mut b, &$p[0], &$p[1]);
        x25519::add(&mut t, &$q[0], &$q[1]);
        b.mul_assign(&t);
        mul(&mut c, &$p[3], &$q[3]);
        c.mul_assign(&D2);
        mul(&mut d, &$p[2], &$q[2]);
        x25519::add(&mut t, &d, &d);
        sub(&mut e, &b, &a);
        sub(&mut f, &t, &c);
        x25519::add(&mut g, &t, &c);
        x25519::add(&mut h, &b, &a);

        mul(&mut $p[0], &e, &f);
        mul(&mut $p[1], &h, &g);
        mul(&mut $p[2], &g, &f);
        mul(&mut $p[3], &e, &h);
    }};
}

fn scalarmult(p: &mut GF4, q: &mut GF4, s: &[u8; 64]) {
    set(&mut p[0], &GF0);
    set(&mut p[1], &GF1);
    set(&mut p[2], &GF1);
    set(&mut p[3], &GF0);

    for i in 0..=255 {
        let idx = 255 - i;
        let b = ((s[idx / 8] >> (idx & 0x7)) & 1) as i64;
        cswap(p, q, b);
        add!(q, p);
        add!(p, p);
        cswap(p, q, b);
    }
}
fn pack(p: &GF4) -> [u8; 32] {
    let mut zi = p[2].clone();
    invert(&mut zi);
    let mut tx = FieldElement([0; 16]);
    let mut ty = FieldElement([0; 16]);
    mul(&mut tx, &p[0], &zi);
    mul(&mut ty, &p[1], &zi);
    let mut r = ty.pack();
    r[31] ^= tx.parity() << 7;
    r
}
fn scalarbase(p: &mut GF4, s: &[u8; 64]) {
    let mut q: GF4 = empty_gf4();
    set(&mut q[0], &X);
    set(&mut q[1], &Y);
    set(&mut q[2], &GF1);
    mul(&mut q[3], &X, &Y);
    scalarmult(p, &mut q, s);
}

#[inline]
fn set(a: &mut FieldElement, b: &FieldElement) {
    for i in 0..16 {
        a[i] = b[i];
    }
}

#[inline]
fn cswap(p: &mut GF4, q: &mut GF4, b: i64) {
    p[0].swap(&mut q[0], b);
    p[1].swap(&mut q[1], b);
    p[2].swap(&mut q[2], b);
    p[3].swap(&mut q[3], b);
}

static GF0: FieldElement = FieldElement([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
static GF1: FieldElement = FieldElement([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
static _D: FieldElement = FieldElement([
    0x78a3, 0x1359, 0x4dca, 0x75eb, 0xd8ab, 0x4141, 0x0a4d, 0x0070, 0xe898, 0x7779, 0x4079, 0x8cc7,
    0xfe73, 0x2b6f, 0x6cee, 0x5203,
]);
static D2: FieldElement = FieldElement([
    0xf159, 0x26b2, 0x9b94, 0xebd6, 0xb156, 0x8283, 0x149a, 0x00e0, 0xd130, 0xeef3, 0x80f2, 0x198e,
    0xfce7, 0x56df, 0xd9dc, 0x2406,
]);
static X: FieldElement = FieldElement([
    0xd51a, 0x8f25, 0x2d60, 0xc956, 0xa7b2, 0x9525, 0xc760, 0x692c, 0xdc5c, 0xfdd6, 0xe231, 0xc0a4,
    0x53fe, 0xcd6e, 0x36d3, 0x2169,
]);
static Y: FieldElement = FieldElement([
    0x6658, 0x6666, 0x6666, 0x6666, 0x6666, 0x6666, 0x6666, 0x6666, 0x6666, 0x6666, 0x6666, 0x6666,
    0x6666, 0x6666, 0x6666, 0x6666,
]);
static _I: FieldElement = FieldElement([
    0xa0b0, 0x4a0e, 0x1b27, 0xc4ee, 0xe478, 0xad2f, 0x1806, 0x2f43, 0xd7a7, 0x3dfb, 0x0099, 0x2b4d,
    0xdf0b, 0x4fc1, 0x2480, 0x2b83,
]);

#[cfg(test)]
mod tests {
    use crate::ed25519::{Ed25519PublicKey, Ed25519SecretKey};
    use irox_bits::{BitsError, BitsErrorKind};
    use irox_tools::hex::try_from_hex_str;
    use irox_tools::{assert_eq_hex_slice, hex};
    use std::io::BufRead;

    #[derive(Debug)]
    enum HexStr<const N: usize> {
        Str(String),
        Hex([u8; N]),
    }
    impl<const N: usize> Default for HexStr<N> {
        fn default() -> Self {
            HexStr::Hex([0; N])
        }
    }
    impl<const N: usize> HexStr<N> {
        pub fn as_u8(&mut self) -> Result<&[u8; N], BitsError> {
            Ok(match self {
                HexStr::Str(s) => {
                    let v = try_from_hex_str(s)?;
                    *self = HexStr::Hex(v);
                    self.as_u8()?
                }
                HexStr::Hex(a) => a,
            })
        }
    }

    #[derive(Default, Debug)]
    struct TV {
        msg: Option<String>,
        pbk: Option<HexStr<32>>,
        sk: Option<HexStr<32>>,
        sig: Option<HexStr<64>>,
    }
    impl TV {
        pub fn has_enough(&self) -> bool {
            self.pbk.is_some() && self.msg.is_some() && self.sig.is_some()
        }
        pub fn check_sig(&mut self) -> Result<(), BitsError> {
            if !self.has_enough() {
                return Err(BitsErrorKind::InvalidInput.into());
            }
            Ok(())
        }
        pub fn sk(&mut self) -> Result<Ed25519SecretKey, BitsError> {
            let Some(sk) = self.sk.as_mut() else {
                return Err(BitsErrorKind::InvalidInput.into());
            };
            let sk: Ed25519SecretKey = sk.as_u8()?.into();
            Ok(sk)
        }
        pub fn pk(&mut self) -> Result<Ed25519PublicKey, BitsError> {
            let Some(pk) = self.pbk.as_mut() else {
                return Err(BitsErrorKind::InvalidInput.into());
            };
            let pk: Ed25519PublicKey = pk.as_u8()?.into();
            Ok(pk)
        }
        pub fn check_pk(&mut self) -> Result<(), BitsError> {
            let sk = self.sk()?;
            let pk = self.pk()?;
            let gpk = sk.generate_public_key();
            assert_eq_hex_slice!(pk.0, &gpk.0);
            Ok(())
        }
    }

    static RFCTESTVECS: &[([u8; 32], [u8; 32])] = &[(
        hex!("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"),
        hex!("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"),
    )];
    #[test]
    pub fn test_vectors() {
        for (sk, pk) in RFCTESTVECS {
            let gsk = Ed25519SecretKey(*sk);
            let gpk = gsk.generate_public_key();
            assert_eq_hex_slice!(pk, &gpk.0);
        }
    }

    #[test]
    pub fn test_vectors_2() -> Result<(), BitsError> {
        let f = std::fs::OpenOptions::new()
            .read(true)
            .create(false)
            .open("doc/tests_t-ed25519.inp")?;
        let f = std::io::BufReader::new(f);

        let mut num_tests = 0;

        let mut tv = TV::default();
        for line in f.lines() {
            let line = line?;
            if num_tests == 1024 {
                break;
            }
            let Some((ty, data)) = line.split_once(":") else {
                continue;
            };
            let ty = ty.trim().to_ascii_lowercase();
            let data = data.trim();
            match ty.as_str() {
                "msg" => {
                    tv.msg = Some(data.to_string());
                }
                "pbk" | "pk" => {
                    tv.pbk = Some(HexStr::Str(data.to_string()));
                    tv.check_pk()?;
                    num_tests += 1;
                }
                "sig" => {
                    tv.sig = Some(HexStr::Str(data.to_string()));
                }
                "sk" => {
                    tv.sk = Some(HexStr::Str(data.to_string()));
                }
                _ => {
                    continue;
                }
            }
        }

        assert_eq!(num_tests, 1024);

        Ok(())
    }

    #[test]
    #[ignore]
    pub fn _test_vectors_1() -> Result<(), BitsError> {
        let f = std::fs::OpenOptions::new()
            .read(true)
            .create(false)
            .open("doc/x25519-test-vectors.txt")?;
        let f = std::io::BufReader::new(f);

        let mut tv = TV::default();
        for line in f.lines() {
            let line = line?;

            let Some((ty, data)) = line.split_once("=") else {
                continue;
            };

            match ty {
                "msg" => {
                    tv.msg = Some(data.to_string());
                }
                "pbk" => {
                    tv.pbk = Some(HexStr::Str(data.to_string()));
                }
                "sig" => {
                    tv.sig = Some(HexStr::Str(data.to_string()));
                }
                _ => return Err(BitsErrorKind::InvalidInput.into()),
            }
        }

        Ok(())
    }

    pub fn _test_reject_small_order() {
        // let m1 = hex!("53656 e 6 4 2 0 3 1 3 0 3 0 2 0 5 5 5 3 4 4 2 0 7 4 6 f 2 0 4 1 6 c 6 9 6 3 6 5 ");
    }
}
