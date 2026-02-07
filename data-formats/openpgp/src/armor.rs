// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;
use crate::packets::*;
use alloc::collections::VecDeque;
use irox_bits::{Bits, BitsWrapper, Error, ErrorKind};
use irox_tools::base64::base64_decode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmorType {
    Message,
    Signature,
    PubKey,
    PrivKey,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArmorResult {
    pub armor_type: ArmorType,
    pub headers: Vec<(String, String)>,
    pub data: Option<Vec<u8>>,
}
pub trait Dearmor<T: Bits> {
    fn dearmor(&mut self) -> Dearmorer<'_, T>;
}
pub struct Dearmorer<'a, T: Bits> {
    inner: BitsWrapper<'a, T>,
    buf: VecDeque<u8>,
    databuf: Option<Vec<u8>>,
    headers: Vec<(String, String)>,
    armor_type: Option<ArmorType>,
    done: bool,
}
impl<'a, T: Bits> Dearmorer<'a, T> {
    pub fn new(inner: BitsWrapper<'a, T>) -> Self {
        Self {
            inner,
            buf: VecDeque::new(),
            databuf: Default::default(),
            headers: Default::default(),
            armor_type: None,
            done: false,
        }
    }
    fn set_armor_type(&mut self, armor_type: ArmorType) -> Result<(), Error> {
        if let Some(ArmorType::Message) = self.armor_type {
            if armor_type == ArmorType::Signature {
                self.armor_type = Some(ArmorType::Signature);
                return Ok(());
            }
        }
        if self.armor_type.is_some() {
            return Err(ErrorKind::AlreadyExists.into());
        }
        self.armor_type = Some(armor_type);
        Ok(())
    }
    fn try_consume_headers(&mut self) -> Result<(), Error> {
        while let Some(line) = self.inner.read_line_str()? {
            let line = line.trim();
            if line.is_empty() {
                break;
            }
            let Some(parts) = line.split_once(": ") else {
                return Err(ErrorKind::InvalidInput.into());
            };
            let mut key = parts.0.trim().to_string();
            let mut value = parts.1.trim().to_string();
            if key.to_lowercase() == "comment" {
                if let Some(v) = value.split_once(':') {
                    key = v.0.trim().to_string();
                    value = v.1.trim().to_string();
                }
            }
            self.headers.push((key, value));
        }
        Ok(())
    }

    fn fill_buf(&mut self) -> Result<usize, Error> {
        if self.done {
            return Ok(0);
        }
        while let Some(line) = self.inner.read_line_str()? {
            let trline = line.trim();
            match trline {
                MESSAGE_HEADER | SIGNED_MESSAGE_HEADER => {
                    self.set_armor_type(ArmorType::Message)?;
                    self.try_consume_headers()?;
                }
                MESSAGE_FOOTER => {
                    self.done = true;
                    let Some(ArmorType::Message) = self.armor_type else {
                        return Err(ErrorKind::InvalidInput.into());
                    };
                }
                SIG_HEADER | SIGNED_MESSAGE_SIGNATURE => {
                    self.set_armor_type(ArmorType::Signature)?;
                    self.try_consume_headers()?;
                }
                SIG_FOOTER => {
                    self.done = true;
                    let Some(ArmorType::Signature) = self.armor_type else {
                        return Err(ErrorKind::InvalidInput.into());
                    };
                }
                PUBKEY_HEADER => {
                    self.set_armor_type(ArmorType::PubKey)?;
                    self.try_consume_headers()?;
                }
                PUBKEY_FOOTER => {
                    self.done = true;
                    let Some(ArmorType::PubKey) = self.armor_type else {
                        return Err(ErrorKind::InvalidInput.into());
                    };
                }
                PRIVKEY_HEADER => {
                    self.set_armor_type(ArmorType::PrivKey)?;
                    self.try_consume_headers()?;
                }
                PRIVKEY_FOOTER => {
                    self.done = true;
                    let Some(ArmorType::PrivKey) = self.armor_type else {
                        return Err(ErrorKind::InvalidInput.into());
                    };
                }
                SIGNED_MESSAGE_FOOTER => {
                    self.done = true;
                    match self.armor_type {
                        Some(ArmorType::Message) | Some(ArmorType::Signature) => {
                            // okay
                        }
                        _ => {
                            return Err(ErrorKind::InvalidInput.into());
                        }
                    }
                }

                _ => {
                    if let Some(ArmorType::Message) = self.armor_type {
                        let buf = self.databuf.get_or_insert_default();
                        let line = line.trim_end();
                        buf.extend_from_slice(line.as_bytes());
                        buf.extend_from_slice(b"\r\n");
                        continue;
                    }
                    // base64 decode the line
                    if line.starts_with('=') || line.trim().is_empty() {
                        // checksum, skip.
                        continue;
                    }
                    return base64_decode(line.as_bytes(), &mut self.buf);
                }
            }
        }
        Ok(0)
    }
    pub fn finish(mut self) -> Result<ArmorResult, Error> {
        let Some(armor_type) = self.armor_type else {
            return Err(ErrorKind::InvalidInput.into());
        };
        if let Some(data) = &mut self.databuf {
            if data.ends_with(b"\r\n") {
                let _ = data.pop();
                let _ = data.pop();
            } else if data.ends_with(b"\n") {
                let _ = data.pop();
            }
        }
        Ok(ArmorResult {
            armor_type,
            headers: self.headers,
            data: self.databuf,
        })
    }
}
impl<T: Bits> Bits for Dearmorer<'_, T> {
    fn next_u8(&mut self) -> Result<Option<u8>, Error> {
        if self.buf.is_empty() {
            self.fill_buf()?;
            if self.buf.is_empty() {
                return Ok(None);
            }
        }
        Ok(self.buf.pop_front())
    }
}

pub trait Armor<T> {
    fn armor(&mut self) -> BitsWrapper<'_, T>;
}

impl<T: Bits> Dearmor<T> for T {
    fn dearmor(&mut self) -> Dearmorer<'_, T> {
        Dearmorer::new(BitsWrapper::Borrowed(self))
    }
}
