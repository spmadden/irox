// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use std::fmt::{Display, Formatter};
use std::io::{Read, Write};

use irox_carto::gps::{DOPs, GPSFixType};
use irox_tools::iterators::Itertools;
use irox_tools::options::MaybeInto;
use irox_tools::packetio::{Packet, PacketBuilder};
use irox_tools::read::read_exact;

use crate::{calculate_checksum, Error, MessageType};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum SelectionMode {
    Manual,
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
}

impl Display for GSA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "SelMode[{:?}] FixMode[{:?}] FixSats{:?} DOPs[{}]",
            self.selection_mode, self.fix_mode, self.fix_sats, self.dops
        ))
    }
}

impl Packet for GSA {
    type PacketType = MessageType;

    fn get_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
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

        buf.write_fmt(format_args!(
            "$GPGSA,{mode1},{mode2},{sats},{pdop},{hdop},{vdop}*"
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

    fn build_from<T: Read>(&self, input: &mut T) -> Result<GSA, Self::Error> {
        let mut buf = String::new();
        let _read = input.read_to_string(&mut buf)?;

        let mut split = buf.split(',');
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
        })
    }
}
