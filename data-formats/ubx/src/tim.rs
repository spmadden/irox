// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::{UBXClass, UBXRawMessage};
use core::fmt::{Display, Formatter};
use irox_bits::Error;
use irox_enums::{EnumName, EnumTryFromRepr, EnumTryFromStr};
use irox_structs::Struct;

#[repr(u8)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Ord,
    PartialOrd,
    EnumName,
    EnumTryFromRepr,
    EnumTryFromStr,
)]
#[allow(clippy::upper_case_acronyms)]
pub enum UBXTim {
    DOSC = 0x11,
    FCHG = 0x16,
    HOC = 0x17,
    SMEAS = 0x13,
    SVIN = 0x04,
    TM2 = 0x03,
    TOS = 0x12,
    TP = 0x01,
    VCOCAL = 0x15,
    VRFY = 0x06,
}
impl UBXTim {
    pub fn try_parse_payload(self, pld: &[u8]) -> Result<UBXTimPayload, Error> {
        let mut pld = pld;
        Ok(match self {
            UBXTim::SVIN => UBXTimPayload::SVIN(UBXTimSvin::parse_from(&mut pld)?),
            UBXTim::SMEAS => UBXTimPayload::SMEAS(UBXTimSMEAS::parse_from(&mut pld)?),
            UBXTim::TM2 => UBXTimPayload::TM2(UBXTimTM2::parse_from(&mut pld)?),
            UBXTim::TOS => UBXTimPayload::TOS(UBXTimTOS::parse_from(&mut pld)?),
            UBXTim::TP => UBXTimPayload::TP(UBXTimTP::parse_from(&mut pld)?),
            _ => UBXTimPayload::Unknown {
                id: self as u8,
                payload: pld.to_vec().into_boxed_slice(),
            },
        })
    }
    pub fn try_parse(id: u8, pld: &[u8]) -> Result<UBXTimPayload, Error> {
        let Ok(msg) = UBXTim::try_from(id) else {
            return Ok(UBXTimPayload::Unknown {
                id,
                payload: pld.to_vec().into_boxed_slice(),
            });
        };
        msg.try_parse_payload(pld)
    }
}
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UBXTimPayload {
    SVIN(UBXTimSvin),
    SMEAS(UBXTimSMEAS),
    TM2(UBXTimTM2),
    TOS(UBXTimTOS),
    TP(UBXTimTP),
    Unknown { id: u8, payload: Box<[u8]> },
}
impl Display for UBXTimPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}
impl TryFrom<UBXRawMessage> for UBXTimPayload {
    type Error = UBXRawMessage;

    fn try_from(value: UBXRawMessage) -> Result<Self, Self::Error> {
        if value.class != UBXClass::TIM as u8 {
            return Err(value);
        }
        let Ok(id) = UBXTim::try_from(value.id) else {
            return Ok(UBXTimPayload::Unknown {
                id: value.id,
                payload: value.payload,
            });
        };

        match id.try_parse_payload(&value.payload) {
            Ok(v) => Ok(v),
            Err(_e) => Err(value),
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Struct)]
#[little_endian]
pub struct UBXTimSvin {
    pub dur: u32,
    pub mean_x: i32,
    pub mean_y: i32,
    pub mean_z: i32,
    pub mean_v: u32,
    pub obs: u32,
    pub valid: u8,
    pub active: u8,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Struct)]
#[little_endian]
pub struct UBXTimTM2 {
    pub channel: u8,
    pub flags: u8,
    pub count: u16,
    pub weekno_rising_edge: u16,
    pub weekno_falling_edge: u16,
    pub tow_ms_rising_edge: u32,
    pub tow_subms_rising_edge: u32,
    pub tow_ms_falling_edge: u32,
    pub tow_subms_falling_edge: u32,
    pub acc_est_ns: u32,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Struct)]
#[little_endian]
pub struct UBXTimTOS {
    pub version: u8,
    pub gnss_id: u8,
    pub resv1: u16,
    pub flags: u32,
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub utc_standard: u8,
    pub utc_offset_ns: i32,
    pub utc_uncertainty_ns: u32,
    pub week: u32,
    pub tow_sec: u32,
    pub gnss_offset_ns: i32,
    pub gnss_uncertainty_ns: u32,
    //TODO Scale 2^-8
    pub int_osc_offset_ppb: i32,
    //TODO Scale 2^-8
    pub int_osc_uncertainty_ppb: u32,
    //TODO Scale 2^-8
    pub ext_osc_offset_ppb: i32,
    //TODO Scale 2^-8
    pub ext_osc_uncertainty_ppb: u32,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Struct)]
#[little_endian]
pub struct UBXTimTP {
    pub tow_ms: u32,
    pub tow_subms: u32,
    pub q_err_ps: i32,
    pub week: u16,
    pub flags: u8,
    pub ref_info: u8,
}
#[derive(Debug, Clone, PartialEq, Eq, Struct)]
#[little_endian]
pub struct UBXTimSMEAS {
    pub version: u8,
    pub num_measurements: u8,
    pub resv1: u16,
    pub tow_ms: u32,
    pub resv2: u32,
    pub measurement: Vec<Measurement>,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Struct)]
#[little_endian]
pub struct Measurement {
    pub source_id: u8,
    pub flags: u8,
    //TODO Scale 2^-8
    pub phase_offset_fraction_ns: i8,
    //TODO Scale 2^-8
    pub phase_uncertainty_fraction_ns: u8,
    pub phase_offset_ns: i32,
    pub phase_uncertainty_ns: u32,
    pub resvd3: u32,
    //TODO Scale 2^-8
    pub freq_offset_ppb: i32,
    //TODO Scale 2^-8
    pub freq_uncertainty_ppb: u32,
}
