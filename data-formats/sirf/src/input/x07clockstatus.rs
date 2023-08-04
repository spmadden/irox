use crate::input::util::read_gpstow;
use crate::packet::PacketType;
use irox_tools::bits::{Bits, MutBits};
use irox_tools::packetio::{Packet, PacketBuilder};

#[derive(Default, Debug, Clone)]
pub struct ClockStatus {
    extended_gps_week: u16,

    /// gps time of week, seconds
    gps_tow: f64,

    /// sats
    svs: u8,

    /// clock drive in hz
    clock_drift: u32,

    /// clock bias in ns
    clock_bias: u32,

    /// est gps time in ms
    est_gps_time: u32,
}

impl Packet for ClockStatus {
    type PacketType = PacketType;
    type Error = std::io::Error;

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

pub struct ClockStatusBuilder;
pub static BUILDER: ClockStatusBuilder = ClockStatusBuilder;
impl PacketBuilder<ClockStatus> for ClockStatusBuilder {
    type Error = std::io::Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<ClockStatus, Self::Error> {
        let extended_gps_week = input.read_be_u16()?;
        let gps_tow = read_gpstow(input)?;
        let svs = input.read_u8()?;
        let clock_drift = input.read_be_u32()?;
        let clock_bias = input.read_be_u32()?;
        let est_gps_time = input.read_be_u32()?;

        Ok(ClockStatus {
            extended_gps_week,
            gps_tow,
            svs,
            clock_drift,
            clock_bias,
            est_gps_time,
        })
    }
}
