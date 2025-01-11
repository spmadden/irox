// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

///
/// ChaCha20 implementation based on [RFC8349](https://datatracker.ietf.org/doc/html/rfc8439).
use irox_bits::{array_split_16, Bits, BitsWrapper, Error, FromLEBytes, MutBits};
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
        let (a, b) = array_split_16(key);
        let ku = <[u32; 4]>::from_le_bytes(a);
        let kl = <[u32; 4]>::from_le_bytes(b);
        let [k1, k2, k3, k4] = ku;
        let [k5, k6, k7, k8] = kl;
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
impl<'a, B: MutBits> MutBits for ChaCha20Filter<'a, B> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        let p = val ^ self.keystream.next_key();
        self.io.write_u8(p)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.io.flush()
    }
}
impl<'a, B: Bits> Bits for ChaCha20Filter<'a, B> {
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
    use irox_bits::{Bits, Error};
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
    pub fn tv1() -> Result<(), Error> {
        let key = [0u8; 32];
        let mut stream = Chacha20KeyGenerator::generate(key, 1, [0; 12]);

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
    pub fn tv2() -> Result<(), Error> {
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
