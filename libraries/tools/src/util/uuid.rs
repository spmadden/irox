// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

//!
//! A basic implementation of a UUID
//!

use core::fmt::{Display, Formatter};

use irox_bits::{Bits, Error, MutBits};

///
/// A basic UUID structure.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UUID {
    inner: u128,
}

impl From<u128> for UUID {
    fn from(value: u128) -> Self {
        UUID { inner: value }
    }
}

impl From<&u128> for UUID {
    fn from(value: &u128) -> Self {
        UUID { inner: *value }
    }
}

impl From<UUID> for u128 {
    fn from(value: UUID) -> Self {
        value.inner
    }
}

impl From<&UUID> for u128 {
    fn from(value: &UUID) -> Self {
        value.inner
    }
}
impl From<UUID> for [u8; 16] {
    fn from(value: UUID) -> Self {
        value.inner.to_be_bytes()
    }
}

impl From<&UUID> for [u8; 16] {
    fn from(value: &UUID) -> Self {
        value.inner.to_be_bytes()
    }
}

impl From<[u8; 16]> for UUID {
    fn from(value: [u8; 16]) -> Self {
        u128::from_be_bytes(value).into()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UUIDParseError {
    WrongSize,
    InvalidCharacter,
}

impl TryFrom<&[u8]> for UUID {
    type Error = UUIDParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 16 {
            return Err(UUIDParseError::WrongSize);
        }
        let mut inner = 0u128;
        let mut shift = 128;
        for b in value {
            shift -= 8;
            let b = (*b as u128).wrapping_shl(shift);
            inner |= b;
        }
        Ok(UUID { inner })
    }
}

impl TryFrom<&str> for UUID {
    type Error = UUIDParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let no_dashes = value.replace('-', "");
        if no_dashes.len() != 32 {
            return Err(UUIDParseError::WrongSize);
        }
        let mut inner: u128 = 0;
        let mut shift = 128;
        for c in no_dashes.as_bytes() {
            let val = match *c as char {
                '0' => 0u8,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'A' | 'a' => 10,
                'B' | 'b' => 11,
                'C' | 'c' => 12,
                'D' | 'd' => 13,
                'E' | 'e' => 14,
                'F' | 'f' => 15,
                _ => return Err(UUIDParseError::InvalidCharacter),
            };
            shift -= 4;
            inner |= (val as u128).wrapping_shl(shift);
        }
        Ok(UUID { inner })
    }
}

///
/// A trait that can be applied to a Reader, or other bit stream.
pub trait UUIDReader {
    ///
    /// Attempts to read a UUID from this data source, returning the UUID read, or an error if it
    /// could not.
    fn read_uuid(&mut self) -> Result<UUID, Error>;
}

impl<T> UUIDReader for T
where
    T: Bits,
{
    fn read_uuid(&mut self) -> Result<UUID, Error> {
        Ok(self.read_be_u128()?.into())
    }
}

///
/// A trait that can be applied to a Writer, or other bit stream.
pub trait UUIDWriter {
    ///
    /// Attempts to write a UUID to this data source
    fn write_uuid(&mut self, uuid: &UUID) -> Result<(), Error>;
}

impl<T> UUIDWriter for T
where
    T: MutBits,
{
    fn write_uuid(&mut self, uuid: &UUID) -> Result<(), Error> {
        self.write_be_u128(uuid.inner)
    }
}

impl Display for UUID {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // 8-4-4-4-12 chars : 4-2-2-2-6 bytes : 32-16-16-16-48 bits
        let a = (self.inner & 0xFFFFFFFF_0000_0000_0000_000000000000) >> 96;
        let b = (self.inner & 0x00000000_FFFF_0000_0000_000000000000) >> 80;
        let c = (self.inner & 0x00000000_0000_FFFF_0000_000000000000) >> 64;
        let d = (self.inner & 0x00000000_0000_0000_FFFF_000000000000) >> 48;
        let e = self.inner & 0x00000000_0000_0000_0000_FFFFFFFFFFFF;
        f.write_fmt(format_args!("{a:08X}-{b:04X}-{c:04X}-{d:04X}-{e:012X}"))
    }
}

impl UUID {
    ///
    /// Generates a new random UUID
    #[must_use]
    pub fn new_random() -> UUID {
        use crate::random::PRNG;
        let mut random = crate::random::Random::default();
        UUID {
            inner: random.next_u128(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::uuid::{UUIDParseError, UUID};

    #[test]
    pub fn display_test() {
        let uuid = UUID { inner: 0 };
        assert_eq!("00000000-0000-0000-0000-000000000000", format!("{uuid}"));

        let uuid = UUID { inner: u128::MAX };
        assert_eq!("FFFFFFFF-FFFF-FFFF-FFFF-FFFFFFFFFFFF", format!("{uuid}"));
    }

    #[test]
    pub fn parse_test() -> Result<(), UUIDParseError> {
        let uuid = UUID::new_random();
        let disp = format!("{uuid}");

        let parsed: UUID = disp.as_str().try_into()?;
        assert_eq!(parsed, uuid);

        let parsed: u128 = parsed.into();
        let parsed: UUID = (parsed.to_be_bytes().as_slice()).try_into()?;
        assert_eq!(parsed, uuid);

        Ok(())
    }
}
