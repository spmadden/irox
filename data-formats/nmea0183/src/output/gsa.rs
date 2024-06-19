// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::{Display, Formatter};
use std::io::Write;

use irox_bits::{Bits, BitsError};
use irox_carto::gps::{DOPs, GPSFixType};
use irox_enums::EnumName;
use irox_tools::iterators::Itertools;
use irox_tools::options::MaybeInto;
use irox_tools::packetio::{Packet, PacketBuilder};
use irox_tools::read::read_exact;

use crate::{calculate_checksum, Error, MessageType};

#[derive(Debug, Default, Copy, Clone, PartialEq, EnumName)]
pub enum GNSSSystemID {
    MULTI,
    GPS,
    GLONASS,
    GALILEO,
    BEIDU,
    QZSS,
    #[default]
    UNKNOWN,
}

impl Display for GNSSSystemID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl GNSSSystemID {
    pub fn get_id(&self) -> Option<&'static str> {
        match self {
            GNSSSystemID::MULTI => Some("0"),
            GNSSSystemID::GPS => Some("1"),
            GNSSSystemID::GLONASS => Some("2"),
            GNSSSystemID::GALILEO => Some("3"),
            GNSSSystemID::BEIDU => Some("4"),
            GNSSSystemID::QZSS => Some("5"),
            GNSSSystemID::UNKNOWN => None,
        }
    }
    pub fn from_sender(value: &str) -> Self {
        if value.starts_with("$GP") {
            GNSSSystemID::GPS
        } else if value.starts_with("$BD") {
            GNSSSystemID::BEIDU
        } else if value.starts_with("$PQ") {
            GNSSSystemID::QZSS
        } else if value.starts_with("$GL") {
            GNSSSystemID::GLONASS
        } else if value.starts_with("$GA") {
            GNSSSystemID::GALILEO
        } else if value.starts_with("$GN") {
            GNSSSystemID::MULTI
        } else {
            GNSSSystemID::UNKNOWN
        }
    }
}

impl From<Option<&str>> for GNSSSystemID {
    fn from(value: Option<&str>) -> Self {
        if let Some(value) = value {
            return value
                .chars()
                .next()
                .map_or(GNSSSystemID::UNKNOWN, |f| match f {
                    '0' => GNSSSystemID::MULTI,
                    '1' => GNSSSystemID::GPS,
                    '2' => GNSSSystemID::GLONASS,
                    '3' => GNSSSystemID::GALILEO,
                    '4' => GNSSSystemID::BEIDU,
                    '5' => GNSSSystemID::QZSS,
                    _ => GNSSSystemID::UNKNOWN,
                });
        }
        GNSSSystemID::UNKNOWN
    }
}

/// Mode that the specific unit is in
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum SelectionMode {
    /// Position is manually set
    Manual,
    /// Standard mode, unit determines position autonomously
    Auto2D3D,

    #[default]
    Unknown,
}

impl From<Option<&str>> for SelectionMode {
    fn from(value: Option<&str>) -> Self {
        if let Some(value) = value {
            return value
                .chars()
                .next()
                .map_or(SelectionMode::Unknown, |f| match f {
                    'M' => SelectionMode::Manual,
                    'A' => SelectionMode::Auto2D3D,
                    _ => SelectionMode::Unknown,
                });
        }
        SelectionMode::Unknown
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct GSA {
    selection_mode: SelectionMode,
    fix_mode: GPSFixType,
    fix_sats: [u8; 12],
    dops: DOPs,
    system_id: GNSSSystemID,
}

impl Display for GSA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "SelMode[{:?}] FixMode[{:?}] FixSats{:?} DOPs[{}] System[{}]",
            self.selection_mode, self.fix_mode, self.fix_sats, self.dops, self.system_id
        ))
    }
}

impl Packet for GSA {
    type PacketType = MessageType;

    fn get_bytes(&self) -> Result<Vec<u8>, BitsError> {
        let mut buf: Vec<u8> = Vec::new();

        let mode1 = match self.selection_mode {
            SelectionMode::Manual => "M",
            SelectionMode::Auto2D3D => "A",
            SelectionMode::Unknown => "",
        };
        let mode2 = match self.fix_mode {
            GPSFixType::Unknown => "",
            GPSFixType::NoFix => "1",
            GPSFixType::TwoDim => "2",
            GPSFixType::ThreeDim => "3",
        };
        let sats = self
            .fix_sats
            .iter()
            .map(|f| format!("{f}"))
            .collect_exact_or_default(12)
            .as_slice()
            .join(",");
        let pdop = self
            .dops
            .position
            .map(|f| format!("{:01.1}", f.0))
            .unwrap_or_default();
        let hdop = self
            .dops
            .horizontal
            .map(|f| format!("{:01.1}", f.0))
            .unwrap_or_default();
        let vdop = self
            .dops
            .vertical
            .map(|f| format!("{:01.1}", f.0))
            .unwrap_or_default();
        let system_id = self
            .system_id
            .get_id()
            .map(|v| format!(",{v}"))
            .unwrap_or_default();

        buf.write_fmt(format_args!(
            "$GPGSA,{mode1},{mode2},{sats},{pdop},{hdop},{vdop}{system_id}*"
        ))?;

        let cksm = calculate_checksum(&buf);
        buf.write_fmt(format_args!("{cksm:02X}\r\n"))?;
        Ok(buf)
    }

    fn get_type(&self) -> Self::PacketType {
        MessageType::GSA
    }
}

pub struct GSABuilder;
impl PacketBuilder<GSA> for GSABuilder {
    type Error = Error;

    fn build_from<T: Bits>(&self, input: &mut T) -> Result<GSA, Self::Error> {
        let buf = input.read_all_str_lossy()?;

        let mut split = buf.split(',');
        let key = split.next();
        let selection_mode = split.next().into();
        let fix_mode = split.next().into();
        let mut sats: Vec<Option<&str>> = Vec::new();
        for _idx in 0..12 {
            sats.push(split.next());
        }
        let mut dops = DOPs::new();
        dops.position = split.next().maybe_into().maybe_into();
        dops.horizontal = split.next().maybe_into().maybe_into();
        dops.vertical = split.next().maybe_into().maybe_into();

        let mut system_id: GNSSSystemID = split.next().into();
        if system_id == GNSSSystemID::UNKNOWN {
            system_id = key.map(GNSSSystemID::from_sender).unwrap_or_default();
        }

        let fix_sats: Vec<u8> = sats
            .iter()
            .filter_map(|s| s.and_then(|s| s.parse().ok()))
            .collect_exact_or_default(12);

        let fix_sats = read_exact(&mut fix_sats.as_slice())?;

        Ok(GSA {
            fix_mode,
            selection_mode,
            dops,
            fix_sats,
            system_id,
        })
    }
}
