// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::{UBXClass, UBXRawMessage};
use core::fmt::{Debug, Display, Formatter};
use irox_bits::{Bits, BitsError, BitsErrorKind, Error};
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
pub enum UBXCfg {
    ANT = 0x13,
    BATCH = 0x93,
    CFG = 0x09,
    DAT = 0x06,
    DGNSS = 0x70,
    DOSC = 0x61,
    ESFALG = 0x56,
    ESFA = 0x4C,
    ESFG = 0x4D,
    WSFWT = 0x82,
    ESRC = 0x60,
    GEOFENCE = 0x69,
    GNSS = 0x3E,
    HNR = 0x5C,
    INF = 0x02,
    ITFM = 0x39,
    LOGFILTER = 0x47,
    MSG = 0x01,
    NAV5 = 0x24,
    NAVX5 = 0x23,
    NMEA = 0x17,
    ODO = 0x1E,
    OTP = 0x41,
    PM2 = 0x3B,
    PMS = 0x86,
    PRT = 0x00,
    PWR = 0x57,
    RATE = 0x08,
    RINV = 0x34,
    RST = 0x04,
    RXM = 0x11,
    SBAS = 0x16,
    SENIF = 0x88,
    SLAS = 0x8D,
    SMGR = 0x62,
    SPT = 0x64,
    TMODE2 = 0x3D,
    TMODE3 = 0x71,
    TP5 = 0x31,
    TXSLOT = 53,
    USB = 0x1B,
    VALDEL = 0x8C,
    VALGET = 0x8B,
    VALSET = 0x8A,
}

impl UBXCfg {
    pub fn try_parse_payload(&self, pld: &[u8]) -> Result<UBXCfgPayload, Error> {
        Ok(match self {
            UBXCfg::VALGET => UBXCfgPayload::ValGet(UBXValGet::parse_from(pld)?),
            UBXCfg::OTP => UBXCfgPayload::OTP(UBXConfigOTP::parse_from(pld)?),
            _ => UBXCfgPayload::Unknown {
                id: *self as u8,
                payload: pld.to_vec().into_boxed_slice(),
            },
        })
    }
    pub fn try_parse(id: u8, pld: &[u8]) -> Result<UBXCfgPayload, Error> {
        let Ok(msg) = UBXCfg::try_from(id) else {
            return Ok(UBXCfgPayload::Unknown {
                id,
                payload: pld.to_vec().into_boxed_slice(),
            });
        };
        msg.try_parse_payload(pld)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UBXCfgPayload {
    ValGet(UBXValGet),
    OTP(UBXConfigOTP),
    Unknown { id: u8, payload: Box<[u8]> },
}
impl Display for UBXCfgPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}
impl TryFrom<UBXRawMessage> for UBXCfgPayload {
    type Error = UBXRawMessage;

    fn try_from(value: UBXRawMessage) -> Result<Self, Self::Error> {
        if value.class != UBXClass::CFG as u8 {
            return Err(value);
        }
        let Ok(id) = UBXCfg::try_from(value.id) else {
            return Ok(UBXCfgPayload::Unknown {
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
pub enum UBXCfgLayer {
    /// Local RAM
    RAM = 0x0,
    /// Battery-backed RAM
    BBR = 0x1,
    /// Local-onboard NV Flash
    FLASH = 0x02,
    /// Default for the particular chip
    DEFAULT = 0x07,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UBXValGet {
    pub version: u8,
    pub layer: UBXCfgLayer,
    pub cfg_data: Vec<ConfigData>,
}
impl UBXValGet {
    pub fn parse_from(mut source: &[u8]) -> Result<UBXValGet, Error> {
        let version = source.read_u8()?;
        let layer = source.read_u8()?;
        let Ok(layer) = UBXCfgLayer::try_from(layer) else {
            return Err(BitsError::new(
                BitsErrorKind::Unsupported,
                "unsupported UBXCfgLayer encountered",
            ));
        };
        let _position = source.read_le_u16()?;
        let cfg_data = ConfigData::read_config_items(source)?;
        Ok(UBXValGet {
            version,
            layer,
            cfg_data,
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ConfigData {
    pub key: ConfigKey,
    pub raw_value: ConfigRawValue,
}
impl ConfigData {
    pub fn read_config_items(mut source: &[u8]) -> Result<Vec<Self>, Error> {
        let mut cfg_data = Vec::<ConfigData>::new();
        while !source.is_empty() {
            let key: ConfigKey = source.read_le_u32()?.into();
            let raw_value: ConfigRawValue = key.read_next_value(&mut source)?;
            cfg_data.push(ConfigData { key, raw_value });
        }
        Ok(cfg_data)
    }
}
impl Debug for ConfigData {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "ConfigData({})={:?}", self.key, self.raw_value)
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ConfigKey {
    pub size: u8,
    pub group: u8,
    pub id: u16,
}
impl Display for ConfigKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let grp = ConfigGroup::try_from(self.group);
        let (grp, id) = match grp {
            Ok(group) => (group.name().to_string(), group.item_from_u16(self.id)),
            Err(()) => (
                format!("UNK(0x{:0X})", self.group),
                format!("UNK(0x{:0X})", self.id),
            ),
        };

        write!(f, "CFG-{grp}-{id}")
    }
}
impl From<u32> for ConfigKey {
    fn from(value: u32) -> Self {
        let id = (value & 0xFFF) as u16;
        let group = ((value & 0x0FF0000) >> 16) as u8;
        let size = ((value & 0x70000000) >> 28) as u8;
        Self { size, group, id }
    }
}
impl ConfigKey {
    pub fn as_raw(&self) -> u32 {
        (self.id as u32) | ((self.group as u32) << 16) | ((self.size as u32) << 28)
    }
    pub fn read_next_value<T: Bits>(&self, source: &mut T) -> Result<ConfigRawValue, Error> {
        match self.size {
            0x01 => {
                let val = source.read_u8()?;
                Ok(ConfigRawValue::Bool(val != 0))
            }
            0x02 => {
                let val = source.read_u8()?;
                Ok(ConfigRawValue::OneByte(val))
            }
            0x03 => Ok(ConfigRawValue::TwoBytes(source.read_exact::<2>()?)),
            0x04 => Ok(ConfigRawValue::FourBytes(source.read_exact::<4>()?)),
            0x05 => Ok(ConfigRawValue::EightBytes(source.read_exact::<8>()?)),
            _ => Err(BitsError::new(
                BitsErrorKind::Unsupported,
                "unsupported UBXConfigKey size",
            )),
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ConfigRawValue {
    Bool(bool),
    OneByte(u8),
    TwoBytes([u8; 2]),
    FourBytes([u8; 4]),
    EightBytes([u8; 8]),
}
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
pub enum ConfigGroup {
    ANA = 0x23,
    BATCH = 0x26,
    BDS = 0x34,
    GAL = 0x35,
    GEOFENCE = 0x24,
    HW = 0xA3,
    I2C = 0x51,
    I2CINPROT = 0x71,
    I2COUTPROT = 0x72,
    INFMSG = 0x92,
    ITFM = 0x41,
    LOGFILTER = 0xDE,
    MOT = 0x25,
    MSGOUT = 0x91,
    NAV2 = 0x17,
    NAVHPG = 0x14,
    NAVMASK = 0x18,
    NAVSPG = 0x11,
    NMEA = 0x93,
    ODO = 0x22,
    PM = 0xD0,
    PMP = 0xB1,
    QZSS = 0x37,
    RATE = 0x21,
    RINV = 0xC7,
    RTCM = 0x09,
    SBAS = 0x36,
    SEC = 0xF6,
    SFCORE = 0x08,
    SFIMU = 0x06,
    SFODO = 0x07,
    SIGNAL = 0x31,
    SPARTN = 0xA7,
    SPI = 0x64,
    SPIINPROT = 0x79,
    SPIOUTPROT = 0x7A,
    TMODE = 0x03,
    TP = 0x05,
    TXREADY = 0xA2,
    UART1 = 0x52,
    UART1INPROT = 0x73,
    UART1OUTPROT = 0x74,
    UART2 = 0x53,
    UART2INPROT = 0x75,
    UART2OUTPROT = 0x76,
    UNITTEST = 0xFE,
    USB = 0x65,
    USBINPROT = 0x77,
    USBOUTPROT = 0x78,
}
impl ConfigGroup {
    pub fn name_from_u8(id: u8) -> String {
        match ConfigGroup::try_from(id) {
            Ok(group) => group.name().to_string(),
            Err(()) => format!("UNK(0x{id:0X})"),
        }
    }
    pub fn item_from_u16(&self, item: u16) -> String {
        match self {
            ConfigGroup::TMODE => ConfigTmode::name_from_u16(item).to_string(),
            _ => format!("UNK(0x{item:0X})"),
        }
    }
}
#[allow(non_camel_case_types)]
#[repr(u16)]
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
pub enum ConfigTmode {
    MODE = 0x0001,
    POS_TYPE = 0x0002,
    ECEF_X = 0x0003,
    ECEF_Y = 0x0004,
    ECEF_Z = 0x0005,
    ECEF_X_HP = 0x0006,
    ECEF_Y_HP = 0x0007,
    ECEF_Z_HP = 0x0008,
    LAT = 0x0009,
    LON = 0x000A,
    HEIGHT = 0x000B,
    LAT_HP = 0x000C,
    LON_HP = 0x000D,
    HEIGHT_HP = 0x000E,
    FIXED_POS_ACC = 0x000F,
    SVIN_MIN_DUR = 0x0010,
    SVIN_ACC_LIMIT = 0x0011,
}
impl ConfigTmode {
    pub fn name_from_u16(id: u16) -> &'static str {
        match ConfigTmode::try_from(id) {
            Ok(mode) => mode.name(),
            Err(()) => "UNK",
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UBXConfigOTP {
    pub header: [u8; 12],
    pub cfg_data: Vec<ConfigData>,
}
impl UBXConfigOTP {
    pub fn parse_from(mut source: &[u8]) -> Result<UBXConfigOTP, Error> {
        let header = source.read_exact::<12>()?;
        let cfg_data = ConfigData::read_config_items(source)?;
        Ok(UBXConfigOTP { header, cfg_data })
    }
}

#[cfg(test)]
mod test {
    use crate::UBXParser;
    use irox_tools::hex;
    use irox_tools::packetio::PacketBuilder;

    #[test]
    pub fn test_key1() {
        let hex = hex!("B562064110000300041F545E79BF28EF1205FDFFFFFF8F0DB56206411C000401A410BD34F91228EF12050500A44000B0710B0A00A44000D8B805DEAE");
        let mut hex = hex.as_slice();
        let res = UBXParser::default().build_from(&mut hex).unwrap();
        println!("{res:#?}");
    }
}
