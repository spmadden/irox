// SPDX-License-Identifier: MIT
// Copyright 2024 IROX Contributors
//

use crate::tags::{get_geokey_directory_tags, GEO_KEY_DIRECTORY, KNOWN_TAG_TYPES};
use crate::tiff::geo::GeoKeyDirectory;
use crate::{ImageError, ImageErrorType};
use core::cmp::Ordering;
use core::fmt::Debug;
use irox_bits::{Bits, BitsError, ByteOrder, Seek, SeekFrom};
use irox_log::log::{debug, warn};
use std::collections::BTreeMap;

pub mod geo;
pub mod tags;

pub struct TiffImage {
    ifd: BTreeMap<u16, TiffTag>,
}
impl TiffImage {
    pub fn ifd(&self) -> &BTreeMap<u16, TiffTag> {
        &self.ifd
    }
}
impl Debug for TiffImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = f.debug_struct("TiffImage");
        for v in self.ifd.values() {
            let name = if let Some(tag) = &v.identified_tag {
                format!("{}({})", tag.name, tag.tag_id)
            } else {
                format!("UNK({})", v.tag)
            };
            if let TiffTagValue::ParsedAscii(val) = &v.value {
                str.field(&name, val);
            } else {
                str.field(&name, &format!("{:?}", v.value));
            }
        }
        str.finish()
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TiffTagFormat {
    /// u8
    Byte = 1,
    /// i8 >= 0 (null terminated count)
    Ascii = 2,
    /// u16
    Short = 3,
    /// u32
    Long = 4,
    /// u32/u32 = u64
    Rational = 5,
    /// i8
    SByte = 6,
    /// u8?
    Undefined = 7,
    /// i16,
    SShort = 8,
    /// i32,
    SLong = 9,
    /// i32/i32 = u64
    SRational = 10,
    /// f32
    Float = 11,
    /// f64
    Double = 12,
}
impl TiffTagFormat {
    pub fn get_size(&self, count: u32) -> u32 {
        #[allow(clippy::match_same_arms)]
        let size = match self {
            TiffTagFormat::Byte => 1,
            TiffTagFormat::Ascii => {
                return count + 1;
            }
            TiffTagFormat::Short => 2,
            TiffTagFormat::Long => 4,
            TiffTagFormat::Rational => 8,
            TiffTagFormat::SByte => 1,
            TiffTagFormat::Undefined => 1,
            TiffTagFormat::SShort => 2,
            TiffTagFormat::SLong => 4,
            TiffTagFormat::SRational => 8,
            TiffTagFormat::Float => 4,
            TiffTagFormat::Double => 8,
        };
        size * count
    }
}
impl TryFrom<u16> for TiffTagFormat {
    type Error = ImageError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(TiffTagFormat::Byte),
            2 => Ok(TiffTagFormat::Ascii),
            3 => Ok(TiffTagFormat::Short),
            4 => Ok(TiffTagFormat::Long),
            5 => Ok(TiffTagFormat::Rational),
            6 => Ok(TiffTagFormat::SByte),
            7 => Ok(TiffTagFormat::Undefined),
            8 => Ok(TiffTagFormat::SShort),
            9 => Ok(TiffTagFormat::SLong),
            10 => Ok(TiffTagFormat::SRational),
            11 => Ok(TiffTagFormat::Float),
            12 => Ok(TiffTagFormat::Double),
            ty => Err(ImageError::bad_type(ty)),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TiffTagValue {
    Offset(u32),
    ParsedByte(Vec<u8>),
    ParsedAscii(String),
    ParsedShort(u16),
    ParsedShorts(Vec<u16>),
    ParsedLong(u32),
    ParsedLongs(Vec<u32>),
    ParsedRational(Vec<(u32, u32)>),
    ParsedSByte(Vec<i8>),
    ParsedSShort(Vec<i16>),
    ParsedSLong(Vec<i32>),
    ParsedSRational(Vec<(i32, i32)>),
    ParsedFloat(f32),
    ParsedFloats(Vec<f32>),
    ParsedDouble(f64),
    ParsedDoubles(Vec<f64>),
}

#[derive(Debug, Copy, Clone)]
pub struct TiffTagType {
    name: &'static str,
    tag_id: u16,
    format: TiffTagFormat,
    usual_value_count: u32,
}
impl PartialEq for TiffTagType {
    fn eq(&self, other: &Self) -> bool {
        self.tag_id == other.tag_id
    }
}
impl Eq for TiffTagType {}
impl PartialOrd for TiffTagType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.tag_id.cmp(&other.tag_id))
    }
}
impl Ord for TiffTagType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.tag_id.cmp(&other.tag_id)
    }
}
impl TiffTagType {
    pub const fn new(
        name: &'static str,
        tag_id: u16,
        format: TiffTagFormat,
        usual_value_count: u32,
    ) -> Self {
        Self {
            name,
            tag_id,
            format,
            usual_value_count,
        }
    }
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn tag_id(&self) -> u16 {
        self.tag_id
    }
    pub fn format(&self) -> TiffTagFormat {
        self.format
    }
    pub fn usual_value_count(&self) -> u32 {
        self.usual_value_count
    }
}

#[derive(Debug, Clone)]
pub struct TiffTag {
    tag: u16,
    identified_tag: Option<TiffTagType>,
    field_type: TiffTagFormat,
    value_count: u32,
    value: TiffTagValue,
}

impl TiffTag {
    pub fn tag(&self) -> u16 {
        self.tag
    }
    pub fn identified_tag(&self) -> &Option<TiffTagType> {
        &self.identified_tag
    }
    pub fn field_type(&self) -> &TiffTagFormat {
        &self.field_type
    }
    pub fn value_count(&self) -> u32 {
        self.value_count
    }
    pub fn value(&self) -> &TiffTagValue {
        &self.value
    }
    pub fn read<T: Bits>(source: &mut T, order: ByteOrder) -> Result<TiffTag, ImageError> {
        let tag = source.read_u16(order)?;
        let field_type = source.read_u16(order)?.try_into()?;
        let value_count = source.read_u32(order)?;
        let value = TiffTagValue::Offset(source.read_u32(order)?);

        let identified_tag = KNOWN_TAG_TYPES.iter().find(|v| v.tag_id == tag).copied();

        Ok(TiffTag {
            tag,
            identified_tag,
            field_type,
            value_count,
            value,
        })
    }
    pub fn try_resolve_value<T: Bits + Seek>(
        &mut self,
        source: &mut T,
        order: ByteOrder,
    ) -> Result<(), BitsError> {
        let size = self.field_type.get_size(self.value_count);
        let TiffTagValue::Offset(offset) = self.value else {
            return Ok(());
        };
        if size <= 4 {
            #[allow(clippy::match_same_arms)]
            match self.field_type {
                TiffTagFormat::Byte => {
                    todo!()
                }
                TiffTagFormat::Ascii => {
                    todo!()
                }
                TiffTagFormat::Short => {
                    self.value = TiffTagValue::ParsedShort(offset as u16);
                }
                TiffTagFormat::Long => {
                    self.value = TiffTagValue::ParsedLong(offset);
                }
                TiffTagFormat::Rational => {
                    todo!()
                }
                TiffTagFormat::SByte => {
                    todo!()
                }
                TiffTagFormat::Undefined => {
                    todo!()
                }
                TiffTagFormat::SShort => {
                    todo!()
                }
                TiffTagFormat::SLong => {
                    todo!()
                }
                TiffTagFormat::SRational => {
                    todo!()
                }
                TiffTagFormat::Float => {
                    todo!()
                }
                TiffTagFormat::Double => {
                    todo!()
                }
            }
        } else {
            source.seek(SeekFrom::Start(offset as u64))?;
            if let TiffTagFormat::Ascii = self.field_type {
                let size = (self.value_count as usize).saturating_sub(1);
                let s = source.read_str_sized_lossy(size)?;
                self.value = TiffTagValue::ParsedAscii(s);
                return Ok(());
            }
            match self.field_type {
                TiffTagFormat::Short => {
                    let mut out = Vec::new();
                    for _ in 0..self.value_count {
                        out.push(source.read_u16(order)?);
                    }
                    self.value = TiffTagValue::ParsedShorts(out);
                }
                TiffTagFormat::Long => {
                    let mut out = Vec::new();
                    for _ in 0..self.value_count {
                        out.push(source.read_u32(order)?);
                    }
                    self.value = TiffTagValue::ParsedLongs(out);
                }
                TiffTagFormat::Float => {
                    let mut out = Vec::new();
                    for _ in 0..self.value_count {
                        out.push(source.read_f32(order)?);
                    }
                    self.value = TiffTagValue::ParsedFloats(out);
                }
                TiffTagFormat::Double => {
                    let mut out = Vec::new();
                    for _ in 0..self.value_count {
                        out.push(source.read_f64(order)?);
                    }
                    self.value = TiffTagValue::ParsedDoubles(out);
                }
                TiffTagFormat::Rational => {
                    let mut out = Vec::new();
                    for _ in 0..self.value_count {
                        let v = (source.read_u32(order)?, source.read_u32(order)?);
                        out.push(v);
                    }
                    if out.len() == 1 {
                        let v = out.pop().unwrap_or_default();
                        if v.1 == 1 {
                            self.value = TiffTagValue::ParsedLong(v.0);
                            return Ok(());
                        }
                    }
                    self.value = TiffTagValue::ParsedRational(out);
                }
                _ => {
                    for _ in 0..self.value_count {
                        todo!("{:?}", self.field_type)
                    }
                }
            }
        };

        Ok(())
    }
}

pub struct TiffImageReader;

impl TiffImageReader {
    pub fn read<T: Seek + Bits>(mut source: T) -> Result<TiffImage, ImageError> {
        let order = source.read_be_u16()?;
        let order = match order {
            0x4949 => ByteOrder::LittleEndian,
            0x4D4D => ByteOrder::BigEndian,
            _ => return ImageErrorType::BadByteOrder.into(),
        };
        let magic = source.read_u16(order)?;
        if magic != 42 {
            return ImageErrorType::BadMagic.into();
        }
        let ifd_offset = source.read_u32(order)?;
        source.seek(SeekFrom::Start(ifd_offset as u64))?;

        let ifd_count = source.read_u16(order)?;
        let mut ifd = BTreeMap::new();
        for _ in 0..ifd_count {
            let tag = TiffTag::read(&mut source, order)?;
            ifd.insert(tag.tag, tag);
        }

        for ifd in &mut ifd.values_mut() {
            ifd.try_resolve_value(&mut source, order)?;
        }
        if let Some(gkd) = ifd.get(&GEO_KEY_DIRECTORY.tag_id) {
            if let TiffTagValue::ParsedShorts(shorts) = &gkd.value {
                let dir = GeoKeyDirectory::parse_from(shorts)?;
                for key in &dir.keys {
                    let Some(ent) = get_geokey_directory_tags().get(&key.id) else {
                        warn!("Cannot find ID {} in known geokey tags", key.id);
                        continue;
                    };
                    if key.location == 0 {
                        // directly in value
                        let value = TiffTagValue::ParsedShort(key.value_offset);
                        let tag = key.id;
                        let field_type = ent.format;
                        let value_count = key.count as u32;
                        let identified_tag = Some(*ent);
                        ifd.insert(
                            key.id,
                            TiffTag {
                                value,
                                tag,
                                field_type,
                                value_count,
                                identified_tag,
                            },
                        );
                    } else {
                        // find some other tag.
                        let Some(deref) = ifd.get(&key.location) else {
                            warn!("Cannot find location {} in known ifd tags", key.location);
                            continue;
                        };
                        match deref.field_type {
                            TiffTagFormat::Ascii => {
                                let TiffTagValue::ParsedAscii(val) = &deref.value else {
                                    warn!(
                                        "Expected a parsed ascii value, but was: {:#?}",
                                        &deref.value
                                    );
                                    continue;
                                };
                                let start = key.value_offset as usize;
                                let end = start + key.count as usize - 1;
                                let val = val.get(start..end).unwrap_or_default();
                                ifd.insert(
                                    key.id,
                                    TiffTag {
                                        field_type: TiffTagFormat::Ascii,
                                        tag: key.id,
                                        value_count: 1,
                                        identified_tag: Some(*ent),
                                        value: TiffTagValue::ParsedAscii(val.to_string()),
                                    },
                                );
                            }
                            TiffTagFormat::Double => {
                                debug!("{:#?} {:#?}", key, ent);
                                let TiffTagValue::ParsedDoubles(val) = &deref.value else {
                                    warn!(
                                        "Expected a parsed double values, but was: {:#?}",
                                        &deref.value
                                    );
                                    continue;
                                };
                                let start = key.value_offset as usize;
                                if key.count == 1 {
                                    let val = val.get(start).copied().unwrap_or_default();
                                    ifd.insert(
                                        key.id,
                                        TiffTag {
                                            field_type: TiffTagFormat::Double,
                                            tag: key.id,
                                            value_count: 1,
                                            identified_tag: Some(*ent),
                                            value: TiffTagValue::ParsedDouble(val),
                                        },
                                    );
                                    continue;
                                }

                                todo!()
                            }
                            _ => {
                                warn!("Unsupported GKD field type: {:?}", deref.field_type);
                            }
                        }
                    }
                }
            };
        }
        Ok(TiffImage { ifd })
    }
}

#[cfg(test)]
mod test {
    use crate::tiff::TiffImageReader;
    use crate::ImageError;
    use irox_log::log::Level;
    use std::fs::OpenOptions;

    #[test]
    pub fn test() -> Result<(), ImageError> {
        irox_log::init_console_level(Level::Debug);
        let path = "E:/charts/FAA_Charts/New_York/New York SEC.tif";
        let file = OpenOptions::new()
            .read(true)
            .create(false)
            .open(path)
            .unwrap();
        let img = TiffImageReader::read(file)?;
        println!("{:#?}", img);

        Ok(())
    }
}
