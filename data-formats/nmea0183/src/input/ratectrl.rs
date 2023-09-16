// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors

use irox_tools::packetio::Packet;

use crate::MessageType;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ControlCommand {
    SetRate,
    QueryOnce,
    ABPOn,
    ABPOff,
    ReverseEEOn,
    ReverseEEOff,
    NavOn5hz,
    NavOff5hz,
    SBASRangingOn,
    SBASRangingOff,
    FTSOn,
    FTSOff,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RateControlRF103 {
    pub msg: MessageType,
    pub command: ControlCommand,
    pub rate_seconds: u8,
    pub cksum_enable: bool,
}

impl RateControlRF103 {
    pub fn set_rate(msg: MessageType, rate_seconds: u8) -> RateControlRF103 {
        RateControlRF103 {
            msg,
            rate_seconds,
            command: ControlCommand::SetRate,
            cksum_enable: true,
        }
    }
    pub fn enable_abp(enabled: bool) -> RateControlRF103 {
        let command = match enabled {
            true => ControlCommand::ABPOn,
            false => ControlCommand::ABPOff,
        };
        RateControlRF103 {
            msg: MessageType::GGA,
            rate_seconds: 0,
            cksum_enable: true,
            command,
        }
    }
    pub fn enable_reverseee(enabled: bool) -> RateControlRF103 {
        let command = match enabled {
            true => ControlCommand::ReverseEEOn,
            false => ControlCommand::ReverseEEOff,
        };
        RateControlRF103 {
            msg: MessageType::GGA,
            rate_seconds: 0,
            cksum_enable: true,
            command,
        }
    }
    pub fn enable_5hz_nav(enabled: bool) -> RateControlRF103 {
        let command = match enabled {
            true => ControlCommand::NavOn5hz,
            false => ControlCommand::NavOff5hz,
        };
        RateControlRF103 {
            msg: MessageType::GGA,
            rate_seconds: 0,
            cksum_enable: true,
            command,
        }
    }
    pub fn enable_sbas_ranging(enabled: bool) -> RateControlRF103 {
        let command = match enabled {
            true => ControlCommand::SBASRangingOn,
            false => ControlCommand::SBASRangingOff,
        };
        RateControlRF103 {
            msg: MessageType::GGA,
            rate_seconds: 0,
            cksum_enable: true,
            command,
        }
    }
    pub fn enable_fts(enabled: bool) -> RateControlRF103 {
        let command = match enabled {
            true => ControlCommand::FTSOn,
            false => ControlCommand::FTSOff,
        };
        RateControlRF103 {
            msg: MessageType::GGA,
            rate_seconds: 0,
            cksum_enable: true,
            command,
        }
    }
}

impl Packet for RateControlRF103 {
    type PacketType = MessageType;

    fn get_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let msg = match self.msg {
            MessageType::GGA => 0,
            MessageType::GLL => 1,
            MessageType::GSA => 2,
            MessageType::GSV => 3,
            MessageType::RMC => 4,
            MessageType::VTG => 5,
            MessageType::MSS => 6,
            MessageType::ZDA => 8,
            _ => return Err(std::io::ErrorKind::InvalidInput.into()),
        };
        let cmd = match self.command {
            ControlCommand::SetRate => 0,
            ControlCommand::QueryOnce => 1,
            ControlCommand::ABPOn => 2,
            ControlCommand::ABPOff => 3,
            ControlCommand::ReverseEEOn => 4,
            ControlCommand::ReverseEEOff => 5,
            ControlCommand::NavOn5hz => 6,
            ControlCommand::NavOff5hz => 7,
            ControlCommand::SBASRangingOn => 8,
            ControlCommand::SBASRangingOff => 9,
            ControlCommand::FTSOn => 10,
            ControlCommand::FTSOff => 11,
        };
        let cksum = match self.cksum_enable {
            true => 1,
            false => 0,
        };
        let out = format!(
            "$PSRF103,{:02},{:02},{:02},{:02}*",
            msg, cmd, self.rate_seconds, cksum
        );
        let cksum = crate::calculate_checksum(&out);
        Ok(Vec::from(format!("{out}{cksum:02X}\r\n")))
    }

    fn get_type(&self) -> Self::PacketType {
        MessageType::SRF103
    }
}

#[cfg(test)]
mod test {
    use irox_tools::packetio::Packet;

    use crate::input::ratectrl::{ControlCommand, RateControlRF103};
    use crate::MessageType;

    #[test]
    fn test() {
        let exp = "$PSRF103,00,01,00,01*25\r\n";
        let pkt = RateControlRF103 {
            msg: MessageType::GGA,
            command: ControlCommand::QueryOnce,
            rate_seconds: 0,
            cksum_enable: true,
        };

        let buf = pkt.get_bytes().unwrap();
        assert_eq!(buf, exp.as_bytes());
    }
}
