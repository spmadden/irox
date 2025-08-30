// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! u-blox / ubx packet formats
//!

#![forbid(unsafe_code)]
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod cfg;
pub mod mon;
pub mod nav;
pub mod ubx_dumptxtfile;

use crate::cfg::UBXCfg;
use crate::mon::{UBXMon, UBXMonPayload};
use crate::nav::{UBXNav, UBXNavPayload};
use core::fmt::{Debug, Display, Formatter};
use irox_bits::{Bits, BitsErrorKind};
use irox_enums::{EnumName, EnumTryFromRepr, EnumTryFromStr};
use irox_tools::packetio::PacketBuilder;

///
/// Class of ubx message - the third byte of the message
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
pub enum UBXClass {
    NAV = 0x01,
    RXM = 0x02,
    TRK = 0x03,
    INF = 0x04,
    ACK = 0x05,
    CFG = 0x06,
    UPD = 0x09,
    MON = 0x0A,
    AID = 0x0B,
    TIM = 0x0D,
    ESF = 0x10,
    MGA = 0x13,
    LOG = 0x21,
    SEC = 0x27,
    HNR = 0x28,
    NAV2 = 0x29,
}
impl UBXClass {
    pub fn try_parse_payload(&self, id: u8, pld: &[u8]) -> Result<UBXPayload, irox_bits::Error> {
        match self {
            UBXClass::NAV => Ok(UBXPayload::Nav(UBXNav::try_parse(id, pld)?)),
            UBXClass::MON => Ok(UBXPayload::Mon(UBXMon::try_parse(id, pld)?)),
            //TODO add the rest of the classes
            _ => Ok(UBXPayload::Unknown(pld.into())),
        }
    }
    pub fn try_get_id(&self, msgname: &str) -> Option<u8> {
        match self {
            UBXClass::NAV => UBXNav::try_from(msgname).ok().map(|v| v as u8),
            UBXClass::MON => UBXMon::try_from(msgname).ok().map(|v| v as u8),
            UBXClass::CFG => UBXCfg::try_from(msgname).ok().map(|v| v as u8),
            //TODO add the rest of the classes
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UBXPayload {
    Nav(UBXNavPayload),
    Mon(UBXMonPayload),
    // TODO add the rest of the classes
    Unknown(Box<[u8]>),
}

/// A semi-encoded UBX message with enough information to exactly reconstruct
/// the packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UBXMessage {
    pub class: u8,
    pub id: u8,
    pub payload: UBXPayload,
    pub checksum: u16,
}
impl Display for UBXMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match &self.payload {
            UBXPayload::Nav(p) => {
                write!(f, "UBXPayload::NAV {p:?}")
            }
            UBXPayload::Mon(m) => {
                write!(f, "UBXPayload::MON {m:?}")
            }
            UBXPayload::Unknown(p) => {
                write!(
                    f,
                    "UBXPayload::Unknown (0x{:02X}/0x{:02X}) {}",
                    self.class,
                    self.id,
                    &irox_tools::hex::to_hex_str_upper(p)
                )
            }
        }
    }
}
///
/// A raw ubx message with the exact bytes read from the input stream
#[derive(Clone, PartialEq, Eq)]
pub struct UBXRawMessage {
    pub class: u8,
    pub id: u8,
    pub payload: Box<[u8]>,
    pub checksum: u16,
    pub payload_hex: Option<String>,
}
impl Debug for UBXRawMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("UBXRawMessage")
            .field("class", &self.class)
            .field("id", &self.id)
            .field("payload", &irox_tools::hex::to_hex_str_upper(&self.payload))
            .field("checksum", &self.checksum)
            .finish()
    }
}
impl UBXRawMessage {
    pub const fn get_checksum(&self) -> u16 {
        self.checksum
    }
    pub const fn calculate_checksum(&self) -> u16 {
        calculate_checksum(
            self.class,
            self.id,
            self.payload.len() as u16,
            &self.payload,
        )
    }
}

///
/// A parser for ubx messages.
#[derive(Default, Debug, Copy, Clone)]
pub struct UBXParser {
    pub ignore_checksum_mismatch: bool,
}
impl PacketBuilder<UBXMessage> for UBXParser {
    type Error = irox_bits::Error;

    fn build_from<T: Bits>(&self, source: &mut T) -> Result<UBXMessage, Self::Error> {
        loop {
            let val = source.read_u8()?;
            if val == 0xb5 && source.read_u8()? == 0x62 {
                break;
            }
        }
        let mut payload = Vec::<u8>::new();
        let class = source.read_u8()?;
        let id = source.read_u8()?;
        let payload_len = source.read_le_u16()?;
        source.read_exact_into(payload_len as usize, &mut payload)?;
        let checksum = source.read_le_u16()?;
        let calc_checksum = calculate_checksum(class, id, payload_len, &payload);
        if !self.ignore_checksum_mismatch && calc_checksum != checksum {
            return Err(BitsErrorKind::InvalidInput.into());
        }
        let Ok(clazz) = UBXClass::try_from(class) else {
            return Ok(UBXMessage {
                class,
                id,
                payload: UBXPayload::Unknown(payload.into_boxed_slice()),
                checksum,
            });
        };

        let payload = clazz.try_parse_payload(id, &payload)?;

        Ok(UBXMessage {
            class,
            id,
            payload,
            checksum,
        })
    }
}

///
/// Calculates the expected checksum from the
pub const fn calculate_checksum(class: u8, id: u8, length: u16, pld: &[u8]) -> u16 {
    let mut a = class.wrapping_add(id);
    let mut b = class.wrapping_add(a);
    a = a.wrapping_add(length as u8);
    b = b.wrapping_add(a);
    a = a.wrapping_add((length >> 8) as u8);
    b = b.wrapping_add(a);
    let mut i = 0;
    while i < pld.len() {
        #[allow(clippy::indexing_slicing)]
        let byte = pld[i];

        a = a.wrapping_add(byte);
        b = b.wrapping_add(a);
        i += 1;
    }
    ((b as u16) << 8) | a as u16
}
