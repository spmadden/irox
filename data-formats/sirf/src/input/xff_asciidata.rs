// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_tools::bits::{Bits, MutBits};
use irox_tools::packetio::{Packet, PacketBuilder};

#[derive(Debug, Default, Clone)]
pub struct AsciiData {
    message: String,
}

impl Packet for AsciiData {
    type PacketType = ();
    type Error = crate::error::Error;

    fn write_to<T: MutBits>(&self, out: &mut T) -> Result<(), Self::Error> {
        Ok(out.write_all(self.get_bytes()?.as_slice())?)
    }

    fn get_bytes(&self) -> Result<Vec<u8>, Self::Error> {
        Ok(self.message.clone().into_bytes())
    }

    fn get_type(&self) -> Self::PacketType {
        todo!()
    }
}

pub struct AsciiDataBuilder;
pub static BUILDER: AsciiDataBuilder = AsciiDataBuilder;
impl PacketBuilder<AsciiData> for AsciiDataBuilder {
    type Error = crate::error::Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<AsciiData, Self::Error> {
        let mut str = String::new();
        input.read_to_string(&mut str)?;
        Ok(AsciiData { message: str })
    }
}
