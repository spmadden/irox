// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use irox_bits::{
    Bits, Error, MutBits, ReadFromBEBits, ReadFromLEBits, WriteToBEBits, WriteToLEBits,
};

///
/// Wrapper enum around the common string representations
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum StrWrapper<'a> {
    Shared(Arc<String>),
    Owned(String),
    Borrowed(&'a str),
}
impl Default for StrWrapper<'_> {
    fn default() -> Self {
        StrWrapper::Owned(Default::default())
    }
}
impl AsRef<str> for StrWrapper<'_> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl AsMut<str> for StrWrapper<'_> {
    fn as_mut(&mut self) -> &mut str {
        match self {
            StrWrapper::Owned(o) => o,
            StrWrapper::Shared(s) => {
                *self = StrWrapper::Owned(s.to_string());
                self.as_mut()
            }
            StrWrapper::Borrowed(b) => {
                *self = StrWrapper::Owned((*b).to_string());
                self.as_mut()
            }
        }
    }
}
impl<'a> StrWrapper<'a> {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            StrWrapper::Shared(s) => s.as_str(),
            StrWrapper::Owned(s) => s.as_str(),
            StrWrapper::Borrowed(s) => s,
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.as_str().len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.as_str().is_empty()
    }

    #[must_use]
    pub fn to_owned(&self) -> Self {
        match self {
            StrWrapper::Shared(a) => a.as_ref().clone().into(),
            StrWrapper::Owned(o) => o.clone().into(),
            StrWrapper::Borrowed(s) => (*s).to_string().into(),
        }
    }

    #[must_use]
    pub fn to_shared(&self) -> Self {
        match self {
            StrWrapper::Shared(a) => a.clone().into(),
            StrWrapper::Owned(o) => Arc::new(o.clone()).into(),
            StrWrapper::Borrowed(s) => Arc::new((*s).to_string()).into(),
        }
    }

    pub fn make_shared(&mut self) {
        match self {
            StrWrapper::Shared(_) => {}
            StrWrapper::Owned(o) => {
                *self = StrWrapper::Shared(Arc::new(o.clone()));
            }
            StrWrapper::Borrowed(b) => {
                *self = StrWrapper::Shared(Arc::new((*b).to_string()));
            }
        }
    }
}
impl<'a> From<&'a str> for StrWrapper<'a> {
    fn from(s: &'a str) -> Self {
        StrWrapper::Borrowed(s)
    }
}
impl<'a> From<String> for StrWrapper<'a> {
    fn from(s: String) -> Self {
        StrWrapper::Owned(s)
    }
}
impl<'a> From<Arc<String>> for StrWrapper<'a> {
    fn from(s: Arc<String>) -> Self {
        StrWrapper::Shared(s)
    }
}
impl<'a> From<&Arc<String>> for StrWrapper<'a> {
    fn from(s: &Arc<String>) -> Self {
        StrWrapper::Shared(s.clone())
    }
}

impl<'a> WriteToBEBits for StrWrapper<'a> {
    fn write_be_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        self.as_str().write_be_to(bits)
    }
}
impl<'a> ReadFromBEBits for StrWrapper<'a> {
    fn read_from_be_bits<T: Bits>(inp: &mut T) -> Result<Self, Error> {
        ReadFromBEBits::read_from_be_bits(inp).map(StrWrapper::Owned)
    }
}
impl<'a> WriteToLEBits for StrWrapper<'a> {
    fn write_le_to<T: MutBits + ?Sized>(&self, bits: &mut T) -> Result<usize, Error> {
        self.as_str().write_le_to(bits)
    }
}
impl<'a> ReadFromLEBits for StrWrapper<'a> {
    fn read_from_le_bits<T: Bits>(inp: &mut T) -> Result<Self, Error> {
        ReadFromLEBits::read_from_le_bits(inp).map(StrWrapper::Owned)
    }
}
