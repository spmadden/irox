// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_structs::Struct;
use irox_tools::bits::{Bits, MutBits};
use irox_tools::packetio::{Packet, PacketBuilder};

use crate::input::util;
use crate::packet::PacketType;

pub const MESSAGE_ID: usize = 0x02;
pub const PAYLOAD_SIZE: usize = 41;
pub const VELOCITY_SCALE: f32 = 8.0;
pub const HDOP_SCALE: f32 = 5.0;

#[derive(Default, Clone, Debug, Struct)]
pub struct MeasuredNavigationData {
    pub x_position: i32,
    pub y_position: i32,
    pub z_position: i32,

    pub x_velocity: f32,
    pub y_velocity: f32,
    pub z_velocity: f32,

    pub mode: u8,
    pub hdop: f32,
    pub mode2: u8,
    pub gps_week: u16,
    pub gps_tow: f64,

    pub svs_in_fix: u8,

    pub ch1_prn: u8,
    pub ch2_prn: u8,
    pub ch3_prn: u8,
    pub ch4_prn: u8,
    pub ch5_prn: u8,
    pub ch6_prn: u8,
    pub ch7_prn: u8,
    pub ch8_prn: u8,
    pub ch9_prn: u8,
    pub ch10_prn: u8,
    pub ch11_prn: u8,
    pub ch12_prn: u8,
}

impl Packet for MeasuredNavigationData {
    type PacketType = PacketType;

    fn get_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut out: Vec<u8> = Vec::with_capacity(PAYLOAD_SIZE);
        out.write_be_i32(self.x_position)?;
        out.write_be_i32(self.y_position)?;
        out.write_be_i32(self.z_position)?;

        write_velocity(&mut out, self.x_velocity)?;
        write_velocity(&mut out, self.y_velocity)?;
        write_velocity(&mut out, self.z_velocity)?;

        out.write_u8(self.mode)?;
        write_hdop(&mut out, self.hdop)?;
        out.write_u8(self.mode2)?;

        out.write_be_u16(self.gps_week)?;

        util::write_gpstow(&mut out, self.gps_tow)?;

        out.write_u8(self.svs_in_fix)?;
        out.write_u8(self.ch1_prn)?;
        out.write_u8(self.ch2_prn)?;
        out.write_u8(self.ch3_prn)?;
        out.write_u8(self.ch4_prn)?;
        out.write_u8(self.ch5_prn)?;
        out.write_u8(self.ch6_prn)?;
        out.write_u8(self.ch7_prn)?;
        out.write_u8(self.ch8_prn)?;
        out.write_u8(self.ch9_prn)?;
        out.write_u8(self.ch10_prn)?;
        out.write_u8(self.ch11_prn)?;
        out.write_u8(self.ch12_prn)?;

        Ok(out)
    }

    fn get_type(&self) -> Self::PacketType {
        PacketType::MeasuredNavigationData(self.clone())
    }
}

pub struct MeasuredNavDataBuilder;
pub static BUILDER: MeasuredNavDataBuilder = MeasuredNavDataBuilder;

impl PacketBuilder<MeasuredNavigationData> for MeasuredNavDataBuilder {
    type Error = std::io::Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<MeasuredNavigationData, Self::Error> {
        let x_position = input.read_be_i32()?;
        let y_position = input.read_be_i32()?;
        let z_position = input.read_be_i32()?;
        let x_velocity = read_velocity(input)?;
        let y_velocity = read_velocity(input)?;
        let z_velocity = read_velocity(input)?;
        let mode = input.read_u8()?;
        let hdop = read_hdop(input)?;
        let mode2 = input.read_u8()?;
        let gps_week = input.read_be_u16()?;
        let gps_tow = util::read_gpstow(input)?;
        let svs_in_fix = input.read_u8()?;
        let ch1_prn = input.read_u8()?;
        let ch2_prn = input.read_u8()?;
        let ch3_prn = input.read_u8()?;
        let ch4_prn = input.read_u8()?;
        let ch5_prn = input.read_u8()?;
        let ch6_prn = input.read_u8()?;
        let ch7_prn = input.read_u8()?;
        let ch8_prn = input.read_u8()?;
        let ch9_prn = input.read_u8()?;
        let ch10_prn = input.read_u8()?;
        let ch11_prn = input.read_u8()?;
        let ch12_prn = input.read_u8()?;

        Ok(MeasuredNavigationData {
            x_position,
            y_position,
            z_position,
            x_velocity,
            y_velocity,
            z_velocity,
            mode,
            hdop,
            mode2,
            gps_week,
            gps_tow,
            svs_in_fix,
            ch1_prn,
            ch2_prn,
            ch3_prn,
            ch4_prn,
            ch5_prn,
            ch6_prn,
            ch7_prn,
            ch8_prn,
            ch9_prn,
            ch10_prn,
            ch11_prn,
            ch12_prn,
        })
    }
}

fn write_velocity<T: MutBits>(out: &mut T, val: f32) -> Result<(), std::io::Error> {
    let enc = (val * VELOCITY_SCALE).round() as i16;
    out.write_be_i16(enc)
}

fn read_velocity<T: Bits>(out: &mut T) -> Result<f32, std::io::Error> {
    let read = out.read_be_i16()?;
    Ok(f32::from(read) / VELOCITY_SCALE)
}

fn write_hdop<T: MutBits>(out: &mut T, val: f32) -> Result<(), std::io::Error> {
    let enc = (val * HDOP_SCALE).round() as u8;
    out.write_u8(enc)
}

fn read_hdop<T: Bits>(out: &mut T) -> Result<f32, std::io::Error> {
    let read = out.read_u8()?;
    Ok(f32::from(read) / HDOP_SCALE)
}
