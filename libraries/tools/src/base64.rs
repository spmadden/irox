// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! RFC-4648 Compliant Base64, Base32, and Base16 encoders and decoders
//!
extern crate alloc;
use crate::bits::{Bits, Error, ErrorKind, MutBits};
use crate::codec::Codec;
use alloc::collections::BTreeMap;
use alloc::string::String;

/// `A-Z,a-z,0-9,+,/` - not filesystem or URL-safe
pub static BASE64_ALPHABET: [u8; 64] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
    b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'+', b'/',
];
/// `A-Z,a-z,0-9,-,_` - filesystem and URL-safe
pub static BASE64URL_ALPHABET: [u8; 64] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
    b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'-', b'_',
];
/// `A-Z,2-7` - filesystem and URL-safe
pub static BASE32_ALPHABET: [u8; 32] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'2', b'3', b'4', b'5', b'6', b'7',
];
/// `0-9,A-V` - "Extended Hex", filesystem and URL-safe
pub static BASE32HEX_ALPHABET: [u8; 32] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
    b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V',
];
/// `0-9,A-F` - Standard Hex.
pub static BASE16_ALPHABET: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
];

macro_rules! getalpha {
    ($alpha:ident,$idx:tt) => {
        $alpha.get($idx).map(|v| *v).unwrap_or_default()
    };
}
///
/// A Six Bit Codec encodes 3x eight-bit bytes (24 bits) into 4x six-bit symbols.  Likewise, reads
/// 4x six-bit symbols and decodes them into 3x eight-bit bytes.
///
/// The alphabet size is `2^6 = 64` symbols.
pub struct SixBitCodec {
    alphabet: &'static [u8; 64],
    reverse: BTreeMap<u8, u8>,
    fail_on_invalid_decode: bool,
    pad: u8,
}
impl SixBitCodec {
    /// Creates a new codec, using the provided alphabet.
    pub fn new(alphabet: &'static [u8; 64]) -> Self {
        let reverse: BTreeMap<u8, u8> = alphabet
            .iter()
            .enumerate()
            .map(|(idx, v)| (*v, idx as u8))
            .collect::<BTreeMap<_, _>>();
        SixBitCodec {
            alphabet,
            reverse,
            fail_on_invalid_decode: false,
            pad: b'=',
        }
    }
    /// sets the end padding character (defaults to `'='`)
    pub fn set_pad(&mut self, pad: u8) -> &mut Self {
        self.pad = pad;
        self
    }
    /// if set, decoding will return an error on invalid character - otherwise will just skip it.
    pub fn set_fail_on_invalid_character(&mut self) -> &mut Self {
        self.fail_on_invalid_decode = true;
        self
    }
}
impl Codec for SixBitCodec {
    fn encode<I: Bits, O: MutBits>(&self, mut input: I, output: &mut O) -> Result<usize, Error> {
        let mut buf: u32 = 0;
        let mut ctr = 0;
        let mut written = 0;
        let alpha = self.alphabet;
        loop {
            let Some(v) = input.next_u8()? else {
                break;
            };

            buf <<= 8;
            buf |= v as u32;
            ctr += 1;
            if ctr == 3 {
                let a = ((buf & 0xFC_0000) >> 18) as usize;
                let b = ((buf & 0x03_F000) >> 12) as usize;
                let c = ((buf & 0x00_0FC0) >> 6) as usize;
                let d = (buf & 0x00_003F) as usize;
                output.write_all_bytes(&[
                    getalpha!(alpha, a),
                    getalpha!(alpha, b),
                    getalpha!(alpha, c),
                    getalpha!(alpha, d),
                ])?;
                ctr = 0;
                buf = 0;
                written += 4;
            }
        }
        if ctr == 2 {
            buf <<= 2;
            let a = ((buf & 0x03_F000) >> 12) as usize;
            let b = ((buf & 0x00_0FC0) >> 6) as usize;
            let c = (buf & 0x00_003F) as usize;
            output.write_all_bytes(&[
                getalpha!(alpha, a),
                getalpha!(alpha, b),
                getalpha!(alpha, c),
                self.pad,
            ])?;
            written += 4;
        } else if ctr == 1 {
            buf <<= 4;
            let a = ((buf & 0xFC0) >> 6) as usize;
            let b = (buf & 0x030) as usize;
            output.write_all_bytes(&[
                getalpha!(alpha, a),
                getalpha!(alpha, b),
                self.pad,
                self.pad,
            ])?;
            written += 4;
        }
        Ok(written)
    }

    fn decode<I: Bits, O: MutBits>(&self, mut input: I, output: &mut O) -> Result<usize, Error> {
        let mut buf: u32 = 0;
        let mut ctr = 0;
        let mut written = 0;
        loop {
            let Some(var) = input.next_u8()? else {
                break;
            };
            if var == self.pad {
                continue;
            }
            let Some(dec) = self.reverse.get(&var) else {
                if self.fail_on_invalid_decode {
                    return Err(ErrorKind::InvalidData.into());
                }
                continue;
            };
            buf <<= 6;
            buf |= *dec as u32;
            ctr += 1;
            if ctr == 4 {
                let [_, a, b, c] = buf.to_be_bytes();
                output.write_all_bytes(&[a, b, c])?;
                ctr = 0;
                buf = 0;
                written += 3;
            }
        }
        if ctr == 3 {
            buf >>= 2;
            output.write_be_u16((buf & 0xFFFF) as u16)?;
            written += 2;
        } else if ctr == 2 {
            // write 1
            buf >>= 4;
            output.write_u8((buf & 0xFF) as u8)?;
            written += 1;
        } else if ctr == 1 {
            // invalid!
        }

        Ok(written)
    }
}
/// Creates and returns a [`SixBitCodec`] compliant with the RFC4648 "Base64" standard alphabet
/// ([`BASE64_ALPHABET`]) - this alphabet contains characters inconsistent with URLs and Filenames.
pub fn new_base64_codec() -> SixBitCodec {
    SixBitCodec::new(&BASE64_ALPHABET)
}
/// Creates and returns a [`SixBitCodec`] compliant with the RFC4648 "Base64 URL" standard alphabet
/// ([`BASE64URL_ALPHABET`]) - this alphabet contains characters compatible with URLs and Filenames.
pub fn new_base64_safe_codec() -> SixBitCodec {
    SixBitCodec::new(&BASE64URL_ALPHABET)
}

/// Encodes the provided the input, writing the encoding to output, using the standard RFC-4648
/// [`BASE64_ALPHABET`], upon success, returns the number of bytes written out
pub fn base64_encode<I: Bits, O: MutBits>(input: I, output: &mut O) -> Result<usize, Error> {
    new_base64_codec().encode(input, output)
}
/// Decodes the provided the input, writing the decoded data to output, using the standard RFC-4648
/// [`BASE64_ALPHABET`], upon success, returns the number of bytes written out
pub fn base64_decode<I: Bits, O: MutBits>(input: I, output: &mut O) -> Result<usize, Error> {
    new_base64_codec().decode(input, output)
}
/// Encodes the provided input to a string, using the standard RFC-4648 [`BASE64_ALPHABET`]
pub fn base64_encode_to_str<I: Bits>(input: I) -> Result<String, Error> {
    new_base64_codec().encode_to_str(input)
}
/// Decodes the provided input to a string, using the standard RFC-4648 [`BASE64_ALPHABET`], dropping
/// any characters that aren't UTF-8.
pub fn base64_decode_to_str_lossy<I: Bits>(input: I) -> Result<String, Error> {
    new_base64_codec().decode_to_str_lossy(input)
}

/// Encodes the provided the input, writing the encoding to output, using the filesystem and
/// URL-safe RFC-4648 [`BASE64URL_ALPHABET`], , upon success, returns the number of bytes written out
pub fn base64_encode_safe<I: Bits, O: MutBits>(input: I, output: &mut O) -> Result<usize, Error> {
    new_base64_safe_codec().encode(input, output)
}
/// Decodes the provided the input, writing the decoded data to output, using the filesystem and
/// URL-safe RFC-4648 [`BASE64URL_ALPHABET`], upon success, returns the number of bytes written out
pub fn base64_decode_safe<I: Bits, O: MutBits>(input: I, output: &mut O) -> Result<usize, Error> {
    new_base64_safe_codec().decode(input, output)
}
/// Encodes the provided input to a string, using the using the filesystem and
/// URL-safe RFC-4648 [`BASE64URL_ALPHABET`]
pub fn base64_encode_safe_to_str<I: Bits>(input: I) -> Result<String, Error> {
    new_base64_safe_codec().encode_to_str(input)
}
/// Decodes the provided the input, to a string, using the filesystem and URL-safe RFC-4648
/// [`BASE64URL_ALPHABET`], any characters not valid UTF-8 are dropped.
pub fn base64_decode_safe_to_str_lossy<I: Bits>(input: I) -> Result<String, Error> {
    new_base64_safe_codec().decode_to_str_lossy(input)
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use crate::base64::new_base64_codec;
    use crate::codec::Codec;

    #[allow(clippy::panic_in_result_fn)]
    #[test]
    pub fn base64_tests() -> Result<(), std::io::Error> {
        let codec = new_base64_codec();
        let tests: [(&str, &str); 7] = [
            ("", ""),
            ("f", "Zg=="),
            ("fo", "Zm8="),
            ("foo", "Zm9v"),
            ("foob", "Zm9vYg=="),
            ("fooba", "Zm9vYmE="),
            ("foobar", "Zm9vYmFy"),
        ];
        for (i, o) in tests {
            assert_eq!(o, codec.encode_to_str(i.as_bytes())?);
            assert_eq!(i, codec.decode_to_str_lossy(o.as_bytes())?);
        }

        Ok(())
    }
}
