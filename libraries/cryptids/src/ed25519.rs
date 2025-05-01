// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Implementation of Ed25519 for ECDSA (Signatures) based on multiple sources.
//!
//! Validations:
//! * Constant Time to avoid timing attacks
//! * Checks canonical encodings
//! * Checks low-order scalars
//!
//! Terminology:
//! * `Secure Random [32B/256b]`/`RND`
//! * `Private Key`/`SK`/`Secret Scalar`/`d`/`s`
//! * `Public Key`/`PK`/`Point`/`A`
//! * `Shared Secret`/`SS`/`R`
//! * `Base Point`/`B` = [`x25519::BASE`]
//! * `Clamp` =
//! * `Expanded SK`/`h` = `SHA512(RND)`
//!
//! Alice:
//! * Generates 256b `RND`
//! * Generates `h` = `clamp(SHA512())`
//! *  
//!
//! Bob:
//!
//! Sources:
//! [RFC8032](https://www.rfc-editor.org/rfc8032)
//! [nacl](https://nacl.cr.yp.to)
//! [tweetnacl](https://tweetnacl.cr.yp.to)

#![allow(clippy::indexing_slicing)]

use crate::x25519;
use crate::x25519::{
    check_valid_publickey, invert, mul, sub, zeroize_u8, FieldElement, ED25519_ORDER,
};
use core::ops::MulAssign;
use core::ops::{AddAssign, SubAssign};
use irox_bits::{Bits, BitsError, Error, MutBits, ReadFromBEBits, WriteToBEBits};
use irox_tools::arrays::copy_subset;
use irox_tools::hash::SHA512;
use irox_tools::hex;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Ed25519Error {
    InvalidSignature,
    InvalidPublicKeyNotCannonical,
    InvalidPublicKeyLowOrder,
    InvalidPublicKey,
    InvalidSecretKey,
    InvalidMessage,
    InvalidInput,
}
impl Ed25519Error {
    pub fn msg(&self) -> &'static str {
        match self {
            Ed25519Error::InvalidSignature => "Invalid signature",
            Ed25519Error::InvalidPublicKeyNotCannonical => "Invalid public key (not cannonical)",
            Ed25519Error::InvalidPublicKeyLowOrder => "Invalid public key (low order)",
            Ed25519Error::InvalidPublicKey => "Invalid public key",
            Ed25519Error::InvalidSecretKey => "Invalid secret key",
            Ed25519Error::InvalidMessage => "Invalid message",
            Ed25519Error::InvalidInput => "Invalid input",
        }
    }
}
impl From<BitsError> for Ed25519Error {
    fn from(_: BitsError) -> Self {
        Ed25519Error::InvalidInput
    }
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

///
/// A public key generated using the algorithm in RFC 8032
#[derive(Clone, Eq, PartialEq)]
pub struct Ed25519PublicKey([u8; 32]);
impl TryFrom<[u8; 32]> for Ed25519PublicKey {
    type Error = Ed25519Error;

    fn try_from(value: [u8; 32]) -> Result<Self, Self::Error> {
        check_valid_publickey(&value)?;
        Ok(Ed25519PublicKey(value))
    }
}
impl TryFrom<&[u8; 32]> for Ed25519PublicKey {
    type Error = Ed25519Error;

    fn try_from(value: &[u8; 32]) -> Result<Self, Self::Error> {
        check_valid_publickey(value)?;
        Ok(Ed25519PublicKey(*value))
    }
}
impl TryFrom<&[u8]> for Ed25519PublicKey {
    type Error = Ed25519Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 32 {
            return Err(Ed25519Error::InvalidPublicKey);
        }
        TryInto::<[u8; 32]>::try_into(value)
            .unwrap_or_default()
            .try_into()
    }
}
impl AsRef<[u8; 32]> for Ed25519PublicKey {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}
impl core::ops::Deref for Ed25519PublicKey {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Ed25519PublicKey {
    pub fn from_bytes(value: &[u8; 32]) -> Result<Ed25519PublicKey, Ed25519Error> {
        check_valid_publickey(value)?;
        Ok(Ed25519PublicKey(*value))
    }

    ///
    /// Verifies the signature (sig) of the message (msg) that was signed using this public key.
    pub fn verify_signed_message(&self, msg: &[u8], sig: &[u8; 64]) -> Result<(), Ed25519Error> {
        let mut q = empty_gf4();
        let mut p = empty_gf4();
        unpack_neg(&mut q, &self.0)?;

        let mut h = SHA512::default();
        h.write(&sig[0..32]);
        h.write(&self.0);
        let mut h = h.hash(msg);
        swap_reduce(&mut h);
        let mut e: [u8; 32] = copy_subset(&h);

        scalarmult(&mut p, &mut q, &e);
        zeroize_u8(&mut e);

        scalarbase(&mut q, &copy_subset(&sig[32..]));
        add!(p, q);

        let t = pack(&p);
        let sm: [u8; 32] = copy_subset(sig);

        // debug_assert_eq!(
        //     sm, t,
        //     "Signature verification failed.  Signature is invalid.");
        if !equal(&sm, &t) {
            return Err(Ed25519Error::InvalidSignature);
        }

        Ok(())
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
        let e: [u8; 32] = copy_subset(&d);
        scalarbase(&mut p, &e);
        Ed25519PublicKey(pack(&p))
    }
}
pub struct Ed25519KeyPair {
    pub public_key: Ed25519PublicKey,
    pub secret_key: Ed25519SecretKey,
}

impl Ed25519KeyPair {
    ///
    /// Loads the provided secret key and generates the associated public key
    pub fn from_secret_bytes(sb: [u8; 32]) -> Ed25519KeyPair {
        let secret_key: Ed25519SecretKey = Ed25519SecretKey(sb);
        let public_key: Ed25519PublicKey = secret_key.generate_public_key();
        Ed25519KeyPair {
            secret_key,
            public_key,
        }
    }
    ///
    /// Loads the provided keypair `<sk[0..32]>||<pk[0..32]>` and does not verify the loaded
    /// PK was derived from the SK.
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
    ///
    /// Loads the provided keypair `<sk[0..32]>||<pk[0..32]>` and verifies the loaded
    /// PK was derived from the SK.
    pub fn from_full_bytes(fb: [u8; 64]) -> Result<Ed25519KeyPair, Ed25519Error> {
        let kp = Ed25519KeyPair::from_full_bytes_unchecked(fb);
        let gpk = kp.secret_key.generate_public_key();
        if kp.public_key != gpk || check_valid_publickey(&kp.public_key.0).is_err() {
            return Err(Ed25519Error::InvalidPublicKey);
        }

        Ok(kp)
    }

    ///
    /// Loads the provided secret key and public key, verifying the pk was generated from the sk.
    pub fn from_parts(sk: [u8; 32], pk: [u8; 32]) -> Result<Ed25519KeyPair, Ed25519Error> {
        let kp = Self::from_secret_bytes(sk);
        if kp.public_key != Ed25519PublicKey(pk) || check_valid_publickey(&kp.public_key.0).is_err()
        {
            return Err(Ed25519Error::InvalidPublicKey);
        }
        Ok(kp)
    }

    ///
    /// Signs the provided message using the associated secret key.  Returns a
    /// detatched signature.
    #[allow(non_snake_case)]
    #[allow(clippy::manual_memcpy)]
    pub fn sign_message(&self, msg: &[u8]) -> Ed25519Signature {
        let mut d = SHA512::default().hash(&self.secret_key.0);
        d[0] &= 0xF8;
        d[31] &= 0x7F;
        d[31] |= 0x40;
        let mut h = SHA512::default();
        h.write(&d[32..]);
        let mut r = h.hash(msg);
        swap_reduce(&mut r);
        let mut p = empty_gf4();
        let t = copy_subset(&r);
        scalarbase(&mut p, &t);

        let R = pack(&p);
        let mut h = SHA512::default();
        h.write(&R);
        h.write(&self.public_key.0);
        let mut h = h.hash(msg);
        swap_reduce(&mut h);
        let mut x = [0i64; 64];

        for i in 0..32 {
            x[i] = r[i] as i64;
        }
        for i in 0..32 {
            for j in 0..32 {
                x[i + j] += h[i] as i64 * d[j] as i64;
            }
        }
        mod_l(&mut h, &mut x);
        let mut sig = [0u8; 64];
        for i in 0..32 {
            sig[i] = R[i];
            sig[i + 32] = h[i];
        }
        Ed25519Signature {
            pubkey: self.public_key.clone(),
            signature: sig,
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
pub struct Ed25519Signature {
    pub pubkey: Ed25519PublicKey,
    pub signature: [u8; 64],
}
impl Ed25519Signature {
    pub fn validate_hash(&self, hash: &[u8]) -> Result<(), Ed25519Error> {
        self.pubkey.verify_signed_message(hash, &self.signature)
    }
}
impl ReadFromBEBits for Ed25519Signature {
    fn read_from_be_bits<R: Bits>(reader: &mut R) -> Result<Self, BitsError> {
        let pubkey = reader.read_exact()?;
        let signature = reader.read_exact()?;
        Ok(Ed25519Signature {
            pubkey: Ed25519PublicKey(pubkey),
            signature,
        })
    }
}
impl WriteToBEBits for Ed25519Signature {
    fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        bits.write_all_bytes(&self.pubkey.0)?;
        bits.write_all_bytes(&self.signature)?;
        Ok(96)
    }
}

fn scalarmult(p: &mut GF4, q: &mut GF4, s: &[u8; 32]) {
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
fn scalarbase(p: &mut GF4, s: &[u8; 32]) {
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

fn equal(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let mut d: u32 = 0;
    for idx in 0..32 {
        d |= a[idx] as u32 ^ b[idx] as u32;
    }

    d == 0
}
fn equal_25519(a: FieldElement, b: FieldElement) -> bool {
    let a = a.pack();
    let b = b.pack();
    equal(&a, &b)
}

fn unpack_neg(r: &mut GF4, p: &[u8; 32]) -> Result<(), Ed25519Error> {
    let mut num = FieldElement([0; 16]);
    let mut den = FieldElement([0; 16]);
    let mut den2 = FieldElement([0; 16]);
    let mut den4 = FieldElement([0; 16]);
    let mut den6 = FieldElement([0; 16]);
    let mut t = FieldElement([0; 16]);
    let mut chk = FieldElement([0; 16]);

    set(&mut r[2], &GF1);
    r[1] = FieldElement::unpack(p);
    num.square_assign(&r[1]);
    mul(&mut den, &num, &D);
    num.sub_assign(&r[2]);
    den.add_assign(&r[2]);

    den2.square_assign(&den);
    den4.square_assign(&den2);
    mul(&mut den6, &den4, &den2);
    mul(&mut t, &den6, &num);
    t.mul_assign(&den);

    t.pow2523();
    t.mul_assign(&num);
    t.mul_assign(&den);
    t.mul_assign(&den);
    mul(&mut r[0], &t, &den);

    chk.square_assign(&r[0]);
    chk.mul_assign(&den);
    if !equal_25519(chk.clone(), num.clone()) {
        r[0].mul_assign(&I);
    }
    chk.square_assign(&r[0]);
    chk.mul_assign(&den);
    if !equal_25519(chk, num) {
        return Err(Ed25519Error::InvalidSignature);
    }
    if r[0].parity() == (p[31] >> 7) {
        r[0].sub_rassign(&GF0);
    }
    mul(&mut t, &r[0], &r[1]);
    r[3] = t;
    Ok(())
}
fn swap_reduce(r: &mut [u8; 64]) {
    let mut x = [0i64; 64];
    for i in 0..64 {
        x[i] = i64::from(r[i]);
        r[i] = 0;
    }
    mod_l(r, &mut x);
}
fn mod_l(r: &mut [u8; 64], x: &mut [i64; 64]) {
    let mut carry;
    for i in (32..64).rev() {
        carry = 0;
        for j in (i - 32)..(i - 12) {
            x[j] += carry - 16 * x[i] * ED25519_ORDER[j - (i - 32)] as i64;
            carry = (x[j] + 128) >> 8;
            x[j] -= carry << 8;
        }
        x[i - 12] += carry;
        x[i] = 0;
    }
    let mut carry = 0;
    for j in 0..32 {
        x[j] += carry - (x[31] >> 4) * ED25519_ORDER[j] as i64;
        carry = x[j] >> 8;
        x[j] &= 255;
    }
    for j in 0..32 {
        x[j] -= carry * ED25519_ORDER[j] as i64;
    }
    for i in 0..32 {
        x[i + 1] += x[i] >> 8;
        r[i] = x[i] as u8;
        x[i] = 0;
    }
}

static GF0: FieldElement = FieldElement([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
static GF1: FieldElement = FieldElement([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
static D: FieldElement = FieldElement([
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
static I: FieldElement = FieldElement([
    0xa0b0, 0x4a0e, 0x1b27, 0xc4ee, 0xe478, 0xad2f, 0x1806, 0x2f43, 0xd7a7, 0x3dfb, 0x0099, 0x2b4d,
    0xdf0b, 0x4fc1, 0x2480, 0x2b83,
]);
pub static ED25519_BASE: &[u8; 32] =
    &hex!("4A87A6EC35B2148A5427BEBEB2F58FFF6717868886BF38738C0190D41193FC2D");
pub static ED25519_GX: &[u8; 32] =
    &hex!("1AD5258F602D56C9B2A7259560C72C695CDCD6FD31E2A4C0FE536ECDD3366921");
pub static ED25519_GY: &[u8; 32] =
    &hex!("5866666666666666666666666666666666666666666666666666666666666666");
pub static ED25519_G: &[u8; 65] = &hex!(
    "5866666666666666666666666666666666666666666666666666666666666666"
    "1AD5258F602D56C9B2A7259560C72C695CDCD6FD31E2A4C0FE536ECDD3366921"
    "04");

#[cfg(test)]
mod tests {
    use crate::ed25519::{Ed25519Error, Ed25519KeyPair, Ed25519PublicKey, Ed25519SecretKey};
    use irox_bits::{BitsError, BitsErrorKind};
    use irox_tools::hex::{from_hex_str, HexStr};
    use irox_tools::options::MaybeMap;
    use irox_tools::{assert_eq_hex_slice, hex};
    use std::io::BufRead;

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
        pub fn check_sig(&mut self) -> Result<(), Ed25519Error> {
            if !self.has_enough() {
                return Err(Ed25519Error::InvalidInput.into());
            }
            let pk = self.pk()?;
            let msg = self.msg()?;
            let sig = self.sig()?;
            pk.verify_signed_message(&msg, sig)?;
            Ok(())
        }

        fn msg(&mut self) -> Result<Box<[u8]>, Ed25519Error> {
            let Some(msg) = self.msg.as_ref().maybe_map(|s| from_hex_str(&s).ok()) else {
                return Err(Ed25519Error::InvalidInput.into());
            };
            Ok(msg)
        }

        pub fn make_sig(&mut self) -> Result<(), Ed25519Error> {
            if !self.has_enough() {
                return Err(Ed25519Error::InvalidInput.into());
            }
            let sk = self.sk()?;
            let pk = self.pk()?;
            let kp = Ed25519KeyPair::from_secret_bytes(sk.0);
            let m = self.msg()?;
            let sig = kp.sign_message(&m);
            let check = self.sig()?;

            pk.verify_signed_message(&m, &sig.signature)?;
            assert_eq_hex_slice!(&sig.signature, check);
            Ok(())
        }
        pub fn sig(&mut self) -> Result<&[u8; 64], BitsError> {
            let Some(sig) = self.sig.as_mut() else {
                return Err(BitsErrorKind::InvalidInput.into());
            };
            let sk = sig.as_u8()?;
            Ok(sk)
        }
        pub fn sk(&mut self) -> Result<Ed25519SecretKey, BitsError> {
            let Some(sk) = self.sk.as_mut() else {
                return Err(BitsErrorKind::InvalidInput.into());
            };
            let sk: Ed25519SecretKey = sk.as_u8()?.into();
            Ok(sk)
        }
        pub fn pk(&mut self) -> Result<Ed25519PublicKey, Ed25519Error> {
            let Some(pk) = self.pbk.as_mut() else {
                return Err(Ed25519Error::InvalidInput);
            };
            let pk: Ed25519PublicKey = pk.as_u8()?.try_into()?;
            Ok(pk)
        }
        pub fn check_pk(&mut self) -> Result<(), Ed25519Error> {
            let sk = self.sk()?;
            let pk = self.pk()?;
            let gpk = sk.generate_public_key();
            assert_eq_hex_slice!(pk.0, &gpk.0);
            Ok(())
        }
    }

    struct STV {
        sk: [u8; 32],
        pk: [u8; 32],
        msg: &'static [u8],
        sig: [u8; 64],
    }

    static RFCTESTVECS: &[STV] = &[
        STV {
            sk: hex!("6df9340c138cc188b5fe4464ebaa3f7fc206a2d55c3434707e74c9fc04e20ebb"),
            pk: hex!("c0dac102c4533186e25dc43128472353eaabdb878b152aeb8e001f92d90233a7"),
            msg: &hex!("5f4c8989"),
            sig: hex!("124f6fc6b0d100842769e71bd530664d888df8507df6c56dedfdb509aeb93416e26b918d38aa06305df3095697c18b2aa832eaa52edc0ae49fbae5a85e150c07"),
        },
        STV {
            sk: hex!("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"),
            pk: hex!("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"),
            msg: &[],
            sig: hex!("e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e065224901555fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b"),
        },
    ];
    #[test]
    pub fn test_vectors() {
        for STV { sk, pk, msg, sig } in RFCTESTVECS {
            let gsk = Ed25519SecretKey(*sk);
            let gpk = gsk.generate_public_key();
            assert_eq_hex_slice!(pk, &gpk.0);
            gpk.verify_signed_message(msg, sig).unwrap();

            let kp = Ed25519KeyPair {
                public_key: gpk,
                secret_key: gsk,
            };
            let gsig = kp.sign_message(*msg);
            assert_eq_hex_slice!(sig, &gsig.signature);
        }
    }

    #[test]
    pub fn test_vectors_2() -> Result<(), Ed25519Error> {
        let f = std::fs::OpenOptions::new()
            .read(true)
            .create(false)
            .open("doc/tests_t-ed25519.inp")
            .unwrap();
        let f = std::io::BufReader::new(f);

        let mut num_tests = 0;

        let mut tv = TV::default();
        for line in f.lines() {
            let line = line.unwrap_or_default();
            if num_tests == 2048 {
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
                    if let Err(e) = tv.check_sig() {
                        println!("{:?} on test {num_tests}", e);
                        return Err(e);
                    };
                    if let Err(e) = tv.make_sig() {
                        println!("{:?} on test {num_tests}", e);
                        return Err(e);
                    };
                    num_tests += 1;
                }
                "sk" => {
                    tv.sk = Some(HexStr::Str(data.to_string()));
                }
                _ => {
                    continue;
                }
            }
        }

        assert_eq!(num_tests, 2048);

        Ok(())
    }
}
