// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::{UBXClass, UBXRawMessage};
use core::fmt::{Display, Formatter};
use irox_bits::{Bits, Error};
use irox_enums::{EnumName, EnumTryFromRepr, EnumTryFromStr};

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
pub enum UBXMon {
    COMMS = 0x36,
    GNSS = 0x28,
    HW = 0x09,
    HW2 = 0x0b,
    HW3 = 0x37,
    IO = 0x02,
    MSGPP = 0x06,
    PATCH = 0x27,
    RF = 0x38,
    RXBUF = 0x07,
    RXR = 0x21,
    SPAN = 0x31,
    SYS = 0x39,
    TXBUF = 0x08,
    VER = 0x04,
}
impl UBXMon {
    pub fn try_parse_payload(&self, pld: &[u8]) -> Result<UBXMonPayload, Error> {
        Ok(match self {
            UBXMon::VER => UBXMonPayload::Ver(UBXMonVer::try_from(pld)?),
            _ => UBXMonPayload::Unknown {
                id: *self as u8,
                payload: pld.to_vec().into_boxed_slice(),
            },
        })
    }
    pub fn try_parse(id: u8, pld: &[u8]) -> Result<UBXMonPayload, Error> {
        let Ok(msg) = UBXMon::try_from(id) else {
            return Ok(UBXMonPayload::Unknown {
                id,
                payload: pld.to_vec().into_boxed_slice(),
            });
        };
        msg.try_parse_payload(pld)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UBXMonPayload {
    Ver(UBXMonVer),
    Unknown { id: u8, payload: Box<[u8]> },
}
impl Display for UBXMonPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}
impl TryFrom<UBXRawMessage> for UBXMonPayload {
    type Error = UBXRawMessage;

    fn try_from(value: UBXRawMessage) -> Result<Self, Self::Error> {
        if value.class != UBXClass::MON as u8 {
            return Err(value);
        }
        let Ok(id) = UBXMon::try_from(value.id) else {
            return Ok(UBXMonPayload::Unknown {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UBXMonVer {
    pub sw_version: String,
    pub hw_version: String,
    pub extensions: Vec<String>,
}
impl TryFrom<&[u8]> for UBXMonVer {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let (mut swver, rest) = value.split_at(30);
        let sw_version = swver.read_str_null_terminated()?;
        let (mut hwver, rest) = rest.split_at(10);
        let hw_version = hwver.read_str_null_terminated()?;
        let mut extensions = Vec::new();
        for mut buf in rest.chunks(30) {
            let ext = buf.read_str_null_terminated()?;
            extensions.push(ext);
        }
        Ok(Self {
            sw_version,
            hw_version,
            extensions,
        })
    }
}
