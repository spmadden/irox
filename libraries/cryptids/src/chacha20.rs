// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

///
/// ChaCha20 implementation based on [RFC8349](https://datatracker.ietf.org/doc/html/rfc8439).
use irox_bits::{Bits, BitsWrapper, Error, FromLEBytes, MutBits};
use irox_tools::buf::{Buffer, RoundU8Buffer};

const E1: u32 = 0x61707865;
const E2: u32 = 0x3320646e;
const E3: u32 = 0x79622d32;
const E4: u32 = 0x6b206574;
const ROUNDS: usize = 20;
macro_rules! qr {
    (
        $a:expr,
        $b:expr,
        $c:expr,
        $d:expr
    ) => {
        *$a = ($a.wrapping_add(*$b));
        *$d ^= *$a;
        *$d = ($d.rotate_left(16));
        *$c = ($c.wrapping_add(*$d));
        *$b ^= *$c;
        *$b = ($b.rotate_left(12));
        *$a = ($a.wrapping_add(*$b));
        *$d ^= *$a;
        *$d = ($d.rotate_left(8));
        *$c = ($c.wrapping_add(*$d));
        *$b ^= *$c;
        *$b = ($b.rotate_left(7));
    };
}

///
/// Expands the provided key, counter, and nonce into a single 64-byte/512-bit ChaCha20 block.
/// Xor this block byte-for-byte with the ciphertext/plaintext to perform the decrypt/encrypt
/// operation.
///
/// Note: The RFC8349 variant uses a 32 bit block counter, which rolls back to zero every
/// 0x40_0000_0000 consumed key bytes. (256GiB) - ensure the nonce is rotated before this point.
pub struct Chacha20KeyGenerator;
impl Chacha20KeyGenerator {
    pub fn generate(key: [u8; 32], counter: u32, nonce: [u8; 12]) -> RoundU8Buffer<64> {
        let [k1, k2, k3, k4, k5, k6, k7, k8] = <[u32; 8]>::from_le_bytes(key);
        let [n1, n2, n3] = <[u32; 3]>::from_le_bytes(nonce);
        let mut state = [
            E1, E2, E3, E4, k1, k2, k3, k4, k5, k6, k7, k8, counter, n1, n2, n3,
        ];
        for _i in 0..(ROUNDS / 2) {
            qr!(&mut state[0], &mut state[4], &mut state[8], &mut state[12]);
            qr!(&mut state[1], &mut state[5], &mut state[9], &mut state[13]);
            qr!(&mut state[2], &mut state[6], &mut state[10], &mut state[14]);
            qr!(&mut state[3], &mut state[7], &mut state[11], &mut state[15]);

            qr!(&mut state[0], &mut state[5], &mut state[10], &mut state[15]);
            qr!(&mut state[1], &mut state[6], &mut state[11], &mut state[12]);
            qr!(&mut state[2], &mut state[7], &mut state[8], &mut state[13]);
            qr!(&mut state[3], &mut state[4], &mut state[9], &mut state[14]);
        }
        let [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] = state;
        let mut out = RoundU8Buffer::<64>::default();
        let _ = out.write_le_u32(E1.wrapping_add(a));
        let _ = out.write_le_u32(E2.wrapping_add(b));
        let _ = out.write_le_u32(E3.wrapping_add(c));
        let _ = out.write_le_u32(E4.wrapping_add(d));
        let _ = out.write_le_u32(k1.wrapping_add(e));
        let _ = out.write_le_u32(k2.wrapping_add(f));
        let _ = out.write_le_u32(k3.wrapping_add(g));
        let _ = out.write_le_u32(k4.wrapping_add(h));
        let _ = out.write_le_u32(k5.wrapping_add(i));
        let _ = out.write_le_u32(k6.wrapping_add(j));
        let _ = out.write_le_u32(k7.wrapping_add(k));
        let _ = out.write_le_u32(k8.wrapping_add(l));
        let _ = out.write_le_u32(counter.wrapping_add(m));
        let _ = out.write_le_u32(n1.wrapping_add(n));
        let _ = out.write_le_u32(n2.wrapping_add(o));
        let _ = out.write_le_u32(n3.wrapping_add(p));
        out
    }
}

///
/// This struct uses [`Chacha20KeyGenerator`] to repeatedly generate expanded 512-bit
/// key blocks, incrementing the block counter once per-block, and provide the key
/// block one byte at a time using [`next_key`]
///
/// The block counter is initialized by default to 1.
///
/// Note: The RFC8349 variant uses a 32 bit block counter, which rolls back to zero every
/// 0x40_0000_0000 consumed key bytes. (256GiB) - ensure the nonce is rotated before this point.
pub struct Chacha20KeyStream {
    key: [u8; 32],
    counter: u32,
    nonce: [u8; 12],
    buf: RoundU8Buffer<64>,
}
impl Chacha20KeyStream {
    pub fn new(key: [u8; 32], nonce: [u8; 12]) -> Chacha20KeyStream {
        Self {
            key,
            nonce,
            counter: 1,
            buf: RoundU8Buffer::<64>::default(),
        }
    }
    ///
    /// Resets the counter to the specified value and clears out the internal key buffer.
    pub fn set_counter(&mut self, counter: u32) {
        self.counter = counter;
        self.buf.clear();
    }

    ///
    /// Resets the nonce to the specified value and clears out the internal key buffer.
    pub fn set_nonce(&mut self, nonce: [u8; 12]) {
        self.nonce = nonce;
        self.buf.clear();
    }

    ///
    /// Resets the counter and nonce to the specified values and clears out the internal key buffer.
    pub fn set_counter_and_nonce(&mut self, counter: u32, nonce: [u8; 12]) {
        self.counter = counter;
        self.nonce = nonce;
        self.buf.clear();
    }

    ///
    /// Retrieves and returns the next key in the block.  If no keys remain in
    /// the current block, increments the counter
    pub fn next_key(&mut self) -> u8 {
        if self.buf.is_empty() {
            self.buf = Chacha20KeyGenerator::generate(self.key, self.counter, self.nonce);
            self.counter += 1;
        }
        self.buf.pop_front().unwrap_or_default()
    }
}
///
/// Encrypts or Decrypts (processes) the provided data with a [`Chacha20KeyStream`].
///
/// This struct can be re-used by calling [`set_counter(1)`] if the key or nonce isn't expected to
/// change.
///
/// Note: The RFC8349 variant uses a 32 bit block counter, which rolls back to zero every
/// 0x40_0000_0000 consumed key bytes. (256GiB) - ensure the nonce is rotated before this point.
pub struct Chacha20 {
    keystream: Chacha20KeyStream,
}
impl Chacha20 {
    pub fn new(key: [u8; 32], nonce: [u8; 12]) -> Chacha20 {
        Self {
            keystream: Chacha20KeyStream::new(key, nonce),
        }
    }
    ///
    /// Resets and clears the internal keystream block counter to the provided value.
    pub fn set_counter(&mut self, counter: u32) {
        self.keystream.set_counter(counter);
    }

    ///
    /// Resets and clears the internal keystream nonce to the provided value, does NOT reset the
    /// counter.
    pub fn set_nonce(&mut self, nonce: [u8; 12]) {
        self.keystream.set_nonce(nonce);
    }

    ///
    /// Resets and clears the internal block counter and nonce to the provided values.
    pub fn set_counter_and_nonce(&mut self, counter: u32, nonce: [u8; 12]) {
        self.keystream.set_counter_and_nonce(counter, nonce);
    }

    ///
    /// Process (xor) the provided input stream with the ChaCha20 key stream and
    /// write it to the output buffer.  Returns the number of bytes that were consumed from
    /// the input buffer and written to the output buffer.
    pub fn process(&mut self, input: &[u8], output: &mut [u8]) -> usize {
        let mut used = 0;
        for (o, i) in output.iter_mut().zip(input.iter()) {
            *o = *i ^ self.keystream.next_key();
            used += 1;
        }
        used
    }
}

///
/// Passthrough-filter that encrypts or decrypts the provided data using ChaCha20.
///
/// Pass a ciphertext through to decrypt it.  Pass plaintext through to encrypt it.
pub struct ChaCha20Filter<'a, B> {
    keystream: Chacha20KeyStream,
    io: BitsWrapper<'a, B>,
}
impl<'a, B> ChaCha20Filter<'a, B> {
    ///
    /// Creates a new ChaCha20 filter using the provided values.
    pub fn new(key: [u8; 32], nonce: [u8; 12], io: BitsWrapper<'a, B>) -> ChaCha20Filter<'a, B> {
        Self {
            keystream: Chacha20KeyStream::new(key, nonce),
            io,
        }
    }

    ///
    /// Resets and clears the internal keystream block counter to the provided value.
    pub fn set_counter(&mut self, counter: u32) {
        self.keystream.set_counter(counter);
    }

    ///
    /// Resets and clears the internal keystream nonce to the provided value, does NOT reset the
    /// counter.
    pub fn set_nonce(&mut self, nonce: [u8; 12]) {
        self.keystream.set_nonce(nonce);
    }

    ///
    /// Resets and clears the internal block counter and nonce to the provided values.
    pub fn set_counter_and_nonce(&mut self, counter: u32, nonce: [u8; 12]) {
        self.keystream.set_counter_and_nonce(counter, nonce);
    }
}
impl<B: MutBits> MutBits for ChaCha20Filter<'_, B> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        let p = val ^ self.keystream.next_key();
        self.io.write_u8(p)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.io.flush()
    }
}
impl<B: Bits> Bits for ChaCha20Filter<'_, B> {
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        let v = self.io.next_u8()?;
        let Some(v) = v else {
            return Ok(None);
        };
        Ok(Some(v ^ self.keystream.next_key()))
    }
}

#[cfg(test)]
mod test {
    use crate::{Chacha20, Chacha20KeyGenerator};
    use irox_bits::{Bits, Error, FromBEBytes};
    use irox_tools::buf::ZeroedBuffer;
    use irox_tools::{assert_eq_hex, assert_eq_hex_slice};

    #[test]
    pub fn qr() -> Result<(), Error> {
        let mut state: [u32; 4] = [0x11111111, 0x01020304, 0x9b8d6f43, 0x01234567];
        qr!(&mut state[0], &mut state[1], &mut state[2], &mut state[3]);

        assert_eq_hex!(0xea2a92f4, state[0]);
        assert_eq_hex!(0xcb1cf8ce, state[1]);
        assert_eq_hex!(0x4581472e, state[2]);
        assert_eq_hex!(0x5881c4bb, state[3]);

        let mut state: [u32; 4] = [0x516461b1, 0x2a5f714c, 0x53372767, 0x3d631689];
        qr!(&mut state[0], &mut state[1], &mut state[2], &mut state[3]);
        assert_eq_hex!(0xbdb886dc, state[0]);
        assert_eq_hex!(0xcfacafd2, state[1]);
        assert_eq_hex!(0xe46bea80, state[2]);
        assert_eq_hex!(0xccc07c79, state[3]);

        Ok(())
    }

    #[test]
    pub fn tvk1() -> Result<(), Error> {
        let key = [0u8; 32];
        let mut stream = Chacha20KeyGenerator::generate(key, 0, [0; 12]);

        let exp = [
            0x76b8e0ada0f13d90u64,
            0x405d6ae55386bd28,
            0xbdd219b8a08ded1a,
            0xa836efcc8b770dc7,
            0xda41597c5157488d,
            0x7724e03fb8d84a37,
            0x6a43b8f41518a11c,
            0xc387b669b2ee6586,
        ];

        for e in exp {
            assert_eq_hex!(e, stream.read_be_u64()?);
        }

        Ok(())
    }

    #[test]
    pub fn tvk2() -> Result<(), Error> {
        let key = [0u8; 32];
        let mut stream = Chacha20KeyGenerator::generate(key, 1, [0; 12]);

        let exp = [
            0x9f07e7be5551387a98ba977c732d080du128,
            0xcb0f29a048e3656912c6533e32ee7aed,
            0x29b721769ce64e43d57133b074d839d5,
            0x31ed1f28510afb45ace10a1f4b794d6f,
        ];

        for e in exp {
            assert_eq_hex!(e, stream.read_be_u128()?);
        }

        Ok(())
    }
    #[test]
    pub fn tvk3() -> Result<(), Error> {
        let mut key = [0u8; 32];
        key[31] = 0x1;
        let mut stream = Chacha20KeyGenerator::generate(key, 1, [0; 12]);

        let exp = [
            0x3aeb5224ecf849929b9d828db1ced4ddu128,
            0x832025e8018b8160b82284f3c949aa5a,
            0x8eca00bbb4a73bdad192b5c42f73f2fd,
            0x4e273644c8b36125a64addeb006c13a0,
        ];

        for e in exp {
            assert_eq_hex!(e, stream.read_be_u128()?);
        }

        Ok(())
    }

    #[test]
    pub fn tvk4() -> Result<(), Error> {
        let mut key = [0u8; 32];
        key[1] = 0xff;
        let mut stream = Chacha20KeyGenerator::generate(key, 2, [0; 12]);

        let exp = [
            0x72d54dfbf12ec44b362692df94137f32u128,
            0x8fea8da73990265ec1bbbea1ae9af0ca,
            0x13b25aa26cb4a648cb9b9d1be65b2c09,
            0x24a66c54d545ec1b7374f4872e99f096,
        ];

        for e in exp {
            assert_eq_hex!(e, stream.read_be_u128()?);
        }

        Ok(())
    }

    #[test]
    pub fn tvk5() -> Result<(), Error> {
        let key = [0u8; 32];
        let nonce = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2];

        let mut stream = Chacha20KeyGenerator::generate(key, 0, nonce);

        let exp = [
            0xc2c64d378cd536374ae204b9ef933fcdu128,
            0x1a8b2288b3dfa49672ab765b54ee27c7,
            0x8a970e0e955c14f3a88e741b97c286f7,
            0x5f8fc299e8148362fa198a39531bed6d,
        ];

        for e in exp {
            assert_eq_hex!(e, stream.read_be_u128()?);
        }

        Ok(())
    }

    #[test]
    pub fn tve1() -> Result<(), Error> {
        let key = [0u8; 32];
        let nonce = [0u8; 12];
        let pt = [0u8; 64];

        let mut out = [0u8; 64];
        let mut enc = Chacha20::new(key, nonce);
        enc.set_counter(0);
        let processed = enc.process(&pt, &mut out);

        assert_eq!(processed, 64);

        let exp = [
            0x76b8e0ada0f13d90u64,
            0x405d6ae55386bd28,
            0xbdd219b8a08ded1a,
            0xa836efcc8b770dc7,
            0xda41597c5157488d,
            0x7724e03fb8d84a37,
            0x6a43b8f41518a11c,
            0xc387b669b2ee6586,
        ];

        let out = <[u64; 8]>::from_be_bytes(out);

        assert_eq_hex_slice!(&out, &exp);
        Ok(())
    }

    #[test]
    pub fn tve2() -> Result<(), Error> {
        let mut key = [0u8; 32];
        key[31] = 0x1;
        let mut nonce = [0u8; 12];
        nonce[11] = 0x2;

        let pt = concat!(
            "Any submission to the IETF intended by the Contributor for publi",
            "cation as all or part of an IETF Internet-Draft or RFC and any s",
            "tatement made within the context of an IETF activity is consider",
            "ed an \"IETF Contribution\". Such statements include oral statemen",
            "ts in IETF sessions, as well as written and electronic communica",
            "tions made at any time or place, which are addressed to",
        );
        let pt = pt.as_bytes();

        let mut out = Vec::<u8>::new_zeroed(pt.len());
        let mut enc = Chacha20::new(key, nonce);
        enc.set_counter(1);
        let processed = enc.process(&pt, &mut out);

        assert_eq!(processed, pt.len());

        let exp = [
            0xa3, 0xfb, 0xf0, 0x7d, 0xf3, 0xfa, 0x2f, 0xde, 0x4f, 0x37, 0x6c, 0xa2, 0x3e, 0x82,
            0x73, 0x70, 0x41, 0x60, 0x5d, 0x9f, 0x4f, 0x4f, 0x57, 0xbd, 0x8c, 0xff, 0x2c, 0x1d,
            0x4b, 0x79, 0x55, 0xec, 0x2a, 0x97, 0x94, 0x8b, 0xd3, 0x72, 0x29, 0x15, 0xc8, 0xf3,
            0xd3, 0x37, 0xf7, 0xd3, 0x70, 0x05, 0x0e, 0x9e, 0x96, 0xd6, 0x47, 0xb7, 0xc3, 0x9f,
            0x56, 0xe0, 0x31, 0xca, 0x5e, 0xb6, 0x25, 0x0d, 0x40, 0x42, 0xe0, 0x27, 0x85, 0xec,
            0xec, 0xfa, 0x4b, 0x4b, 0xb5, 0xe8, 0xea, 0xd0, 0x44, 0x0e, 0x20, 0xb6, 0xe8, 0xdb,
            0x09, 0xd8, 0x81, 0xa7, 0xc6, 0x13, 0x2f, 0x42, 0x0e, 0x52, 0x79, 0x50, 0x42, 0xbd,
            0xfa, 0x77, 0x73, 0xd8, 0xa9, 0x05, 0x14, 0x47, 0xb3, 0x29, 0x1c, 0xe1, 0x41, 0x1c,
            0x68, 0x04, 0x65, 0x55, 0x2a, 0xa6, 0xc4, 0x05, 0xb7, 0x76, 0x4d, 0x5e, 0x87, 0xbe,
            0xa8, 0x5a, 0xd0, 0x0f, 0x84, 0x49, 0xed, 0x8f, 0x72, 0xd0, 0xd6, 0x62, 0xab, 0x05,
            0x26, 0x91, 0xca, 0x66, 0x42, 0x4b, 0xc8, 0x6d, 0x2d, 0xf8, 0x0e, 0xa4, 0x1f, 0x43,
            0xab, 0xf9, 0x37, 0xd3, 0x25, 0x9d, 0xc4, 0xb2, 0xd0, 0xdf, 0xb4, 0x8a, 0x6c, 0x91,
            0x39, 0xdd, 0xd7, 0xf7, 0x69, 0x66, 0xe9, 0x28, 0xe6, 0x35, 0x55, 0x3b, 0xa7, 0x6c,
            0x5c, 0x87, 0x9d, 0x7b, 0x35, 0xd4, 0x9e, 0xb2, 0xe6, 0x2b, 0x08, 0x71, 0xcd, 0xac,
            0x63, 0x89, 0x39, 0xe2, 0x5e, 0x8a, 0x1e, 0x0e, 0xf9, 0xd5, 0x28, 0x0f, 0xa8, 0xca,
            0x32, 0x8b, 0x35, 0x1c, 0x3c, 0x76, 0x59, 0x89, 0xcb, 0xcf, 0x3d, 0xaa, 0x8b, 0x6c,
            0xcc, 0x3a, 0xaf, 0x9f, 0x39, 0x79, 0xc9, 0x2b, 0x37, 0x20, 0xfc, 0x88, 0xdc, 0x95,
            0xed, 0x84, 0xa1, 0xbe, 0x05, 0x9c, 0x64, 0x99, 0xb9, 0xfd, 0xa2, 0x36, 0xe7, 0xe8,
            0x18, 0xb0, 0x4b, 0x0b, 0xc3, 0x9c, 0x1e, 0x87, 0x6b, 0x19, 0x3b, 0xfe, 0x55, 0x69,
            0x75, 0x3f, 0x88, 0x12, 0x8c, 0xc0, 0x8a, 0xaa, 0x9b, 0x63, 0xd1, 0xa1, 0x6f, 0x80,
            0xef, 0x25, 0x54, 0xd7, 0x18, 0x9c, 0x41, 0x1f, 0x58, 0x69, 0xca, 0x52, 0xc5, 0xb8,
            0x3f, 0xa3, 0x6f, 0xf2, 0x16, 0xb9, 0xc1, 0xd3, 0x00, 0x62, 0xbe, 0xbc, 0xfd, 0x2d,
            0xc5, 0xbc, 0xe0, 0x91, 0x19, 0x34, 0xfd, 0xa7, 0x9a, 0x86, 0xf6, 0xe6, 0x98, 0xce,
            0xd7, 0x59, 0xc3, 0xff, 0x9b, 0x64, 0x77, 0x33, 0x8f, 0x3d, 0xa4, 0xf9, 0xcd, 0x85,
            0x14, 0xea, 0x99, 0x82, 0xcc, 0xaf, 0xb3, 0x41, 0xb2, 0x38, 0x4d, 0xd9, 0x02, 0xf3,
            0xd1, 0xab, 0x7a, 0xc6, 0x1d, 0xd2, 0x9c, 0x6f, 0x21, 0xba, 0x5b, 0x86, 0x2f, 0x37,
            0x30, 0xe3, 0x7c, 0xfd, 0xc4, 0xfd, 0x80, 0x6c, 0x22, 0xf2, 0x21,
        ];

        assert_eq_hex_slice!(&out, &exp);
        Ok(())
    }

    #[test]
    pub fn tve3() -> Result<(), Error> {
        let key = [
            0x1c, 0x92, 0x40, 0xa5, 0xeb, 0x55, 0xd3, 0x8a, 0xf3, 0x33, 0x88, 0x86, 0x04, 0xf6,
            0xb5, 0xf0, 0x47, 0x39, 0x17, 0xc1, 0x40, 0x2b, 0x80, 0x09, 0x9d, 0xca, 0x5c, 0xbc,
            0x20, 0x70, 0x75, 0xc0,
        ];
        let nonce = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2];

        let pt = concat!(
            "'Twas brillig, and the slithy toves\n",
            "Did gyre and gimble in the wabe:\n",
            "All mimsy were the borogoves,\n",
            "And the mome raths outgrabe.",
        );
        let pt = pt.as_bytes();

        let mut out = [0u8; 127];
        let mut enc = Chacha20::new(key, nonce);
        enc.set_counter(42);
        let processed = enc.process(&pt, &mut out);

        assert_eq!(processed, pt.len());

        let exp = [
            0x62, 0xe6, 0x34, 0x7f, 0x95, 0xed, 0x87, 0xa4, 0x5f, 0xfa, 0xe7, 0x42, 0x6f, 0x27,
            0xa1, 0xdf, 0x5f, 0xb6, 0x91, 0x10, 0x04, 0x4c, 0x0d, 0x73, 0x11, 0x8e, 0xff, 0xa9,
            0x5b, 0x01, 0xe5, 0xcf, 0x16, 0x6d, 0x3d, 0xf2, 0xd7, 0x21, 0xca, 0xf9, 0xb2, 0x1e,
            0x5f, 0xb1, 0x4c, 0x61, 0x68, 0x71, 0xfd, 0x84, 0xc5, 0x4f, 0x9d, 0x65, 0xb2, 0x83,
            0x19, 0x6c, 0x7f, 0xe4, 0xf6, 0x05, 0x53, 0xeb, 0xf3, 0x9c, 0x64, 0x02, 0xc4, 0x22,
            0x34, 0xe3, 0x2a, 0x35, 0x6b, 0x3e, 0x76, 0x43, 0x12, 0xa6, 0x1a, 0x55, 0x32, 0x05,
            0x57, 0x16, 0xea, 0xd6, 0x96, 0x25, 0x68, 0xf8, 0x7d, 0x3f, 0x3f, 0x77, 0x04, 0xc6,
            0xa8, 0xd1, 0xbc, 0xd1, 0xbf, 0x4d, 0x50, 0xd6, 0x15, 0x4b, 0x6d, 0xa7, 0x31, 0xb1,
            0x87, 0xb5, 0x8d, 0xfd, 0x72, 0x8a, 0xfa, 0x36, 0x75, 0x7a, 0x79, 0x7a, 0xc1, 0x88,
            0xd1,
        ];

        assert_eq_hex_slice!(&out, &exp);
        Ok(())
    }

    #[test]
    pub fn tvka() -> Result<(), Error> {
        let key = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
            0x1c, 0x1d, 0x1e, 0x1f,
        ];
        let nonce = [0, 0, 0, 9, 0, 0, 0, 0x4a, 0, 0, 0, 0];
        let counter = 0x00000001;
        let mut stream = Chacha20KeyGenerator::generate(key, counter, nonce);
        let exp = [
            0xe4e7f110, 0x15593bd1, 0x1fdd0f50, 0xc47120a3, 0xc7f4d1c7, 0x0368c033, 0x9aaa2204,
            0x4e6cd4c3, 0x466482d2, 0x09aa9f07, 0x05d7c214, 0xa2028bd9, 0xd19c12b5, 0xb94e16de,
            0xe883d0cb, 0x4e3c50a2,
        ];

        for e in exp {
            assert_eq_hex!(e, stream.read_le_u32()?);
        }

        let plaintext = "Ladies and Gentlemen of the class of '99: If I could offer you only one tip for the future, sunscreen would be it.";
        let plaintext = plaintext.as_bytes();
        let mut encrypted = Vec::<u8>::new_zeroed(plaintext.len());
        assert_eq!(encrypted.len(), plaintext.len());
        let nonce = [0, 0, 0, 0, 0, 0, 0, 0x4a, 0, 0, 0, 0];
        let mut encryptor = Chacha20::new(key, nonce);
        let length = encryptor.process(plaintext, &mut encrypted);
        assert_eq!(length, encrypted.len());

        let expected = &[
            0x6e, 0x2e, 0x35, 0x9a, 0x25, 0x68, 0xf9, 0x80, 0x41, 0xba, 0x07, 0x28, 0xdd, 0x0d,
            0x69, 0x81, 0xe9, 0x7e, 0x7a, 0xec, 0x1d, 0x43, 0x60, 0xc2, 0x0a, 0x27, 0xaf, 0xcc,
            0xfd, 0x9f, 0xae, 0x0b, 0xf9, 0x1b, 0x65, 0xc5, 0x52, 0x47, 0x33, 0xab, 0x8f, 0x59,
            0x3d, 0xab, 0xcd, 0x62, 0xb3, 0x57, 0x16, 0x39, 0xd6, 0x24, 0xe6, 0x51, 0x52, 0xab,
            0x8f, 0x53, 0x0c, 0x35, 0x9f, 0x08, 0x61, 0xd8, 0x07, 0xca, 0x0d, 0xbf, 0x50, 0x0d,
            0x6a, 0x61, 0x56, 0xa3, 0x8e, 0x08, 0x8a, 0x22, 0xb6, 0x5e, 0x52, 0xbc, 0x51, 0x4d,
            0x16, 0xcc, 0xf8, 0x06, 0x81, 0x8c, 0xe9, 0x1a, 0xb7, 0x79, 0x37, 0x36, 0x5a, 0xf9,
            0x0b, 0xbf, 0x74, 0xa3, 0x5b, 0xe6, 0xb4, 0x0b, 0x8e, 0xed, 0xf2, 0x78, 0x5e, 0x42,
            0x87, 0x4d,
        ];
        assert_eq_hex_slice!(expected, &encrypted);
        let mut recovered = Vec::<u8>::new_zeroed(encrypted.len());
        assert_eq!(recovered.len(), encrypted.len());

        encryptor.set_counter(1);
        let length = encryptor.process(&encrypted, &mut recovered);
        assert_eq!(length, recovered.len());
        assert_eq!(plaintext.len(), recovered.len());

        assert_eq_hex_slice!(plaintext, &recovered);

        Ok(())
    }
}
