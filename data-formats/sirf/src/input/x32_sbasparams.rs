// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_bits::{Bits, Error};
use irox_structs::Struct;
use irox_tools::packetio::{Packet, PacketBuilder};

#[derive(Default, Debug, Copy, Clone, PartialEq, Struct)]
pub struct SBASParameters {
    sbas_prn: u8,
    sbas_mode: u8,
    dgps_timeout: u8,
    flag_bits: u8,
    spare: u128,
}

impl Packet for SBASParameters {
    type PacketType = ();

    fn get_bytes(&self) -> Result<Vec<u8>, Error> {
        Struct::as_bytes(self)
    }

    fn get_type(&self) -> Self::PacketType {
        todo!()
    }
}

pub struct SBASParamsBuilder;
pub static BUILDER: SBASParamsBuilder = SBASParamsBuilder;
impl PacketBuilder<SBASParameters> for SBASParamsBuilder {
    type Error = Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<SBASParameters, Self::Error> {
        SBASParameters::parse_from(input)
    }
}
