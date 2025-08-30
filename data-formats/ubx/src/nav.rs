// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! = UBX NAV Messages
//!
//! === Definitions:
//! * Navigation Epoch: The 1khz aligned internal tick of the navigation solution
//! * Clock Bias: The difference between GNSS System Time and the receiver local time
//! * Clock Drift: The rate at which the Clock Bias is changing
//! * iTOW: The GPS Time at which the navigation solution was computed
//!
//!

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
pub enum UBXNav {
    AOPSTATUS = 0x60,
    ATT = 0x05,
    CLOCK = 0x22,
    COV = 0x36,
    DGPS = 0x31,
    DOP = 0x04,
    EELL = 0x3D,
    EOE = 0x61,
    GEOFENCE = 0x39,
    HPPOSECEF = 0x13,
    HPPOSLLH = 0x14,
    NMI = 0x28,
    ODO = 0x09,
    ORB = 0x34,
    POSECEF = 0x01,
    POSLLH = 0x02,
    PVT = 0x07,
    RELPOSNED = 0x3C,
    RESETODO = 0x10,
    SAT = 0x35,
    SBAS = 0x32,
    SOL = 0x06,
    STATUS = 0x03,
    SVINFO = 0x30,
    SVIN = 0x3B,
    TIMEBDS = 0x24,
    TIMEGAL = 0x25,
    TIMEGLO = 0x23,
    TIMEGPS = 0x20,
    TIMELS = 0x26,
    TIMEUTC = 0x21,
    VELECEF = 0x11,
    VELNED = 0x12,
}
impl UBXNav {
    pub fn try_parse_payload(&self, pld: &[u8]) -> Result<UBXNavPayload, Error> {
        let mut pld = pld;
        Ok(match self {
            UBXNav::CLOCK => UBXNavPayload::Clock(UBXNavClockRaw::parse_from(&mut pld)?),
            _ => UBXNavPayload::Unknown {
                id: *self as u8,
                payload: pld.to_vec().into_boxed_slice(),
            },
        })
    }
    pub fn try_parse(id: u8, pld: &[u8]) -> Result<UBXNavPayload, Error> {
        let Ok(msg) = UBXNav::try_from(id) else {
            return Ok(UBXNavPayload::Unknown {
                id,
                payload: pld.to_vec().into_boxed_slice(),
            });
        };
        msg.try_parse_payload(pld)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UBXNavPayload {
    Clock(UBXNavClockRaw),
    Unknown { id: u8, payload: Box<[u8]> },
}
impl Display for UBXNavPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}
impl TryFrom<UBXRawMessage> for UBXNavPayload {
    type Error = UBXRawMessage;

    fn try_from(value: UBXRawMessage) -> Result<Self, Self::Error> {
        if value.class != UBXClass::NAV as u8 {
            return Err(value);
        }
        let Ok(id) = UBXNav::try_from(value.id) else {
            return Ok(UBXNavPayload::Unknown {
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
pub struct UBXNavClockRaw {
    /// Time of week in milliseconds of the navigation epoch.
    pub itow_ms: u32,
    /// Clock bias in nanoseconds.
    pub clock_bias_ns: i32,
    /// Clock drift in nanoseconds per second.
    pub clock_drift_ns_sec: i32,
    /// Time accuracy in nanoseconds per second.
    pub time_accuracy_ns: u32,
    /// Frequency accuracy in picoseconds per second
    pub freq_accuracy_ps_sec: u32,
}

#[cfg(test)]
mod tests {
    use crate::nav::{UBXNavClockRaw, UBXNavPayload};
    use crate::{UBXMessage, UBXParser, UBXPayload};
    use irox_tools::hex;
    use irox_tools::packetio::PacketBuilder;

    #[test]
    pub fn test_parse_nav_clock() {
        let inp = &hex!("B56201221400F80B1401E4F80800000000001A0000001E0300006E11");
        let parser = UBXParser {
            ignore_checksum_mismatch: false,
        };
        let parsed = parser.build_from(&mut inp.as_ref()).unwrap();
        let exp = UBXMessage {
            class: 0x01,
            id: 0x22,
            payload: UBXPayload::Nav(UBXNavPayload::Clock(UBXNavClockRaw {
                itow_ms: 18091000,
                clock_bias_ns: 588004,
                clock_drift_ns_sec: 0,
                time_accuracy_ns: 26,
                freq_accuracy_ps_sec: 798,
            })),
            checksum: 0x116E,
        };
        assert_eq!(parsed, exp);
    }
}
