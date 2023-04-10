// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_tools::bits::{Bits, MutBits};
use irox_tools::packetio::{Packet, PacketBuilder};

#[derive(Default, Debug, Copy, Clone)]
pub struct SBASParameters {
    sbas_prn: u8,
    sbas_mode: u8,
    dgps_timeout: u8,
    flag_bits: u8,
    // spare: u128,
}

impl Packet for SBASParameters {
    type PacketType = ();
    type Error = ();

    fn write_to<T: MutBits>(&self, out: &mut T) -> Result<(), Self::Error> {
        todo!()
    }

    fn get_bytes(&self) -> Result<Vec<u8>, Self::Error> {
        todo!()
    }

    fn get_type(&self) -> Self::PacketType {
        todo!()
    }
}

pub struct SBASParamsBuilder;
pub static BUILDER: SBASParamsBuilder = SBASParamsBuilder;
impl PacketBuilder<SBASParameters> for SBASParamsBuilder {
    type Error = std::io::Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<SBASParameters, Self::Error> {
        let sbas_prn = input.read_u8()?;
        let sbas_mode = input.read_u8()?;
        let dgps_timeout = input.read_u8()?;
        let flag_bits = input.read_u8()?;
        // let spare = input.read_be_u128()?;
        Ok(SBASParameters {
            sbas_prn,
            sbas_mode,
            dgps_timeout,
            flag_bits,
            // spare,
        })
    }
}
