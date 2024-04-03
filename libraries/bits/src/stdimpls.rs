// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::mutbits::MutBits;
use crate::{Bits, Error};

impl Bits for std::fs::File {
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        use std::io::Read;
        let mut buf: [u8; 1] = [0];
        let read = self.read(&mut buf)?;
        if read == 1 {
            return Ok(Some(buf[0]));
        }
        Ok(None)
    }
}

impl MutBits for std::fs::File {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        use std::io::Write;
        Ok(self.write_all(&[val])?)
    }
}
