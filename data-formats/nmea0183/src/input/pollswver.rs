// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use crate::{MessageType, Packet};
use irox_bits::Error;

pub struct PollSWVersion;

impl Packet for PollSWVersion {
    type PacketType = MessageType;

    fn get_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(Vec::from("$PSRF125*21\r\n"))
    }

    fn get_type(&self) -> Self::PacketType {
        MessageType::SRF125
    }
}

#[cfg(test)]
mod test {
    use irox_tools::packetio::Packet;

    use crate::input::pollswver::PollSWVersion;

    #[test]
    pub fn test() {
        let byte = PollSWVersion.get_bytes().unwrap();
        assert_eq!("$PSRF125*21\r\n".as_bytes(), byte)
    }
}
