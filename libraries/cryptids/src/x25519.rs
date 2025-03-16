// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Implementation of x25519 for ECDH (Diffie-Helman) based on multiple sources.
//! 
//! Terminology:
//! * `Private Key`/`SK`/`Secret Scalar`/`d`
//! * `Public Key`/`PK`/`Point`/`A`
//! * `Shared Secret`/`SS`/`R`
//! * `Base Point`/`B`
//! 
//! Alice:
//! * Generate `SK:d` = `
//! * Generate `PK:Alice` = `SK:d * B`
//! * Generate `SS:R` = `SK:d * PK:Bob`
//! 
//! Bob:
//! * Generate `SK:d`
//! * Generate `PK:Bob` = `SK:d * B`
//! * Generate `SS:R` = `SK:d * PK:Alice`
//! 
//! Sources:
//! * [RFC7748](https://www.rfc-editor.org/rfc/rfc7748)
//! * [LibSodium](https://github.com/jedisct1/libsodium)
//! * [nacl](https://nacl.cr.yp.to/)
//! * [tweetnacl](https://tweetnacl.cr.yp.to)

#![allow(clippy::indexing_slicing)]
#![allow(clippy::manual_memcpy)]

use crate::ed25519::Ed25519Error;
use core::ops::{AddAssign, MulAssign, SubAssign};
use core::ops::{Index, IndexMut};
use irox_tools::hex;

macro_rules! zeroize {
    ($name:ident,$ty:ty) => {
        #[inline]
        pub fn $name(v: &mut [$ty]) {
            v.iter_mut().for_each(|x| *x = 0);
        }
    };
}
zeroize!(zeroize_u8, u8);
zeroize!(zeroize_i64, i64);

///
/// Curve25519 Base Point - '9'
pub static BASE: &[u8; 32] = &hex!("0900000000000000000000000000000000000000000000000000000000000000");

///
/// Secret Key - usually random bytes.  Generation of a good random value is important here.
pub struct SecretKey(pub [u8; 32]);
///
/// A public key generated using the original Curve25519 method of multiplying the
/// secret key with the base point.  This is probably not what you want.
pub struct Curve25519PublicKey(pub [u8; 32]);
///
/// A shared key generated using the original Curve25519 method of multiplying owned secret key with
/// a Curve25519PublicKey.  This is probably not what you want.
pub struct SharedCurve25519Secret(pub [u8; 32]);

impl SecretKey {
    ///
    /// Generates a public key using the original Curve25519 method of multiplying the
    /// secret key with the base point.  This is probably not what you want.
    pub fn generate_curve25519_pubkey(&self) -> Result<Curve25519PublicKey, Ed25519Error> {
        Ok(Curve25519PublicKey(scalarmult(&self.0, BASE)?))
    }
    ///
    /// Generates a shared key using the original Curve25519 method of multiplying owned secret key with
    /// a Curve25519PublicKey.  This is probably not what you want.
    pub fn generate_curve25519_shared_secret(
        &self,
        pubkey: &Curve25519PublicKey,
    ) -> Result<SharedCurve25519Secret, Ed25519Error> {
        Ok(SharedCurve25519Secret(scalarmult(&self.0, &pubkey.0)?))
    }
}
impl Drop for SecretKey {
    fn drop(&mut self) {
        zeroize_u8(&mut self.0);
    }
}
impl Curve25519PublicKey {
    ///
    /// Generates a shared key using the original Curve25519 method of multiplying owned secret key with
    /// a Curve25519PublicKey.  This is probably not what you want.
    pub fn generate_shared_secret(&self, secret_key: &SecretKey) -> Result<SharedCurve25519Secret, Ed25519Error> {
        Ok(SharedCurve25519Secret(scalarmult(&secret_key.0, &self.0)?))
    }
}
impl Drop for SharedCurve25519Secret {
    fn drop(&mut self) {
        zeroize_u8(&mut self.0);
    }
}
impl AsRef<[u8]> for SharedCurve25519Secret {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
impl AsRef<[u8; 32]> for SharedCurve25519Secret {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

pub fn scalarmult(scalar: &[u8; 32], point: &[u8; 32]) -> Result<[u8; 32], Ed25519Error> {
    check_scalar_clamped(scalar)?;
    let mut clamp = *scalar;
    clamp[0] &= 0xF8;
    clamp[31] &= 0x7F;
    clamp[31] |= 0x40;

    let x = FieldElement::unpack(point);
    let mut b = x.clone();
    let mut a = FieldElement([0i64; 16]);
    let mut d = FieldElement([0i64; 16]);
    let mut c = FieldElement([0i64; 16]);
    let mut e = FieldElement([0i64; 16]);
    let mut f = FieldElement([0i64; 16]);
    a[0] = 1;
    d[0] = 1;
    for i in 0..=254 {
        let i = 254 - i;
        let bit = ((clamp[i >> 3] >> (i & 0x7)) & 1) as i64;
        a.swap(&mut b, bit);
        c.swap(&mut d, bit);
        add(&mut e, &a, &c);
        a -= &c;
        add(&mut c, &b, &d);
        b -= &d;
        d.square_assign(&e);
        f.square_assign(&a);
        a.mul_rassign(&c);
        mul(&mut c, &b, &e);
        add(&mut e, &a, &c);
        a -= &c;
        b.square_assign(&a);
        sub(&mut c, &d, &f);
        mul(&mut a, &c, &DB41);
        a += &d;
        c *= &a;
        mul(&mut a, &d, &f);
        mul(&mut d, &b, &x);
        b.square_assign(&e);
        a.swap(&mut b, bit);
        c.swap(&mut d, bit);
    }
    invert(&mut c);
    a *= &c;
    Ok(a.pack())
}

#[derive(Clone)]
pub(crate) struct FieldElement(pub(crate) [i64; 16]);
impl Drop for FieldElement {
    fn drop(&mut self) {
        zeroize_i64(&mut self.0);
    }
}
static DB41: FieldElement = FieldElement([0xDB41, 0x1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
impl FieldElement {
    pub(crate) fn carry(&mut self) {
        let mut carry;
        for i in 0..15 {
            carry = self[i] >> 16;
            self[i] -= carry << 16;
            self[i + 1] += carry;
        }
        carry = self[15] >> 16;
        self[15] -= carry << 16;
        self[0] += 38 * carry;
    }

    pub(crate) fn swap<T: IndexMut<usize, Output = i64>>(&mut self, other: &mut T, bit: i64) {
        let c = !(bit - 1);
        for i in 0..16 {
            let t = c & (self[i] ^ other[i]);
            self[i] ^= t;
            other[i] ^= t;
        }
    }
    pub(crate) fn pack(self) -> [u8; 32] {
        let mut t = self;
        t.carry();
        t.carry();
        t.carry();
        let mut m = [0; 16];
        for _ in 0..2 {
            m[0] = t[0] - 0xFFED;
            for i in 1..15 {
                m[i] = t[i] - 0xFFFF - ((m[i - 1] >> 16) & 1);
                m[i - 1] &= 0xFFFF;
            }
            m[15] = t[15] - 0x7FFF - ((m[14] >> 16) & 1);
            let carry = (m[15] >> 16) & 1;
            m[14] &= 0xFFFF;
            t.swap(&mut m, 1 - carry);
        }

        let mut out = [0u8; 32];
        for i in 0..16 {
            out[2 * i] = t[i] as u8;
            out[2 * i + 1] = (t[i] >> 8) as u8;
        }
        out
    }
    pub(crate) fn unpack(inp: &[u8; 32]) -> Self {
        let mut out = FieldElement([0; 16]);
        for i in 0..16 {
            out[i] = (inp[2 * i] as i64) | ((inp[2 * i + 1] as i64) << 8);
        }
        out[15] &= 0x7FFF;
        out
    }
    pub(crate) fn square(&mut self) {
        let mut t = [0i64; 32];
        for i in 0..16 {
            for j in 0..16 {
                t[i + j] += self[i] * self[j];
            }
        }
        for i in 0..15 {
            t[i] += 38 * t[i + 16];
        }
        for i in 0..16 {
            self[i] = t[i];
        }
        self.carry();
        self.carry();
    }
    pub(crate) fn square_assign(&mut self, a: &FieldElement) {
        let mut t = [0i64; 32];
        for i in 0..16 {
            for j in 0..16 {
                t[i + j] += a[i] * a[j];
            }
        }
        for i in 0..15 {
            t[i] += 38 * t[i + 16];
        }
        for i in 0..16 {
            self[i] = t[i];
        }
        self.carry();
        self.carry();
    }
    pub(crate) fn mul_rassign(&mut self, rhs: &FieldElement) {
        let mut t = [0i64; 32];
        for i in 0..16 {
            for j in 0..16 {
                t[i + j] += rhs[i] * self[j];
            }
        }
        for i in 0..15 {
            t[i] += 38 * t[i + 16];
        }
        for i in 0..16 {
            self[i] = t[i];
        }
        self.carry();
        self.carry();
    }
    pub(crate) fn sub_rassign(&mut self, lhs: &FieldElement) {
        for i in 0..16 {
            self[i] = lhs[i] - self[i]
        }
    }
    pub(crate) fn parity(&self) -> u8 {
        self.clone().pack()[0] & 1
    }
    pub(crate) fn pow2523(&mut self) {
        let i = self.clone();
        for _ in 0..249 {
            self.square();
            self.mul_assign(&i);
        }
        self.square();
        self.square();
        self.mul_assign(&i);
    }
}
impl Index<usize> for FieldElement {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for FieldElement {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
pub(crate) fn add(out: &mut FieldElement, a: &FieldElement, b: &FieldElement) {
    for i in 0..16 {
        out[i] = a[i] + b[i];
    }
}
impl AddAssign<&FieldElement> for FieldElement {
    fn add_assign(&mut self, rhs: &FieldElement) {
        for i in 0..16 {
            self[i] += rhs[i];
        }
    }
}
pub(crate) fn sub(out: &mut FieldElement, a: &FieldElement, b: &FieldElement) {
    for i in 0..16 {
        out[i] = a[i] - b[i];
    }
}
impl SubAssign<&FieldElement> for FieldElement {
    fn sub_assign(&mut self, rhs: &FieldElement) {
        for i in 0..16 {
            self[i] -= rhs[i];
        }
    }
}
pub(crate) fn mul(out: &mut FieldElement, a: &FieldElement, b: &FieldElement) {
    let mut t = [0i64; 32];
    for i in 0..16 {
        for j in 0..16 {
            t[i + j] += a[i] * b[j];
        }
    }
    for i in 0..15 {
        t[i] += 38 * t[i + 16];
    }
    for i in 0..16 {
        out[i] = t[i];
    }
    out.carry();
    out.carry();
}
impl MulAssign<&FieldElement> for FieldElement {
    fn mul_assign(&mut self, rhs: &FieldElement) {
        let mut t = [0i64; 32];
        for i in 0..16 {
            for j in 0..16 {
                t[i + j] += self[i] * rhs[j];
            }
        }
        for i in 0..15 {
            t[i] += 38 * t[i + 16];
        }
        for i in 0..16 {
            self[i] = t[i];
        }
        self.carry();
        self.carry();
    }
}
pub(crate) fn invert(v: &mut FieldElement) {
    let mut out = v.clone();
    for _ in 0..249 {
        out.square();
        out *= v;
    }
    // 4
    out.square();
    // 3
    out.square();
    out *= v;
    // 2
    out.square();
    //1
    out.square();
    out *= v;
    // 0
    out.square();
    out *= v;

    *v = out;
}
pub(crate) fn check_scalar_clamped(scalar: &[u8;32]) -> Result<(), Ed25519Error> {
    if scalar[0] < 0xED {
        return Ok(())
    }

    for i in 1..31 {
        if scalar[i] != 0xFF {
            return Ok(())
        }
    }
    if scalar[31] | 0x7F != 0xFF {
        return Ok(())
    }
    Err(Ed25519Error::InvalidPublicKeyNotCannonical)
}

pub fn check_valid_publickey(key: &[u8;32]) -> Result<(), Ed25519Error> {
    let mut err = check_scalar_clamped(key).err();

    for blk in &BLOCKLIST {
        if key == *blk {
            err.get_or_insert(Ed25519Error::InvalidPublicKeyLowOrder);
        }
    }

    match err {
        Some(e) => Err(e),
        None => Ok(())
    }
}
/// 2^255 - 19
pub static X255M19: &[u8; 32] =
    &hex!("EDFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF7F");
/// 2^255 + 19
pub static X255P19: &[u8; 32] =
    &hex!("1300000000000000000000000000000000000000000000000000000000000080");
// order of ed25519 signature as per [RFC8032](https://tools.ietf.org/html/rfc8032)
pub static ED25519_ORDER: &[u8; 32] =
    &hex!("EDD3F55C1A631258D69CF7A2DEF9DE1400000000000000000000000000000010");
pub static CLAMP_MASK: &[u8; 32] =
    &hex!("F8FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF7F");
pub static CLAMP_SET: &[u8;32] =
    &hex!("0000000000000000000000000000000000000000000000000000000000000040");
///
/// The following are malicious public keys crafted to exploit weaknesses in the montgomery
/// curve multiplication logic.
pub static BLOCKLIST: [&[u8; 32];17] = [
    // 0 (order 4), check 4
    &hex!("0000000000000000000000000000000000000000000000000000000000000000"),
    // 1 ( order 1), check 1
    &hex!("0100000000000000000000000000000000000000000000000000000000000000"),
    // 325606250916557431795983626356110631294008115727848805560023387167927233504 (order 8)
    &hex!("e0eb7a7c3b41b8ae1656e3faf19fc46ada098deb9c32b1fd866205165f49b800"),
    // 39382357235489614581723060781553021112529911719440698176882885853963445705823 (oder 8)
    &hex!("5f9c95bca3508c24b1d0b1559c83ef5b04445cc4581c8e86d8224eddd09f1157"),
    // p-1 (order 2), check 2
    &hex!("ECFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF7F"),
    // Check 10
    &hex!("ECFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"),
    // Check 13
    &hex!("EDFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"),
    // p (=0, order 4), Check 14
    X255M19,
    // p+1 (=1, order 1, Check 11
    &hex!("EEFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF7F"),
    // Check 12
    &hex!("EEFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"),
    // BFN, check 3
    &hex!("0000000000000000000000000000000000000000000000000000000000000080"),
    // Check 5
    &hex!("C7176A703D4DD84FBA3C0B760D10670F2A2053FA2C39CCC64EC7FD7792AC037A"),
    // Check 6
    &hex!("C7176A703D4DD84FBA3C0B760D10670F2A2053FA2C39CCC64EC7FD7792AC03FA"),
    // Check 7
    &hex!("26E8958FC2B227B045C3F489F2EF98F0D5DFAC05D3C63339B13802886D53FC05"),
    // Check 8
    &hex!("26E8958FC2B227B045C3F489F2EF98F0D5DFAC05D3C63339B13802886D53FC85"),
    // Check 9
    &hex!("0100000000000000000000000000000000000000000000000000000000000080"),
    // order of ed25519 as per [RFC8032](https://tools.ietf.org/html/rfc8032)
    ED25519_ORDER,
];

#[cfg(test)]
mod tests {
    use crate::ed25519::Ed25519Error;
    use crate::x25519::{scalarmult, BASE};
    use crate::x25519::{Curve25519PublicKey, SecretKey, SharedCurve25519Secret};
    use irox_tools::{assert_eq_hex_slice, hex};

    type TV = ([u8; 32], [u8; 32], [u8; 32]);
    macro_rules! tv {
        ($scalar:literal, $point:literal, $exp:literal) => {
            (hex!($scalar), hex!($point), hex!($exp))
        };
    }
    static SMULT_TEST_VECTORS: &[TV] = &[
        tv! {"a546e36bf0527c9d3b16154b82465edd62144c0ac1fc5a18506a2244ba449ac4", "e6db6867583030db3594c1a424b15f7c726624ec26b3353b10a903a6d0ab1c4c", "c3da55379de9c6908e94ea4df28d084f32eccf03491c71f754b4075577a28552"},
        tv! {"4b66e9d4d1b4673c5ad22691957d6af5c11b6421e0ea01d42ca4169e7918ba0d", "e5210f12786811d3f4b7959d0538ae2c31dbe7106fc03c3efc4cd549c715a493", "95cbde9476e8907d7aade45cb4b873f88b595a68799fa152e6f8f7647aac7957"},
    ];
    static ALICE_SECRET: SecretKey = SecretKey(hex!(
        "77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a"
    ));
    static ALICE_PUBLIC: Curve25519PublicKey = Curve25519PublicKey(hex!(
        "8520f0098930a754748b7ddcb43ef75a0dbf3a0d26381af4eba4a98eaa9b4e6a"
    ));
    static BOB_SECRET: SecretKey = SecretKey(hex!(
        "5dab087e624a8a4b79e17f8b83800ee66f3bb1292618b6fd1c2f8b27ff88e0eb"
    ));
    static BOB_PUBLIC: Curve25519PublicKey = Curve25519PublicKey(hex!(
        "de9edb7d7b7dc1b4d35b61c2ece435373f8343c85b78674dadfc7e146f882b4f"
    ));
    static ALICE_BOB_SHARED: SharedCurve25519Secret = SharedCurve25519Secret(hex!(
        "4a5d9d5ba4ce2de1728e3bf480350f25e07e21c947d19e3376f09b3c1e161742"
    ));

    #[test]
    pub fn test_alice_pubkey() -> Result<(), Ed25519Error> {
        let pk = ALICE_SECRET.generate_curve25519_pubkey()?;
        assert_eq_hex_slice!(pk.0, ALICE_PUBLIC.0);
        Ok(())
    }
    #[test]
    pub fn test_bob_pubkey() -> Result<(), Ed25519Error>{
        let pk = BOB_SECRET.generate_curve25519_pubkey()?;
        assert_eq_hex_slice!(pk.0, BOB_PUBLIC.0);
        Ok(())
    }
    #[test]
    pub fn test_alice_bob_shared() -> Result<(), Ed25519Error>{
        let alice_shared = ALICE_SECRET.generate_curve25519_shared_secret(&BOB_PUBLIC)?;
        let bob_shared = BOB_SECRET.generate_curve25519_shared_secret(&ALICE_PUBLIC)?;
        assert_eq_hex_slice!(alice_shared.0, bob_shared.0);
        assert_eq_hex_slice!(ALICE_BOB_SHARED.0, alice_shared.0);
        Ok(())
    }

    #[test]
    pub fn scalarmult_test_vectors() -> Result<(), Ed25519Error>{
        for tv in SMULT_TEST_VECTORS {
            let scalar = tv.0;
            let point = tv.1;
            let exp = tv.2;
            let res = scalarmult(&scalar, &point)?;
            assert_eq_hex_slice!(res, exp);
        }
        Ok(())
    }

    macro_rules! impl_iter_vectors {
        ($name:ident, $iter:literal, $exp:literal) => {
            #[test]
            pub fn $name() -> Result<(), Ed25519Error> {
                core_affinity::set_for_current(core_affinity::CoreId { id: 0 });
                let mut k = *BASE;
                let mut u = *BASE;
                let start = std::time::Instant::now();
                let mut start_ctr = irox_arch_x86_64::cpu::rdtsc();

                for i in 0..$iter {
                    let res = scalarmult(&k, &u)?;
                    u = k;
                    k = res;
                    if i % 1000 == 0 {
                        let e = irox_arch_x86_64::cpu::rdtsc();
                        let elapsed = start.elapsed();
                        let progress = (i as f64) / ($iter as f64);
                        let speed = (i as f64) / elapsed.as_secs_f64();
                        let c = (e - start_ctr) as f64 / 1000.;
                        start_ctr = e;
                        let remaining = (elapsed.as_secs_f64() * (1.0 - progress)) / progress;
                        println!(
                            "{}/{} ({:.2}%) in {:.2}s ({:.2}k/s) est: {:.2}s c: {c:.3}",
                            i,
                            $iter,
                            progress * 100.0,
                            elapsed.as_secs_f64(),
                            speed / 1000.0,
                            remaining
                        );
                    }
                }
                assert_eq_hex_slice!(k, hex!($exp));
                Ok(())
            }
        };
    }
    impl_iter_vectors!(
        iter_test_vectors1,
        1,
        "422c8e7a6227d7bca1350b3e2bb7279f7897b87bb6854b783c60e80311ae3079"
    );
    impl_iter_vectors!(
        iter_test_vectors_1k,
        1000,
        "684cf59ba83309552800ef566f2f4d3c1c3887c49360e3875f2eb94d99532c51"
    );
    #[cfg(feature = "_toobig-tests")]
    impl_iter_vectors!(
        iter_test_vectors_1m,
        1000000,
        "7c3911e0ab2586fd864497297e575e6f3bc601c0883c30df5f4dd2d24f665424"
    );
}
