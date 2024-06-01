// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::bits::Bits;
use crate::mutbits::MutBits;
use crate::Error;

///
/// Wraps a borrowed [`std::io::Read`] or [`std::io::Write`] and provides a basic implementation
/// of [`Bits`] for [`std::io::Read`] and [`MutBits`] for [`std::io::Write`]
pub struct BitsWrapper<'a, T>(pub &'a mut T);

impl<'a, T> Bits for BitsWrapper<'a, T>
where
    T: std::io::Read,
{
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        let mut byte: u8 = 0;
        let read = self.0.read(core::slice::from_mut(&mut byte))?;
        if read < 1 {
            return Ok(None);
        }
        Ok(Some(byte))
    }
}

impl<'a, T> MutBits for BitsWrapper<'a, T>
where
    T: std::io::Write,
{
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        Ok(self.0.write_all(&[val])?)
    }
}
