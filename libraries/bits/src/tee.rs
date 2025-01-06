// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::{Error, MutBits};

///
/// Simple output stream splitter.  Writes the output value to both streams simultaneously.
pub struct Tee<O1: MutBits, O2: MutBits> {
    output1: O1,
    output2: O2,
}
impl<O1: MutBits, O2: MutBits> Tee<O1, O2> {
    pub fn new(output1: O1, output2: O2) -> Self {
        Self { output1, output2 }
    }
}
impl<O1: MutBits, O2: MutBits> Drop for Tee<O1, O2> {
    fn drop(&mut self) {
        let _ = self.output1.flush();
        let _ = self.output2.flush();
    }
}
impl<O1: MutBits, O2: MutBits> MutBits for Tee<O1, O2> {
    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        let r1 = self.output1.write_u8(val);
        let r2 = self.output2.write_u8(val);
        r1?;
        r2
    }
}
