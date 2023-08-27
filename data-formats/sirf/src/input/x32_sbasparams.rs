// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_structs::Struct;
use irox_tools::bits::{Bits, MutBits};
use irox_tools::packetio::{Packet, PacketBuilder};

#[derive(Default, Debug, Copy, Clone, Struct)]
pub struct SBASParameters {
    sbas_prn: u8,
    sbas_mode: u8,
    dgps_timeout: u8,
    flag_bits: u8,
    spare: u128,
}

impl Packet for SBASParameters {
    type PacketType = ();
    type Error = crate::error::Error;

    fn write_to<T: MutBits>(&self, out: &mut T) -> Result<(), Self::Error> {
        Ok(Struct::write_to(self, out)?)
    }

    fn get_bytes(&self) -> Result<Vec<u8>, Self::Error> {
        Ok(Struct::as_bytes(self)?)
    }

    fn get_type(&self) -> Self::PacketType {
        todo!()
    }
}

pub struct SBASParamsBuilder;
pub static BUILDER: SBASParamsBuilder = SBASParamsBuilder;
impl PacketBuilder<SBASParameters> for SBASParamsBuilder {
    type Error = crate::error::Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<SBASParameters, Self::Error> {
        Ok(SBASParameters::parse_from(input)?)
    }
}
