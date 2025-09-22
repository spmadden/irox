// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::{UBXClass, UBXMessage};
use irox_bits::{Bits, ErrorKind};
use irox_tools::packetio::PacketBuilder;

pub struct DumpTxtFileParser;

impl PacketBuilder<Option<UBXMessage>> for DumpTxtFileParser {
    type Error = irox_bits::Error;

    fn build_from<T: Bits>(&self, source: &mut T) -> Result<Option<UBXMessage>, Self::Error> {
        // format: one per line, "msg - text bytes"
        let Some(line) = source.read_line_str_lossy()? else {
            return Ok(None);
        };

        let Some((msg, data)) = line.split_once(" - ") else {
            return Err(ErrorKind::InvalidData.into());
        };
        let Some((msgtype, msgname)) = msg.split_once("-") else {
            return Err(ErrorKind::InvalidData.into());
        };
        let Ok(class) = UBXClass::try_from(msgtype) else {
            return Err(ErrorKind::AddrNotAvailable.into());
        };
        let Some(id) = class.try_get_id(msgname) else {
            return Err(ErrorKind::FormatError.into());
        };
        let chars = data
            .chars()
            .filter_map(|v| if v == ' ' { None } else { Some(v as u8) })
            .collect::<Vec<_>>();
        let chars = chars.as_slice();
        let s = String::from_utf8_lossy(chars);
        let bytes = irox_tools::hex::from_hex_str(&s)?;
        let bytes = bytes.as_ref();
        let (_skip, bytes) = bytes.split_at(4);
        let msg = class.try_parse_payload(id, bytes.as_ref())?;

        Ok(Some(UBXMessage {
            class: class as u8,
            id,
            payload: msg,
            checksum: 0,
        }))
    }
}
