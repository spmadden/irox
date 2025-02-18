// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![allow(clippy::indexing_slicing)]
#![allow(clippy::manual_memcpy)]

use core::ops::{Add, Index, IndexMut, Mul, Not, Sub};

const fn basepoint() -> [u8; 32] {
    let mut out = [0u8; 32];
    out[0] = 9;
    out
}
///
/// Curve25519 Base Point - '9'
pub const BASE: [u8; 32] = basepoint();

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
    pub fn generate_curve25519_pubkey(&self) -> Curve25519PublicKey {
        Curve25519PublicKey(scalarmult(&self.0, &BASE))
    }
    ///
    /// Generates a shared key using the original Curve25519 method of multiplying owned secret key with
    /// a Curve25519PublicKey.  This is probably not what you want.
    pub fn generate_curve25519_shared_secret(
        &self,
        pubkey: &Curve25519PublicKey,
    ) -> SharedCurve25519Secret {
        SharedCurve25519Secret(scalarmult(&self.0, &pubkey.0))
    }
}
impl Curve25519PublicKey {
    ///
    /// Generates a shared key using the original Curve25519 method of multiplying owned secret key with
    /// a Curve25519PublicKey.  This is probably not what you want. 
    pub fn generate_shared_secret(&self, secret_key: &SecretKey) -> SharedCurve25519Secret {
        SharedCurve25519Secret(scalarmult(&secret_key.0, &self.0))
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

pub fn scalarmult(scalar: &[u8; 32], point: &[u8; 32]) -> [u8; 32] {
    let mut clamp = *scalar;
    clamp[0] &= 0xF8;
    clamp[31] &= 0x7F;
    clamp[31] |= 0x40;

    let x = FieldElement::unpack(point);
    let mut b = x;
    let mut a = FieldElement([0i64; 16]);
    let mut d = a;
    let mut c = a;
    let mut e: FieldElement;
    let mut f: FieldElement;
    a[0] = 1;
    d[0] = 1;
    for i in 0..=254 {
        let i = 254 - i;
        let bit = ((clamp[i >> 3] >> (i & 0x7)) & 1) as i64;
        a.swap(&mut b, bit);
        c.swap(&mut d, bit);
        e = a + c;
        a = a - c;
        c = b + d;
        b = b - d;
        d = e * e;
        f = a * a;
        a = c * a;
        c = b * e;
        e = a + c;
        a = a - c;
        b = a * a;
        c = d - f;
        a = c * DB41;
        a = a + d;
        c = c * a;
        a = d * f;
        d = b * x;
        b = e * e;
        a.swap(&mut b, bit);
        c.swap(&mut d, bit);
    }
    c = c.not();
    a = a * c;
    a.pack()
}

#[derive(Clone, Copy)]
struct FieldElement([i64; 16]);
const DB41: FieldElement = FieldElement([0xDB41, 0x1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
impl FieldElement {
    fn carry(&mut self) {
        let mut carry;
        for i in 0..16 {
            carry = self[i] >> 16;
            self[i] -= carry << 16;
            if i < 15 {
                self[i + 1] += carry;
            } else {
                self[0] += 38 * carry;
            }
        }
    }

    fn swap(&mut self, other: &mut Self, bit: i64) {
        let c = !(bit - 1);
        for i in 0..16 {
            let t = c & (self[i] ^ other[i]);
            self[i] ^= t;
            other[i] ^= t;
        }
    }
    fn pack(self) -> [u8; 32] {
        let mut t = self;
        t.carry();
        t.carry();
        t.carry();
        let mut m = FieldElement([0; 16]);
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
    fn unpack(inp: &[u8; 32]) -> Self {
        let mut out = FieldElement([0; 16]);
        for i in 0..16 {
            out[i] = (inp[2 * i] as i64) | ((inp[2 * i + 1] as i64) << 8);
        }
        out[15] &= 0x7FFF;
        out
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
impl Add for FieldElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = [0i64; 16];
        for i in 0..16 {
            out[i] = self[i] + rhs[i];
        }
        FieldElement(out)
    }
}
impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut out = [0i64; 16];
        for i in 0..16 {
            out[i] = self[i] - rhs[i];
        }
        FieldElement(out)
    }
}
impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut t = [0i64; 31];
        for i in 0..16 {
            for j in 0..16 {
                t[i + j] += self[i] * rhs[j];
            }
        }
        for i in 0..15 {
            t[i] += 38 * t[i + 16];
        }
        let mut out = [0i64; 16];
        for i in 0..16 {
            out[i] = t[i];
        }
        let mut out = FieldElement(out);
        out.carry();
        out.carry();
        out
    }
}
impl Not for FieldElement {
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut out = self;
        for i in 0..=253 {
            let i = 253 - i;
            out = out * out;
            if i != 2 && i != 4 {
                out = out * self;
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use crate::x25519::{scalarmult, BASE};
    use crate::x25519::{Curve25519PublicKey, SecretKey, SharedCurve25519Secret};
    use irox_bits::BitsError;
    use irox_tools::{assert_eq_hex_slice, hex};
    use std::io::BufRead;

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
    pub fn test_alice_pubkey() {
        let pk = ALICE_SECRET.generate_curve25519_pubkey();
        assert_eq_hex_slice!(pk.0, ALICE_PUBLIC.0);
    }
    #[test]
    pub fn test_bob_pubkey() {
        let pk = BOB_SECRET.generate_curve25519_pubkey();
        assert_eq_hex_slice!(pk.0, BOB_PUBLIC.0);
    }
    #[test]
    pub fn test_alice_bob_shared() {
        let alice_shared = ALICE_SECRET.generate_curve25519_shared_secret(&BOB_PUBLIC);
        let bob_shared = BOB_SECRET.generate_curve25519_shared_secret(&ALICE_PUBLIC);
        assert_eq_hex_slice!(alice_shared.0, bob_shared.0);
        assert_eq_hex_slice!(ALICE_BOB_SHARED.0, alice_shared.0);
    }

    #[test]
    pub fn scalarmult_test_vectors() {
        for tv in SMULT_TEST_VECTORS {
            let scalar = tv.0;
            let point = tv.1;
            let exp = tv.2;
            let res = scalarmult(&scalar, &point);
            assert_eq_hex_slice!(res, exp);
        }
    }

    macro_rules! impl_iter_vectors {
        ($name:ident, $iter:literal, $exp:literal) => {
            #[test]
            pub fn $name() {
                core_affinity::set_for_current(core_affinity::CoreId { id: 0 });
                let mut k = BASE;
                let mut u = BASE;
                let start = std::time::Instant::now();
                let mut start_ctr = irox_arch_x86_64::cpu::rdtsc();

                for i in 0..$iter {
                    let res = scalarmult(&k, &u);
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
                assert_eq_hex_slice!(k, hex!($exp))
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

    pub fn _test_vectors_1() -> Result<(), BitsError> {
        struct TV {
            msg: String,
            pbk: String,
            sig: String,
        }
        let f = std::fs::OpenOptions::new()
            .read(true)
            .create(false)
            .open("doc/x25519-test-vectors.txt")?;
        let mut f = std::io::BufReader::new(f);

        for line in f.lines() {
            let line = line?;

            let Some((ty, data)) = line.split_once("=") else {
                continue;
            };
        }

        Ok(())
    }

    pub fn _test_reject_small_order() {
        // let m1 = hex!("53656 e 6 4 2 0 3 1 3 0 3 0 2 0 5 5 5 3 4 4 2 0 7 4 6 f 2 0 4 1 6 c 6 9 6 3 6 5 ");
    }
}
