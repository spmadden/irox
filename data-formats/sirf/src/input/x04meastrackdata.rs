// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_tools::bits::{Bits, MutBits};
use irox_tools::packetio::{Packet, PacketBuilder};

#[derive(Copy, Clone, Debug, Default)]
pub struct MeasuredTrackChannel {
    sv_id: u8,
    azimuth: u8,
    elevation: u8,
    state: u16,
    cno_1: u8,
    cno_2: u8,
    cno_3: u8,
    cno_4: u8,
    cno_5: u8,
    cno_6: u8,
    cno_7: u8,
    cno_8: u8,
    cno_9: u8,
    cno_10: u8,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct MeasuredTrackData {
    gps_week: u16,
    gps_tow: u32,
    num_channels: u8,
    channels: [MeasuredTrackChannel; 12],
}

impl Packet for MeasuredTrackData {
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
pub struct MeasuredTrackDataBuilder;
pub static BUILDER: MeasuredTrackDataBuilder = MeasuredTrackDataBuilder;
impl PacketBuilder<MeasuredTrackData> for MeasuredTrackDataBuilder {
    type Error = std::io::Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<MeasuredTrackData, Self::Error> {
        let gps_week = input.read_be_u16()?;
        let gps_tow = input.read_be_u32()?;
        let num_channels = input.read_u8()?;

        let channels: [MeasuredTrackChannel; 12] = [
            read_channel(input)?,
            read_channel(input)?,
            read_channel(input)?,
            read_channel(input)?,
            read_channel(input)?,
            read_channel(input)?,
            read_channel(input)?,
            read_channel(input)?,
            read_channel(input)?,
            read_channel(input)?,
            read_channel(input)?,
            read_channel(input)?,
        ];

        Ok(MeasuredTrackData {
            gps_week,
            gps_tow,
            num_channels,
            channels,
        })
    }
}

fn read_channel<T: Bits>(input: &mut T) -> Result<MeasuredTrackChannel, std::io::Error> {
    let sv_id = input.read_u8()?;
    let azimuth = input.read_u8()?;
    let elevation = input.read_u8()?;
    let state = input.read_be_u16()?;
    let cno_1 = input.read_u8()?;
    let cno_2 = input.read_u8()?;
    let cno_3 = input.read_u8()?;
    let cno_4 = input.read_u8()?;
    let cno_5 = input.read_u8()?;
    let cno_6 = input.read_u8()?;
    let cno_7 = input.read_u8()?;
    let cno_8 = input.read_u8()?;
    let cno_9 = input.read_u8()?;
    let cno_10 = input.read_u8()?;
    Ok(MeasuredTrackChannel {
        sv_id,
        azimuth,
        elevation,
        state,
        cno_1,
        cno_2,
        cno_3,
        cno_4,
        cno_5,
        cno_6,
        cno_7,
        cno_8,
        cno_9,
        cno_10,
    })
}
