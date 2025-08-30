// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_bits::{Bits, Error, MutBits};
use irox_structs::Struct;
use irox_tools::packetio::{Packet, PacketBuilder};

#[derive(Copy, Clone, Debug, Default, PartialEq, Struct)]
pub struct MeasuredTrackChannel {
    pub sv_id: u8,
    pub azimuth: u8,
    pub elevation: u8,
    pub state: u16,
    pub cno_1: u8,
    pub cno_2: u8,
    pub cno_3: u8,
    pub cno_4: u8,
    pub cno_5: u8,
    pub cno_6: u8,
    pub cno_7: u8,
    pub cno_8: u8,
    pub cno_9: u8,
    pub cno_10: u8,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct MeasuredTrackData {
    gps_week: u16,
    gps_tow: u32,
    num_channels: u8,
    channels: [MeasuredTrackChannel; 12],
}

impl Packet for MeasuredTrackData {
    type PacketType = ();

    fn get_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut buf: Vec<u8> = Vec::new();
        buf.write_be_u16(self.gps_week)?;
        buf.write_be_u32(self.gps_tow)?;
        buf.write_u8(self.num_channels)?;

        for channel in self.channels {
            channel.write_to(&mut buf)?;
        }
        Ok(buf)
    }

    fn get_type(&self) -> Self::PacketType {
        todo!()
    }
}
pub struct MeasuredTrackDataBuilder;
pub static BUILDER: MeasuredTrackDataBuilder = MeasuredTrackDataBuilder;
impl PacketBuilder<MeasuredTrackData> for MeasuredTrackDataBuilder {
    type Error = Error;

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

fn read_channel<T: Bits>(input: &mut T) -> Result<MeasuredTrackChannel, Error> {
    MeasuredTrackChannel::parse_from(input)
}
