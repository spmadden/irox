use irox_tools::bits::{Bits, MutBits};
use irox_tools::packetio::{Packet, PacketBuilder};

#[derive(Default, Debug, Copy, Clone)]
pub struct NavLibSVState {
    sv_id: u8,
    gps_time: f64,
    ecef_pos_x: f64,
    ecef_pos_y: f64,
    ecef_pos_z: f64,
    ecef_vel_x: f64,
    ecef_vel_y: f64,
    ecef_vel_z: f64,
    clock_bias: f64,
    clock_drift: f32,
    ephemeris_flag: u8,
    reserved_1: u32,
    reserved_2: u32,
    ionospheric_delay: f32,
}

impl Packet for NavLibSVState {
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

pub struct NavLibSVStateBuilder;
pub static BUILDER: NavLibSVStateBuilder = NavLibSVStateBuilder;
impl PacketBuilder<NavLibSVState> for NavLibSVStateBuilder {
    type Error = std::io::Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<NavLibSVState, Self::Error> {
        let sv_id = input.read_u8()?;
        let gps_time = input.read_f64()?;
        let ecef_pos_x = input.read_f64()?;
        let ecef_pos_y = input.read_f64()?;
        let ecef_pos_z = input.read_f64()?;
        let ecef_vel_x = input.read_f64()?;
        let ecef_vel_y = input.read_f64()?;
        let ecef_vel_z = input.read_f64()?;
        let clock_bias = input.read_f64()?;
        let clock_drift = input.read_f32()?;
        let ephemeris_flag = input.read_u8()?;
        let reserved_1 = input.read_be_u32()?;
        let reserved_2 = input.read_be_u32()?;
        let ionospheric_delay = input.read_f32()?;
        Ok(NavLibSVState {
            sv_id,
            gps_time,
            ecef_pos_x,
            ecef_pos_y,
            ecef_pos_z,
            ecef_vel_x,
            ecef_vel_y,
            ecef_vel_z,
            clock_bias,
            clock_drift,
            ephemeris_flag,
            reserved_1,
            reserved_2,
            ionospheric_delay,
        })
    }
}
